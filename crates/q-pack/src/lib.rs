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
}

fn default_deadline() -> u64 {
    5_000
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Integrity {
    pub algorithm: String,
    pub bundle_sha256: String,
    pub routes_sha256: String,
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
    pub bundle: String,
    #[serde(default)]
    pub source_map: Option<String>,
    pub routes: Vec<RouteEntry>,
    /// schema IR registry: key -> IR node (q-schema-runtime types)
    #[serde(default)]
    pub schemas: BTreeMap<String, q_schema_runtime::SchemaIr>,
    #[serde(default)]
    pub policies: BTreeMap<String, PolicyEntry>,
    #[serde(default)]
    pub capabilities: Vec<String>,
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
        let routes_canonical = self.routes_canonical_json();
        let routes_hash = hex(&Sha256::digest(routes_canonical.as_bytes()));
        if routes_hash != self.integrity.routes_sha256 {
            return reject("integrity failure: routes/schemas/policies sha256 mismatch".into());
        }
        if self.integrity.algorithm != "sha256" {
            return reject(format!(
                "unsupported integrity algorithm {}",
                self.integrity.algorithm
            ));
        }
        // handler table sanity
        if self.handler_table.is_empty() {
            return reject("empty handler table".into());
        }
        let mut seen = std::collections::BTreeSet::new();
        for route in &self.routes {
            if !seen.insert(route.id.clone()) {
                return reject(format!("duplicate route id {}", route.id));
            }
            if !self.handler_table.contains_key(&route.handler) {
                return reject(format!(
                    "route {} references unknown handler table key {}",
                    route.id, route.handler
                ));
            }
            if let Some(p) = &route.policy {
                if !self.policies.contains_key(p) {
                    return reject(format!(
                        "route {} references unknown policy {}",
                        route.id, p
                    ));
                }
            }
            for binding in [&route.params, &route.query, &route.body]
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
        }
        let c = Canonical {
            routes: &self.routes,
            schemas: &self.schemas,
            policies: &self.policies,
            capabilities: &self.capabilities,
        };
        serde_json::to_string(&c).expect("canonical serialization cannot fail")
    }
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
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
            bundle: "function health_live(){return {status:'ok'}} __velquRegister('health.live', health_live);".into(),
            source_map: None,
            routes: vec![route],
            schemas: BTreeMap::new(),
            policies: BTreeMap::new(),
            capabilities: vec![],
            handler_table: BTreeMap::from([("health.live".into(), "health_live".into())]),
            integrity: Integrity { algorithm: "sha256".into(), bundle_sha256: String::new(), routes_sha256: String::new() },
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
}
