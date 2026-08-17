//! raw-rust baseline: minimal hyper 1.x server implementing the frozen fixture
//! contract (benchmarks/fixtures/fixture-contract.json). This is a transport
//! lower bound: it has NO framework/Treaty parity by design. Hand-rolled
//! routing; JSON via serde_json.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::service::Service;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use serde_json::json;

static REQ_CLOCK: AtomicU64 = AtomicU64::new(1);

struct AppState {
    users: Mutex<Option<HashMap<String, serde_json::Value>>>,
    next_user: AtomicU64,
    n_routes: usize,
}

impl AppState {
    /// lazy in-memory service: first use seeds the fixture user
    fn with_users<T>(&self, f: impl FnOnce(&mut HashMap<String, serde_json::Value>) -> T) -> T {
        let mut guard = self.users.lock().unwrap_or_else(|p| p.into_inner());
        let mut map = guard.take().unwrap_or_default();
        if map.is_empty() {
            map.insert(
                "usr_1".to_string(),
                json!({"id": "usr_1", "name": "Ada", "email": "ada@example.org"}),
            );
        }
        let out = f(&mut map);
        *guard = Some(map);
        out
    }
    fn user_get(&self, id: &str) -> Option<String> {
        self.with_users(|m| {
            m.get(id).map(|u| {
                format!(
                    "{{\"id\":\"{}\",\"name\":\"{}\",\"email\":\"{}\"}}",
                    u["id"].as_str().unwrap_or_default(),
                    u["name"].as_str().unwrap_or_default(),
                    u["email"].as_str().unwrap_or_default()
                )
            })
        })
    }
    fn user_create(&self, name: &str, email: &str) -> String {
        let id = format!("usr_{}", self.next_user.fetch_add(1, Ordering::Relaxed));
        let u = json!({"id": id, "name": name, "email": email});
        let body = format!("{{\"id\":\"{id}\",\"name\":\"{name}\",\"email\":\"{email}\"}}");
        self.with_users(|m| {
            m.insert(id, u);
        });
        body
    }
}

fn raw_json_resp(status: u16, body: String) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(body)))
        .unwrap()
}

fn json_resp(status: u16, body: serde_json::Value) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(body.to_string())))
        .unwrap()
}

fn validation_problem(field: &str, code: &str, message: &str) -> Response<Full<Bytes>> {
    json_resp(
        422,
        json!({
            "type": "https://velqu.dev/problems/validation",
            "title": "Validation failed",
            "status": 422,
            "errors": [{"path": field, "code": code, "message": message}]
        }),
    )
}

async fn handle(req: Request<hyper::body::Incoming>, state: Arc<AppState>) -> Response<Full<Bytes>> {
    let method = req.method().as_str().to_uppercase();
    let uri = req.uri().clone();
    let path = uri.path().to_string();
    let query = uri.query().unwrap_or("").to_string();

    // generated item routes (route-count benchmark)
    if state.n_routes > 0 && path.starts_with("/res") {
        if let Some((n, id)) = parse_item_route(&path) {
            if method == "GET" && n < state.n_routes && id >= 1 && id <= state.n_routes as i64 {
                return json_resp(200, json!({"id": id, "n": state.n_routes}));
            }
            return validation_problem("id", "minimum", "out of range");
        }
    }

    let auth_ok = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == "Bearer q-demo-token")
        .unwrap_or(false);

    match (method.as_str(), path.as_str()) {
        ("GET", "/health/live") | ("HEAD", "/health/live") => {
            json_resp(200, json!({"status": "ok"}))
        }
        ("GET", "/js-text") => Response::builder()
            .status(200)
            .header("content-type", "text/plain; charset=utf-8")
            .body(Full::new(Bytes::from("plain")))
            .unwrap(),
        ("GET", "/js-json") => json_resp(200, json!({"ok": true})),
        ("POST", "/js-text") | ("POST", "/js-json") | ("POST", "/async") | ("POST", "/cancel")
        | ("POST", "/throw") => Response::builder()
            .status(405)
            .header("content-type", "application/json")
            .header("allow", "GET, HEAD")
            .body(Full::new(Bytes::from(
                json!({"type": "https://velqu.dev/problems/method", "title": "Method Not Allowed", "status": 405}).to_string(),
            )))
            .unwrap(),
        ("GET", "/users") => Response::builder()
            .status(405)
            .header("content-type", "application/json")
            .header("allow", "POST")
            .body(Full::new(Bytes::from(
                json!({"type": "https://velqu.dev/problems/method", "title": "Method Not Allowed", "status": 405}).to_string(),
            )))
            .unwrap(),
        (m, p) if m == "GET" && p.starts_with("/hello/") => {
            let name = percent_decode(p.trim_start_matches("/hello/"));
            if name.is_empty() || name.len() > 60 {
                return validation_problem("name", "maxLength", "must be at most 60 characters");
            }
            json_resp(200, json!({"message": format!("Hello {name}")}))
        }
        ("POST", "/users") => {
            let body = match req.into_body().collect().await {
                Ok(c) => c.to_bytes(),
                Err(_) => return validation_problem("body", "type", "unreadable body"),
            };
            let parsed: serde_json::Value = match serde_json::from_slice(&body) {
                Ok(v) => v,
                Err(_) => {
                    return json_resp(
                        422,
                        json!({"type": "https://velqu.dev/problems/validation", "title": "Validation failed", "status": 422, "detail": "malformed JSON body"}),
                    )
                }
            };
            let name = parsed.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let email = parsed.get("email").and_then(|v| v.as_str()).unwrap_or("");
            if name.is_empty() || name.len() > 60 {
                return validation_problem("name", "maxLength", "must be 1-60 characters");
            }
            if !email.contains('@') || !email.contains('.') || email.starts_with('@') {
                return validation_problem("email", "format", "must be a valid email");
            }
            let u = state.user_create(name, email);
            raw_json_resp(201, u)
        }
        (m, p) if m == "GET" && p.starts_with("/users/") => {
            if !auth_ok {
                return json_resp(
                    401,
                    json!({"type": "https://velqu.dev/problems/unauthorized", "title": "Unauthorized", "status": 401}),
                );
            }
            let id = percent_decode(p.trim_start_matches("/users/"));
            if !id.starts_with("usr_") || !id[4..].chars().all(|c| c.is_ascii_digit()) || id.len() == 4 {
                return validation_problem("id", "pattern", "must match ^usr_[0-9]+$");
            }
            match state.user_get(&id) {
                Some(u) => raw_json_resp(200, u),
                None => json_resp(
                    404,
                    json!({"type": "https://velqu.dev/problems/not-found", "title": "Not Found", "status": 404}),
                ),
            }
        }
        ("GET", "/async") => {
            let ms = parse_query_ms(&query, 10).clamp(1, 1000);
            tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
            json_resp(200, json!({"waited": ms}))
        }
        ("GET", "/cancel") => {
            let ms = parse_query_ms(&query, 1000).clamp(1, 5000);
            tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
            json_resp(200, json!({"cancelled": false, "waited": ms}))
        }
        ("GET", "/throw") => {
            // transport lower bound: redacted 500, no details
            json_resp(
                500,
                json!({"type": "https://velqu.dev/problems/internal", "title": "Internal Server Error", "status": 500}),
            )
        }
        _ => json_resp(
            404,
            json!({"type": "https://velqu.dev/problems/not-found", "title": "Not Found", "status": 404}),
        ),
    }
}

fn parse_item_route(path: &str) -> Option<(usize, i64)> {
    // /res{n}/item/{id}
    let rest = path.strip_prefix("/res")?;
    let (n, rest) = rest.split_once("/item/")?;
    let n: usize = n.parse().ok()?;
    let id: i64 = rest.parse().ok()?;
    Some((n, id))
}

fn parse_query_ms(query: &str, default: u64) -> u64 {
    for pair in query.split('&') {
        if let Some(v) = pair.strip_prefix("ms=") {
            if let Ok(n) = v.parse() {
                return n;
            }
        }
    }
    default
}

fn percent_decode(s: &str) -> String {
    // minimal: pass through (fixture uses plain names)
    s.to_string()
}

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let n_routes: usize = std::env::var("N_ROUTES").ok().and_then(|p| p.parse().ok()).unwrap_or(0);

    let state = Arc::new(AppState {
        users: Mutex::new(None),
        next_user: AtomicU64::new(1),
        n_routes,
    });

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port)).await.expect("bind");
    println!("raw-rust ready port={port}");

    loop {
        let (stream, _) = match listener.accept().await {
            Ok(x) => x,
            Err(_) => continue,
        };
        let io = TokioIo::new(stream);
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            let service = RawService { state };
            let conn = hyper::server::conn::http1::Builder::new()
                .keep_alive(true)
                .serve_connection(io, service);
            let _ = conn.await;
        });
    }
}

struct RawService {
    state: Arc<AppState>,
}

impl Service<Request<hyper::body::Incoming>> for RawService {
    type Response = Response<Full<Bytes>>;
    type Error = std::convert::Infallible;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn call(&self, req: Request<hyper::body::Incoming>) -> Self::Future {
        let state = Arc::clone(&self.state);
        Box::pin(async move { Ok(handle(req, state).await) })
    }
}
