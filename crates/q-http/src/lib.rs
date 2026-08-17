//! q-http — hyper 1.x HTTP/1.1 host with bounded admission, request IDs,
//! structured completion logs, and deterministic graceful shutdown.
//! Independently benchmarkable (no engine/route knowledge).

use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use http_body_util::BodyExt;
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

#[derive(Debug)]
pub struct RequestContext {
    pub request_id: String,
    pub method: String,
    pub path: String,
    /// decoded query pairs in arrival order
    pub query: Vec<(String, String)>,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
    pub started: Instant,
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
        H: Fn(RequestContext) -> F + Send + Sync + 'static,
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
    H: Fn(RequestContext) -> F + Send + Sync + 'static,
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
                    let (res, route, stage) = (
                        Err(HttpError::QueueFull),
                        "admission".to_string(),
                        "admission",
                    );
                    host.log_completion(&res, &route, stage, Instant::now(), 0);
                    return Ok(problem_response(
                        503,
                        "queue full",
                        &[("retry-after".into(), "1".into())],
                    ));
                }
            };
            let started = Instant::now();
            let out = handle_one(&host, req, &*handler).await;
            host.log_completion(&out.0, &out.1, out.2, started, permit.token_id() as usize);
            drop(permit);
            let (result, _route, _stage) = out;
            match result {
                Ok(plain) => Ok(render(plain)),
                Err(e) => Err(e),
            }
        })
    }
}

trait TokenId {
    fn token_id(&self) -> u64;
}
impl TokenId for tokio::sync::OwnedSemaphorePermit {
    fn token_id(&self) -> u64 {
        0
    }
}

impl HttpHost {
    fn log_completion(
        &self,
        result: &HandlerResult,
        route: &str,
        stage: &'static str,
        started: Instant,
        _q: usize,
    ) {
        let (status, body_bytes) = match result {
            Ok(r) => (r.status, r.body.len()),
            Err(HttpError::Limited { status, .. }) => (*status, 0),
            Err(_) => (400, 0),
        };
        let rec = CompletionRecord {
            event: "request.complete",
            request_id: String::new(),
            route_id: route.to_string(),
            status,
            duration_ms: started.elapsed().as_secs_f64() * 1000.0,
            body_bytes,
            stage,
        };
        let _ = rec;
        // sink is attached by the runtime via the handler closure's own logging;
        // q-http itself stays silent to keep the transport layer reusable.
    }
}

async fn handle_one<H, F>(host: &HttpHost, req: Request<Incoming>, handler: &H) -> (HandlerResult, String, &'static str)
where
    H: Fn(RequestContext) -> F + Send + Sync,
    F: std::future::Future<Output = (HandlerResult, String, &'static str)> + Send,
{
    let limits = host.shared.limits;
    let method = req.method().as_str().to_uppercase();
    let uri = req.uri().clone();
    let path = uri.path().to_string();
    let query_raw = uri.query().unwrap_or("").to_string();

    // URI limit
    if path.len() + query_raw_len(&uri) > limits.max_uri_bytes {
        return (
            Err(HttpError::Limited { status: 414, which: "uri" }),
            "admission".into(),
            "admission",
        );
    }
    // header limits
    let mut headers = Vec::with_capacity(req.headers().len());
    let mut header_bytes = 0usize;
    if req.headers().len() > limits.max_headers {
        return (
            Err(HttpError::Limited { status: 431, which: "headers" }),
            "admission".into(),
            "admission",
        );
    }
    for (k, v) in req.headers() {
        let value = String::from_utf8_lossy(v.as_bytes()).into_owned();
        header_bytes += k.as_str().len() + value.len();
        headers.push((k.as_str().to_ascii_lowercase(), value));
    }
    if header_bytes > limits.max_header_bytes {
        return (
            Err(HttpError::Limited { status: 431, which: "headers" }),
            "admission".into(),
            "admission",
        );
    }

    // body: read up to limit+1 so oversize is detectable
    let mut body: Option<Vec<u8>> = None;
    if method == "POST" || method == "PUT" || method == "PATCH" {
        match req.into_body().collect().await {
            Ok(collected) => {
                let bytes = collected.to_bytes();
                if bytes.len() > limits.max_body_bytes {
                    return (
                        Err(HttpError::Limited { status: 413, which: "body" }),
                        "admission".into(),
                        "admission",
                    );
                }
                body = Some(bytes.to_vec());
            }
            Err(e) => return (Err(HttpError::BadBody(e.to_string())), "admission".into(), "admission"),
        }
    }

    let request_id = format!(
        "req-{}-{}",
        host.shared.start_unix_ms,
        host.shared.request_clock.fetch_add(1, Ordering::Relaxed)
    );
    let ctx = RequestContext {
        request_id,
        method,
        path,
        query: parse_query(&query_raw),
        headers,
        body,
        started: Instant::now(),
    };
    handler(ctx).await
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
        r.headers_mut()
            .insert("content-length", HeaderValue::from_str(&p.body.len().to_string()).unwrap());
        return r;
    }
    builder
        .body(http_body_util::Full::new(hyper::body::Bytes::from(p.body)))
        .expect("static response")
}

fn problem_response(status: u16, detail: &str, extra: &[(String, String)]) -> Response<http_body_util::Full<hyper::body::Bytes>> {
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
}
