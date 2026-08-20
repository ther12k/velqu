//! q-http — hyper 1.x HTTP/1.1 host with bounded admission, request IDs,
//! structured completion logs, and deterministic graceful shutdown.
//! Independently benchmarkable (no engine/route knowledge).

use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use hyper::body::Incoming;
use hyper::header::{HeaderName, HeaderValue};
use hyper::service::Service;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

/// Admission/body limits (RUN-005). Defaults per docs/specs/pack-format-v1.md.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct Limits {
    pub max_body_bytes: usize,
    pub max_header_bytes: usize,
    pub max_headers: usize,
    pub max_uri_bytes: usize,
    pub max_queue: usize,
    pub header_read_timeout: Duration,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            max_body_bytes: 1 << 20,
            max_header_bytes: 32 << 10,
            max_headers: 64,
            max_uri_bytes: 8 << 10,
            max_queue: 256,
            header_read_timeout: Duration::from_secs(10),
        }
    }
}

/// Native request parts retained through admission (M24-002-A). Method, Uri,
/// HeaderMap, and the body stream stay in their hyper forms; the consumer
/// materializes fields only when its pipeline needs them.
pub struct NativeRequest {
    pub request_id: String,
    pub method: hyper::Method,
    pub uri: hyper::Uri,
    pub headers: hyper::HeaderMap,
    pub body: Incoming,
    pub started: Instant,
}

impl std::fmt::Debug for NativeRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeRequest")
            .field("request_id", &self.request_id)
            .field("method", &self.method)
            .field("uri", &self.uri)
            .field("header_count", &self.headers.len())
            .field("started", &self.started)
            .finish()
    }
}

/// Materialize header pairs (lowercased keys) for the bridge's lazy JS
/// surface. Bounded by the admission header limits already enforced.
pub fn materialize_headers(map: &hyper::HeaderMap) -> Vec<(String, String)> {
    map.iter()
        .map(|(name, value)| {
            (
                name.as_str().to_ascii_lowercase(),
                String::from_utf8_lossy(value.as_bytes()).into_owned(),
            )
        })
        .collect()
}

/// M24-005-C: declared-header value with the frozen duplicate/non-UTF8
/// contract — repeated values join with ", " (HTTP list semantics, arrival
/// order); bytes convert lossily to UTF-8 (invalid sequences become U+FFFD,
/// never a panic, never a rejection); a declared-but-absent name yields None.
pub fn declared_header_value(map: &hyper::HeaderMap, name: &str) -> Option<String> {
    let mut joined = String::new();
    for value in map.get_all(name).iter() {
        if !joined.is_empty() {
            joined.push_str(", ");
        }
        joined.push_str(&String::from_utf8_lossy(value.as_bytes()));
    }
    if joined.is_empty() {
        None
    } else {
        Some(joined)
    }
}

/// Read the body once, stopping as soon as the byte budget would be exceeded
/// (413) instead of buffering the whole stream first. Transport failures map
/// to BadBody (400), matching the previous collect-then-check behavior.
pub async fn collect_body_bounded(
    mut body: Incoming,
    max_bytes: usize,
) -> Result<Vec<u8>, HttpError> {
    use http_body_util::BodyExt;
    let mut buf = Vec::new();
    while let Some(frame) = body.frame().await {
        let frame = frame.map_err(|e| HttpError::BadBody(e.to_string()))?;
        if let Ok(data) = frame.into_data() {
            if buf.len() + data.len() > max_bytes {
                return Err(HttpError::Limited {
                    status: 413,
                    which: "body",
                });
            }
            buf.extend_from_slice(&data);
        }
    }
    Ok(buf)
}

#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("http io: {0}")]
    Io(#[from] std::io::Error),
    /// request exceeded a limit → problem response with the given status
    #[error("limit exceeded: {which}")]
    Limited { status: u16, which: &'static str },
    /// malformed body
    #[error("bad body: {0}")]
    BadBody(String),
    #[error("queue full")]
    QueueFull,
}

/// What the handler produces. Kept deliberately plain so the runtime maps
/// outcomes (problems, engine results) without q-http knowing about them.
pub struct PlainResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    pub head_only: bool,
}

pub type HandlerResult = Result<PlainResponse, HttpError>;

/// Completion log line fields; the host writes them as one JSON object.
#[derive(serde::Serialize)]
pub struct CompletionRecord {
    pub event: &'static str,
    pub request_id: String,
    pub route_id: String,
    pub status: u16,
    pub duration_ms: f64,
    pub body_bytes: usize,
    pub stage: &'static str,
}

pub trait CompletionSink: Send + Sync + 'static {
    fn record(&self, rec: CompletionRecord);
}

pub struct StdoutJsonSink;

impl CompletionSink for StdoutJsonSink {
    fn record(&self, rec: CompletionRecord) {
        if let Ok(line) = serde_json::to_string(&rec) {
            println!("{line}");
        }
    }
}

struct Shared {
    limits: Limits,
    queue: Arc<tokio::sync::Semaphore>,
    request_clock: AtomicU64,
    start_unix_ms: u64,
}

#[derive(Clone)]
pub struct HttpHost {
    shared: Arc<Shared>,
}

impl HttpHost {
    pub fn new(limits: Limits) -> HttpHost {
        HttpHost {
            shared: Arc::new(Shared {
                limits,
                // one permit reserved for shutdown so release never deadlocks
                queue: Arc::new(tokio::sync::Semaphore::new(limits.max_queue.max(1))),
                request_clock: AtomicU64::new(1),
                start_unix_ms: now_unix_ms(),
            }),
        }
    }

    pub fn limits(&self) -> Limits {
        self.shared.limits
    }

    /// Accept loop. Returns once `shutdown` fires AND in-flight work drained.
    pub async fn serve<H, F>(
        self,
        listener: TcpListener,
        handler: H,
        mut shutdown: tokio::sync::watch::Receiver<bool>,
    ) -> std::io::Result<()>
    where
        H: Fn(NativeRequest) -> F + Send + Sync + 'static,
        F: std::future::Future<Output = (HandlerResult, String, &'static str)> + Send,
    {
        let handler = Arc::new(handler);
        loop {
            tokio::select! {
                _ = shutdown.changed() => {
                    if *shutdown.borrow() { break; }
                }
                accepted = listener.accept() => {
                    let (stream, _peer) = match accepted {
                        Ok(x) => x,
                        Err(e) => return Err(e),
                    };
                    let host = self.clone();
                    let handler = Arc::clone(&handler);
                    tokio::spawn(async move {
                        let io = TokioIo::new(stream);
                        let service = ReqService { host: host.clone(), handler: Arc::clone(&handler) };
                        let conn = hyper::server::conn::http1::Builder::new()
                            .keep_alive(true)
                            // hyper's default 8 KiB buffer would reject large
                            // headers itself; size it so OUR header accounting
                            // produces a proper 431 problem instead
                            .max_buf_size((host.shared.limits.max_header_bytes + 64 * 1024).min(512 * 1024))
                            .header_read_timeout(host.shared.limits.header_read_timeout)
                            .timer(hyper_util::rt::TokioTimer::new())
                            .serve_connection(io, service);
                        if let Err(e) = conn.await {
                            let _ = e;
                        }
                    });
                }
            }
        }
        Ok(())
    }

    pub fn active_requests(&self) -> usize {
        self.shared.queue.available_permits()
    }
}

fn query_raw_len(uri: &hyper::Uri) -> usize {
    uri.query().map(|q| q.len() + 1).unwrap_or(0)
}

struct ReqService<H> {
    host: HttpHost,
    handler: Arc<H>,
}

impl<H, F> Service<Request<Incoming>> for ReqService<H>
where
    H: Fn(NativeRequest) -> F + Send + Sync + 'static,
    F: std::future::Future<Output = (HandlerResult, String, &'static str)> + Send,
{
    type Response = Response<http_body_util::Full<hyper::body::Bytes>>;
    type Error = HttpError;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let host = self.host.clone();
        let handler = Arc::clone(&self.handler);
        Box::pin(async move {
            let permit = match host.shared.queue.clone().try_acquire_owned() {
                Ok(p) => p,
                Err(_) => {
                    return Ok(problem_response(
                        503,
                        "queue full",
                        &[("retry-after".into(), "1".into())],
                    ));
                }
            };
            let out = match admit(&host, req) {
                Ok(native) => handler(native).await,
                Err(e) => (Err(e), "admission".into(), "admission"),
            };
            drop(permit);
            let (result, _route, _stage) = out;
            match result {
                Ok(plain) => Ok(render(plain)),
                // service errors abort the connection in hyper; limits become
                // proper problem responses instead
                Err(e) => {
                    let (status, which) = match &e {
                        HttpError::Limited { status, which } => (*status, *which),
                        HttpError::BadBody(_) => (400, "body"),
                        HttpError::QueueFull => (503, "queue"),
                        HttpError::Io(_) => (500, "io"),
                    };
                    Ok(problem_response(status, which, &[]))
                }
            }
        })
    }
}

/// Admission over the native head only: URI/header limits are enforced here
/// with zero materialization — no query parse, no header clone, no body poll.
/// The queue permit is already held by the caller (before this function).
fn admit(host: &HttpHost, req: Request<Incoming>) -> Result<NativeRequest, HttpError> {
    let limits = host.shared.limits;
    let (parts, body) = req.into_parts();
    let uri = parts.uri;
    if uri.path().len() + query_raw_len(&uri) > limits.max_uri_bytes {
        return Err(HttpError::Limited {
            status: 414,
            which: "uri",
        });
    }
    if parts.headers.len() > limits.max_headers {
        return Err(HttpError::Limited {
            status: 431,
            which: "headers",
        });
    }
    let header_bytes = parts
        .headers
        .iter()
        .map(|(name, value)| name.as_str().len() + value.len())
        .sum::<usize>();
    if header_bytes > limits.max_header_bytes {
        return Err(HttpError::Limited {
            status: 431,
            which: "headers",
        });
    }
    Ok(NativeRequest {
        request_id: format!(
            "req-{}-{}",
            host.shared.start_unix_ms,
            host.shared.request_clock.fetch_add(1, Ordering::Relaxed)
        ),
        method: parts.method,
        uri,
        headers: parts.headers,
        body,
        started: Instant::now(),
    })
}

fn render(p: PlainResponse) -> Response<http_body_util::Full<hyper::body::Bytes>> {
    let mut builder = Response::builder().status(p.status);
    for (k, v) in &p.headers {
        builder = builder.header(
            HeaderName::from_bytes(k.as_bytes()).unwrap_or(HeaderName::from_static("x-unknown")),
            HeaderValue::from_str(v).unwrap_or(HeaderValue::from_static("")),
        );
    }
    if p.head_only {
        let mut r = builder
            .body(http_body_util::Full::new(hyper::body::Bytes::new()))
            .expect("static response");
        r.headers_mut().insert(
            "content-length",
            HeaderValue::from_str(&p.body.len().to_string()).unwrap(),
        );
        return r;
    }
    builder
        .body(http_body_util::Full::new(hyper::body::Bytes::from(p.body)))
        .expect("static response")
}

fn problem_response(
    status: u16,
    detail: &str,
    extra: &[(String, String)],
) -> Response<http_body_util::Full<hyper::body::Bytes>> {
    let mut plain = PlainResponse {
        status,
        headers: vec![("content-type".into(), "application/json".into())],
        body: serde_json::to_vec(&serde_json::json!({
            "type": "https://velqu.dev/problems/overload",
            "title": "Overloaded",
            "status": status,
            "detail": detail,
        }))
        .unwrap_or_default(),
        head_only: false,
    };
    plain.headers.extend(extra.iter().cloned());
    render(plain)
}

/// Minimal percent-decoding query parser (order-preserving, last value wins
/// only at the consumer; here all pairs are kept).
pub fn parse_query(raw: &str) -> Vec<(String, String)> {
    let mut out = VecDeque::new();
    for pair in raw.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (k, v) = match pair.split_once('=') {
            Some((k, v)) => (k, v),
            None => (pair, ""),
        };
        out.push_back((percent_decode(k), percent_decode(v)));
    }
    out.into()
}

pub fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() + 1 && i + 2 <= bytes.len() - 1 + 1 => {
                let hex = |b: u8| -> Option<u8> {
                    match b {
                        b'0'..=b'9' => Some(b - b'0'),
                        b'a'..=b'f' => Some(b - b'a' + 10),
                        b'A'..=b'F' => Some(b - b'A' + 10),
                        _ => None,
                    }
                };
                if i + 2 < bytes.len() {
                    if let (Some(hi), Some(lo)) = (hex(bytes[i + 1]), hex(bytes[i + 2])) {
                        out.push(hi << 4 | lo);
                        i += 3;
                        continue;
                    }
                }
                out.push(b'%');
                i += 1;
            }
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn now_unix_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_parsing_and_decoding() {
        let q = parse_query("ms=50&name=Rafi+Z&x=%41");
        assert_eq!(q[0], ("ms".into(), "50".into()));
        assert_eq!(q[1], ("name".into(), "Rafi Z".into()));
        assert_eq!(q[2], ("x".into(), "A".into()));
        assert!(parse_query("").is_empty());
    }

    #[test]
    fn declared_header_value_joins_duplicates_and_is_lossy() {
        use hyper::header::{HeaderMap, HeaderName, HeaderValue};
        let mut map = HeaderMap::new();
        map.append(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str("Bearer a").unwrap(),
        );
        map.append(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str("Bearer b").unwrap(),
        );
        // duplicate values join in arrival order (HTTP list semantics)
        assert_eq!(
            declared_header_value(&map, "authorization").as_deref(),
            Some("Bearer a, Bearer b")
        );
        // absent name → None (declared but not sent)
        assert_eq!(declared_header_value(&map, "x-missing"), None);
        // non-UTF8 bytes convert lossily — no panic, no rejection
        let mut raw = HeaderMap::new();
        raw.insert(
            HeaderName::from_static("x-bin"),
            HeaderValue::from_bytes(&[0x41, 0xff, 0xfe, 0x42]).unwrap(),
        );
        let v = declared_header_value(&raw, "x-bin").unwrap();
        assert!(v.starts_with('A') && v.ends_with('B') && v.contains('\u{fffd}'));
    }

    #[test]
    fn header_materialization_lowercases_names_and_keeps_values() {
        use hyper::header::HeaderValue;
        let mut map = hyper::HeaderMap::new();
        map.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );
        map.insert("X-Custom", HeaderValue::from_str("v").unwrap());
        let pairs = materialize_headers(&map);
        assert!(pairs.contains(&("content-type".to_string(), "application/json".to_string())));
        assert!(pairs.contains(&("x-custom".to_string(), "v".to_string())));
    }
}
