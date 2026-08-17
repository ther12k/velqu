//! Request pipeline: route natively (RUN-002) → C0 static liveness without
//! JavaScript → native input validation → engine invocation → outcome mapping
//! with redaction (RUN-007).

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use q_engine::Engine as _;
use q_engine_quickjs::QuickJsEngine;
use q_engine::{BodyOut, InvocationSpec, Outcome, ResponseStrategy};
use q_http::{HandlerResult, HttpError, PlainResponse, RequestContext};
use q_router::MatchResult;
use q_schema_runtime::{validate_params, validate_query, Source};
use serde_json::Value;

use crate::problems;

pub struct ServeState {
    pub pack: Arc<q_pack::QPack>,
    pub router: q_router::Router,
    pub engine: Mutex<QuickJsEngine>,
    pub store: Arc<q_bridge::RequestStore>,
    pub invocation_clock: AtomicU64,
}

pub fn make_handler(
    state: Arc<ServeState>,
) -> impl Fn(RequestContext) -> std::pin::Pin<Box<dyn std::future::Future<Output = (HandlerResult, String, &'static str)> + Send>> + Clone + Send + Sync + 'static {
    move |ctx: RequestContext| {
        let state = Arc::clone(&state);
        Box::pin(async move {
            let started = Instant::now();
            let (result, route_id, stage) = pipeline(&state, &ctx).await;
            log_completion(&state, &ctx, &result, &route_id, stage, started);
            (result, route_id, stage)
        })
    }
}

fn log_completion(
    _state: &ServeState,
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
    // OPS-001: structured completion log; header values never logged (SEC-004)
    println!(
        "{}",
        serde_json::json!({
            "level": "info",
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

async fn pipeline(state: &ServeState, ctx: &RequestContext) -> (HandlerResult, String, &'static str) {
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
        MatchResult::Found { route_index, params, head } => {
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

            // ---- native input validation (params/query)
            let mut params_value: Option<Value> = None;
            if let Some(binding) = &route.params {
                if let Some(key) = &binding.schema {
                    let ir = &state.pack.schemas[key];
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
            }

            let mut query_value: Option<Value> = None;
            if let Some(binding) = &route.query {
                if let Some(key) = &binding.schema {
                    let ir = &state.pack.schemas[key];
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
                if let Some(key) = &binding.schema {
                    let ir = &state.pack.schemas[key];
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

            let default_status = route
                .responses
                .contains_key("200")
                .then_some(200)
                .or_else(|| route.responses.keys().next().and_then(|k| k.parse().ok()))
                .unwrap_or(200);
            let allowed_statuses: Vec<u16> =
                route.responses.keys().filter_map(|k| k.parse().ok()).collect();
            let response_strategy = match route.responses.get(&default_status.to_string()) {
                Some(decl) => match decl.strategy {
                    q_pack::Strategy::Native => ResponseStrategy::Native,
                    q_pack::Strategy::Js => ResponseStrategy::Js,
                },
                None => ResponseStrategy::Js,
            };

            let spec = InvocationSpec {
                id: invocation_id,
                request_id: ctx.request_id.clone(),
                route_id: route.id.clone(),
                handler_key: route.handler.clone(),
                policy_key: route.policy.clone(),
                slot,
                generation,
                params: params_value,
                query: query_value,
                headers: None,
                body: body_value,
                allowed_statuses,
                default_status,
                response_strategy,
                deadline: Instant::now() + Duration::from_millis(route.deadline_ms),
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
            let mapped = match outcome {
                Outcome::Response { status, body, headers } => {
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
                            headers: vec![("content-type".into(), "text/plain; charset=utf-8".into())],
                            body: t.into_bytes(),
                            head_only: head,
                        },
                        BodyOut::Bytes(b) => PlainResponse {
                            status,
                            headers: vec![("content-type".into(), "application/octet-stream".into())],
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
                    (Ok(json_response(p.status, &body)), route_id, "engine.problem")
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
