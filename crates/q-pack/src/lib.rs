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
        }
        let c = Canonical {
            routes: &self.routes,
            schemas: &self.schemas,
            policies: &self.policies,
            capabilities: &self.capabilities,
        };
        let mut hasher = Sha256::new();
        serde_json::to_writer(&mut hasher, &c).expect("canonical serialization cannot fail");
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
}
