//! q-engine — the narrow, engine-agnostic engine boundary (ADR-0006).
//!
//! Only one implementation exists in M0–M2 (`q-engine-quickjs`, quickjs-ng via
//! rquickjs). The seam exists so upstream QuickJS remains a measurable
//! alternative without touching the runtime. This trait is internal API.
//!
//! BWASM-K-001: the host-independent model types (IDs, route/contract
//! models, result structures, problem types, stats) live in
//! `q-runtime-model` and are re-exported here unchanged, so every
//! existing native path keeps its identifier. Only the native surface
//! stays local: `InvocationSpec` (its deadline is a host `Instant`) and
//! the tokio-coupled `Engine` trait.

use std::time::Instant;

use serde_json::Value;

pub use q_runtime_model::{
    BodyOut, DispatchRoute, EngineLoadPlan, EngineStats, FieldErrorOut, FieldNeeds, FunctionDecl,
    FunctionKind, HandlerId, LoadStats, OriginalLocation, Outcome, ParamSpec, PolicyId, ProblemOut,
    RequestMeta, ResponseStrategy, RouteId, SchemaId, SourceLocation, NO_REQUEST_SLOT,
};

/// Cross-target model identity re-exported beside the types it versions.
pub use q_runtime_model::MODEL_ABI_VERSION;

/// Native invocation job. NOT part of the portable model: `deadline` is a
/// host `Instant` (no browser meaning); the browser kernel defines its
/// own message ABI (BWASM-K-005) over the portable types.
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
    /// M24-003-A: request data is moved into the worker and inserted into its
    /// local slab immediately before invocation. The host never allocates a
    /// worker slot or publishes a generation.
    pub request: Option<RequestMeta>,
    /// M24-003-B will replace these raw capability fields with a typed handle.
    /// They remain as the JS ABI pair during the A packet.
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
    /// M25-007-B: the route declared the `raw-response` capability —
    /// handlers may return tagged raw envelopes; without it a raw return
    /// is a contract violation (fallback never activates silently).
    pub raw_response: bool,
    pub deadline: Instant,
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
