//! q-pack — versioned application pack reader/verifier (velqu.qpack v1).
//!
//! The pack is the single production artifact (see `docs/specs/pack-format-v1.md`).
//! Everything here is load-and-verify only: no compilation, no discovery.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const PACK_FORMAT_VERSION: u32 = 1;
pub const RUNTIME_ABI: u32 = 1;
pub const SCHEMA_IR_VERSION: u32 = 1;
pub const CONTRACT_VERSION: u32 = 1;
/// Engine this runtime build embeds (quickjs-ng vendored by rquickjs =0.12.2).
pub const ENGINE_NAME: &str = "quickjs-ng";
pub const ENGINE_VERSION: &str = "0.15.1";
pub const ENGINE_BINDING: &str = "rquickjs-0.12.2";

#[derive(Debug, thiserror::Error)]
pub enum PackError {
    #[error("pack io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("pack is not valid JSON: {0}")]
    Malformed(String),
    #[error("pack rejected: {0}")]
    Rejected(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EngineRef {
    pub name: String,
    pub version: String,
    pub binding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BuiltBy {
    pub compiler: String,
    #[serde(default)]
    pub typescript: String,
    #[serde(default)]
    pub bun: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SegKind {
    Static,
    Param,
    Wildcard,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PathSegment {
    pub kind: SegKind,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Strategy {
    #[serde(rename = "native")]
    Native,
    #[default]
    #[serde(rename = "js")]
    Js,
}

impl Strategy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Strategy::Native => "native",
            Strategy::Js => "js",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceBinding {
    /// schema registry key, or null when the route declares no schema for this source
    #[serde(default)]
    pub schema: Option<String>,
    /// path | query (coercion source)
    #[serde(default)]
    pub coerce: Option<String>,
    #[serde(default)]
    pub content_type: Option<String>,
    #[serde(default = "default_body_limit")]
    pub limit_bytes: u64,
}

fn default_body_limit() -> u64 {
    65_536
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseDecl {
    #[serde(default)]
    pub schema: Option<String>,
    #[serde(default)]
    pub strategy: Strategy,
    #[serde(default)]
    pub problem: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LivenessSpec {
    pub status: u16,
    pub content_type: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FieldNeeds {
    #[serde(default)]
    pub params: bool,
    #[serde(default)]
    pub query: bool,
    #[serde(default)]
    pub headers: bool,
    #[serde(default)]
    pub body: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RoutePlanDecl {
    pub route_id: u32,
    pub handler_id: u32,
    #[serde(default)]
    pub policy_id: Option<u32>,
    #[serde(default)]
    pub policy_handler_id: Option<u32>,
    #[serde(default)]
    pub params_schema_id: Option<u32>,
    #[serde(default)]
    pub query_schema_id: Option<u32>,
    #[serde(default)]
    pub headers_schema_id: Option<u32>,
    #[serde(default)]
    pub body_schema_id: Option<u32>,
    #[serde(default)]
    pub default_status: u16,
    #[serde(default)]
    pub allowed_statuses: Vec<u16>,
    #[serde(default)]
    pub field_needs: FieldNeeds,
    #[serde(default)]
    pub response_strategy: Strategy,
    #[serde(default = "default_deadline")]
    pub deadline_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SchemaDecl {
    pub id: u32,
    pub key: String,
    pub ir: q_schema_runtime::SchemaIr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityReq {
    pub scheme: String,
    pub header: String,
    #[serde(default)]
    pub problem_status: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteEntry {
    pub id: String,
    pub module_id: String,
    pub method: String,
    pub path: String,
    /// Pre-compiled segments; if absent the loader rejects the pack (no runtime parsing).
    pub path_segments: Vec<PathSegment>,
    pub handler: String,
    #[serde(default)]
    pub policy: Option<String>,
    #[serde(default)]
    pub params: Option<SourceBinding>,
    #[serde(default)]
    pub query: Option<SourceBinding>,
    #[serde(default)]
    pub body: Option<SourceBinding>,
    #[serde(default)]
    pub headers: Option<SourceBinding>,
    pub responses: BTreeMap<String, ResponseDecl>,
    #[serde(default)]
    pub validation_strategy: Strategy,
    #[serde(default)]
    pub native_liveness: Option<LivenessSpec>,
    #[serde(default)]
    pub security: Vec<SecurityReq>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default = "default_deadline")]
    pub deadline_ms: u64,
    /// M2.3: Precompiled numeric route plan for $O(1)$ dispatch without string parsing
    #[serde(default)]
    pub plan: Option<RoutePlanDecl>,
}

fn default_deadline() -> u64 {
    5_000
}

pub use q_engine::{FunctionDecl, FunctionKind};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SerializedStaticEdge {
    pub segment: String,
    pub target_node: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SerializedTerminal {
    pub method_mask: u16,
    pub route_by_method: [Option<usize>; 7],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SerializedRouterNode {
    #[serde(default)]
    pub static_edges: Vec<SerializedStaticEdge>,
    #[serde(default)]
    pub param_edge: Option<usize>,
    #[serde(default)]
    pub wildcard_edge: Option<usize>,
    #[serde(default)]
    pub terminal: Option<SerializedTerminal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SerializedRouter {
    pub nodes: Vec<SerializedRouterNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyEntry {
    pub id: String,
    pub handler: String,
    pub declared_statuses: Vec<u16>,
    #[serde(default)]
    pub provides: Option<String>,
}

/// QuickJS module bytecode, produced at BUILD time by the exact engine build
/// (ADR-0014/ADR-0017). The runtime loads it only on an exact version match;
/// any mismatch or absence falls back to evaluating `bundle` source.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BytecodeTarget {
    pub arch: String,
    pub os: String,
    pub pointer_width: u8,
    pub endianness: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleBytecode {
    pub quickjs: String,
    pub binding: String,
    /// "little" | "big" — bytecode is not endian-portable
    pub endianness: String,
    /// Compilation target fingerprint (arch, OS, pointer width, endianness)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<BytecodeTarget>,
    /// base64-encoded module bytecode
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Integrity {
    pub algorithm: String,
    pub bundle_sha256: String,
    pub routes_sha256: String,
    /// required when bundleBytecode is present
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytecode_sha256: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QPack {
    pub format_version: u32,
    pub kind: String,
    pub runtime_abi: u32,
    pub engine: EngineRef,
    pub schema_ir_version: u32,
    pub contract_version: u32,
    #[serde(default)]
    pub contract_hash: String,
    #[serde(default)]
    pub built_by: BuiltBy,
    pub app_id: String,
    pub modules: Vec<String>,
    pub entry: String,
    /// "script" (register protocol, M1 form) or "module" (named-export form).
    /// Absent = "script" (backward compatible).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_form: Option<String>,
    pub bundle: String,
    #[serde(default)]
    pub source_map: Option<String>,
    /// Optional build-time bytecode (module form only); see BundleBytecode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_bytecode: Option<BundleBytecode>,
    pub routes: Vec<RouteEntry>,
    /// schema IR registry: key -> IR node (q-schema-runtime types)
    #[serde(default)]
    pub schemas: BTreeMap<String, q_schema_runtime::SchemaIr>,
    #[serde(default)]
    pub policies: BTreeMap<String, PolicyEntry>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub functions: Vec<FunctionDecl>,
    #[serde(default)]
    pub schema_manifest: Vec<SchemaDecl>,
    #[serde(default)]
    pub router: Option<SerializedRouter>,
    pub handler_table: BTreeMap<String, String>,
    pub integrity: Integrity,
}

impl QPack {
    /// Load + fully verify a pack. Fails before any serving can happen.
    pub fn load_and_verify(path: &std::path::Path) -> Result<QPack, PackError> {
        let bytes = std::fs::read(path)?;
        let pack: QPack =
            serde_json::from_slice(&bytes).map_err(|e| PackError::Malformed(e.to_string()))?;
        pack.verify()?;
        Ok(pack)
    }

    pub fn verify(&self) -> Result<(), PackError> {
        let reject = |msg: String| Err(PackError::Rejected(msg));
        if self.kind != "velqu.qpack" {
            return reject(format!("unexpected kind {:?}", self.kind));
        }
        if self.format_version != PACK_FORMAT_VERSION {
            return reject(format!(
                "pack formatVersion {} not supported (runtime supports {})",
                self.format_version, PACK_FORMAT_VERSION
            ));
        }
        if self.runtime_abi != RUNTIME_ABI {
            return reject(format!(
                "runtime ABI {} != pack {}",
                RUNTIME_ABI, self.runtime_abi
            ));
        }
        if self.schema_ir_version != SCHEMA_IR_VERSION {
            return reject(format!(
                "schema IR version {} not supported",
                self.schema_ir_version
            ));
        }
        if self.contract_version != CONTRACT_VERSION {
            return reject(format!(
                "contract version {} not supported",
                self.contract_version
            ));
        }
        if self.engine.name != ENGINE_NAME
            || self.engine.version != ENGINE_VERSION
            || self.engine.binding != ENGINE_BINDING
        {
            return reject(format!(
                "engine mismatch: pack wants {} {} via {}, runtime embeds {} {} via {} (SEC-001 exact match)",
                self.engine.name, self.engine.version, self.engine.binding,
                ENGINE_NAME, ENGINE_VERSION, ENGINE_BINDING
            ));
        }
        // integrity
        let bundle_hash = hex(&Sha256::digest(self.bundle.as_bytes()));
        if bundle_hash != self.integrity.bundle_sha256 {
            return reject(
                "integrity failure: bundle sha256 mismatch (tampered or corrupt pack)".into(),
            );
        }
        if let Some(bc) = &self.bundle_bytecode {
            if bc.quickjs != ENGINE_VERSION || bc.binding != ENGINE_BINDING {
                return reject(format!(
                    "bytecode engine mismatch: pack wants quickjs {} via {}, runtime embeds {} via {} (SEC-001)",
                    bc.quickjs, bc.binding, ENGINE_VERSION, ENGINE_BINDING
                ));
            }
            let expect_endianness = if cfg!(target_endian = "big") {
                "big"
            } else {
                "little"
            };
            if bc.endianness != expect_endianness {
                return reject(format!(
                    "bytecode endianness mismatch: {} vs host {}",
                    bc.endianness, expect_endianness
                ));
            }
            let data = base64_decode(&bc.data)
                .ok_or_else(|| PackError::Rejected("bytecode is not valid base64".into()))?;
            let bc_hash = hex(&Sha256::digest(&data));
            let want = self.integrity.bytecode_sha256.as_ref().ok_or_else(|| {
                PackError::Rejected(
                    "bundleBytecode present without integrity.bytecodeSha256".into(),
                )
            })?;
            if bc_hash != *want {
                return reject(
                    "integrity failure: bytecode sha256 mismatch (tampered or corrupt)".into(),
                );
            }
        } else if self.integrity.bytecode_sha256.is_some() {
            return reject(
                "integrity declares bytecodeSha256 but no bundleBytecode present".into(),
            );
        }
        let routes_hash = self.routes_canonical_sha256();
        if routes_hash != self.integrity.routes_sha256 {
            return reject("integrity failure: routes/schemas/policies sha256 mismatch".into());
        }
        if self.integrity.algorithm != "sha256" {
            return reject(format!(
                "unsupported integrity algorithm {}",
                self.integrity.algorithm
            ));
        }
        // function manifest validation (M2.3 numeric mode)
        if !self.functions.is_empty() {
            let mut seen_keys = std::collections::BTreeSet::new();
            for (idx, fn_decl) in self.functions.iter().enumerate() {
                if fn_decl.id != idx as u32 {
                    return reject(format!(
                        "function manifest id {} does not match index {idx} (must be dense 0..N)",
                        fn_decl.id
                    ));
                }
                if !seen_keys.insert(&fn_decl.key) {
                    return reject(format!("duplicate function key {}", fn_decl.key));
                }
                if !self.handler_table.is_empty() && !self.handler_table.contains_key(&fn_decl.key)
                {
                    return reject(format!(
                        "function manifest key {} not found in handler table",
                        fn_decl.key
                    ));
                }
            }
        }

        // schema manifest validation (M2.3-r2/r3 numeric mode)
        if !self.schema_manifest.is_empty() {
            let mut seen_keys = std::collections::BTreeSet::new();
            for (idx, schema_decl) in self.schema_manifest.iter().enumerate() {
                if schema_decl.id != idx as u32 {
                    return reject(format!(
                        "schema manifest id {} does not match index {idx} (must be dense 0..N)",
                        schema_decl.id
                    ));
                }
                if !seen_keys.insert(&schema_decl.key) {
                    return reject(format!("duplicate schema key {}", schema_decl.key));
                }
                let Some(actual_ir) = self.schemas.get(&schema_decl.key) else {
                    return reject(format!(
                        "schema manifest key {} not found in schemas table",
                        schema_decl.key
                    ));
                };
                if *actual_ir != schema_decl.ir {
                    return reject(format!(
                        "schema manifest entry {} ({}) IR does not match declared schema IR",
                        schema_decl.id, schema_decl.key
                    ));
                }
            }
        }

        // router validation (if pre-compiled router is present)
        if let Some(ref r) = self.router {
            let node_count = r.nodes.len();
            for (n_idx, node) in r.nodes.iter().enumerate() {
                for edge in &node.static_edges {
                    if edge.target_node >= node_count {
                        return reject(format!(
                            "router node {n_idx} static edge target {} out of range ({node_count})",
                            edge.target_node
                        ));
                    }
                }
                if let Some(target) = node.param_edge {
                    if target >= node_count {
                        return reject(format!(
                            "router node {n_idx} param edge target {target} out of range ({node_count})"
                        ));
                    }
                }
                if let Some(target) = node.wildcard_edge {
                    if target >= node_count {
                        return reject(format!(
                            "router node {n_idx} wildcard edge target {target} out of range ({node_count})"
                        ));
                    }
                }
                if let Some(ref t) = node.terminal {
                    for &opt_r_idx in t.route_by_method.iter() {
                        if let Some(r_idx) = opt_r_idx {
                            if r_idx >= self.routes.len() {
                                return reject(format!(
                                    "router node {n_idx} terminal route_index {r_idx} out of range ({})",
                                    self.routes.len()
                                ));
                            }
                        }
                    }
                }
            }
        }

        // handler table sanity
        if self.handler_table.is_empty() && self.functions.is_empty() {
            return reject("empty handler table and empty function manifest".into());
        }
        let mut seen = std::collections::BTreeSet::new();
        for (route_idx, route) in self.routes.iter().enumerate() {
            if !seen.insert(route.id.clone()) {
                return reject(format!("duplicate route id {}", route.id));
            }
            if !self.handler_table.is_empty() && !self.handler_table.contains_key(&route.handler) {
                return reject(format!(
                    "route {} references unknown handler table key {}",
                    route.id, route.handler
                ));
            }
            if let Some(p) = &route.policy {
                let Some(policy_entry) = self.policies.get(p) else {
                    return reject(format!(
                        "route {} references unknown policy {}",
                        route.id, p
                    ));
                };
                if !self.handler_table.is_empty()
                    && !self.handler_table.contains_key(&policy_entry.handler)
                {
                    return reject(format!(
                        "policy {} references unknown handler {}",
                        p, policy_entry.handler
                    ));
                }
            }
            for binding in [&route.params, &route.query, &route.body, &route.headers]
                .into_iter()
                .flatten()
            {
                if let Some(key) = &binding.schema {
                    if !self.schemas.contains_key(key) {
                        return reject(format!(
                            "route {} references unknown schema {}",
                            route.id, key
                        ));
                    }
                }
            }
            if route.responses.is_empty() {
                return reject(format!("route {} declares no responses", route.id));
            }
            let mut declared_statuses = std::collections::BTreeSet::new();
            for status_str in route.responses.keys() {
                let s_num: u16 = status_str.parse().map_err(|_| {
                    PackError::Rejected(format!(
                        "route {} has invalid response status code {status_str}",
                        route.id
                    ))
                })?;
                if !(100..=599).contains(&s_num) {
                    return reject(format!(
                        "route {} declared response status code {s_num} outside valid range 100..=599",
                        route.id
                    ));
                }
                declared_statuses.insert(s_num);
            }

            // Exact RoutePlan Equivalence (M2.3-r2)
            if let Some(plan) = &route.plan {
                if plan.route_id != route_idx as u32 {
                    return reject(format!(
                        "route {} plan.route_id {} does not match route index {route_idx}",
                        route.id, plan.route_id
                    ));
                }
                if plan.deadline_ms != route.deadline_ms {
                    return reject(format!(
                        "route {} plan.deadline_ms {} does not match route.deadline_ms {}",
                        route.id, plan.deadline_ms, route.deadline_ms
                    ));
                }

                // Check allowed_statuses uniqueness and validity
                let mut planned_statuses = std::collections::BTreeSet::new();
                for &s in &plan.allowed_statuses {
                    if !(100..=599).contains(&s) {
                        return reject(format!(
                            "route {} plan.allowed_statuses contains invalid HTTP status code {s}",
                            route.id
                        ));
                    }
                    if !planned_statuses.insert(s) {
                        return reject(format!(
                            "route {} plan.allowed_statuses contains duplicate status code {s}",
                            route.id
                        ));
                    }
                }

                // Exact bidirectional equivalence: declared == planned
                if declared_statuses != planned_statuses {
                    return reject(format!(
                        "route {} plan.allowed_statuses {:?} does not match declared response statuses {:?}",
                        route.id, plan.allowed_statuses, declared_statuses
                    ));
                }

                // Default status must be in declared responses
                if !declared_statuses.contains(&plan.default_status) {
                    return reject(format!(
                        "route {} plan.default_status {} is not in declared response statuses {:?}",
                        route.id, plan.default_status, declared_statuses
                    ));
                }

                // Expected response strategy
                let expected_strategy =
                    if let Some(decl) = route.responses.get(&plan.default_status.to_string()) {
                        decl.strategy
                    } else {
                        route
                            .responses
                            .values()
                            .next()
                            .map(|d| d.strategy)
                            .unwrap_or(Strategy::Js)
                    };
                if plan.response_strategy != expected_strategy {
                    return reject(format!(
                        "route {} plan.response_strategy {:?} != declared response strategy {:?}",
                        route.id, plan.response_strategy, expected_strategy
                    ));
                }

                // Exact FieldNeeds equivalence
                let expected_field_needs = FieldNeeds {
                    params: route.params.is_some(),
                    query: route.query.is_some(),
                    body: route.body.is_some(),
                    headers: route.headers.is_some() || !route.security.is_empty(),
                };
                if plan.field_needs != expected_field_needs {
                    return reject(format!(
                        "route {} plan.field_needs {:?} != declared needs {:?}",
                        route.id, plan.field_needs, expected_field_needs
                    ));
                }

                // Schema ID validation (if schema manifest is present)
                if !self.schema_manifest.is_empty() {
                    let check_schema = |opt_binding: Option<&SourceBinding>,
                                        opt_id: Option<u32>,
                                        source_name: &str|
                     -> Result<(), PackError> {
                        match (opt_binding.and_then(|b| b.schema.as_ref()), opt_id) {
                            (Some(key), Some(id)) => {
                                if (id as usize) >= self.schema_manifest.len() {
                                    return reject(format!(
                                        "route {} plan.{}SchemaId {} out of range",
                                        route.id, source_name, id
                                    ));
                                }
                                if self.schema_manifest[id as usize].key != *key {
                                    return reject(format!(
                                        "route {} plan.{}SchemaId {} ({}) != declared schema key {}",
                                        route.id,
                                        source_name,
                                        id,
                                        self.schema_manifest[id as usize].key,
                                        key
                                    ));
                                }
                            }
                            (None, Some(id)) => {
                                return reject(format!(
                                    "route {} has no {} schema but declares plan.{}SchemaId {}",
                                    route.id, source_name, source_name, id
                                ));
                            }
                            (Some(key), None) => {
                                return reject(format!(
                                    "route {} declares {} schema {} but plan.{}SchemaId is None",
                                    route.id, source_name, key, source_name
                                ));
                            }
                            (None, None) => {}
                        }
                        Ok(())
                    };
                    check_schema(route.params.as_ref(), plan.params_schema_id, "params")?;
                    check_schema(route.query.as_ref(), plan.query_schema_id, "query")?;
                    check_schema(route.body.as_ref(), plan.body_schema_id, "body")?;
                    check_schema(route.headers.as_ref(), plan.headers_schema_id, "headers")?;
                }

                if !self.functions.is_empty() {
                    if (plan.handler_id as usize) >= self.functions.len() {
                        return reject(format!(
                            "route {} plan.handler_id {} out of range (functions count: {})",
                            route.id,
                            plan.handler_id,
                            self.functions.len()
                        ));
                    }
                    let handler_decl = &self.functions[plan.handler_id as usize];
                    if handler_decl.key != route.handler {
                        return reject(format!(
                            "route {} plan.handler_id {} ({}) does not match route.handler ({})",
                            route.id, plan.handler_id, handler_decl.key, route.handler
                        ));
                    }
                    if handler_decl.kind != FunctionKind::RouteHandler {
                        return reject(format!(
                            "route {} plan.handler_id {} points to non-route function kind {:?}",
                            route.id, plan.handler_id, handler_decl.kind
                        ));
                    }
                    if let Some(p) = &route.policy {
                        let policy_entry = self.policies.get(p).unwrap();
                        let Some(p_fn_id) = plan.policy_handler_id else {
                            return reject(format!(
                                "route {} declares policy {} but plan.policy_handler_id is None",
                                route.id, p
                            ));
                        };
                        if (p_fn_id as usize) >= self.functions.len() {
                            return reject(format!(
                                "route {} plan.policy_handler_id {} out of range (functions count: {})",
                                route.id, p_fn_id, self.functions.len()
                            ));
                        }
                        let policy_fn_decl = &self.functions[p_fn_id as usize];
                        if policy_fn_decl.key != policy_entry.handler {
                            return reject(format!(
                                "route {} policy {} handler {} does not match plan.policy_handler_id {} ({})",
                                route.id, p, policy_entry.handler, p_fn_id, policy_fn_decl.key
                            ));
                        }
                        if policy_fn_decl.kind != FunctionKind::PolicyHandler {
                            return reject(format!(
                                "route {} policy {} plan.policy_handler_id {} points to non-policy function kind {:?}",
                                route.id, p, p_fn_id, policy_fn_decl.kind
                            ));
                        }
                    } else if plan.policy_handler_id.is_some() {
                        return reject(format!(
                            "route {} has no policy but declares plan.policy_handler_id {:?}",
                            route.id, plan.policy_handler_id
                        ));
                    }
                }
            } else if !self.functions.is_empty() {
                return reject(format!(
                    "route {} missing required plan in numeric pack mode",
                    route.id
                ));
            }
        }
        for (policy_id, policy) in &self.policies {
            if policy.id != *policy_id {
                return reject(format!("policy key {policy_id} != entry id {}", policy.id));
            }
            if !self.handler_table.is_empty() && !self.handler_table.contains_key(&policy.handler) {
                return reject(format!(
                    "policy {policy_id} references unknown handler {}",
                    policy.handler
                ));
            }
            if !self.functions.is_empty() {
                let found = self
                    .functions
                    .iter()
                    .any(|f| f.key == policy.handler && f.kind == FunctionKind::PolicyHandler);
                if !found {
                    return reject(format!(
                        "policy {policy_id} handler {} not found in function manifest as PolicyHandler",
                        policy.handler
                    ));
                }
            }
        }
        for cap in &self.capabilities {
            if !["timer"].contains(&cap.as_str()) {
                return reject(format!("unknown capability {} declared", cap));
            }
        }
        Ok(())
    }

    /// Canonical JSON over the route/schema/policy graph (sorted keys via BTreeMap;
    /// routes keep their deterministic declaration order).
    pub fn routes_canonical_json(&self) -> String {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Canonical<'a> {
            routes: &'a [RouteEntry],
            schemas: &'a BTreeMap<String, q_schema_runtime::SchemaIr>,
            policies: &'a BTreeMap<String, PolicyEntry>,
            capabilities: &'a [String],
            functions: &'a [FunctionDecl],
        }
        let c = Canonical {
            routes: &self.routes,
            schemas: &self.schemas,
            policies: &self.policies,
            capabilities: &self.capabilities,
            functions: &self.functions,
        };
        serde_json::to_string(&c).expect("canonical serialization cannot fail")
    }

    /// Verification path: hash the canonical form through a Writer adapter —
    /// semantically identical to hashing `routes_canonical_json()` without
    /// materializing the (potentially ~500 KiB) string.
    pub fn routes_canonical_sha256(&self) -> String {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Canonical<'a> {
            routes: &'a [RouteEntry],
            schemas: &'a BTreeMap<String, q_schema_runtime::SchemaIr>,
            policies: &'a BTreeMap<String, PolicyEntry>,
            capabilities: &'a [String],
            functions: &'a [FunctionDecl],
        }
        let c = Canonical {
            routes: &self.routes,
            schemas: &self.schemas,
            policies: &self.policies,
            capabilities: &self.capabilities,
            functions: &self.functions,
        };
        let mut hasher = Sha256::new();
        serde_json::to_writer(&mut hasher, &c).expect("canonical serialization cannot fail");
        hex(&hasher.finalize())
    }

    /// Canonical JSON over the public API contract only (method, path, request/response schemas, security, errors).
    /// M2.3-r3: Stable across internal function/plan reordering (P1 fix).
    pub fn public_contract_canonical_json(&self) -> String {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct PublicRoute<'a> {
            id: &'a str,
            method: &'a str,
            path: &'a str,
            params: Option<&'a str>,
            query: Option<&'a str>,
            body: Option<&'a str>,
            responses: &'a BTreeMap<String, ResponseDecl>,
            security: &'a [SecurityReq],
        }
        let routes: Vec<PublicRoute> = self
            .routes
            .iter()
            .map(|r| PublicRoute {
                id: &r.id,
                method: &r.method,
                path: &r.path,
                params: r.params.as_ref().and_then(|p| p.schema.as_deref()),
                query: r.query.as_ref().and_then(|q| q.schema.as_deref()),
                body: r.body.as_ref().and_then(|b| b.schema.as_deref()),
                responses: &r.responses,
                security: &r.security,
            })
            .collect();
        serde_json::to_string(&(&routes, &self.schemas, &self.policies))
            .expect("canonical serialization cannot fail")
    }

    pub fn public_contract_sha256(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.public_contract_canonical_json().as_bytes());
        hex(&hasher.finalize())
    }
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

const B64_ALPHA: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Standard base64 encoder (RFC 4648, padded).
pub fn base64_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(B64_ALPHA[((n >> 18) & 63) as usize] as char);
        out.push(B64_ALPHA[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            B64_ALPHA[((n >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            B64_ALPHA[(n & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

/// Standard base64 decoder (RFC 4648, padded).
pub fn base64_decode(s: &str) -> Option<Vec<u8>> {
    let lookup = |c: u8| -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    };
    let bytes = s.trim().as_bytes();
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
    for chunk in bytes.chunks(4) {
        if chunk.len() < 2 {
            return None;
        }
        let c0 = lookup(chunk[0])? as u32;
        let c1 = lookup(chunk[1])? as u32;
        out.push(((c0 << 2) | (c1 >> 4)) as u8);

        if chunk.len() > 2 && chunk[2] != b'=' {
            let c2 = lookup(chunk[2])? as u32;
            out.push((((c1 & 0x0f) << 4) | (c2 >> 2)) as u8);
            if chunk.len() > 3 && chunk[3] != b'=' {
                let c3 = lookup(chunk[3])? as u32;
                out.push((((c2 & 0x03) << 6) | c3) as u8);
            }
        }
    }
    Some(out)
}

/// Test-support pack used by the fuzz integration test (deterministic,
/// integrity-hashed). Not part of the public API contract.
#[doc(hidden)]
pub fn minimal_pack_public() -> QPack {
    use sha2::{Digest, Sha256};
    let route = RouteEntry {
        id: "health.live".into(),
        module_id: "health".into(),
        method: "GET".into(),
        path: "/health/live".into(),
        path_segments: vec![
            PathSegment {
                kind: SegKind::Static,
                value: "health".into(),
            },
            PathSegment {
                kind: SegKind::Static,
                value: "live".into(),
            },
        ],
        handler: "health.live".into(),
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
        plan: None,
    };
    let mut pack = QPack {
        format_version: PACK_FORMAT_VERSION,
        kind: "velqu.qpack".into(),
        runtime_abi: RUNTIME_ABI,
        engine: EngineRef {
            name: ENGINE_NAME.into(),
            version: ENGINE_VERSION.into(),
            binding: ENGINE_BINDING.into(),
        },
        schema_ir_version: SCHEMA_IR_VERSION,
        contract_version: CONTRACT_VERSION,
        contract_hash: String::new(),
        built_by: BuiltBy {
            compiler: "0.1.0".into(),
            typescript: String::new(),
            bun: String::new(),
        },
        app_id: "fuzz".into(),
        modules: vec!["health".into()],
        entry: "app.js".into(),
        bundle_form: None,
        bundle: "function h(){} __velquRegister('health.live', h);".into(),
        source_map: None,
        bundle_bytecode: None,
        routes: vec![route],
        schemas: BTreeMap::new(),
        policies: BTreeMap::new(),
        capabilities: vec![],
        functions: vec![],
        schema_manifest: vec![],
        router: None,
        handler_table: BTreeMap::from([("health.live".into(), "health.live".into())]),
        integrity: Integrity {
            algorithm: "sha256".into(),
            bundle_sha256: String::new(),
            routes_sha256: String::new(),
            bytecode_sha256: None,
        },
    };
    pack.integrity.bundle_sha256 = hex(&Sha256::digest(pack.bundle.as_bytes()));
    pack.integrity.routes_sha256 = hex(&Sha256::digest(pack.routes_canonical_json().as_bytes()));
    pack
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn minimal_pack() -> QPack {
        let route = RouteEntry {
            id: "health.live".into(),
            module_id: "health".into(),
            method: "GET".into(),
            path: "/health/live".into(),
            path_segments: vec![
                PathSegment {
                    kind: SegKind::Static,
                    value: "health".into(),
                },
                PathSegment {
                    kind: SegKind::Static,
                    value: "live".into(),
                },
            ],
            handler: "health.live".into(),
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
            native_liveness: Some(LivenessSpec {
                status: 200,
                content_type: "application/json".into(),
                body: "{\"status\":\"ok\"}".into(),
            }),
            security: vec![],
            capabilities: vec![],
            deadline_ms: 5000,
            plan: None,
        };
        let mut pack = QPack {
            format_version: PACK_FORMAT_VERSION,
            kind: "velqu.qpack".into(),
            runtime_abi: RUNTIME_ABI,
            engine: EngineRef {
                name: ENGINE_NAME.into(),
                version: ENGINE_VERSION.into(),
                binding: ENGINE_BINDING.into(),
            },
            schema_ir_version: SCHEMA_IR_VERSION,
            contract_version: CONTRACT_VERSION,
            contract_hash: String::new(),
            built_by: BuiltBy {
                compiler: "0.1.0".into(),
                typescript: String::new(),
                bun: String::new(),
            },
            app_id: "test".into(),
            modules: vec!["health".into()],
            entry: "app.js".into(),
            bundle_form: None,
            bundle: "function health_live(){return {status:'ok'}} __velquRegister('health.live', health_live);".into(),
            source_map: None,
            bundle_bytecode: None,
            routes: vec![route],
            schemas: BTreeMap::new(),
            policies: BTreeMap::new(),
            capabilities: vec![],
            functions: vec![],
            schema_manifest: vec![],
            router: None,
            handler_table: BTreeMap::from([("health.live".into(), "health_live".into())]),
            integrity: Integrity { algorithm: "sha256".into(), bundle_sha256: String::new(), routes_sha256: String::new(), bytecode_sha256: None },
        };
        pack.integrity.bundle_sha256 = hex(&Sha256::digest(pack.bundle.as_bytes()));
        pack.integrity.routes_sha256 =
            hex(&Sha256::digest(pack.routes_canonical_json().as_bytes()));
        pack
    }

    #[test]
    fn verifies_minimal_pack() {
        minimal_pack().verify().expect("valid pack");
    }

    #[test]
    fn rejects_tampered_bundle() {
        let mut p = minimal_pack();
        p.bundle.push(' ');
        assert!(
            matches!(p.verify(), Err(PackError::Rejected(m)) if m.contains("bundle sha256 mismatch"))
        );
    }

    #[test]
    fn rejects_tampered_routes() {
        let mut p = minimal_pack();
        p.routes[0].path = "/tampered".into();
        assert!(
            matches!(p.verify(), Err(PackError::Rejected(m)) if m.contains("routes/schemas/policies sha256 mismatch"))
        );
    }

    #[test]
    fn rejects_engine_mismatch() {
        let mut p = minimal_pack();
        p.engine.version = "0.99.0".into();
        assert!(matches!(p.verify(), Err(PackError::Rejected(m)) if m.contains("engine mismatch")));
    }

    #[test]
    fn rejects_abi_mismatch_and_duplicate_ids() {
        let mut p = minimal_pack();
        p.runtime_abi = 99;
        assert!(p.verify().is_err());
        // structural diagnostics model a (buggy) compiler output: integrity is
        // recomputed so the structural check itself is what fires
        let mut p = minimal_pack();
        p.routes.push(p.routes[0].clone());
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        assert!(
            matches!(p.verify(), Err(PackError::Rejected(m)) if m.contains("duplicate route id"))
        );
    }

    #[test]
    fn rejects_unknown_handler_reference() {
        let mut p = minimal_pack();
        p.routes[0].handler = "missing".into();
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        assert!(
            matches!(p.verify(), Err(PackError::Rejected(m)) if m.contains("unknown handler table key"))
        );
    }

    /// M2.2.1-r2 (fail closed): a policy entry whose declared handler is not
    /// in handler_table must be rejected at pack verification — otherwise the
    /// engine would face a protected route with an unresolvable policy.
    #[test]
    fn rejects_policy_with_unknown_handler() {
        let mut p = minimal_pack();
        p.routes[0].policy = Some("auth.session".into());
        p.policies.insert(
            "auth.session".into(),
            PolicyEntry {
                id: "auth.session".into(),
                handler: "auth.session.missing".into(),
                declared_statuses: vec![401],
                provides: Some("session".into()),
            },
        );
        // integrity must cover the mutated routes and policy table so the
        // structural check is what fires, not the digest check
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().expect_err("policy handler gap must be rejected");
        assert!(
            matches!(&err, PackError::Rejected(m) if m.contains("unknown handler")),
            "unexpected error: {err:?}"
        );
    }

    /// A policy whose handler IS present must verify cleanly.
    #[test]
    fn accepts_policy_with_resolvable_handler() {
        let mut p = minimal_pack();
        p.bundle = format!(
            "{} __velquRegister('auth.session', function(){{}});",
            p.bundle
        );
        p.integrity.bundle_sha256 = hex(&Sha256::digest(p.bundle.as_bytes()));
        p.handler_table
            .insert("auth.session".into(), "auth.session".into());
        p.routes[0].policy = Some("auth.session".into());
        p.policies.insert(
            "auth.session".into(),
            PolicyEntry {
                id: "auth.session".into(),
                handler: "auth.session".into(),
                declared_statuses: vec![401],
                provides: Some("session".into()),
            },
        );
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        p.verify()
            .expect("policy with resolvable handler must verify");
    }

    #[test]
    fn accepts_valid_numeric_pack() {
        let mut p = minimal_pack();
        p.functions = vec![FunctionDecl {
            id: 0,
            key: "health.live".into(),
            kind: FunctionKind::RouteHandler,
        }];
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 0,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            default_status: 200,
            allowed_statuses: vec![200],
            field_needs: FieldNeeds::default(),
            response_strategy: Strategy::Js,
            deadline_ms: 5000,
        });
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        p.verify().expect("valid numeric pack must verify");
    }

    #[test]
    fn rejects_non_dense_function_manifest() {
        let mut p = minimal_pack();
        p.functions = vec![FunctionDecl {
            id: 1, // should be 0
            key: "health.live".into(),
            kind: FunctionKind::RouteHandler,
        }];
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 0,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            default_status: 200,
            allowed_statuses: vec![200],
            field_needs: FieldNeeds::default(),
            response_strategy: Strategy::Js,
            deadline_ms: 5000,
        });
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("must be dense 0..N")));
    }

    #[test]
    fn rejects_out_of_range_handler_id() {
        let mut p = minimal_pack();
        p.functions = vec![FunctionDecl {
            id: 0,
            key: "health.live".into(),
            kind: FunctionKind::RouteHandler,
        }];
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 5, // out of range
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            default_status: 200,
            allowed_statuses: vec![200],
            field_needs: FieldNeeds::default(),
            response_strategy: Strategy::Js,
            deadline_ms: 5000,
        });
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("out of range")));
    }

    #[test]
    fn rejects_mismatched_handler_id() {
        let mut p = minimal_pack();
        p.functions = vec![
            FunctionDecl {
                id: 0,
                key: "other.handler".into(),
                kind: FunctionKind::RouteHandler,
            },
            FunctionDecl {
                id: 1,
                key: "health.live".into(),
                kind: FunctionKind::RouteHandler,
            },
        ];
        p.handler_table
            .insert("other.handler".into(), "other.handler".into());
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 0, // points to other.handler, while route.handler is health.live
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            default_status: 200,
            allowed_statuses: vec![200],
            field_needs: FieldNeeds::default(),
            response_strategy: Strategy::Js,
            deadline_ms: 5000,
        });
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(
            matches!(err, PackError::Rejected(m) if m.contains("does not match route.handler"))
        );
    }

    #[test]
    fn rejects_wrong_function_kind() {
        let mut p = minimal_pack();
        p.functions = vec![FunctionDecl {
            id: 0,
            key: "health.live".into(),
            kind: FunctionKind::PolicyHandler, // wrong kind for route handler!
        }];
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 0,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            default_status: 200,
            allowed_statuses: vec![200],
            field_needs: FieldNeeds::default(),
            response_strategy: Strategy::Js,
            deadline_ms: 5000,
        });
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("non-route function kind")));
    }

    #[test]
    fn rejects_undeclared_default_status() {
        let mut p = minimal_pack();
        p.functions = vec![FunctionDecl {
            id: 0,
            key: "health.live".into(),
            kind: FunctionKind::RouteHandler,
        }];
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 0,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            default_status: 201, // 201 not in allowed_statuses [200]
            allowed_statuses: vec![200],
            field_needs: FieldNeeds::default(),
            response_strategy: Strategy::Js,
            deadline_ms: 5000,
        });
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(
            matches!(err, PackError::Rejected(m) if m.contains("not in declared response statuses"))
        );
    }

    #[test]
    fn rejects_unmapped_response_status() {
        let mut p = minimal_pack();
        p.functions = vec![FunctionDecl {
            id: 0,
            key: "health.live".into(),
            kind: FunctionKind::RouteHandler,
        }];
        // Route only declares 200, but plan specifies [200, 418]
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 0,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            default_status: 200,
            allowed_statuses: vec![200, 418],
            field_needs: FieldNeeds::default(),
            response_strategy: Strategy::Js,
            deadline_ms: 5000,
        });
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(
            matches!(err, PackError::Rejected(m) if m.contains("does not match declared response statuses"))
        );
    }

    #[test]
    fn rejects_mismatched_field_needs() {
        let mut p = minimal_pack();
        p.functions = vec![FunctionDecl {
            id: 0,
            key: "health.live".into(),
            kind: FunctionKind::RouteHandler,
        }];
        // Route has no query binding, but plan claims query: true
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 0,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            default_status: 200,
            allowed_statuses: vec![200],
            field_needs: FieldNeeds {
                params: false,
                query: true,
                headers: false,
                body: false,
            },
            response_strategy: Strategy::Js,
            deadline_ms: 5000,
        });
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("plan.field_needs")));
    }

    #[test]
    fn rejects_mismatched_deadline() {
        let mut p = minimal_pack();
        p.functions = vec![FunctionDecl {
            id: 0,
            key: "health.live".into(),
            kind: FunctionKind::RouteHandler,
        }];
        p.routes[0].deadline_ms = 5000;
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 0,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            default_status: 200,
            allowed_statuses: vec![200],
            field_needs: FieldNeeds::default(),
            response_strategy: Strategy::Js,
            deadline_ms: 1000, // differs from route.deadline_ms
        });
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("deadline_ms")));
    }

    #[test]
    fn rejects_schema_id_mismatch() {
        let mut p = minimal_pack();
        p.functions = vec![FunctionDecl {
            id: 0,
            key: "health.live".into(),
            kind: FunctionKind::RouteHandler,
        }];
        p.schemas.insert(
            "sch:health.query".into(),
            q_schema_runtime::SchemaIr::Object {
                properties: BTreeMap::new(),
                required: vec![],
            },
        );
        p.schema_manifest = vec![SchemaDecl {
            id: 0,
            key: "sch:health.query".into(),
            ir: q_schema_runtime::SchemaIr::Object {
                properties: BTreeMap::new(),
                required: vec![],
            },
        }];
        p.routes[0].query = Some(SourceBinding {
            schema: Some("sch:health.query".into()),
            coerce: Some("query".into()),
            content_type: None,
            limit_bytes: 0,
        });
        // Plan has querySchemaId = None while route declares query schema
        p.routes[0].plan = Some(RoutePlanDecl {
            route_id: 0,
            handler_id: 0,
            policy_id: None,
            policy_handler_id: None,
            params_schema_id: None,
            query_schema_id: None,
            headers_schema_id: None,
            body_schema_id: None,
            default_status: 200,
            allowed_statuses: vec![200],
            field_needs: FieldNeeds {
                params: false,
                query: true,
                headers: false,
                body: false,
            },
            response_strategy: Strategy::Js,
            deadline_ms: 5000,
        });
        p.integrity.routes_sha256 = hex(&Sha256::digest(p.routes_canonical_json().as_bytes()));
        let err = p.verify().unwrap_err();
        assert!(matches!(err, PackError::Rejected(m) if m.contains("querySchemaId is None")));
    }

    #[test]
    fn public_contract_hash_is_stable_when_function_ids_are_reordered() {
        let mut p1 = minimal_pack();
        p1.functions = vec![
            FunctionDecl {
                id: 0,
                key: "health.live".into(),
                kind: FunctionKind::RouteHandler,
            },
            FunctionDecl {
                id: 1,
                key: "other.route".into(),
                kind: FunctionKind::RouteHandler,
            },
        ];
        let mut p2 = minimal_pack();
        p2.functions = vec![
            FunctionDecl {
                id: 0,
                key: "other.route".into(),
                kind: FunctionKind::RouteHandler,
            },
            FunctionDecl {
                id: 1,
                key: "health.live".into(),
                kind: FunctionKind::RouteHandler,
            },
        ];
        // Public contract hash depends on public API routes/schemas, NOT internal function ordering
        assert_eq!(p1.public_contract_sha256(), p2.public_contract_sha256());
        // Execution graph hash DOES change because internal layout changed
        assert_ne!(p1.routes_canonical_sha256(), p2.routes_canonical_sha256());
    }
}
