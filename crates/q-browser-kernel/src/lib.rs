//! q-browser-kernel — the Rust/WASM request kernel for the Velqu
//! Browser-WASM runtime (BWASM-K-005, ADR-0037 §1/§6).
//!
//! The kernel owns every compatibility-critical step around the
//! handler: artifact initialization with integrity/compatibility
//! checks, request planning (routing, parameter materialization, schema
//! validation, capability authorization), and handler-result
//! completion (declared-status and response-schema validation, problem
//! normalization). Generated TypeScript handlers execute in an isolated
//! Worker (ADR-0037 §3); everything here is host-independent Rust that
//! compiles for wasm32 with no host runtime.
//!
//! # ABI (versioned message boundary)
//!
//! All boundary crossings are bounded JSON messages with an explicit
//! `abiVersion`. [`KERNEL_ABI_VERSION`] must match on both sides; a
//! mismatch is a stable `abi` problem, never a crash. Message sizes
//! are capped ([`MAX_MESSAGE_BYTES`], [`MAX_PACK_BYTES`]).
//!
//! The wasm-bindgen surface is behind the `bindgen` feature so native
//! and wasm32-wasip1 test builds exercise the same kernel core without
//! the JS shim layer; the browser cdylib build enables it.
//!
//! # Problems
//!
//! Every failure is a typed problem with a stable registry id and the
//! native problem URIs (`https://velqu.dev/problems/...`), plus two
//! kernel-specific ids documented here: `artifact` (initialization
//! failed: integrity, compatibility, or size) and `capability`
//! (declared capability absent from the artifact inventory — the
//! ADR-0037 §5 deployment-required class). Panic paths never become
//! success: the bindgen layer catches Rust panics at the boundary and
//! converts them to `internal` problems.

use std::collections::HashMap;

use q_pack::{BytecodePolicy, QPack};
use q_router::{MatchResult, Router};
use q_schema_runtime::{SchemaIr, Source};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Kernel/JS ABI version of the message boundary. Bump on any change
/// to a message shape; both sides must agree.
pub const KERNEL_ABI_VERSION: u16 = 1;

/// Hard cap on pack bytes accepted at initialization (16 MiB).
pub const MAX_PACK_BYTES: usize = 16 << 20;
/// Hard cap on any single ABI message (1 MiB) — requests, plans,
/// completions, and responses alike.
pub const MAX_MESSAGE_BYTES: usize = 1 << 20;

// ---------------------------------------------------------------------------
// Problem model (stable JSON shape; field order fixed by this struct)
// ---------------------------------------------------------------------------

pub mod problem_ids {
    pub const NOT_FOUND: &str = "not-found";
    pub const METHOD: &str = "method";
    pub const VALIDATION: &str = "validation";
    pub const BODY: &str = "body";
    pub const LIMIT: &str = "limit";
    pub const INTERNAL: &str = "internal";
    /// Kernel-specific: artifact initialization failed (integrity,
    /// compatibility, or size). Not a request-path problem.
    pub const ARTIFACT: &str = "artifact";
    /// Kernel-specific: a declared capability is absent from the
    /// artifact inventory (ADR-0037 §5 deployment-required class).
    pub const CAPABILITY: &str = "capability";
    /// Kernel-specific: ABI version mismatch on a boundary message.
    pub const ABI: &str = "abi";
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FieldError {
    pub path: String,
    pub code: String,
    pub message: String,
}

impl FieldError {
    fn from_schema(e: q_schema_runtime::FieldError) -> Self {
        FieldError {
            path: e.path,
            code: e.code,
            message: e.message,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KernelProblem {
    pub problem_id: String,
    #[serde(rename = "type")]
    pub type_uri: String,
    pub title: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub errors: Vec<FieldError>,
    /// 405 only: sorted allowed methods.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub allow: Vec<String>,
}

impl KernelProblem {
    fn registry(id: &str, status: u16, title: &str) -> Self {
        KernelProblem {
            problem_id: id.to_string(),
            type_uri: format!("https://velqu.dev/problems/{id}"),
            title: title.to_string(),
            status,
            detail: None,
            errors: Vec::new(),
            allow: Vec::new(),
        }
    }

    fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    fn not_found() -> Self {
        Self::registry(problem_ids::NOT_FOUND, 404, "Not Found")
    }
    fn method(allow: Vec<String>) -> Self {
        let mut p = Self::registry(problem_ids::METHOD, 405, "Method Not Allowed");
        p.allow = allow;
        p
    }
    fn validation(errors: Vec<FieldError>) -> Self {
        let mut p = Self::registry(problem_ids::VALIDATION, 400, "Validation Failed");
        p.errors = errors;
        p
    }
    fn body(detail: impl Into<String>) -> Self {
        Self::registry(problem_ids::BODY, 400, "Unsupported body").with_detail(detail)
    }
    fn limit(detail: impl Into<String>) -> Self {
        Self::registry(problem_ids::LIMIT, 413, "Payload too large").with_detail(detail)
    }
    fn internal(detail: impl Into<String>) -> Self {
        Self::registry(problem_ids::INTERNAL, 500, "Internal").with_detail(detail)
    }
    pub fn artifact(detail: impl Into<String>) -> Self {
        Self::registry(problem_ids::ARTIFACT, 500, "Artifact rejected").with_detail(detail)
    }
    fn capability(cap: &str) -> Self {
        Self::registry(problem_ids::CAPABILITY, 501, "Capability unavailable")
            .with_detail(format!(
                "route declares capability {cap:?} which the artifact inventory does not carry (deployment-required; ADR-0037)"
            ))
    }
    fn abi(expected: u16, got: u16) -> Self {
        Self::registry(problem_ids::ABI, 400, "ABI mismatch").with_detail(format!(
            "kernel ABI {expected} cannot serve message ABI {got}"
        ))
    }
}

// ---------------------------------------------------------------------------
// Message shapes (camelCase wire forms)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanRequest {
    pub abi_version: u16,
    pub method: String,
    pub path: String,
    #[serde(default)]
    pub query: Vec<(String, String)>,
    #[serde(default)]
    pub headers: Vec<(String, String)>,
    /// Raw request-body text (JSON), if any.
    #[serde(default)]
    pub body: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanInvoke {
    pub kind: &'static str,
    pub abi_version: u16,
    pub route_id: u32,
    pub handler_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    pub allowed_statuses: Vec<u16>,
    pub default_status: u16,
    pub deadline_ms: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanProblem {
    pub kind: &'static str,
    pub problem: KernelProblem,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Completion {
    pub abi_version: u16,
    pub route_id: u32,
    pub result: HandlerResult,
}

#[derive(Debug, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum HandlerResult {
    Response {
        status: u16,
        #[serde(default)]
        headers: Vec<(String, String)>,
        #[serde(default)]
        body: Option<serde_json::Value>,
    },
    Problem {
        problem_id: String,
        #[serde(default)]
        detail: Option<String>,
        #[serde(default)]
        errors: Vec<FieldErrorIn>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldErrorIn {
    pub path: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteResponse {
    pub kind: &'static str,
    pub status: u16,
    pub headers: Vec<(String, String)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Kernel
// ---------------------------------------------------------------------------

/// Initialized browser kernel. Construction verifies the artifact;
/// every later call is plan/complete around handler execution. Drop
/// disposes everything (explicit `dispose` exists on the bindgen layer
/// for JS visibility).
pub struct BrowserKernel {
    pack: QPack,
    router: Router,
    schemas_by_key: HashMap<String, SchemaIr>,
    inventory: Vec<(String, u32)>,
}

impl std::fmt::Debug for BrowserKernel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BrowserKernel")
            .field("app_id", &self.pack.app_id)
            .field("routes", &self.pack.routes.len())
            .field("capabilities", &self.inventory.len())
            .finish_non_exhaustive()
    }
}

impl BrowserKernel {
    /// Initialize from pack bytes with full integrity and
    /// compatibility verification (fail closed — ADR-0026 integrity
    /// semantics; ADR-0037 §4: browser artifacts are integrity-only).
    /// Browser handlers evaluate from the SOURCE bundle: the embedded
    /// QuickJS bytecode policy is `Skip` (native bytecode has no
    /// browser meaning — ADR-0037 §8).
    pub fn init(pack_bytes: &[u8]) -> Result<BrowserKernel, Box<KernelProblem>> {
        if pack_bytes.len() > MAX_PACK_BYTES {
            return Err(Box::new(KernelProblem::artifact(format!(
                "pack is {} bytes; browser kernel accepts at most {MAX_PACK_BYTES}",
                pack_bytes.len()
            ))));
        }
        if let Err(e) = q_pack::reject_mixed_mode_bytes(pack_bytes) {
            return Err(Box::new(KernelProblem::artifact(format!(
                "mixed-mode artifact: {e}"
            ))));
        }
        // Browser compatibility: the pack must not demand native-only
        // engine identity mismatches are caught by verify; source path
        // via Skip policy.
        let pack = QPack::verify_from_slice(pack_bytes, BytecodePolicy::Skip)
            .map_err(|e| Box::new(KernelProblem::artifact(format!("verification failed: {e}"))))?;
        let router = Router::build(&pack.routes)
            .map_err(|e| Box::new(KernelProblem::artifact(format!("router build failed: {e}"))))?;
        let schemas_by_key = pack
            .schema_manifest
            .iter()
            .map(|d| (d.key.clone(), d.ir.clone()))
            .collect();
        let inventory = pack
            .capability_inventory
            .as_ref()
            .map(|entries| entries.iter().map(|e| (e.id.clone(), e.version)).collect())
            .unwrap_or_default();
        Ok(BrowserKernel {
            pack,
            router,
            schemas_by_key,
            inventory,
        })
    }

    pub fn abi_version() -> u16 {
        KERNEL_ABI_VERSION
    }

    /// Plan a request: route (K-003 semantics), materialize raw path
    /// parameters, validate declared schemas, authorize declared
    /// capabilities — or return a complete problem. The handler Worker
    /// receives the returned message verbatim.
    pub fn plan_request(&self, request_json: &str) -> String {
        if request_json.len() > MAX_MESSAGE_BYTES {
            return plan_problem(KernelProblem::limit(format!(
                "request message is {} bytes; ABI accepts at most {MAX_MESSAGE_BYTES}",
                request_json.len()
            )));
        }
        let req: PlanRequest = match serde_json::from_str(request_json) {
            Ok(r) => r,
            Err(e) => {
                return plan_problem(KernelProblem::body(format!(
                    "request message is not valid ABI JSON: {e}"
                )))
            }
        };
        if req.abi_version != KERNEL_ABI_VERSION {
            return plan_problem(KernelProblem::abi(KERNEL_ABI_VERSION, req.abi_version));
        }

        let found = match self.router.resolve(&req.method, &req.path) {
            MatchResult::NotFound => return plan_problem(KernelProblem::not_found()),
            MatchResult::MethodNotAllowed { allow } => {
                return plan_problem(KernelProblem::method(allow))
            }
            MatchResult::Found {
                route_id,
                route_index,
                param_ranges,
                ..
            } => (route_id, route_index, param_ranges),
        };
        let (route_id, route_index, param_ranges) = found;
        let route = &self.pack.routes[route_index];

        // Capability authorization (ADR-0037 §5): every capability the
        // route declares must be carried by the artifact inventory.
        for cap in &route.capabilities {
            if !self.inventory.iter().any(|(id, _)| id == cap) {
                return plan_problem(KernelProblem::capability(cap));
            }
        }

        // Params: raw path bytes materialized lazily at plan time (the
        // only allocation path — K-003 policy), then validated when a
        // schema is declared.
        let params_pairs = self
            .router
            .materialize_params(route_index, &req.path, &param_ranges);
        let params = match self.validate_pairs("params", route.params.as_ref(), params_pairs) {
            Ok(v) => v,
            Err(p) => return plan_problem(*p),
        };
        let query = match self.validate_pairs("query", route.query.as_ref(), req.query.clone()) {
            Ok(v) => v,
            Err(p) => return plan_problem(*p),
        };
        let header_pairs: Vec<(String, String)> = req
            .headers
            .iter()
            .map(|(k, v)| (k.to_ascii_lowercase(), v.clone()))
            .collect();
        let headers = match self.validate_pairs("headers", route.headers.as_ref(), header_pairs) {
            Ok(v) => v,
            Err(p) => return plan_problem(*p),
        };

        let body = match (&route.body, &req.body) {
            (Some(binding), Some(text)) => match &binding.schema {
                Some(key) => {
                    let value: serde_json::Value = match serde_json::from_str(text) {
                        Ok(v) => v,
                        Err(e) => {
                            return plan_problem(KernelProblem::body(format!(
                                "body is not valid JSON: {e}"
                            )))
                        }
                    };
                    match self.validate_value(key, value, "body") {
                        Ok(v) => Some(v),
                        Err(p) => return plan_problem(*p),
                    }
                }
                None => None, // no schema: raw text crosses (bodyText below)
            },
            (Some(_), None) => None,
            (None, Some(_)) => {
                return plan_problem(KernelProblem::body(
                    "route declares no body but the request carries one",
                ))
            }
            (None, None) => None,
        };
        let body_text = if body.is_none() && route.body.is_some() {
            req.body.clone()
        } else {
            None
        };

        let allowed_statuses: Vec<u16> = route
            .responses
            .keys()
            .filter_map(|k| k.parse().ok())
            .collect();
        let default_status = allowed_statuses.first().copied().unwrap_or(200);

        let out = PlanInvoke {
            kind: "invoke",
            abi_version: KERNEL_ABI_VERSION,
            route_id: route_id.0,
            handler_key: route.handler.clone(),
            policy_key: route.policy.clone(),
            params,
            query,
            headers,
            body,
            body_text,
            allowed_statuses,
            default_status,
            deadline_ms: route.deadline_ms,
        };
        serde_json::to_string(&out)
            .unwrap_or_else(|_| plan_problem(KernelProblem::internal("plan serialization failed")))
    }

    /// Complete an invocation: validate the handler's declared status
    /// and response schema, or normalize a typed problem. The host
    /// receives the returned message verbatim as the Response basis.
    pub fn complete_invocation(&self, completion_json: &str) -> String {
        if completion_json.len() > MAX_MESSAGE_BYTES {
            return plan_problem(KernelProblem::limit(format!(
                "completion message is {} bytes; ABI accepts at most {MAX_MESSAGE_BYTES}",
                completion_json.len()
            )));
        }
        let completion: Completion = match serde_json::from_str(completion_json) {
            Ok(c) => c,
            Err(e) => {
                return plan_problem(KernelProblem::internal(format!(
                    "completion message is not valid ABI JSON: {e}"
                )))
            }
        };
        if completion.abi_version != KERNEL_ABI_VERSION {
            return plan_problem(KernelProblem::abi(
                KERNEL_ABI_VERSION,
                completion.abi_version,
            ));
        }
        let Some(route) = self.pack.routes.get(completion.route_id as usize) else {
            return plan_problem(KernelProblem::internal(format!(
                "completion names unknown route id {}",
                completion.route_id
            )));
        };

        match completion.result {
            HandlerResult::Response {
                status,
                headers,
                body,
            } => {
                let Some(decl) = route.responses.get(&status.to_string()) else {
                    // Native semantics: undeclared runtime status is a
                    // contract failure (500), never a silent 200.
                    return plan_problem(KernelProblem::internal(format!(
                        "handler returned undeclared status {status}"
                    )));
                };
                if let Some(key) = &decl.schema {
                    let value = body.clone().unwrap_or(serde_json::Value::Null);
                    if let Err(field_errors) = self.validate_value_errors(key, &value) {
                        let mut problem = KernelProblem::internal(format!(
                            "response schema violation for declared status {status}"
                        ));
                        problem.errors = field_errors;
                        return plan_problem(problem);
                    }
                }
                let out = CompleteResponse {
                    kind: "response",
                    status,
                    headers,
                    body,
                };
                serde_json::to_string(&out).unwrap_or_else(|_| {
                    plan_problem(KernelProblem::internal("completion serialization failed"))
                })
            }
            HandlerResult::Problem {
                problem_id,
                detail,
                errors,
            } => {
                let (type_uri, title, status) = match problem_id.as_str() {
                    "validation" => ("validation", "Validation Failed", 400u16),
                    "not-found" => ("not-found", "Not Found", 404),
                    "method" => ("method", "Method Not Allowed", 405),
                    "body" => ("body", "Unsupported body", 415),
                    "limit" => ("limit", "Payload too large", 413),
                    "timeout" => ("timeout", "Timeout", 504),
                    "overload" => ("overload", "Overloaded", 503),
                    _ => ("internal", "Internal", 500),
                };
                let problem = KernelProblem {
                    problem_id: problem_id.clone(),
                    type_uri: format!("https://velqu.dev/problems/{type_uri}"),
                    title: title.to_string(),
                    status,
                    detail,
                    errors: errors
                        .into_iter()
                        .map(|e| FieldError {
                            path: e.path,
                            code: e.code,
                            message: e.message,
                        })
                        .collect(),
                    allow: Vec::new(),
                };
                plan_problem(problem)
            }
        }
    }

    /// Capability authorization query for the runtime bridge (used
    /// before forwarding a declared capability call from the Worker).
    pub fn authorize_capability(&self, name: &str) -> String {
        if self.inventory.iter().any(|(id, _)| id == name) {
            serde_json::to_string(&json!({"authorized": true, "capability": name}))
                .unwrap_or_else(|_| plan_problem(KernelProblem::internal("serialize failed")))
        } else {
            plan_problem(KernelProblem::capability(name))
        }
    }

    fn schema(&self, key: &str) -> Option<&SchemaIr> {
        self.schemas_by_key.get(key)
    }

    /// Validate declared string-pair sources (params/query/headers).
    /// Returns Ok(None) when the binding declares no schema (values
    /// cross as raw strings in the plan). FAILS CLOSED: a declared
    /// schema that is missing from the manifest is an internal problem,
    /// never a silent skip.
    fn validate_pairs(
        &self,
        source: &str,
        binding: Option<&q_pack::SourceBinding>,
        pairs: Vec<(String, String)>,
    ) -> Result<Option<serde_json::Value>, Box<KernelProblem>> {
        let Some(binding) = binding else {
            return Ok(None);
        };
        let Some(key) = binding.schema.as_deref() else {
            return Ok(None);
        };
        let ir = self.schema(key).ok_or_else(|| {
            Box::new(KernelProblem::internal(format!(
                "schema {key:?} missing for {source}"
            )))
        })?;
        let src = match source {
            "query" => Source::Query,
            "headers" => Source::Body,
            _ => Source::Path,
        };
        q_schema_runtime::validate(ir, &pairs_value(pairs), src)
            .map(Some)
            .map_err(|errors| {
                Box::new(KernelProblem::validation(
                    errors.into_iter().map(FieldError::from_schema).collect(),
                ))
            })
    }

    fn validate_value_errors(
        &self,
        key: &str,
        value: &serde_json::Value,
    ) -> Result<(), Vec<FieldError>> {
        let ir = self.schema(key).ok_or_else(Vec::new)?;
        q_schema_runtime::validate(ir, value, Source::Body)
            .map(|_| ())
            .map_err(|errors| errors.into_iter().map(FieldError::from_schema).collect())
    }

    fn validate_value(
        &self,
        key: &str,
        value: serde_json::Value,
        at: &str,
    ) -> Result<serde_json::Value, Box<KernelProblem>> {
        let ir = self.schema(key).ok_or_else(|| {
            Box::new(KernelProblem::internal(format!(
                "schema {key:?} missing for {at}"
            )))
        })?;
        q_schema_runtime::validate(ir, &value, Source::Body).map_err(|errors| {
            Box::new(KernelProblem::validation(
                errors.into_iter().map(FieldError::from_schema).collect(),
            ))
        })
    }
}

fn pairs_value(pairs: Vec<(String, String)>) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (k, v) in pairs {
        map.insert(k, serde_json::Value::String(v));
    }
    serde_json::Value::Object(map)
}

fn plan_problem(p: KernelProblem) -> String {
    let out = PlanProblem {
        kind: "problem",
        problem: p,
    };
    serde_json::to_string(&out).unwrap_or_else(|_| {
        "{\"kind\":\"problem\",\"problem\":{\"problemId\":\"internal\",\"type\":\"https://velqu.dev/problems/internal\",\"title\":\"Internal\",\"status\":500}}"
            .to_string()
    })
}

// ---------------------------------------------------------------------------
// wasm-bindgen ABI (browser cdylib builds only)
// ---------------------------------------------------------------------------

#[cfg(feature = "bindgen")]
mod bindgen_abi {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn kernel_abi_version() -> u32 {
        KERNEL_ABI_VERSION as u32
    }

    /// JS-facing kernel handle. Construction failure carries the
    /// artifact problem as the error message (stable JSON).
    #[wasm_bindgen]
    pub struct WasmKernel {
        inner: BrowserKernel,
    }

    #[wasm_bindgen]
    impl WasmKernel {
        #[wasm_bindgen(constructor)]
        pub fn new(pack_bytes: &[u8]) -> Result<WasmKernel, JsError> {
            BrowserKernel::init(pack_bytes)
                .map(|inner| WasmKernel { inner })
                .map_err(|p| JsError::new(&serde_json::to_string(&p).unwrap_or_default()))
        }

        pub fn plan_request(&self, request_json: &str) -> String {
            self.inner.plan_request(request_json)
        }

        pub fn complete_invocation(&self, completion_json: &str) -> String {
            self.inner.complete_invocation(completion_json)
        }

        pub fn authorize_capability(&self, name: &str) -> String {
            self.inner.authorize_capability(name)
        }

        /// Explicit disposal for JS visibility; equivalent to Drop.
        pub fn dispose(self) {}
    }
}

#[cfg(feature = "bindgen")]
pub use bindgen_abi::*;
