//! Runtime conformance (M1): spawns the ACTUAL q-runtime binary with a
//! fixture pack implementing the frozen benchmark contract, then drives
//! black-box HTTP requests against it. This is runtime-local evidence —
//! unit-local engine tests do not substitute (TRT-005).

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use serde_json::{json, Value};

// ---------------------------------------------------------------- fixture pack

fn fixture_pack() -> q_pack::QPack {
    use q_pack::*;
    use q_schema_runtime::SchemaIr as S;
    use std::collections::BTreeMap;

    let seg = |parts: &[(&str, &str)]| -> Vec<PathSegment> {
        parts
            .iter()
            .map(|(kind, value)| PathSegment {
                kind: match *kind {
                    "s" => SegKind::Static,
                    "p" => SegKind::Param,
                    _ => SegKind::Wildcard,
                },
                value: value.to_string(),
            })
            .collect()
    };
    let mut schemas = BTreeMap::new();
    schemas.insert(
        "sch:hello.params".to_string(),
        S::Object {
            properties: BTreeMap::from([(
                "name".into(),
                Box::new(S::String {
                    min_length: Some(1),
                    max_length: Some(60),
                    pattern: None,
                    format: None,
                }),
            )]),
            required: vec!["name".into()],
        },
    );
    schemas.insert(
        "sch:users.create.body".to_string(),
        S::Object {
            properties: BTreeMap::from([
                (
                    "name".into(),
                    Box::new(S::String {
                        min_length: Some(1),
                        max_length: Some(60),
                        pattern: None,
                        format: None,
                    }),
                ),
                (
                    "email".into(),
                    Box::new(S::String {
                        min_length: None,
                        max_length: None,
                        pattern: None,
                        format: Some("email".into()),
                    }),
                ),
            ]),
            required: vec!["name".into(), "email".into()],
        },
    );
    schemas.insert(
        "sch:users.get.params".to_string(),
        S::Object {
            properties: BTreeMap::from([(
                "id".into(),
                Box::new(S::String {
                    min_length: None,
                    max_length: None,
                    pattern: Some("^usr_[0-9]+$".into()),
                    format: None,
                }),
            )]),
            required: vec!["id".into()],
        },
    );
    schemas.insert(
        "sch:async.query".to_string(),
        S::Object {
            properties: BTreeMap::from([(
                "ms".into(),
                Box::new(S::Optional {
                    inner: Box::new(S::Integer {
                        minimum: Some(1),
                        maximum: Some(1000),
                    }),
                    default: Some(json!(10)),
                }),
            )]),
            required: vec![],
        },
    );

    let route = |id: &str, method: &str, path: &str, segments: Vec<PathSegment>, handler: &str| {
        RouteEntry {
            id: id.into(),
            module_id: id.split('.').next().unwrap().into(),
            method: method.into(),
            path: path.into(),
            path_segments: segments,
            handler: handler.into(),
            policy: None,
            params: None,
            query: None,
            body: None,
            headers: None,
            responses: BTreeMap::from([(
                "200".into(),
                ResponseDecl {
                    schema: None,
                    strategy: Strategy::Js,
                    problem: None,
                },
            )]),
            validation_strategy: Strategy::Native,
            native_liveness: None,
            security: vec![],
            capabilities: vec![],
            deadline_ms: 5000,
        }
    };

    let mut routes = vec![
        {
            let mut r = route(
                "health.live",
                "GET",
                "/health/live",
                seg(&[("s", "health"), ("s", "live")]),
                "health.live",
            );
            r.native_liveness = Some(LivenessSpec {
                status: 200,
                content_type: "application/json".into(),
                body: "{\"status\":\"ok\"}".into(),
            });
            r
        },
        route(
            "js.text",
            "GET",
            "/js-text",
            seg(&[("s", "js-text")]),
            "js.text",
        ),
        route(
            "js.json",
            "GET",
            "/js-json",
            seg(&[("s", "js-json")]),
            "js.json",
        ),
        {
            let mut r = route(
                "hello.get",
                "GET",
                "/hello/:name",
                seg(&[("s", "hello"), ("p", "name")]),
                "hello.get",
            );
            r.params = Some(SourceBinding {
                schema: Some("sch:hello.params".into()),
                coerce: Some("path".into()),
                content_type: None,
                limit_bytes: 0,
            });
            r.responses.insert(
                "422".into(),
                ResponseDecl {
                    schema: None,
                    strategy: Strategy::Js,
                    problem: Some("validation".into()),
                },
            );
            r
        },
        {
            let mut r = route(
                "users.create",
                "POST",
                "/users",
                seg(&[("s", "users")]),
                "users.create",
            );
            r.body = Some(SourceBinding {
                schema: Some("sch:users.create.body".into()),
                coerce: None,
                content_type: Some("application/json".into()),
                limit_bytes: 65_536,
            });
            r.responses = BTreeMap::from([
                (
                    "201".into(),
                    ResponseDecl {
                        schema: None,
                        strategy: Strategy::Js,
                        problem: None,
                    },
                ),
                (
                    "422".into(),
                    ResponseDecl {
                        schema: None,
                        strategy: Strategy::Js,
                        problem: Some("validation".into()),
                    },
                ),
            ]);
            r
        },
        {
            let mut r = route(
                "users.get",
                "GET",
                "/users/:id",
                seg(&[("s", "users"), ("p", "id")]),
                "users.get",
            );
            r.policy = Some("auth.session".into());
            r.params = Some(SourceBinding {
                schema: Some("sch:users.get.params".into()),
                coerce: Some("path".into()),
                content_type: None,
                limit_bytes: 0,
            });
            r.responses = BTreeMap::from([
                (
                    "200".into(),
                    ResponseDecl {
                        schema: None,
                        strategy: Strategy::Js,
                        problem: None,
                    },
                ),
                (
                    "401".into(),
                    ResponseDecl {
                        schema: None,
                        strategy: Strategy::Js,
                        problem: Some("unauthorized".into()),
                    },
                ),
                (
                    "404".into(),
                    ResponseDecl {
                        schema: None,
                        strategy: Strategy::Js,
                        problem: Some("not-found".into()),
                    },
                ),
            ]);
            r.security = vec![SecurityReq {
                scheme: "bearer".into(),
                header: "authorization".into(),
                problem_status: 401,
            }];
            r
        },
        {
            let mut r = route(
                "async.timer",
                "GET",
                "/async",
                seg(&[("s", "async")]),
                "async.timer",
            );
            r.query = Some(SourceBinding {
                schema: Some("sch:async.query".into()),
                coerce: Some("query".into()),
                content_type: None,
                limit_bytes: 0,
            });
            r.capabilities = vec!["timer".into()];
            r
        },
        {
            let mut r = route(
                "async.cancel",
                "GET",
                "/cancel",
                seg(&[("s", "cancel")]),
                "async.cancel",
            );
            r.query = Some(SourceBinding {
                schema: Some("sch:async.query".into()),
                coerce: Some("query".into()),
                content_type: None,
                limit_bytes: 0,
            });
            r.capabilities = vec!["timer".into()];
            r
        },
        route(
            "throw.redacted",
            "GET",
            "/throw",
            seg(&[("s", "throw")]),
            "throw.redacted",
        ),
    ];

    // timer schema reuse for cancel needs a wider maximum
    schemas.insert(
        "sch:cancel.query".to_string(),
        S::Object {
            properties: BTreeMap::from([(
                "ms".into(),
                Box::new(S::Optional {
                    inner: Box::new(S::Integer {
                        minimum: Some(1),
                        maximum: Some(5000),
                    }),
                    default: Some(json!(1000)),
                }),
            )]),
            required: vec![],
        },
    );
    routes
        .iter_mut()
        .find(|r| r.id == "async.cancel")
        .unwrap()
        .query
        .as_mut()
        .unwrap()
        .schema = Some("sch:cancel.query".into());

    let bundle = FIXTURE_BUNDLE.to_string();
    let mut handler_table = BTreeMap::new();
    for r in &routes {
        handler_table.insert(r.handler.clone(), r.handler.clone());
    }
    handler_table.insert("auth.session".into(), "auth.session".into());

    let mut pack = QPack {
        format_version: q_pack::PACK_FORMAT_VERSION,
        kind: "velqu.qpack".into(),
        runtime_abi: q_pack::RUNTIME_ABI,
        engine: q_pack::EngineRef {
            name: q_pack::ENGINE_NAME.into(),
            version: q_pack::ENGINE_VERSION.into(),
            binding: q_pack::ENGINE_BINDING.into(),
        },
        schema_ir_version: 1,
        contract_version: 1,
        contract_hash: String::new(),
        built_by: q_pack::BuiltBy {
            compiler: "manual-fixture".into(),
            typescript: String::new(),
            bun: String::new(),
        },
        app_id: "proof-fixture".into(),
        modules: vec![
            "health".into(),
            "hello".into(),
            "users".into(),
            "async".into(),
        ],
        entry: "app.js".into(),
        bundle,
        source_map: None,
        routes,
        schemas,
        policies: BTreeMap::from([(
            "auth.session".to_string(),
            PolicyEntry {
                id: "auth.session".into(),
                handler: "auth.session".into(),
                declared_statuses: vec![401],
                provides: Some("session".into()),
            },
        )]),
        capabilities: vec!["timer".into()],
        handler_table,
        integrity: q_pack::Integrity {
            algorithm: "sha256".into(),
            bundle_sha256: String::new(),
            routes_sha256: String::new(),
        },
    };
    use sha2::{Digest, Sha256};
    pack.integrity.bundle_sha256 = hex(&Sha256::digest(pack.bundle.as_bytes()));
    pack.integrity.routes_sha256 = hex(&Sha256::digest(pack.routes_canonical_json().as_bytes()));
    pack
}

#[allow(dead_code)]
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

const FIXTURE_BUNDLE: &str = r#"
"use strict";
var __users = null;
var __nextUser = 1;
function users() {
  // lazy in-memory service: first use seeds the fixture user (C5)
  if (__users === null) {
    __users = new Map();
    __users.set("usr_1", { id: "usr_1", name: "Ada", email: "ada@example.org" });
  }
  return __users;
}

async function health_live() { return { status: "ok" }; }
async function js_text() { return "plain"; }
async function js_json() { return { ok: true }; }
async function hello_get(ctx) {
  return { message: "Hello " + ctx.params.name };
}
async function users_create(ctx) {
  // deterministic in a fresh process: first created user is usr_1
  const id = "usr_" + (__nextUser++);
  const u = { id, name: ctx.body.name, email: ctx.body.email };
  users().set(id, u);
  return { __ok: true, status: 201, value: u };
}
async function users_get(ctx) {
  const u = users().get(ctx.params.id);
  if (!u) return { __problem: true, problem: "not-found", status: 404, detail: "user not found" };
  return u;
}
async function auth_session(req) {
  const token = req.headers.authorization;
  if (token !== "Bearer q-demo-token") {
    return { __problem: true, problem: "unauthorized", status: 401 };
  }
  return { session: { userId: "usr_1" } };
}
async function async_timer(ctx) {
  const waited = await ctx.native.timer.delay(ctx.query.ms);
  return { waited };
}
async function async_cancel(ctx) {
  const waited = await ctx.native.timer.delay(ctx.query.ms);
  return { cancelled: false, waited };
}
async function throw_redacted() {
  throw new Error("secret-boom");
}

__velquRegister("health.live", health_live);
__velquRegister("js.text", js_text);
__velquRegister("js.json", js_json);
__velquRegister("hello.get", hello_get);
__velquRegister("users.create", users_create);
__velquRegister("users.get", users_get);
__velquRegister("auth.session", auth_session);
__velquRegister("async.timer", async_timer);
__velquRegister("async.cancel", async_cancel);
__velquRegister("throw.redacted", throw_redacted);
"#;

// ---------------------------------------------------------------- harness

struct Server {
    child: Child,
    #[allow(dead_code)]
    port: u16,
    log_lines: std::sync::mpsc::Receiver<String>,
}

impl Server {
    fn start(pack_path: &std::path::Path, port: u16) -> Server {
        let bin = env!("CARGO_BIN_EXE_velqu-runtime");
        let mut child = Command::new(bin)
            .arg("--pack")
            .arg(pack_path)
            .arg("--port")
            .arg(port.to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn q-runtime");
        let stdout = child.stdout.take().unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        let _ = tx.send(l);
                    }
                    Err(_) => break,
                }
            }
        });
        // wait for the explicit ready line (bounded)
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            if let Ok(line) = rx.try_recv() {
                if line.contains("\"event\":\"ready\"") {
                    break;
                }
            }
            if Instant::now() > deadline {
                panic!("server did not become ready in time");
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        Server {
            child,
            port,
            log_lines: rx,
        }
    }

    fn drain_logs(&self) -> Vec<String> {
        let mut out = Vec::new();
        while let Ok(l) = self.log_lines.try_recv() {
            out.push(l);
        }
        out
    }

    fn stop(mut self) -> std::process::ExitStatus {
        self.child.kill().expect("kill server");
        self.child.wait().expect("reap server")
    }
}

struct Resp {
    status: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl Resp {
    fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }
    fn json(&self) -> Value {
        serde_json::from_slice(&self.body).unwrap_or(Value::Null)
    }
    fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

fn http(port: u16, req: &str, body: Option<&[u8]>) -> Resp {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap();
    let mut raw = req.to_string();
    raw.push_str("connection: close\r\n");
    if let Some(b) = body {
        raw = format!("{}content-length: {}\r\n\r\n", raw, b.len());
        stream.write_all(raw.as_bytes()).unwrap();
        stream.write_all(b).unwrap();
    } else {
        raw.push_str("\r\n");
        stream.write_all(raw.as_bytes()).unwrap();
    }
    let mut buf = Vec::new();
    // tolerate RST-after-response (HEAD/keep-alive edge cases): parse what arrived
    let _ = stream.read_to_end(&mut buf);
    parse_http(&buf)
}

fn parse_http(raw: &[u8]) -> Resp {
    let split = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .expect("header terminator");
    let head = String::from_utf8_lossy(&raw[..split]).into_owned();
    let mut lines = head.lines();
    let status_line = lines.next().unwrap_or_default();
    let status: u16 = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let mut headers = Vec::new();
    for line in lines {
        if let Some((k, v)) = line.split_once(':') {
            headers.push((k.trim().to_string(), v.trim().to_string()));
        }
    }
    let mut body = raw[split + 4..].to_vec();
    if let Some(len) = headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("content-length"))
        .and_then(|(_, v)| v.parse::<usize>().ok())
    {
        body.truncate(len);
    }
    Resp {
        status,
        headers,
        body,
    }
}

fn write_pack(dir: &std::path::Path) -> PathBuf {
    let pack = fixture_pack();
    let path = dir.join("app.qpack");
    std::fs::write(&path, serde_json::to_vec(&pack).unwrap()).unwrap();
    path
}

#[test]
fn debug_dump_pack() {
    let dir = std::path::PathBuf::from("/tmp/velqu-debug");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    write_pack(&dir);
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("velqu-m1-{}-{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

// ---------------------------------------------------------------- tests

#[test]
fn full_runtime_conformance() {
    let dir = temp_dir("conf");
    let pack_path = write_pack(&dir);
    let port = free_port();
    let server = Server::start(&pack_path, port);

    // C0: native liveness, exact bytes, JS never entered (stage=native header)
    let r = http(port, "GET /health/live HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert_eq!(r.text(), "{\"status\":\"ok\"}");
    assert_eq!(r.header("x-velqu-stage"), Some("native"));

    // HEAD on C0: same status, no body
    let r = http(port, "HEAD /health/live HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert!(r.body.is_empty(), "HEAD must not carry a body");

    // C1: JS text
    let r = http(port, "GET /js-text HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert_eq!(r.text(), "plain");

    // C2: JS JSON
    let r = http(port, "GET /js-json HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert_eq!(r.text(), "{\"ok\":true}");

    // C3: validated path param + happy path
    let r = http(port, "GET /hello/Rafi HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert_eq!(r.text(), "{\"message\":\"Hello Rafi\"}");

    // C3 negative: name too long → 422 identifying field
    let long = "x".repeat(61);
    let r = http(
        port,
        &format!("GET /hello/{} HTTP/1.1\r\nhost: t\r\n", long),
        None,
    );
    assert_eq!(r.status, 422, "body: {}", r.text());
    let j = r.json();
    assert_eq!(
        j["type"],
        "https://velqu.dev/problems/validation",
        "body: {}",
        r.text()
    );
    assert_eq!(j["errors"][0]["path"], "name");
    assert_eq!(j["errors"][0]["code"], "maxLength");

    // POST /users happy → 201 exact bytes
    let body = br#"{"name":"Ada","email":"ada@example.org"}"#;
    let r = http(
        port,
        "POST /users HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
        Some(body),
    );
    assert_eq!(r.status, 201, "body: {}", r.text());
    assert_eq!(
        r.text(),
        "{\"id\":\"usr_1\",\"name\":\"Ada\",\"email\":\"ada@example.org\"}"
    );

    // malformed JSON → 422
    let r = http(
        port,
        "POST /users HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
        Some(br#"{name:"#),
    );
    assert_eq!(r.status, 422);
    assert_eq!(r.json()["status"], 422);

    // schema-invalid email → 422 identifying field
    let r = http(
        port,
        "POST /users HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
        Some(br#"{"name":"Ada","email":"not-an-email"}"#),
    );
    assert_eq!(r.status, 422);
    assert_eq!(r.json()["errors"][0]["path"], "email");

    // C4: policy 401 without credentials
    let r = http(port, "GET /users/usr_1 HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 401);
    assert_eq!(r.json()["type"], "https://velqu.dev/problems/unauthorized");

    // C4: valid credentials → 200 exact bytes (lazy service first use = C5 too)
    let r = http(
        port,
        "GET /users/usr_1 HTTP/1.1\r\nhost: t\r\nauthorization: Bearer q-demo-token\r\n",
        None,
    );
    assert_eq!(r.status, 200, "body: {}", r.text());
    assert_eq!(
        r.text(),
        "{\"id\":\"usr_1\",\"name\":\"Ada\",\"email\":\"ada@example.org\"}"
    );

    // C4: unknown user → typed 404
    let r = http(
        port,
        "GET /users/usr_999 HTTP/1.1\r\nhost: t\r\nauthorization: Bearer q-demo-token\r\n",
        None,
    );
    assert_eq!(r.status, 404);
    assert_eq!(r.json()["type"], "https://velqu.dev/problems/not-found");

    // async: native timer through a promise; default ms=10 (query coercion)
    let r = http(port, "GET /async HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert_eq!(r.text(), "{\"waited\":10}");
    let r = http(port, "GET /async?ms=50 HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.text(), "{\"waited\":50}");

    // throw → redacted 500; body must not leak the secret or any stack
    let r = http(port, "GET /throw HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 500);
    let text = r.text();
    assert!(text.contains("\"status\":500"));
    assert!(!text.contains("secret-boom"), "redaction failed: {text}");
    assert!(!text.contains("at "), "stack leaked: {text}");

    // 404 unknown path (JSON problem)
    let r = http(
        port,
        "GET /definitely/not/here HTTP/1.1\r\nhost: t\r\n",
        None,
    );
    assert_eq!(r.status, 404);
    assert_eq!(r.json()["title"], "Not Found");

    // 405 with Allow (GET-only route hit with POST)
    let r = http(port, "POST /js-text HTTP/1.1\r\nhost: t\r\n", Some(b""));
    assert_eq!(r.status, 405);
    let allow = r.header("allow").expect("Allow header").to_string();
    assert!(
        allow.contains("GET") && allow.contains("HEAD"),
        "allow: {allow}"
    );

    // logs: stage evidence — C0 served natively; JS routes via engine
    std::thread::sleep(Duration::from_millis(100));
    let logs = server.drain_logs();
    let native = logs
        .iter()
        .filter(|l| l.contains("/health/live") && l.contains("\"stage\":\"native\""))
        .count();
    assert!(
        native >= 2,
        "C0 must be served at the native stage: {logs:?}"
    );
    let engine_stage = logs
        .iter()
        .filter(|l| l.contains("\"stage\":\"engine\""))
        .count();
    assert!(engine_stage >= 5, "JS routes must log engine stage");
    let redacted_logs = logs.iter().any(|l| l.contains("secret-boom"));
    assert!(
        !redacted_logs,
        "internal error detail goes to stderr, not stdout logs"
    );

    server.stop();
}

#[test]
fn tampered_pack_fails_before_ready() {
    let dir = temp_dir("tamper");
    let pack_path = write_pack(&dir);
    // mutate the bundle without recomputing integrity
    let raw: Value = serde_json::from_slice(&std::fs::read(&pack_path).unwrap()).unwrap();
    let mut tampered = raw.clone();
    tampered["bundle"] = json!("__velquRegister('health.live', function(){ return {evil:true} });");
    let tampered_path = dir.join("tampered.qpack");
    std::fs::write(&tampered_path, serde_json::to_vec(&tampered).unwrap()).unwrap();

    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let out = Command::new(bin)
        .arg("--pack")
        .arg(&tampered_path)
        .arg("--port")
        .arg(free_port().to_string())
        .output()
        .expect("run tampered pack");
    assert_ne!(
        out.status.code(),
        Some(0),
        "tampered pack must exit non-zero"
    );
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("sha256 mismatch") || err.contains("integrity"),
        "stderr: {err}"
    );
}

#[test]
fn client_abort_leaves_server_healthy() {
    let dir = temp_dir("abort");
    let pack_path = write_pack(&dir);
    let port = free_port();
    let server = Server::start(&pack_path, port);

    // start a long /cancel request, then drop the connection mid-flight
    {
        let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
        s.write_all(b"GET /cancel?ms=1000 HTTP/1.1\r\nhost: t\r\n\r\n")
            .unwrap();
        std::thread::sleep(Duration::from_millis(20));
        drop(s);
    }
    // server must remain healthy: liveness answers within 1s
    let t0 = Instant::now();
    let r = http(port, "GET /health/live HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    assert!(
        t0.elapsed() < Duration::from_secs(1),
        "server unhealthy after abort"
    );

    // and JS still works afterwards
    let r = http(port, "GET /js-json HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 200);
    server.stop();
}

#[test]
fn graceful_shutdown_exits_zero() {
    let dir = temp_dir("shutdown");
    let pack_path = write_pack(&dir);
    let port = free_port();
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let mut child = Command::new(bin)
        .arg("--pack")
        .arg(&pack_path)
        .arg("--port")
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    // wait ready by polling TCP
    let deadline = Instant::now() + Duration::from_secs(10);
    while TcpStream::connect(("127.0.0.1", port)).is_err() {
        if Instant::now() > deadline {
            panic!("not ready");
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    // SIGTERM
    unsafe {
        libc::kill(child.id() as i32, libc::SIGTERM);
    }
    let status = child.wait().unwrap();
    assert!(
        status.success(),
        "graceful shutdown must exit 0, got {status:?}"
    );
}

#[test]
fn source_mapped_exception_identifies_original_location() {
    use sourcemap::SourceMapBuilder;
    // generated bundle with the throw on line 2
    let bundle = "async function thrower() {\n  throw new Error(\"origin-boom\");\n}\n__velquRegister(\"t\", thrower);\n";
    let mut b = SourceMapBuilder::new(None);
    b.add(
        1,
        0,
        41,
        4,
        Some("src/modules/users/routes.ts"),
        None,
        false,
    );
    let map_json = {
        let mut out = Vec::new();
        let sm = b.into_sourcemap();
        sm.to_writer(&mut out).unwrap();
        String::from_utf8(out).unwrap()
    };

    let dir = temp_dir("sourcemap");
    let mut pack = fixture_pack();
    pack.bundle = bundle.to_string();
    pack.source_map = Some(map_json);
    // recompute integrity for the modified bundle
    {
        use sha2::{Digest, Sha256};
        pack.integrity.bundle_sha256 = {
            let h = Sha256::digest(pack.bundle.as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
        pack.integrity.routes_sha256 = {
            let h = Sha256::digest(pack.routes_canonical_json().as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
    }
    // single-route handler table must match the new bundle
    pack.handler_table = std::collections::BTreeMap::from([("t".to_string(), "t".to_string())]);
    let throw_route = pack
        .routes
        .iter()
        .position(|r| r.id == "throw.redacted")
        .unwrap();
    let route = pack.routes[throw_route].clone();
    pack.routes = vec![route];
    pack.routes[0].handler = "t".into();
    pack.policies.clear();
    // recompute integrity again (routes changed too)
    {
        use sha2::{Digest, Sha256};
        pack.integrity.routes_sha256 = {
            let h = Sha256::digest(pack.routes_canonical_json().as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
    }
    let pack_path = dir.join("sourcemap.qpack");
    std::fs::write(&pack_path, serde_json::to_vec(&pack).unwrap()).unwrap();

    let port = free_port();
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let mut child = Command::new(bin)
        .arg("--pack")
        .arg(&pack_path)
        .arg("--port")
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_tcp(port, Duration::from_secs(10));
    let _ = http(port, "GET /throw HTTP/1.1\r\nhost: t\r\n", None);
    std::thread::sleep(Duration::from_millis(150));
    child.kill().unwrap();
    let out = child.wait_with_output().unwrap();
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("origin-boom"),
        "engine detail must reach internal stderr log: {err}"
    );
    assert!(
        err.contains("src/modules/users/routes.ts") && err.contains("42"),
        "original source location must be mapped into diagnostics: {err}"
    );
}

#[test]
fn body_and_header_limits_reject_oversize() {
    let dir = temp_dir("limits");
    let pack_path = write_pack(&dir);
    let port = free_port();
    let server = Server::start(&pack_path, port);

    // body over the route limit (65536) → 413 problem
    let big = vec![b'{'];
    let mut huge = br#"{"name":""#.to_vec();
    huge.extend(vec![b'a'; 70_000]);
    let _ = big;
    let r = http(
        port,
        "POST /users HTTP/1.1\r\nhost: t\r\ncontent-type: application/json\r\n",
        Some(&huge),
    );
    assert_eq!(
        r.status,
        413,
        "oversize body must be 413, got {}: {}",
        r.status,
        r.text()
    );

    // header block over 32 KiB → 431 (below hyper's own buffer cap)
    let mut req = String::from("GET /js-text HTTP/1.1\r\nhost: t\r\n");
    req.push_str(&format!("x-big: {}\r\n", "h".repeat(33_000)));
    req.push_str("connection: close\r\n\r\n");
    let r = http(port, &req, None);
    assert_eq!(
        r.status, 431,
        "oversize headers must be 431, got {}",
        r.status
    );

    server.stop();
}

#[test]
fn queue_limit_returns_503_when_saturated() {
    let dir = temp_dir("queue");
    let pack_path = write_pack(&dir);
    // config: queue of 1
    let cfg = dir.join("limits.json");
    std::fs::write(
        &cfg,
        serde_json::to_vec(&serde_json::json!({"maxQueue": 1})).unwrap(),
    )
    .unwrap();

    let port = free_port();
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let mut child = Command::new(bin)
        .arg("--pack")
        .arg(&pack_path)
        .arg("--config")
        .arg(&cfg)
        .arg("--port")
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    wait_tcp(port, Duration::from_secs(10));

    // request 1 occupies the single admission slot for ~1.5s
    let mut slow = TcpStream::connect(("127.0.0.1", port)).unwrap();
    slow.write_all(b"GET /cancel?ms=1500 HTTP/1.1\r\nhost: t\r\nconnection: close\r\n\r\n")
        .unwrap();
    std::thread::sleep(Duration::from_millis(100)); // let it enter the pipeline
                                                    // request 2 while saturated → 503
    let r = http(port, "GET /health/live HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(r.status, 503, "saturated queue must 503, got {}", r.status);
    drop(slow);
    child.kill().unwrap();
    let _ = child.wait();
}

fn wait_tcp(port: u16, deadline: Duration) {
    let end = Instant::now() + deadline;
    while TcpStream::connect(("127.0.0.1", port)).is_err() {
        if Instant::now() > end {
            panic!("server not ready");
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[test]
fn response_schema_violation_is_a_controlled_500() {
    // handler returns a shape that does NOT match its declared response schema
    let bundle = r#"
async function bad_shape(ctx) { return { wrong: true }; }
__velquRegister("bad.shape", bad_shape);
"#;
    let dir = temp_dir("respval");
    let mut pack = fixture_pack();
    pack.bundle = bundle.to_string();
    pack.handler_table =
        std::collections::BTreeMap::from([("bad.shape".to_string(), "bad.shape".to_string())]);
    let route = pack
        .routes
        .iter()
        .position(|r| r.id == "users.get")
        .unwrap();
    let mut r = pack.routes[route].clone();
    r.id = "bad.shape".into();
    r.handler = "bad.shape".into();
    r.policy = None;
    r.responses = {
        let mut m = std::collections::BTreeMap::new();
        m.insert(
            "200".to_string(),
            q_pack::ResponseDecl {
                schema: Some("sch:bad.200".to_string()),
                strategy: q_pack::Strategy::Native,
                problem: None,
            },
        );
        m
    };
    pack.routes = vec![r];
    pack.policies.clear();
    pack.schemas.insert(
        "sch:bad.200".to_string(),
        q_schema_runtime::SchemaIr::Object {
            properties: std::collections::BTreeMap::from([(
                "expected".to_string(),
                Box::new(q_schema_runtime::SchemaIr::String {
                    min_length: Some(1),
                    max_length: None,
                    pattern: None,
                    format: None,
                }),
            )]),
            required: vec!["expected".into()],
        },
    );
    {
        use sha2::{Digest, Sha256};
        pack.integrity.bundle_sha256 = {
            let h = Sha256::digest(pack.bundle.as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
        pack.integrity.routes_sha256 = {
            let h = Sha256::digest(pack.routes_canonical_json().as_bytes());
            h.iter().map(|b| format!("{:02x}", b)).collect()
        };
    }
    let pack_path = dir.join("respval.qpack");
    std::fs::write(&pack_path, serde_json::to_vec(&pack).unwrap()).unwrap();

    let port = free_port();
    let bin = env!("CARGO_BIN_EXE_velqu-runtime");
    let mut child = Command::new(bin)
        .arg("--pack")
        .arg(&pack_path)
        .arg("--port")
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_tcp(port, Duration::from_secs(10));
    let r = http(port, "GET /users/usr_1 HTTP/1.1\r\nhost: t\r\n", None);
    assert_eq!(
        r.status,
        500,
        "schema-violating response must be a 500, got {}: {}",
        r.status,
        r.text()
    );
    assert!(r.text().contains("\"status\":500"));
    assert!(
        !r.text().contains("wrong"),
        "violation detail must not leak"
    );
    std::thread::sleep(Duration::from_millis(150));
    child.kill().unwrap();
    let out = child.wait_with_output().unwrap();
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("contract.violation.response"),
        "internal log must carry the response contract violation: {err}"
    );
}
