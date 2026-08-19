//! q-engine — the narrow, engine-agnostic engine boundary (ADR-0006).
//!
//! Only one implementation exists in M0–M2 (`q-engine-quickjs`, quickjs-ng via
//! rquickjs). The seam exists so upstream QuickJS remains a measurable
//! alternative without touching the runtime. This trait is internal API.

use std::collections::BTreeMap;
use std::time::Instant;

use serde_json::Value;

#[repr(transparent)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct RouteId(pub u32);

#[repr(transparent)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct HandlerId(pub u32);

#[repr(transparent)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct PolicyId(pub u32);

#[repr(transparent)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct SchemaId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldNeeds {
    pub params: bool,
    pub query: bool,
    pub headers: bool,
    pub body: bool,
}

#[derive(Debug, Clone)]
pub struct InvocationSpec {
    pub id: u64,
    pub request_id: String,
    pub route_id: String,
    /// M2.3-r2: Numeric identities for O(1) vector and schema dispatch
    pub route_id_num: Option<RouteId>,
    /// handler-table key (the cached function reference for legacy mode)
    pub handler_key: String,
    pub policy_key: Option<String>,
    /// M2.3: direct vector index into function table ($O(1)$ lookup, zero map/string lookups)
    pub handler_id: Option<HandlerId>,
    pub policy_id_num: Option<PolicyId>,
    pub policy_handler_id: Option<HandlerId>,
    pub params_schema_id: Option<SchemaId>,
    pub query_schema_id: Option<SchemaId>,
    pub headers_schema_id: Option<SchemaId>,
    pub body_schema_id: Option<SchemaId>,
    pub slot: usize,
    pub generation: u64,
    /// Pre-validated inputs (native strategy). `None` = the handler pulls them
    /// lazily through the request store (js strategy).
    pub params: Option<Value>,
    pub query: Option<Value>,
    pub headers: Option<Value>,
    /// Pre-parsed body (native body strategy only).
    pub body: Option<Value>,
    /// Declared response statuses (undeclared runtime status = contract failure).
    pub allowed_statuses: Vec<u16>,
    pub default_status: u16,
    /// Response serialization strategy for this route.
    pub response_strategy: ResponseStrategy,
    pub deadline: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseStrategy {
    /// engine-side JSON.stringify (the JS world serializes)
    Js,
    /// native conversion: JS value traversed into serde_json, serialized in Rust
    Native,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BodyOut {
    /// serde_json::Value serialized by the host (native strategy)
    Json(Value),
    /// engine-stringified JSON text, written as-is (js strategy)
    JsonText(String),
    Text(String),
    Bytes(Vec<u8>),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProblemOut {
    /// registry id: validation | unauthorized | not-found | method | body | limit | timeout | internal | custom:<type-uri>
    pub problem_id: String,
    pub status: u16,
    pub detail: Option<String>,
    pub errors: Vec<FieldErrorOut>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct FieldErrorOut {
    pub path: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum Outcome {
    Response {
        status: u16,
        body: BodyOut,
        headers: Vec<(String, String)>,
    },
    Problem(ProblemOut),
    /// handler deadline exceeded → runtime maps to the `timeout` problem (504)
    Timeout,
    /// undeclared status or shape violation → 500 + contract-failure log
    ContractViolation(String),
    /// JS exception / engine failure → redacted 500, detail logged internally
    EngineFailure {
        detail: String,
        source: Option<SourceLocation>,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct SourceLocation {
    pub generated: Option<(u32, u32)>,
    pub original: Option<OriginalLocation>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct OriginalLocation {
    pub source: String,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LoadStats {
    pub handlers_registered: usize,
    pub eval_ms: f64,
    pub register_calls: usize,
}

/// M2.3-r2: Explicit loading plan decoupling modern numeric vector loading
/// from the legacy string handler table.
#[derive(Debug, Clone)]
pub enum EngineLoadPlan {
    Numeric {
        count: usize,
    },
    Legacy {
        expected_handlers: BTreeMap<String, String>,
    },
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct EngineStats {
    pub invocations: u64,
    pub policy_calls: u64,
    pub handler_calls: u64,
    pub immediate_results: u64,
    pub promise_results: u64,
    pub promise_watches: u64,
    pub job_queue_drains: u64,
    pub settlement_scans: u64,
    pub timer_ops_started: u64,
    pub timer_ops_completed: u64,
    /// Native async operations alive right now (started, not yet completed)
    pub pending_ops: u64,
    /// M2.2.1-r3/r4: physical Tokio task accounting (separate from logical ops —
    /// an aborted op leaves no task alive)
    pub native_tasks_started: u64,
    pub native_tasks_alive: u64,
    pub native_tasks_completed: u64,
    pub native_tasks_aborted: u64,
    /// M2.2.1-r2: worker observed CURRENT_INVOCATION/sync_deadline not reset
    /// at a message boundary — must stay 0
    pub scheduler_boundary_violations: u64,
    /// M2.2.1-r3: job queue holds an unquiescable pathological microtask
    /// chain; drains are skipped until process restart (engine limitation:
    /// quickjs-ng has no interrupt poll points in tiny promise jobs)
    pub queue_poisoned: bool,
    pub poison_events: u64,
    pub late_completions_dropped: u64,
    pub cancelled_invocations: u64,
    pub timeouts: u64,
    pub engine_failures: u64,
    pub contract_violations: u64,
    /// M2.3: Count of direct vector dispatches ($O(1)$) vs legacy string map dispatches
    pub numeric_dispatches: u64,
    pub legacy_map_dispatches: u64,
    /// JS heap bytes in use (last observation)
    pub heap_used: usize,
}

/// Internal engine boundary. The handle lives on the caller side; the
/// implementation owns its worker/thread details.
pub trait Engine: Send {
    /// Evaluate the bundle once according to the specified EngineLoadPlan.
    /// In Numeric mode, loads and validates the exact function vector without legacy maps.
    /// In Legacy mode, verifies the handler table against the expected keys.
    fn load(
        &mut self,
        bundle: &str,
        bytecode: Option<&[u8]>,
        plan: EngineLoadPlan,
    ) -> Result<LoadStats, String>;

    /// Begin an invocation; the outcome arrives on `reply`. Cancellation is
    /// delivered via `cancel`; the reply may never fire if cancelled.
    fn invoke(&mut self, spec: InvocationSpec, reply: tokio::sync::oneshot::Sender<Outcome>);

    /// Cancel a possibly-running invocation; invalidates its request handle.
    fn cancel(&mut self, invocation_id: u64);

    /// Worker-only error mapping hook for diagnostics.
    fn last_error(&self) -> Option<String>;

    fn stats(&self) -> EngineStats;

    /// Stop the worker and release the engine. Idempotent.
    fn shutdown(&mut self);
}
