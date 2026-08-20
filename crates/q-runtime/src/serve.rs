//! Request pipeline: route natively (RUN-002) → C0 static liveness without
//! JavaScript → native input validation → engine invocation → outcome mapping
//! with redaction (RUN-007).

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use q_engine::Engine as _;
use q_engine::{BodyOut, InvocationSpec, Outcome};
use q_engine_quickjs::QuickJsEngine;
use q_http::{collect_body_bounded, declared_header_value, materialize_headers, parse_query};
use q_http::{HandlerResult, HttpError, NativeRequest, PlainResponse};
use q_router::MatchResult;
use q_schema_runtime::{validate_query, Source};
use serde_json::Value;

use crate::problems;

/// Materialized pipeline inputs (M24-002-A): q-http now hands over native
/// parts and this runtime-local context is built only inside the handler.
/// M24-002-B moves its construction after route resolution.
struct RequestContext {
    request_id: String,
    method: String,
    path: String,
    /// decoded query pairs in arrival order
    query: Vec<(String, String)>,
    headers: Vec<(String, String)>,
    body: Option<bytes::Bytes>,
}

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
    pub invocation_clock: AtomicU64,
    pub log_mode: LogMode,
}

#[allow(clippy::type_complexity)]
pub fn make_handler(
    state: Arc<ServeState>,
) -> impl Fn(
    NativeRequest,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = (HandlerResult, String, &'static str)> + Send>,
> + Clone
       + Send
       + Sync
       + 'static {
    move |req: NativeRequest| {
        let state = Arc::clone(&state);
        Box::pin(async move {
            let started = if state.log_mode != LogMode::Off {
                Some(req.started)
            } else {
                None
            };
            // M24-002-B: routing runs on the native head before any field is
            // materialized; log fields are only built when logging is on.
            let log_ctx = if state.log_mode != LogMode::Off {
                Some((
                    req.request_id.clone(),
                    req.method.as_str().to_uppercase(),
                    req.uri.path().to_string(),
                ))
            } else {
                None
            };
            let (result, route_id, stage) = pipeline(&state, req).await;
            if let Some(started) = started {
                log_completion(&state, log_ctx.as_ref(), &result, &route_id, stage, started);
            }
            (result, route_id, stage)
        })
    }
}

fn log_completion(
    state: &ServeState,
    log_ctx: Option<&(String, String, String)>,
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

    let Some((request_id, method, path)) = log_ctx else {
        return;
    };

    // OPS-001: structured completion log; header values never logged (SEC-004)
    println!(
        "{}",
        serde_json::json!({
            "level": if status < 400 { "info" } else { "warn" },
            "event": "request.complete",
            "requestId": request_id,
            "routeId": route_id,
            "method": method,
            "path": path,
            "status": status,
            "bodyBytes": body_bytes,
            "stage": stage,
            "durationMs": started.elapsed().as_secs_f64() * 1000.0,
        })
    );
}

async fn pipeline(state: &ServeState, req: NativeRequest) -> (HandlerResult, String, &'static str) {
    let NativeRequest {
        request_id,
        method,
        uri,
        headers,
        body,
        started: _,
    } = req;
    let method_str = method.as_str();
    let path = uri.path();
    let is_get_or_head = method_str == "GET" || method_str == "HEAD";
    // ---- M2.2.1-r4.1/r4.2.1: built-in readiness probe (liveness stays a pack route)
    // Liveness = process + listener alive; readiness = engine can execute
    // application requests. A quarantined engine keeps /health/live at 200
    // but flips /health/ready to 503. Lock-free atomic check via EngineHealth.
    if path == "/health/ready" && is_get_or_head {
        if state.health.is_ready() {
            let resp = PlainResponse {
                status: 200,
                headers: vec![("content-type".into(), "application/json".into())],
                body: b"{\"ready\":true}".to_vec(),
                head_only: method_str == "HEAD",
            };
            return (Ok(resp), "(readiness)".into(), "native");
        }
        let body = problems::body(
            "internal",
            Some(503),
            Some("engine quarantined"),
            &[],
            &request_id,
        );
        let mut resp = json_response(503, &body);
        resp.head_only = method_str == "HEAD";
        return (Ok(resp), "(readiness)".into(), "native");
    }

    // ---- native routing BEFORE any JavaScript (RUN-002) — M24-002-B: the
    // match runs on borrowed method/path; every early exit below (404, 405,
    // C0 liveness, quarantine) materializes zero request fields and never
    // polls the body stream.
    match state.router.resolve(method_str, path) {
        MatchResult::NotFound => {
            let body = problems::body("not-found", None, None, &[], &request_id);
            (
                Ok(json_response(404, &body)),
                "(no-route)".into(),
                "routing",
            )
        }
        MatchResult::MethodNotAllowed { allow } => {
            let body = problems::body("method", None, None, &[], &request_id);
            let mut resp = json_response(405, &body);
            resp.headers.push(("allow".into(), allow.join(", ")));
            (Ok(resp), "(method-not-allowed)".into(), "routing")
        }
        MatchResult::Found {
            route_id: route_id_num,
            route_index: _route_index,
            param_ranges,
            head,
        } => {
            // RouteId is the canonical dense route-vector identity. The matcher
            // retains route_index only for diagnostics; numeric execution uses
            // the RouteId selected by the terminal slot.
            let route_index = route_id_num.0 as usize;
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
                    &request_id,
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

            // ---- M24-002-C: FieldNeeds from the verified RoutePlan gates
            // every materialization. QPack::verify has already proven the
            // flags exactly match the route's declared needs (schemas, body
            // binding, and security/policy headers), so an undeclared field
            // copies zero bytes even when the handler runs.
            let needs = compiled
                .plan
                .as_ref()
                .map(|p| p.field_needs)
                .unwrap_or_default();
            let query_pairs = if needs.query {
                parse_query(uri.query().unwrap_or(""))
            } else {
                Vec::new()
            };

            // ---- native input validation (params/query) — SchemaId indexed,
            // zero string-map lookups on the request path (M23R2-004).
            // M24-004-A: parameter strings materialize from capture ranges
            // ONLY when validation or the engine actually needs them; a route
            // that neither validates params nor declares param needs never
            // allocates a parameter value.
            let mut params_value: Option<Value> = None;
            // M24-004-C: numeric/UUID formats validate directly from the
            // captured path bytes (borrowed names + range slices) — an
            // invalid value rejects with 422 before any parameter string is
            // allocated.
            if let Some(sid) = compiled.params_schema_id {
                let ir = &state.schema_vector[sid.0 as usize];
                let names = state.router.param_names(route_index);
                let param_bytes: Vec<(&str, &[u8])> = names
                    .iter()
                    .zip(&param_ranges)
                    .map(|(n, (start, end))| (*n, &path.as_bytes()[*start as usize..*end as usize]))
                    .collect();
                match q_schema_runtime::validate_params_bytes(ir, &param_bytes) {
                    Ok(v) => params_value = Some(v),
                    Err(errors) => {
                        let body = problems::body(
                            "validation",
                            None,
                            Some("path parameter validation failed"),
                            &field_errors(&errors),
                            &request_id,
                        );
                        return (Ok(json_response(422, &body)), route_id, "validation.params");
                    }
                }
            }
            // M24-004-A/B: owned parameter strings exist only after
            // validation passed (or none is declared) AND the engine or a
            // policy actually reads them.
            // M24-004-D: the request carries name+range specs against the
            // stored path — parameter VALUE strings do not exist until a JS
            // key access materializes them from the path.
            let params_needed =
                needs.params || route.policy.is_some() || compiled.policy_id.is_some();
            let param_specs: Vec<q_engine::ParamSpec> = if params_needed {
                state
                    .router
                    .param_names(route_index)
                    .iter()
                    .zip(&param_ranges)
                    .map(|(name, (start, end))| q_engine::ParamSpec {
                        name: name.to_string(),
                        start: *start,
                        end: *end,
                    })
                    .collect()
            } else {
                Vec::new()
            };

            let mut query_value: Option<Value> = None;
            if let Some(sid) = compiled.query_schema_id {
                let ir = &state.schema_vector[sid.0 as usize];
                match validate_query(ir, &query_pairs) {
                    Ok(v) => query_value = Some(v),
                    Err(errors) => {
                        let body = problems::body(
                            "validation",
                            None,
                            Some("query validation failed"),
                            &field_errors(&errors),
                            &request_id,
                        );
                        return (Ok(json_response(422, &body)), route_id, "validation.query");
                    }
                }
            }

            // ---- body: route-bound admission (M24-002-B). The content-type
            // gate runs on the native HeaderMap BEFORE the stream is polled;
            // the single read is bounded by the route's limit_bytes and stops
            // at the budget instead of buffering an oversize body first.
            // Routes without a body binding never poll the stream at all.
            let mut body_value: Option<Value> = None;
            let mut raw_body: Option<bytes::Bytes> = None;
            if needs.body {
                // QPack::verify proves body binding and RoutePlan agree. The
                // plan flag, not HTTP method, controls stream polling.
                let binding = route.body.as_ref().expect("verified body plan binding");
                let ctype = headers
                    .get("content-type")
                    .and_then(|v| v.to_str().ok())
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
                        &request_id,
                    );
                    return (Ok(json_response(415, &body)), route_id, "admission.body");
                }
                let raw = match collect_body_bounded(body, binding.limit_bytes as usize).await {
                    Ok(bytes) => bytes,
                    Err(HttpError::Limited { .. }) => {
                        let body = problems::body("limit", None, None, &[], &request_id);
                        return (Ok(json_response(413, &body)), route_id, "admission.body");
                    }
                    Err(e) => return (Err(e), route_id, "admission.body"),
                };
                let parsed: Value = match serde_json::from_slice(&raw) {
                    Ok(v) => v,
                    Err(_) => {
                        let body = problems::body(
                            "validation",
                            None,
                            Some("malformed JSON body"),
                            &[],
                            &request_id,
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
                                &request_id,
                            );
                            return (Ok(json_response(422, &body)), route_id, "validation.body");
                        }
                    }
                }
                raw_body = Some(raw);
            }

            // Materialized pipeline inputs for the JS invocation surface.
            // Headers copy zero bytes unless the route declared header or
            // security needs (M24-002-C); content-type only exists for
            // body-bound routes where admission already read it natively.
            let ctx = RequestContext {
                request_id,
                method: method_str.to_uppercase(),
                path: path.to_string(),
                query: query_pairs,
                // M24-005-B: only the header names this route's plan
                // declares (security/policy + headers binding) are copied —
                // never the full HeaderMap. Lookup is by name on the native
                // map (first value, same semantics as materialize_headers).
                headers: if needs.headers {
                    let table = &state.pack.header_name_table;
                    compiled
                        .plan
                        .as_ref()
                        .map(|plan| {
                            if plan.header_name_ids.contains(&q_pack::FULL_HEADERS_ID) {
                                // M24-005-D: the EXPLICIT escape hatch — copy
                                // every header. Cost: bounded by the transport
                                // header admission limits (max_headers count /
                                // max_header_bytes), charged per access like
                                // any other materialized field.
                                return materialize_headers(&headers);
                            }
                            plan.header_name_ids
                                .iter()
                                .filter_map(|id| {
                                    let name = table.get(*id as usize)?;
                                    // M24-005-C contract: duplicates join with
                                    // ", ", non-UTF8 converts lossily, absent
                                    // declared names are omitted.
                                    declared_header_value(&headers, name.as_str())
                                        .map(|value| (name.clone(), value))
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default()
                } else {
                    Vec::new()
                },
                body: raw_body,
            };
            let content_type = if needs.body {
                headers
                    .get("content-type")
                    .and_then(|v| v.to_str().ok())
                    .map(str::to_string)
            } else {
                None
            };

            // ---- invocation through the engine
            let invocation_id = state.invocation_clock.fetch_add(1, Ordering::Relaxed);
            let requestless = !needs.params
                && !needs.query
                && !needs.headers
                && !needs.body
                && route.policy.is_none()
                && compiled.policy_id.is_none()
                && compiled.policy_handler_id.is_none();
            let request = if requestless {
                None
            } else {
                Some(q_engine::RequestMeta {
                    method: ctx.method.clone(),
                    path: ctx.path.clone(),
                    param_specs,
                    query: ctx.query.clone(),
                    headers: ctx.headers.clone(),
                    content_type,
                    body: ctx.body.clone(),
                })
            };

            // PolicyId is the canonical dense policy-vector identity. Resolve the
            // precompiled handler through that manifest; QPack::verify has already
            // proven the manifest entry agrees with the route plan.
            let policy_handler_id = compiled.policy_id.and_then(|policy_id| {
                state
                    .pack
                    .policy_manifest
                    .get(policy_id.0 as usize)
                    .map(|decl| q_engine::HandlerId(decl.handler_id))
            });
            let policy_key = if policy_handler_id.is_none() {
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
                policy_handler_id: policy_handler_id.or(compiled.policy_handler_id),
                params_schema_id: compiled.params_schema_id,
                query_schema_id: compiled.query_schema_id,
                headers_schema_id: compiled.headers_schema_id,
                body_schema_id: compiled.body_schema_id,
                request,
                slot: if requestless {
                    q_engine::NO_REQUEST_SLOT
                } else {
                    0
                },
                generation: 0,
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
            // M24-003-C: disconnect/outer-timeout ownership. If this pipeline
            // future is dropped before the engine replies (client disconnect
            // aborts the response future, or the outer deadline fires),
            // cancellation is delivered to the worker so the request slot and
            // its native operations settle exactly once through the single
            // settlement owner. Normal completion disarms the guard.
            let mut cancel_guard = CancelOnDrop {
                state,
                invocation_id,
                armed: true,
            };
            {
                let mut eng = state.engine.lock().unwrap();
                eng.invoke(spec, tx);
            }
            let outcome = tokio::time::timeout(Duration::from_millis(route.deadline_ms + 1000), rx)
                .await
                .map(|r| r.unwrap_or(Outcome::Timeout))
                .unwrap_or(Outcome::Timeout);
            cancel_guard.disarm();

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
                Outcome::RequestCapacity => {
                    let body = problems::body("overload", Some(503), None, &[], &ctx.request_id);
                    let mut response = json_response(503, &body);
                    response.headers.push(("retry-after".into(), "1".into()));
                    (Ok(response), route_id, "engine.capacity")
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

/// M24-003-C: cancellation ownership for the response future. Dropping the
/// pipeline future (client disconnect, host shutdown of the connection task)
/// delivers `Engine::cancel` so the worker's single settlement owner
/// invalidates the slot and aborts native operations exactly once. The guard
/// is disarmed on normal completion, so a delivered outcome is never
/// double-settled.
struct CancelOnDrop<'a> {
    state: &'a ServeState,
    invocation_id: u64,
    armed: bool,
}

impl CancelOnDrop<'_> {
    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for CancelOnDrop<'_> {
    fn drop(&mut self) {
        if self.armed {
            if let Ok(mut eng) = self.state.engine.lock() {
                eng.cancel(self.invocation_id);
            }
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
