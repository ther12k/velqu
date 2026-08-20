//! Request pipeline: route natively (RUN-002) → C0 static liveness without
//! JavaScript → native input validation → engine invocation → outcome mapping
//! with redaction (RUN-007).

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use q_engine::Engine as _;
use q_engine::{BodyOut, InvocationSpec, Outcome};
use q_engine_quickjs::QuickJsEngine;
use q_http::{HandlerResult, HttpError, PlainResponse, RequestContext};
use q_router::MatchResult;
use q_schema_runtime::{validate_params, validate_query, Source};
use serde_json::Value;

use crate::problems;

/// Request logging modes (OPS-001: full mode is opt-in; production default
/// is Errors to avoid per-request serialization cost).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogMode {
    /// No per-request logging at all (fastest)
    Off,
    /// Log only error responses (4xx/5xx) — production default
    Errors,
    /// Log every request completion (development/debug)
    Full,
}

impl LogMode {
    pub fn from_str(s: &str) -> LogMode {
        match s.to_ascii_lowercase().as_str() {
            "off" => LogMode::Off,
            "errors" => LogMode::Errors,
            "full" => LogMode::Full,
            _ => LogMode::Errors,
        }
    }
}

pub struct ServeState {
    pub pack: Arc<q_pack::QPack>,
    pub router: q_router::Router,
    /// Dense schema vector indexed by SchemaId (from the pack's schema manifest);
    /// request admission validates through this vector — zero string lookups.
    pub schema_vector: Vec<q_schema_runtime::SchemaIr>,
    pub engine: Mutex<QuickJsEngine>,
    pub health: q_engine_quickjs::EngineHealth,
    pub store: Arc<q_bridge::RequestStore>,
    pub invocation_clock: AtomicU64,
    pub log_mode: LogMode,
}

#[allow(clippy::type_complexity)]
pub fn make_handler(
    state: Arc<ServeState>,
) -> impl Fn(
    RequestContext,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = (HandlerResult, String, &'static str)> + Send>,
> + Clone
       + Send
       + Sync
       + 'static {
    move |ctx: RequestContext| {
        let state = Arc::clone(&state);
        Box::pin(async move {
            let started = if state.log_mode != LogMode::Off {
                Some(Instant::now())
            } else {
                None
            };
            let (result, route_id, stage) = pipeline(&state, &ctx).await;
            if let Some(started) = started {
                log_completion(&state, &ctx, &result, &route_id, stage, started);
            }
            (result, route_id, stage)
        })
    }
}

fn log_completion(
    state: &ServeState,
    ctx: &RequestContext,
    result: &HandlerResult,
    route_id: &str,
    stage: &'static str,
    started: Instant,
) {
    let (status, body_bytes) = match result {
        Ok(r) => (r.status, r.body.len()),
        Err(HttpError::Limited { status, .. }) => (*status, 0),
        Err(HttpError::QueueFull) => (503, 0),
        Err(_) => (400, 0),
    };

    // Skip serialization when Errors mode and this is a successful response.
    if state.log_mode == LogMode::Errors && status < 400 {
        return;
    }

    // OPS-001: structured completion log; header values never logged (SEC-004)
    println!(
        "{}",
        serde_json::json!({
            "level": if status < 400 { "info" } else { "warn" },
            "event": "request.complete",
            "requestId": ctx.request_id,
            "routeId": route_id,
            "method": ctx.method,
            "path": ctx.path,
            "status": status,
            "bodyBytes": body_bytes,
            "stage": stage,
            "durationMs": started.elapsed().as_secs_f64() * 1000.0,
        })
    );
}

async fn pipeline(
    state: &ServeState,
    ctx: &RequestContext,
) -> (HandlerResult, String, &'static str) {
    // ---- M2.2.1-r4.1/r4.2.1: built-in readiness probe (liveness stays a pack route)
    // Liveness = process + listener alive; readiness = engine can execute
    // application requests. A quarantined engine keeps /health/live at 200
    // but flips /health/ready to 503. Lock-free atomic check via EngineHealth.
    if ctx.path == "/health/ready" && (ctx.method == "GET" || ctx.method == "HEAD") {
        if state.health.is_ready() {
            let resp = PlainResponse {
                status: 200,
                headers: vec![("content-type".into(), "application/json".into())],
                body: b"{\"ready\":true}".to_vec(),
                head_only: ctx.method == "HEAD",
            };
            return (Ok(resp), "(readiness)".into(), "native");
        }
        let body = problems::body(
            "internal",
            Some(503),
            Some("engine quarantined"),
            &[],
            &ctx.request_id,
        );
        let mut resp = json_response(503, &body);
        resp.head_only = ctx.method == "HEAD";
        return (Ok(resp), "(readiness)".into(), "native");
    }

    // ---- native routing BEFORE any JavaScript (RUN-002)
    match state.router.resolve(&ctx.method, &ctx.path) {
        MatchResult::NotFound => {
            let body = problems::body("not-found", None, None, &[], &ctx.request_id);
            (
                Ok(json_response(404, &body)),
                "(no-route)".into(),
                "routing",
            )
        }
        MatchResult::MethodNotAllowed { allow } => {
            let body = problems::body("method", None, None, &[], &ctx.request_id);
            let mut resp = json_response(405, &body);
            resp.headers.push(("allow".into(), allow.join(", ")));
            (Ok(resp), "(method-not-allowed)".into(), "routing")
        }
        MatchResult::Found {
            route_id: route_id_num,
            route_index,
            params,
            head,
        } => {
            let route = &state.pack.routes[route_index];
            let route_id = route.id.clone();

            // ---- C0: static liveness served natively, JS never entered
            if let Some(live) = &route.native_liveness {
                let mut resp = PlainResponse {
                    status: live.status,
                    headers: vec![("content-type".into(), live.content_type.clone())],
                    body: live.body.as_bytes().to_vec(),
                    head_only: head,
                };
                resp.headers.push(("x-velqu-stage".into(), "native".into()));
                return (Ok(resp), route_id, "native");
            }

            // ---- M2.2.1-r4.2.1: a quarantined engine fails dynamic JS routes
            // closed at the HTTP boundary (503, retry-after) with a single
            // lock-free atomic load via EngineHealth (no engine mutex acquisition).
            if state.health.is_quarantined() {
                let body = problems::body(
                    "internal",
                    Some(503),
                    Some("engine quarantined"),
                    &[],
                    &ctx.request_id,
                );
                let mut resp = json_response(503, &body);
                resp.head_only = head;
                resp.headers.push(("retry-after".into(), "1".into()));
                return (Ok(resp), route_id, "quarantined");
            }

            // M23R2: resolve the CompiledRoute once — every subsequent step
            // (validation, invocation, response) reads numeric IDs from it
            let compiled = state
                .router
                .compiled_route(route_index)
                .expect("matched route must exist in router");

            // ---- native input validation (params/query) — SchemaId indexed,
            // zero string-map lookups on the request path (M23R2-004)
            let mut params_value: Option<Value> = None;
            if let Some(sid) = compiled.params_schema_id {
                let ir = &state.schema_vector[sid.0 as usize];
                match validate_params(ir, &params) {
                    Ok(v) => params_value = Some(v),
                    Err(errors) => {
                        let body = problems::body(
                            "validation",
                            None,
                            Some("path parameter validation failed"),
                            &field_errors(&errors),
                            &ctx.request_id,
                        );
                        return (Ok(json_response(422, &body)), route_id, "validation.params");
                    }
                }
            }

            let mut query_value: Option<Value> = None;
            if let Some(sid) = compiled.query_schema_id {
                let ir = &state.schema_vector[sid.0 as usize];
                match validate_query(ir, &ctx.query) {
                    Ok(v) => query_value = Some(v),
                    Err(errors) => {
                        let body = problems::body(
                            "validation",
                            None,
                            Some("query validation failed"),
                            &field_errors(&errors),
                            &ctx.request_id,
                        );
                        return (Ok(json_response(422, &body)), route_id, "validation.query");
                    }
                }
            }

            // ---- body: content-type gate + parse + validate
            let mut body_value: Option<Value> = None;
            if let Some(binding) = &route.body {
                let ctype = ctx
                    .headers
                    .iter()
                    .find(|(k, _)| k == "content-type")
                    .map(|(_, v)| v.as_str())
                    .unwrap_or("");
                let is_json = ctype.is_empty()
                    || ctype.starts_with("application/json")
                    || ctype.starts_with("text/json");
                if !is_json {
                    let body = problems::body(
                        "body",
                        None,
                        Some("expected application/json body"),
                        &[],
                        &ctx.request_id,
                    );
                    return (Ok(json_response(415, &body)), route_id, "admission.body");
                }
                let raw = ctx.body.clone().unwrap_or_default();
                if (raw.len() as u64) > binding.limit_bytes {
                    let body = problems::body("limit", None, None, &[], &ctx.request_id);
                    return (Ok(json_response(413, &body)), route_id, "admission.body");
                }
                let parsed: Value = match serde_json::from_slice(&raw) {
                    Ok(v) => v,
                    Err(_) => {
                        let body = problems::body(
                            "validation",
                            None,
                            Some("malformed JSON body"),
                            &[],
                            &ctx.request_id,
                        );
                        return (Ok(json_response(422, &body)), route_id, "validation.body");
                    }
                };
                if let Some(sid) = compiled.body_schema_id {
                    let ir = &state.schema_vector[sid.0 as usize];
                    match q_schema_runtime::validate(ir, &parsed, Source::Body) {
                        Ok(v) => body_value = Some(v),
                        Err(errors) => {
                            let body = problems::body(
                                "validation",
                                None,
                                Some("body validation failed"),
                                &field_errors(&errors),
                                &ctx.request_id,
                            );
                            return (Ok(json_response(422, &body)), route_id, "validation.body");
                        }
                    }
                }
            }

            // ---- invocation through the engine
            let invocation_id = state.invocation_clock.fetch_add(1, Ordering::Relaxed);
            let (slot, generation) = state.store.insert(q_bridge::RequestMeta {
                method: ctx.method.clone(),
                path: ctx.path.clone(),
                params,
                query: ctx.query.clone(),
                headers: ctx.headers.clone(),
                content_type: ctx
                    .headers
                    .iter()
                    .find(|(k, _)| k == "content-type")
                    .map(|(_, v)| v.clone()),
                body: ctx.body.clone(),
            });

            let policy_key = if compiled.policy_handler_id.is_none() {
                route
                    .policy
                    .as_ref()
                    .map(|policy_id| state.pack.policies[policy_id].handler.clone())
            } else {
                None
            };
            let handler_key = if compiled.handler_id.is_none() {
                route.handler.clone()
            } else {
                String::new()
            };

            let spec = InvocationSpec {
                id: invocation_id,
                request_id: ctx.request_id.clone(),
                route_id: route.id.clone(),
                route_id_num: Some(compiled.route_id),
                handler_key,
                policy_key,
                handler_id: compiled.handler_id,
                policy_id_num: compiled.policy_id,
                policy_handler_id: compiled.policy_handler_id,
                params_schema_id: compiled.params_schema_id,
                query_schema_id: compiled.query_schema_id,
                headers_schema_id: compiled.headers_schema_id,
                body_schema_id: compiled.body_schema_id,
                slot,
                generation,
                params: params_value,
                query: query_value,
                headers: None,
                body: body_value,
                allowed_statuses: compiled.allowed_statuses.clone(),
                default_status: compiled.default_status,
                response_strategy: compiled.response_strategy,
                deadline: Instant::now() + Duration::from_millis(compiled.deadline_ms),
            };

            let (tx, rx) = tokio::sync::oneshot::channel();
            {
                let mut eng = state.engine.lock().unwrap();
                eng.invoke(spec, tx);
            }
            let outcome = tokio::time::timeout(Duration::from_millis(route.deadline_ms + 1000), rx)
                .await
                .map(|r| r.unwrap_or(Outcome::Timeout))
                .unwrap_or(Outcome::Timeout);

            // client gone (connection dropped) → cancel invocation
            // SCHEMA-003: declared response bodies are validated at runtime.
            // Only the native strategy crosses as structured data; the
            // engine-stringified path (js strategy) is disclosed per-route
            // in the build report and skipped here.
            if let Outcome::Response { status, body, .. } = &outcome {
                if let Some(decl) = route.responses.get(&status.to_string()) {
                    if let Some(key) = &decl.schema {
                        if let Some(ir) = state.pack.schemas.get(key) {
                            let candidate = match body {
                                BodyOut::Json(v) => Some(v.clone()),
                                BodyOut::Text(t) => {
                                    // string responses validate against string-kind IR
                                    Some(serde_json::Value::String(t.clone()))
                                }
                                _ => None,
                            };
                            if let Some(v) = candidate {
                                if let Err(errors) =
                                    q_schema_runtime::validate(ir, &v, Source::Body)
                                {
                                    let detail = format!(
                                        "route {} response failed its declared schema ({}): {:?}",
                                        route.id, key, errors
                                    );
                                    eprintln!(
                                        "{}",
                                        serde_json::json!({
                                            "level":"error","event":"contract.violation.response",
                                            "requestId": ctx.request_id, "routeId": route.id, "detail": detail,
                                        })
                                    );
                                    let problem = problems::body(
                                        "internal",
                                        None,
                                        None,
                                        &[],
                                        &ctx.request_id,
                                    );
                                    let mapped = (
                                        Ok(json_response(500, &problem)),
                                        route_id.clone(),
                                        "engine.response-validation",
                                    );
                                    return mapped;
                                }
                            }
                        }
                    }
                }
            }
            let mapped = match outcome {
                Outcome::Response {
                    status,
                    body,
                    headers,
                } => {
                    let mut resp = match body {
                        BodyOut::JsonText(text) => PlainResponse {
                            status,
                            headers: vec![("content-type".into(), "application/json".into())],
                            body: text.into_bytes(),
                            head_only: head,
                        },
                        BodyOut::Json(v) => {
                            let mut resp = json_response(status, &v);
                            resp.head_only = head;
                            resp
                        }
                        BodyOut::Text(t) => PlainResponse {
                            status,
                            headers: vec![(
                                "content-type".into(),
                                "text/plain; charset=utf-8".into(),
                            )],
                            body: t.into_bytes(),
                            head_only: head,
                        },
                        BodyOut::Bytes(b) => PlainResponse {
                            status,
                            headers: vec![(
                                "content-type".into(),
                                "application/octet-stream".into(),
                            )],
                            body: b,
                            head_only: head,
                        },
                        BodyOut::Empty => PlainResponse {
                            status,
                            headers: vec![],
                            body: Vec::new(),
                            head_only: head,
                        },
                    };
                    resp.headers.extend(headers);
                    (Ok(resp), route_id, "engine")
                }
                Outcome::Problem(p) => {
                    let body = problems::body(
                        &p.problem_id,
                        Some(p.status),
                        p.detail.as_deref(),
                        &p.errors,
                        &ctx.request_id,
                    );
                    (
                        Ok(json_response(p.status, &body)),
                        route_id,
                        "engine.problem",
                    )
                }
                Outcome::Timeout => {
                    let body = problems::body("timeout", None, None, &[], &ctx.request_id);
                    (Ok(json_response(504, &body)), route_id, "engine.timeout")
                }
                Outcome::ContractViolation(detail) => {
                    // undeclared status: controlled contract failure; details go
                    // to the log, the response stays generic (SCHEMA-003)
                    eprintln!(
                        "{}",
                        serde_json::json!({
                            "level":"error","event":"contract.violation",
                            "requestId": ctx.request_id, "routeId": route_id, "detail": detail,
                        })
                    );
                    let body = problems::body("internal", None, None, &[], &ctx.request_id);
                    (Ok(json_response(500, &body)), route_id, "engine.contract")
                }
                Outcome::EngineFailure { detail, source } => {
                    // internal detail NEVER crosses to the client (RUN-007)
                    eprintln!(
                        "{}",
                        serde_json::json!({
                            "level":"error","event":"handler.error",
                            "requestId": ctx.request_id, "routeId": route_id,
                            "detail": detail, "source": source,
                        })
                    );
                    let body = problems::body("internal", None, None, &[], &ctx.request_id);
                    (Ok(json_response(500, &body)), route_id, "engine.error")
                }
            };
            mapped
        }
    }
}

fn field_errors(errors: &[q_schema_runtime::FieldError]) -> Vec<q_engine::FieldErrorOut> {
    errors
        .iter()
        .map(|e| q_engine::FieldErrorOut {
            path: e.path.clone(),
            code: e.code.clone(),
            message: e.message.clone(),
        })
        .collect()
}

fn json_response(status: u16, body: &Value) -> PlainResponse {
    PlainResponse {
        status,
        headers: vec![("content-type".into(), "application/json".into())],
        body: serde_json::to_vec(body).unwrap_or_default(),
        head_only: false,
    }
}
