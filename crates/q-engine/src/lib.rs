//! q-engine — the narrow, engine-agnostic engine boundary (ADR-0006).
//!
//! Only one implementation exists in M0–M2 (`q-engine-quickjs`, quickjs-ng via
//! rquickjs). The seam exists so upstream QuickJS remains a measurable
//! alternative without touching the runtime. This trait is internal API.

use std::collections::BTreeMap;
use std::time::Instant;

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct InvocationSpec {
    pub id: u64,
    pub request_id: String,
    pub route_id: String,
    /// handler-table key (the cached function reference)
    pub handler_key: String,
    pub policy_key: Option<String>,
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

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct EngineStats {
    pub invocations: u64,
    pub policy_calls: u64,
    pub handler_calls: u64,
    pub timer_ops_started: u64,
    pub timer_ops_completed: u64,
    pub late_completions_dropped: u64,
    pub cancelled_invocations: u64,
    pub timeouts: u64,
    pub engine_failures: u64,
    pub contract_violations: u64,
    /// JS heap bytes in use (last observation)
    pub heap_used: usize,
}

/// Internal engine boundary. The handle lives on the caller side; the
/// implementation owns its worker/thread details.
pub trait Engine: Send {
    /// Evaluate the bundle once, verify the handler table against the expected
    /// keys, and cache function references. Never called twice.
    fn load(
        &mut self,
        bundle: &str,
        expected_handlers: &BTreeMap<String, String>,
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
