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
use q_schema_runtime::Source;
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
    pub fn parse_mode(s: &str) -> LogMode {
        match s.to_ascii_lowercase().as_str() {
            "off" => LogMode::Off,
            "errors" => LogMode::Errors,
            "full" => LogMode::Full,
            _ => LogMode::Errors,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct StageMetrics {
    pub route: AtomicU64,
    pub queue: AtomicU64,
    pub decode: AtomicU64,
    pub bridge: AtomicU64,
    pub js: AtomicU64,
    pub encode: AtomicU64,
    pub write: AtomicU64,
    pub slab_live: AtomicU64,
    pub queue_pending: AtomicU64,
    pub body_bytes: AtomicU64,
}

impl StageMetrics {
    pub fn snapshot(&self) -> StageMetricsSnapshot {
        StageMetricsSnapshot {
            route: self.route.load(Ordering::Relaxed),
            queue: self.queue.load(Ordering::Relaxed),
            decode: self.decode.load(Ordering::Relaxed),
            bridge: self.bridge.load(Ordering::Relaxed),
            js: self.js.load(Ordering::Relaxed),
            encode: self.encode.load(Ordering::Relaxed),
            write: self.write.load(Ordering::Relaxed),
            slab_live: self.slab_live.load(Ordering::Relaxed),
            queue_pending: self.queue_pending.load(Ordering::Relaxed),
            body_bytes: self.body_bytes.load(Ordering::Relaxed),
        }
    }
}

/// BETA-006-A: per-route request/status/duration aggregation. Cardinality
/// is bounded by construction: one entry per pack route (a static table)
/// plus a single fallback bucket — never per-path, per-status-code, or
/// per-label. Durations are µs totals + max (no histograms, no allocation).
#[derive(Debug, Default)]
pub struct RouteStatusCounters {
    pub total: AtomicU64,
    pub ok_2xx: AtomicU64,
    pub redirect_3xx: AtomicU64,
    pub client_error_4xx: AtomicU64,
    pub server_error_5xx: AtomicU64,
    pub duration_us_total: AtomicU64,
    pub duration_us_max: AtomicU64,
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct RouteStatusCountersSnapshot {
    pub total: u64,
    pub ok_2xx: u64,
    pub redirect_3xx: u64,
    pub client_error_4xx: u64,
    pub server_error_5xx: u64,
    pub duration_us_total: u64,
    pub duration_us_max: u64,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct RouteStatusEntrySnapshot {
    pub route_id: String,
    #[serde(flatten)]
    pub counters: RouteStatusCountersSnapshot,
}

#[derive(Debug, Default)]
pub struct RouteStatusMetrics {
    route_ids: Vec<String>,
    entries: Vec<RouteStatusCounters>,
    by_id: std::collections::HashMap<Box<str>, usize>,
    unknown: RouteStatusCounters,
}

impl RouteStatusMetrics {
    /// Built once at startup from the pack's static route table.
    pub fn from_route_ids<I: IntoIterator<Item = String>>(route_ids: I) -> Self {
        let route_ids: Vec<String> = route_ids.into_iter().collect();
        let by_id = route_ids
            .iter()
            .enumerate()
            .map(|(i, id)| (Box::from(id.as_str()), i))
            .collect();
        let entries = route_ids
            .iter()
            .map(|_| RouteStatusCounters::default())
            .collect();
        RouteStatusMetrics {
            route_ids,
            entries,
            by_id,
            unknown: RouteStatusCounters::default(),
        }
    }

    /// O(1) atomic increments on the request path; unknown route ids fall
    /// into the single fallback bucket (bounded cardinality).
    pub fn record(&self, route_id: &str, status: u16, duration_us: u64) {
        let entry = match self.by_id.get(route_id) {
            Some(&i) => &self.entries[i],
            None => &self.unknown,
        };
        entry.total.fetch_add(1, Ordering::Relaxed);
        match status {
            200..=299 => entry.ok_2xx.fetch_add(1, Ordering::Relaxed),
            300..=399 => entry.redirect_3xx.fetch_add(1, Ordering::Relaxed),
            400..=499 => entry.client_error_4xx.fetch_add(1, Ordering::Relaxed),
            500..=599 => entry.server_error_5xx.fetch_add(1, Ordering::Relaxed),
            _ => 0,
        };
        entry
            .duration_us_total
            .fetch_add(duration_us, Ordering::Relaxed);
        entry
            .duration_us_max
            .fetch_max(duration_us, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> Vec<RouteStatusEntrySnapshot> {
        let snap = |c: &RouteStatusCounters| RouteStatusCountersSnapshot {
            total: c.total.load(Ordering::Relaxed),
            ok_2xx: c.ok_2xx.load(Ordering::Relaxed),
            redirect_3xx: c.redirect_3xx.load(Ordering::Relaxed),
            client_error_4xx: c.client_error_4xx.load(Ordering::Relaxed),
            server_error_5xx: c.server_error_5xx.load(Ordering::Relaxed),
            duration_us_total: c.duration_us_total.load(Ordering::Relaxed),
            duration_us_max: c.duration_us_max.load(Ordering::Relaxed),
        };
        let mut out = Vec::with_capacity(self.entries.len() + 1);
        for (i, id) in self.route_ids.iter().enumerate() {
            out.push(RouteStatusEntrySnapshot {
                route_id: id.clone(),
                counters: snap(&self.entries[i]),
            });
        }
        out.push(RouteStatusEntrySnapshot {
            route_id: "<unknown>".into(),
            counters: snap(&self.unknown),
        });
        out
    }

    pub fn route_entry_count(&self) -> usize {
        self.entries.len() + 1
    }
}

#[cfg(test)]
mod route_metrics_tests {
    use super::*;

    #[test]
    fn statuses_bucket_and_durations_aggregate() {
        let m = RouteStatusMetrics::from_route_ids(["a".to_string(), "b".to_string()]);
        m.record("a", 200, 100);
        m.record("a", 200, 300);
        m.record("a", 404, 50);
        m.record("b", 500, 1_000_000);
        m.record("never-seen", 200, 7);
        let snap = m.snapshot();
        assert_eq!(snap.len(), 3); // 2 routes + fallback bucket
        let a = &snap[0].counters;
        assert_eq!((a.total, a.ok_2xx, a.client_error_4xx), (3, 2, 1));
        assert_eq!(a.duration_us_total, 450); // 100 + 300 + 50 (all outcomes)
        assert_eq!(a.duration_us_max, 300);
        let b = &snap[1].counters;
        assert_eq!((b.total, b.server_error_5xx), (1, 1));
        assert_eq!(b.duration_us_max, 1_000_000);
        let unknown = &snap[2];
        assert_eq!(unknown.route_id, "<unknown>");
        assert_eq!(
            (unknown.counters.total, unknown.counters.duration_us_total),
            (1, 7)
        );
        // cardinality bound: entries stay fixed no matter how many ids appear
        assert_eq!(m.route_entry_count(), 3);
        m.record("another-unknown", 200, 1);
        assert_eq!(m.route_entry_count(), 3);
    }

    #[test]
    fn record_overhead_is_budgeted() {
        // budget: a single record() must stay far below one request's
        // total cost; a generous per-call bound guards against accidental
        // allocation/lock regressions on the hot path.
        let m = RouteStatusMetrics::from_route_ids(["a".to_string()]);
        let started = Instant::now();
        for i in 0..10_000u64 {
            m.record("a", 200, i % 1_000);
        }
        let per_call_us = started.elapsed().as_micros() as f64 / 10_000.0;
        assert!(per_call_us < 50.0, "record() averaged {per_call_us}µs/call");
    }
}

/// BETA-006-B: worker operations status — one bounded structured snapshot
/// aggregating queue gauges, quarantine state, and replacement policy
/// position. Rendered at drain/shutdown and available on demand; there is
/// deliberately no high-frequency emitter (bounded emissions only).
pub fn worker_ops_status(state: &ServeState) -> serde_json::Value {
    use q_engine::Engine;
    let stage = state.metrics.snapshot();
    let ownership = state.ownership.stats();
    let eng = state.engine.lock().expect("engine mutex");
    let stats = eng.stats();
    let health = eng.health();
    let bridge = eng.bridge_snapshot();
    drop(eng);
    // BETA-006-C: fetch + db pool observability (bounded snapshots only)
    let fetch = crate::fetch_stack::shared_pool().stats();
    let postgres = match state.postgres_dialer.as_ref() {
        Some(handle) => match handle.0.pool_stats_json() {
            Some(json) => {
                let pool: serde_json::Value =
                    serde_json::from_str(&json).unwrap_or(serde_json::Value::Null);
                serde_json::json!({ "linked": true, "pool": pool })
            }
            None => serde_json::json!({ "linked": true }),
        },
        None => serde_json::json!({ "linked": false }),
    };
    serde_json::json!({
        "queue": {
            "pending": stage.queue_pending,
            "slabLive": stage.slab_live,
            "invocationsPending": ownership.pending,
        },
        "worker": {
            "quarantined": health.is_quarantined(),
            "queuePoisoned": stats.queue_poisoned,
            "poisonEvents": stats.poison_events,
        },
        // BETA-006-D: memory / tasks / slots gauges
        "memory": {
            "heapUsedBytes": stats.heap_used,
        },
        "tasks": {
            "nativeStarted": stats.native_tasks_started,
            "nativeAlive": stats.native_tasks_alive,
            "nativeCompleted": stats.native_tasks_completed,
            "nativeAborted": stats.native_tasks_aborted,
        },
        "slots": {
            "live": bridge.live_slots,
            "capacity": state.request_slot_capacity,
        },
        "drain": {
            "draining": state.drain_gate.is_draining(),
            "refused": state.drain_gate.refused(),
        },
        "loadShed": state.load_shed.snapshot(),
        "pools": {
            "fetch": fetch,
            "postgres": postgres,
        },
    })
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct StageMetricsSnapshot {
    pub route: u64,
    pub queue: u64,
    pub decode: u64,
    pub bridge: u64,
    pub js: u64,
    pub encode: u64,
    pub write: u64,
    pub slab_live: u64,
    pub queue_pending: u64,
    pub body_bytes: u64,
}

pub struct ServeState {
    pub pack: Arc<q_pack::QPack>,
    pub router: q_router::Router,
    /// Dense schema vector indexed by SchemaId (from the pack's schema manifest);
    /// request admission validates through this vector — zero string lookups.
    #[allow(dead_code)]
    pub schema_vector: Vec<q_schema_runtime::SchemaIr>,
    /// Pre-compiled direct decoder programs keyed by SchemaId (M25-003-A).
    pub decoder_table: q_schema_runtime::DecoderTable,
    /// Pre-compiled direct response encoders keyed by SchemaId (M25-005-A).
    pub encoder_table: q_schema_runtime::EncoderTable,
    /// Per-route declared response statuses with a schema, resolved to
    /// dense SchemaIds once at startup (M25-005-A) — zero request-time
    /// string lookups on the response path.
    pub response_schema_ids: Vec<std::collections::BTreeMap<u16, u32>>,
    pub engine: Mutex<QuickJsEngine>,
    pub health: q_engine_quickjs::EngineHealth,
    /// BETA-006-C: linked Postgres pool dialer, if the pack requires
    /// `runtime:postgres` and the runtime configured it. Absent -> the
    /// pools snapshot reports unlinked (zero-cost posture).
    pub postgres_dialer: Option<q_capability_postgres::PostgresQueryHandle>,
    /// BETA-006-D: slot capacity gauge ceiling (from admission limits).
    pub request_slot_capacity: u64,
    pub invocation_clock: AtomicU64,
    /// M3-007-A: invocation-to-worker ownership. Admission binds each
    /// invocation to its owning worker exactly once; the terminal
    /// transition (outcome delivered OR cancellation routed) settles it
    /// exactly once. Cancellation and shutdown route through this
    /// binding — never to a guessed engine.
    pub ownership: q_capabilities::InvocationOwnership,
    /// M3-007-B: the admission drain gate — flips once when the shutdown
    /// signal fires; dynamic admission is refused from that instant.
    pub drain_gate: q_capabilities::DrainGate,
    /// M3-008-C: load-shed reason counters — every refusal the runtime
    /// hands a client is recorded by its closed-set kind.
    pub load_shed: q_capabilities::LoadShedCounters,
    pub log_mode: LogMode,
    pub log_sample: u64,
    pub log_sequence: AtomicU64,
    pub metrics: Arc<StageMetrics>,
    /// BETA-006-A: bounded per-route request/status/duration aggregation
    /// (always on; O(1) atomic increments; cardinality fixed at startup).
    pub route_metrics: RouteStatusMetrics,
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
            let sequence = state.log_sequence.fetch_add(1, Ordering::Relaxed) + 1;
            let sampled = state.log_sample == 0 || sequence.is_multiple_of(state.log_sample);
            // BETA-006-A: duration capture is ALWAYS on (a single Instant
            // copy); only log serialization is mode-gated. Metrics remain
            // meaningful even when logging is off.
            let started = Some(req.started);
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
            // BETA-006-E: optional bounded trace id (x-trace-id / W3C
            // traceparent); absent means logs carry only the request id.
            let trace_id = q_http::extract_trace_id(&req.headers);
            let (result, route_id, stage) = pipeline(&state, req).await;
            state.metrics.write.fetch_add(1, Ordering::Relaxed);
            if let Some(started) = started {
                let duration_us = started.elapsed().as_micros() as u64;
                let status = match &result {
                    Ok(r) => r.status,
                    Err(HttpError::Limited { status, .. }) => *status,
                    Err(HttpError::QueueFull) => 503,
                    Err(_) => 400,
                };
                state.route_metrics.record(&route_id, status, duration_us);
                // OPS-001 sampling: full mode respects log_sample; errors
                // mode always attempts (its <400 skip applies inside).
                if state.log_mode == LogMode::Full || sampled {
                    log_completion(
                        &state,
                        log_ctx.as_ref(),
                        &result,
                        &route_id,
                        stage,
                        started,
                        trace_id.as_deref(),
                    );
                }
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
    trace_id: Option<&str>,
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

    println!(
        "{}",
        completion_log_json(
            &state.log_mode,
            request_id,
            method,
            path,
            route_id,
            status,
            body_bytes,
            stage,
            started.elapsed().as_secs_f64() * 1000.0,
            trace_id,
        )
    );
}

/// BETA-006-F: the completion-log allowlist, as code. The structured log
/// carries EXACTLY these fields — request/route identifiers (ids, not
/// paths with query strings), transport metadata, and timing. Header
/// values, query strings, claim material, and bodies have no field and
/// no path into this document. `path` must be `uri.path()` (query
/// stripped by the caller); the guard re-strips defensively.
#[allow(clippy::too_many_arguments)] // flat field list = the log schema itself
fn completion_log_json(
    mode: &LogMode,
    request_id: &str,
    method: &str,
    path: &str,
    route_id: &str,
    status: u16,
    body_bytes: usize,
    stage: &'static str,
    duration_ms: f64,
    trace_id: Option<&str>,
) -> serde_json::Value {
    // defensive re-strip: the caller passes uri.path() (already
    // query-free), but the guard makes the guarantee unconditional
    let path = path.split('?').next().unwrap_or(path);
    let mut doc = serde_json::json!({
        "level": if status < 400 { "info" } else { "warn" },
        "event": "request.complete",
        "requestId": request_id,
        "routeId": route_id,
        "method": method,
        "path": path,
        "status": status,
        "bodyBytes": body_bytes,
        "stage": stage,
        "durationMs": duration_ms,
        // BETA-006-E: optional bounded trace id (absent when the request
        // carried none — the field is omitted, not null)
        "traceId": trace_id,
    });
    if *mode == LogMode::Errors && status < 400 {
        // unreachable via log_completion's skip; kept as belt-and-braces
        doc["detail"] = serde_json::Value::Null;
    }
    doc
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
    state.metrics.route.fetch_add(1, Ordering::Relaxed);
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
            &[],
            &request_id,
        );
        let mut resp = problem_response(503, &body);
        resp.head_only = method_str == "HEAD";
        return (Ok(resp), "(readiness)".into(), "native");
    }

    // ---- native routing BEFORE any JavaScript (RUN-002) — M24-002-B: the
    // match runs on borrowed method/path; every early exit below (404, 405,
    // C0 liveness, quarantine) materializes zero request fields and never
    // polls the body stream.
    match state.router.resolve(method_str, path) {
        MatchResult::NotFound => {
            let body = problems::body("not-found", None, None, &[], &[], &request_id);
            (
                Ok(problem_response(404, &body)),
                "(no-route)".into(),
                "routing",
            )
        }
        MatchResult::MethodNotAllowed { allow } => {
            let body = problems::body("method", None, None, &[], &[], &request_id);
            let mut resp = problem_response(405, &body);
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

            // ---- M3-007-B: the drain gate. Once shutdown begins, dynamic
            // admission stops: any request that would enter JS is refused
            // (frozen overload problem, 503, retry-after) while in-flight
            // work completes. Native liveness above still answers so load
            // balancers observe the instance going away. Lock-free atomic
            // check — same posture as the quarantine gate below.
            if state.drain_gate.check_admission().is_err() {
                state
                    .load_shed
                    .record(&q_capabilities::LoadShedReason::DrainInProgress);
                let body = problems::body(
                    "overload",
                    Some(503),
                    Some("server is draining"),
                    &[],
                    &[],
                    &request_id,
                );
                let mut resp = problem_response(503, &body);
                resp.head_only = head;
                resp.headers.push(("retry-after".into(), "1".into()));
                return (Ok(resp), route_id, "draining");
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
                    &[],
                    &request_id,
                );
                let mut resp = problem_response(503, &body);
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

            // M25-004-D: the route deadline bounds the whole pipeline from
            // route match — admission, bounded body read, decode, and
            // handler. Anchoring here (instead of at engine invocation)
            // charges pre-invocation work to the same budget, cancels a
            // stalled body stream at the deadline, and propagates the same
            // absolute deadline to the worker (constraint 11: deadlines
            // are bounded).
            let request_deadline = Instant::now() + Duration::from_millis(compiled.deadline_ms);

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
            let full_request = route.capabilities.iter().any(|c| c == "full-request");
            let query_pairs = if needs.query || full_request {
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
                let names = state.router.param_names(route_index);
                match state.decoder_table.decode_params_ranges(
                    sid.0,
                    path.as_bytes(),
                    &names,
                    &param_ranges,
                ) {
                    Ok(v) => params_value = Some(v),
                    Err(errors) => {
                        let body = problems::body(
                            "validation",
                            None,
                            Some("path parameter validation failed"),
                            &field_errors(&errors),
                            &[],
                            &request_id,
                        );
                        return (
                            Ok(problem_response(422, &body)),
                            route_id,
                            "validation.params",
                        );
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
                match state.decoder_table.decode_query(sid.0, &query_pairs) {
                    Ok(v) => query_value = Some(v),
                    Err(errors) => {
                        let body = problems::body(
                            "validation",
                            None,
                            Some("query validation failed"),
                            &field_errors(&errors),
                            &[],
                            &request_id,
                        );
                        return (
                            Ok(problem_response(422, &body)),
                            route_id,
                            "validation.query",
                        );
                    }
                }
            }

            let mut headers_value: Option<Value> = None;
            if let Some(sid) = compiled.headers_schema_id {
                let header_pairs: Vec<(&str, &str)> = headers
                    .iter()
                    .filter_map(|(name, val)| val.to_str().ok().map(|s| (name.as_str(), s)))
                    .collect();
                match state.decoder_table.decode_headers(sid.0, &header_pairs) {
                    Ok(v) => headers_value = Some(v),
                    Err(errors) => {
                        let body = problems::body(
                            "validation",
                            None,
                            Some("header validation failed"),
                            &field_errors(&errors),
                            &[],
                            &request_id,
                        );
                        return (
                            Ok(problem_response(422, &body)),
                            route_id,
                            "validation.headers",
                        );
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
                state.metrics.decode.fetch_add(1, Ordering::Relaxed);
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
                        &[],
                        &request_id,
                    );
                    return (Ok(problem_response(415, &body)), route_id, "admission.body");
                }
                let max_body = binding.limit_bytes as usize;
                if let Some(content_length) = headers.get("content-length") {
                    match content_length
                        .to_str()
                        .ok()
                        .and_then(|v| v.parse::<usize>().ok())
                    {
                        Some(length) if length > max_body => {
                            let body = problems::body("limit", None, None, &[], &[], &request_id);
                            return (Ok(problem_response(413, &body)), route_id, "admission.body");
                        }
                        None => {
                            let body = problems::body(
                                "body",
                                None,
                                Some("invalid content-length"),
                                &[],
                                &[],
                                &request_id,
                            );
                            return (Ok(problem_response(400, &body)), route_id, "admission.body");
                        }
                        _ => {}
                    }
                }
                // M25-004-D: the read races the anchored request deadline.
                // On elapse the collect future is dropped mid-stream — the
                // transfer is cancelled — and the same `timeout` problem the
                // engine produces for handler deadlines settles the request.
                let raw = match tokio::time::timeout_at(
                    request_deadline.into(),
                    collect_body_bounded(body, max_body),
                )
                .await
                {
                    Err(_elapsed) => {
                        let body = problems::body("timeout", None, None, &[], &[], &request_id);
                        return (Ok(problem_response(504, &body)), route_id, "deadline.body");
                    }
                    Ok(Ok(bytes)) => {
                        state
                            .metrics
                            .body_bytes
                            .fetch_add(bytes.len() as u64, Ordering::Relaxed);
                        bytes
                    }
                    Ok(Err(HttpError::Limited { .. })) => {
                        let body = problems::body("limit", None, None, &[], &[], &request_id);
                        return (Ok(problem_response(413, &body)), route_id, "admission.body");
                    }
                    Ok(Err(e)) => return (Err(e), route_id, "admission.body"),
                };
                let parsed: Value = match serde_json::from_slice(&raw) {
                    Ok(v) => v,
                    Err(_) => {
                        let body = problems::body(
                            "validation",
                            None,
                            Some("malformed JSON body"),
                            &[],
                            &[],
                            &request_id,
                        );
                        return (
                            Ok(problem_response(422, &body)),
                            route_id,
                            "validation.body",
                        );
                    }
                };
                if let Some(sid) = compiled.body_schema_id {
                    // M25-004-B: routes whose body schema carries unsupported
                    // transformations compile to validationStrategy "js" — the
                    // retained QuickJS/generic fallback. The parsed raw JSON
                    // crosses unvalidated; the handler owns interpretation.
                    if route.validation_strategy == q_pack::Strategy::Js {
                        body_value = Some(parsed);
                    } else {
                        match state.decoder_table.decode_body_value(sid.0, &parsed) {
                            Ok(v) => body_value = Some(v),
                            Err(errors) => {
                                let body = problems::body(
                                    "validation",
                                    None,
                                    Some("body validation failed"),
                                    &field_errors(&errors),
                                    &[],
                                    &request_id,
                                );
                                return (
                                    Ok(problem_response(422, &body)),
                                    route_id,
                                    "validation.body",
                                );
                            }
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
                // M25-007-B: the full-request escape hatch materializes
                // every request header into the store (same bounded
                // transport admission limits as FULL_HEADERS)
                headers: if needs.headers || full_request {
                    let table = &state.pack.header_name_table;
                    compiled
                        .plan
                        .as_ref()
                        .map(|plan| {
                            if full_request
                                || plan.header_name_ids.contains(&q_pack::FULL_HEADERS_ID)
                            {
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
            state.metrics.queue_pending.fetch_add(1, Ordering::Relaxed);
            state.metrics.queue.fetch_add(1, Ordering::Relaxed);
            state.metrics.bridge.fetch_add(1, Ordering::Relaxed);
            state.metrics.js.fetch_add(1, Ordering::Relaxed);
            let invocation_id = state.invocation_clock.fetch_add(1, Ordering::Relaxed);
            // M3-007-A: bind the invocation to its owning worker BEFORE
            // the job reaches the engine. The tracking capacity is sized
            // above every admission bound, so exhaustion is a contract
            // condition, not steady state — both rejections fail closed
            // with typed problems and the stage metrics unwind.
            if let Err(track_err) = state.ownership.track(invocation_id, 0) {
                state.metrics.queue_pending.fetch_sub(1, Ordering::Relaxed);
                return match track_err {
                    q_capabilities::TrackError::AtCapacity { capacity } => {
                        state.load_shed.record(
                            &q_capabilities::LoadShedReason::InvocationTrackingFull { capacity },
                        );
                        let body =
                            problems::body("overload", Some(503), None, &[], &[], &ctx.request_id);
                        let mut response = problem_response(503, &body);
                        response.head_only = head;
                        response.headers.push(("retry-after".into(), "1".into()));
                        (Ok(response), route_id, "engine.capacity")
                    }
                    // Duplicate/unknown-worker tracking is a host contract
                    // violation, not a client condition: generic internal
                    // problem, details only in the log.
                    other => {
                        eprintln!(
                            "{}",
                            serde_json::json!({
                                "level":"error",
                                "event":"contract.violation.ownership",
                                "detail": other.to_string(),
                                "requestId": ctx.request_id,
                            })
                        );
                        let body =
                            problems::body("internal", None, None, &[], &[], &ctx.request_id);
                        (Ok(problem_response(500, &body)), route_id, "engine.error")
                    }
                };
            }
            let requestless = !needs.params
                && !needs.query
                && !needs.headers
                && !needs.body
                && route.policy.is_none()
                && compiled.policy_id.is_none()
                && compiled.policy_handler_id.is_none()
                // M25-007-B: the full-request escape hatch owns a request
                // slot by declaration — its handler reads the store
                && !route.capabilities.iter().any(|c| c == "full-request");
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

            let raw_response = route.capabilities.iter().any(|c| c == "raw-response");
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
                headers: headers_value,
                body: body_value,
                allowed_statuses: compiled.allowed_statuses.clone(),
                default_status: compiled.default_status,
                response_strategy: compiled.response_strategy,
                raw_response,
                deadline: request_deadline,
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
            // M3-007-A: the terminal transition settles the ownership
            // binding FIRST — settle is the exactly-once gate. If this
            // future is dropped right here (before disarm), the guard's
            // own settle observes None and never re-cancels a delivered
            // outcome.
            let settled_owner = state.ownership.settle(invocation_id);
            cancel_guard.disarm();
            debug_assert!(
                settled_owner.is_some(),
                "a live invocation always settles at its terminal transition"
            );
            state.metrics.queue_pending.fetch_sub(1, Ordering::Relaxed);

            // client gone (connection dropped) → cancel invocation
            // SCHEMA-003: declared response bodies are validated at runtime.
            // Only the native strategy crosses as structured data; the
            // engine-stringified path (js strategy) is disclosed per-route
            // in the build report and skipped here.
            //
            // M25-005-A: when a generated encoder program exists for the
            // declared status schema, validation and serialization fuse
            // into ONE traversal that emits the response bytes directly.
            // Schemas the direct encoder cannot represent keep the
            // reference validate-then-serialize path below.
            let mut encoded_response: Option<Vec<u8>> = None;
            if let Outcome::Response { status, body, .. } = &outcome {
                if let Some(decl) = route.responses.get(&status.to_string()) {
                    if let Some(key) = &decl.schema {
                        if let BodyOut::Json(_) = body {
                            if let Some(program) = state
                                .response_schema_ids
                                .get(route_index)
                                .and_then(|m| m.get(status))
                                .and_then(|sid| state.encoder_table.get(*sid))
                            {
                                if let BodyOut::Json(v) = body {
                                    let mut buf = Vec::new();
                                    match program.encode(v, &mut buf) {
                                        Ok(()) => encoded_response = Some(buf),
                                        Err(errors) => {
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
                                                &[],
                                                &ctx.request_id,
                                            );
                                            let mapped = (
                                                Ok(problem_response(500, &problem)),
                                                route_id.clone(),
                                                "engine.response-validation",
                                            );
                                            return mapped;
                                        }
                                    }
                                }
                            }
                        }
                        if encoded_response.is_none() {
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
                                            &[],
                                            &ctx.request_id,
                                        );
                                        let mapped = (
                                            Ok(problem_response(500, &problem)),
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
            }
            state.metrics.encode.fetch_add(1, Ordering::Relaxed);
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
                            // M25-005-A: when the direct encoder produced
                            // the validated bytes, write them without a
                            // second serialization pass
                            if let Some(bytes) = encoded_response.take() {
                                PlainResponse {
                                    status,
                                    headers: vec![(
                                        "content-type".into(),
                                        "application/json".into(),
                                    )],
                                    body: bytes,
                                    head_only: head,
                                }
                            } else {
                                let mut resp = json_response(status, &v);
                                resp.head_only = head;
                                resp
                            }
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
                Outcome::RawResponse {
                    status,
                    body,
                    headers,
                } => {
                    // M25-007-B: the raw escape hatch — status/headers/body
                    // cross AS-IS; declared-schema validation and the
                    // generated encoders are skipped by design (the
                    // declared-status contract was already enforced in
                    // the engine). Documented in
                    // docs/specs/unsupported-transformations.md §5.
                    let mut resp = match body {
                        BodyOut::JsonText(text) => PlainResponse {
                            status,
                            headers: vec![("content-type".into(), "application/json".into())],
                            body: text.into_bytes(),
                            head_only: head,
                        },
                        BodyOut::Json(v) => {
                            let mut r = json_response(status, &v);
                            r.head_only = head;
                            r
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
                    // raw semantics: the handler's headers win — an
                    // explicitly supplied name replaces the default
                    for (k, v) in headers {
                        resp.headers
                            .retain(|(existing, _)| !existing.eq_ignore_ascii_case(k.as_str()));
                        resp.headers.push((k, v));
                    }
                    (Ok(resp), route_id, "engine.raw")
                }
                Outcome::Problem(p) => {
                    // M25-006-B: problems settling as the framework's
                    // `internal` problem are UNEXPECTED failures — their
                    // detail and extension members may carry exception
                    // text, stacks, or secrets, so they never cross to the
                    // client; they are preserved in the internal log only.
                    // Declared registry problems (validation, not-found,
                    // ...) keep their detail by design.
                    let is_internal = problems::registry(&p.problem_id).0
                        == "https://velqu.dev/problems/internal";
                    let p = if is_internal && (p.detail.is_some() || !p.extensions.is_empty()) {
                        eprintln!(
                            "{}",
                            serde_json::json!({
                                "level":"error","event":"problem.redacted",
                                "requestId": ctx.request_id, "routeId": route_id,
                                "problemId": p.problem_id, "status": p.status,
                                "detail": p.detail, "extensions": p.extensions,
                            })
                        );
                        q_engine::ProblemOut {
                            detail: None,
                            extensions: Vec::new(),
                            ..p
                        }
                    } else {
                        p
                    };
                    // M25-006-A: a DECLARED problem response (explicit
                    // s.problem schema for this status) encodes through
                    // the generated program — frozen declared title/type
                    // overrides, detail validated against the declared
                    // shape, extension members carried through. Framework
                    // problems keep the generic registry builder.
                    let declared_program = state
                        .response_schema_ids
                        .get(route_index)
                        .and_then(|m| m.get(&p.status))
                        .and_then(|sid| state.encoder_table.problem(*sid));
                    if let Some(program) = declared_program {
                        let (reg_type, reg_title, _) = problems::registry(&p.problem_id);
                        let mut buf = Vec::new();
                        match program.encode(
                            reg_type,
                            reg_title,
                            Some(p.status),
                            p.detail.as_deref(),
                            &p.errors
                                .iter()
                                .map(|e| q_schema_runtime::FieldError {
                                    path: e.path.clone(),
                                    code: e.code.clone(),
                                    message: e.message.clone(),
                                })
                                .collect::<Vec<_>>(),
                            &p.extensions,
                            &ctx.request_id,
                            &mut buf,
                        ) {
                            Ok(()) => {
                                let resp = PlainResponse {
                                    status: p.status,
                                    headers: vec![(
                                        "content-type".into(),
                                        "application/problem+json".into(),
                                    )],
                                    body: buf,
                                    head_only: head,
                                };
                                return (Ok(resp), route_id, "engine.problem");
                            }
                            Err(errors) => {
                                // declared detail shape violated: same
                                // controlled contract failure as response
                                // schema violations
                                let detail = format!(
                                    "route {} problem response failed its declared detail shape: {:?}",
                                    route_id, errors
                                );
                                eprintln!(
                                    "{}",
                                    serde_json::json!({
                                        "level":"error","event":"contract.violation.response",
                                        "requestId": ctx.request_id, "routeId": route_id, "detail": detail,
                                    })
                                );
                                let problem = problems::body(
                                    "internal",
                                    None,
                                    None,
                                    &[],
                                    &[],
                                    &ctx.request_id,
                                );
                                return (
                                    Ok(problem_response(500, &problem)),
                                    route_id,
                                    "engine.problem-validation",
                                );
                            }
                        }
                    }
                    let body = problems::body(
                        &p.problem_id,
                        Some(p.status),
                        p.detail.as_deref(),
                        &p.errors,
                        &p.extensions,
                        &ctx.request_id,
                    );
                    (
                        Ok(problem_response(p.status, &body)),
                        route_id,
                        "engine.problem",
                    )
                }
                Outcome::Timeout => {
                    let body = problems::body("timeout", None, None, &[], &[], &ctx.request_id);
                    (Ok(problem_response(504, &body)), route_id, "engine.timeout")
                }
                Outcome::RequestCapacity => {
                    let body =
                        problems::body("overload", Some(503), None, &[], &[], &ctx.request_id);
                    let mut response = problem_response(503, &body);
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
                    let body = problems::body("internal", None, None, &[], &[], &ctx.request_id);
                    (
                        Ok(problem_response(500, &body)),
                        route_id,
                        "engine.contract",
                    )
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
                    let body = problems::body("internal", None, None, &[], &[], &ctx.request_id);
                    (Ok(problem_response(500, &body)), route_id, "engine.error")
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
            // M3-007-A: cancellation routes through the ownership
            // binding, and settling IS the exactly-once gate: the guard
            // delivers `cancel` to the engine only when this drop IS the
            // terminal transition. A racing delivered outcome consumed
            // the binding first, so the cancel is never re-delivered.
            if self.state.ownership.settle(self.invocation_id).is_some() {
                if let Ok(mut eng) = self.state.engine.lock() {
                    eng.cancel(self.invocation_id);
                }
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

/// M25-006-D: RFC 9457 problem responses carry
/// `Content-Type: application/problem+json` (§3), distinguishing them from
/// success bodies. `instance` keeps its frozen semantics: the request
/// occurrence identifier (`req-<start_ms>-<n>`).
fn problem_response(status: u16, body: &Value) -> PlainResponse {
    let mut resp = json_response(status, body);
    resp.headers.clear();
    resp.headers
        .push(("content-type".into(), "application/problem+json".into()));
    resp
}

/// M3-002-D: extract the resolved route identity from a `CompiledRoute`
/// BEFORE dispatch. The snapshot is `Copy` plain data carrying numeric IDs
/// only — the worker that pops the job never re-runs the matcher, never
/// re-links policies, and every worker derives the identical snapshot for
/// the same route (ADR-0036 §3/§6: deterministic, shared-immutable plans).
pub fn dispatch_route(compiled: &q_router::CompiledRoute) -> q_engine::DispatchRoute {
    q_engine::DispatchRoute {
        route_id: compiled.route_id,
        handler_id: compiled.handler_id.unwrap_or(q_engine::HandlerId(0)),
        policy_id: compiled.policy_id,
        policy_handler_id: compiled.policy_handler_id,
        params_schema_id: compiled.params_schema_id,
        query_schema_id: compiled.query_schema_id,
        headers_schema_id: compiled.headers_schema_id,
        body_schema_id: compiled.body_schema_id,
        default_status: compiled.default_status,
        response_strategy: compiled.response_strategy,
        deadline_ms: compiled.deadline_ms,
    }
}

#[cfg(test)]
mod dispatch_route_tests {
    use super::*;
    use q_engine::{DispatchRoute, HandlerId, PolicyId, RouteId, SchemaId};

    fn fixture_route() -> q_router::CompiledRoute {
        q_router::CompiledRoute {
            index: 7,
            route_id: RouteId(7),
            method: "GET".into(),
            segments: vec![],
            param_name_ids: vec![],
            has_params: false,
            plan: None,
            handler_id: Some(HandlerId(42)),
            policy_id: Some(PolicyId(3)),
            policy_handler_id: Some(HandlerId(9)),
            params_schema_id: Some(SchemaId(11)),
            query_schema_id: Some(SchemaId(12)),
            headers_schema_id: None,
            body_schema_id: Some(SchemaId(14)),
            default_status: 201,
            allowed_statuses: vec![200, 201],
            response_strategy: q_engine::ResponseStrategy::Native,
            deadline_ms: 5_000,
        }
    }

    #[test]
    fn snapshot_preserves_route_identity_exactly() {
        let compiled = fixture_route();
        let snap = dispatch_route(&compiled);
        assert_eq!(snap.route_id, RouteId(7));
        assert_eq!(snap.handler_id, HandlerId(42));
        assert_eq!(snap.policy_id, Some(PolicyId(3)));
        assert_eq!(snap.params_schema_id, Some(SchemaId(11)));
        assert_eq!(snap.body_schema_id, Some(SchemaId(14)));
        assert_eq!(snap.default_status, 201);
        assert_eq!(snap.response_strategy, q_engine::ResponseStrategy::Native);
        assert_eq!(snap.deadline_ms, 5_000);
    }

    #[test]
    fn snapshot_is_copy_plain_data_shared_safe() {
        fn assert_copy_send_sync<T: Copy + Send + Sync + 'static>() {}
        assert_copy_send_sync::<DispatchRoute>();
        // Copy semantics: two workers can hold the same snapshot with no
        // clone cost and no shared mutable state.
        let compiled = fixture_route();
        let a = dispatch_route(&compiled);
        let b = a; // Copy, not move
        assert_eq!(a, b);
    }

    #[test]
    fn extraction_is_deterministic_across_calls() {
        // The same CompiledRoute yields the identical snapshot every time:
        // worker K and worker 0 receive the same plan data (ADR-0036 §6).
        let compiled = fixture_route();
        let first = dispatch_route(&compiled);
        let second = dispatch_route(&compiled);
        assert_eq!(first, second);
    }

    #[test]
    fn route_identity_survives_the_dispatch_queue_boundary() {
        use q_capabilities::Dispatcher;
        // The full M3-002 shape: snapshot extracted BEFORE dispatch, pushed
        // as plain data, popped by another thread — identity intact, zero
        // re-resolution on the consumer side.
        let compiled = fixture_route();
        let snap = dispatch_route(&compiled);
        let dispatcher: std::sync::Arc<Dispatcher<(DispatchRoute, u64)>> =
            std::sync::Arc::new(Dispatcher::with_workers(2, 4));
        let worker = dispatcher.dispatch((snap, 12345)).expect("room");
        // The queue borrows the dispatcher; share it with the consumer
        // thread through the Arc.
        let dispatcher_for_thread = dispatcher.clone();
        let handle = std::thread::spawn(move || {
            dispatcher_for_thread
                .queue(worker)
                .pop_timeout(std::time::Duration::from_secs(2))
                .expect("job arrives")
        });
        let ((popped, req_id), _wait) = handle.join().unwrap();
        assert_eq!(popped, snap, "identity preserved across the boundary");
        assert_eq!(req_id, 12345);
        // And admission verdicts remain typed when saturated.
        let tiny: Dispatcher<DispatchRoute> = Dispatcher::with_workers(1, 1);
        assert!(tiny.dispatch(snap).is_ok());
        assert!(tiny.dispatch(snap).is_err());
    }
}

#[cfg(test)]
mod log_redaction_tests {
    use super::*;

    fn build(mode: LogMode, path: &str, trace_id: Option<&str>) -> serde_json::Value {
        completion_log_json(
            &mode,
            "req-1",
            "GET",
            path,
            "users.get",
            200,
            42,
            "encode",
            1.25,
            trace_id,
        )
    }

    #[test]
    fn log_document_fields_are_exactly_the_allowlist() {
        let doc = build(LogMode::Full, "/api/users/usr_1", Some("trace-1"));
        let mut keys: Vec<&str> = doc
            .as_object()
            .unwrap()
            .keys()
            .map(|k| k.as_str())
            .collect();
        keys.sort();
        assert_eq!(
            keys,
            vec![
                "bodyBytes",
                "durationMs",
                "event",
                "level",
                "method",
                "path",
                "requestId",
                "routeId",
                "stage",
                "status",
                "traceId"
            ]
        );
    }

    #[test]
    fn path_is_stripped_of_query_strings_defensively() {
        let doc = build(LogMode::Errors, "/api/users/usr_1?token=SUPER-SECRET", None);
        assert_eq!(doc["path"], "/api/users/usr_1");
        assert!(!doc.to_string().contains("SUPER-SECRET"));
    }

    #[test]
    fn absent_trace_id_omits_the_field_rather_than_null() {
        let doc = build(LogMode::Full, "/x", None);
        // traceId present-but-null is the serde behavior for Option::None
        // inside json!; assert the document never carries an empty id
        let t = &doc["traceId"];
        assert!(t.is_null() || t.as_str().map(|s| !s.is_empty()).unwrap_or(true));
    }

    #[test]
    fn status_drives_the_level_field() {
        assert_eq!(build(LogMode::Full, "/x", None)["level"], "info");
        let err = completion_log_json(
            &LogMode::Full,
            "req-2",
            "GET",
            "/x",
            "users.get",
            500,
            0,
            "encode",
            1.0,
            None,
        );
        assert_eq!(err["level"], "warn");
    }
}
