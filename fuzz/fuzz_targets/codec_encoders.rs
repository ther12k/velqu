//! #1318 / M6-002 — codec encoder fuzz target (q-schema-runtime, M25-005/006).
//!
//! Covers the "Treaty/response encoder" acceptance surface on the Rust
//! side of the contract: status-specific response encoders
//! (`EncoderProgram::encode`) and RFC 9457 problem encoders
//! (`ProblemProgram::encode`) — the exact bytes Treaty clients decode.
//! Arbitrary extension maps (handler-controlled data in production) and
//! detail/instance strings go in; valid, bounded problem+json/response
//! JSON must come out. The Treaty TS side is exercised by
//! `scripts/ts-fuzz-campaign.ts` (see fuzz/COVERAGE.md).

#![no_main]

use libfuzzer_sys::fuzz_target;
use q_schema_runtime::{EncoderTable, SchemaIr};
use serde_json::{json, Value};
use std::sync::OnceLock;

fn boxed(ir: SchemaIr) -> Box<SchemaIr> {
    Box::new(ir)
}

/// Fixed contract surface: one plain response schema and one RFC 9457
/// problem schema — the two encoder classes.
fn table() -> &'static EncoderTable {
    static T: OnceLock<EncoderTable> = OnceLock::new();
    T.get_or_init(|| {
        EncoderTable::from_schemas(&[
            SchemaIr::Object {
                properties: std::collections::BTreeMap::from([
                    ("message".into(), boxed(SchemaIr::String { min_length: None, max_length: Some(200), pattern: None, format: None })),
                    ("code".into(), boxed(SchemaIr::Integer { minimum: Some(0), maximum: Some(65535) })),
                ]),
                required: vec!["message".into()],
            },
            SchemaIr::Problem {
                type_uri: Some("https://velqu.dev/problems/fuzz".into()),
                title: "Fuzz problem".into(),
                status: 422,
                detail: Some(boxed(SchemaIr::String { min_length: None, max_length: Some(300), pattern: None, format: None })),
            },
        ])
    })
}

fuzz_target!(|data: &[u8]| {
    let lossy = String::from_utf8_lossy(data);
    let t = table();

    // 1) Response encoder: arbitrary JSON through the compiled program.
    //    The encoder classifies and may fail with typed field errors —
    //    but must never panic and never emit invalid JSON on success.
    if let Some(enc) = t.get(0) {
        let value: Value = serde_json::from_str(&lossy).unwrap_or(Value::Null);
        let mut out = Vec::new();
        if enc.encode(&value, &mut out).is_ok() {
            let parsed: Result<Value, _> = serde_json::from_slice(&out);
            assert!(
                parsed.is_ok(),
                "encoder produced invalid JSON: {}",
                String::from_utf8_lossy(&out)
            );
        }
    }

    // 2) Problem encoder: arbitrary detail/instance strings and an
    //    arbitrary extension map (the handler-controlled surface).
    if let Some(prob) = t.problem(1) {
        let value: Value = serde_json::from_str(&lossy).unwrap_or(json!({}));
        let (detail, instance, extensions) = match &value {
            Value::Object(map) => {
                let detail = map.get("detail").and_then(|v| v.as_str());
                let instance = map
                    .get("instance")
                    .and_then(|v| v.as_str())
                    .unwrap_or("/fuzz");
                let extensions: Vec<(String, Value)> = map
                    .iter()
                    .filter(|(k, _)| k.as_str() != "detail" && k.as_str() != "instance")
                    .take(32) // production bounds extensions; mirror them here
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                (detail, instance, extensions)
            }
            other => (None, "/fuzz", vec![("payload".to_string(), other.clone())]),
        };
        let mut out = Vec::new();
        if prob
            .encode(
                "https://velqu.dev/problems/fuzz",
                "Fuzz problem",
                None,
                detail,
                &[],
                &extensions,
                instance,
                &mut out,
            )
            .is_ok()
        {
            let parsed: Result<Value, _> = serde_json::from_slice(&out);
            let parsed = expect_parsed(parsed, &out);
            // RFC 9457 invariants on the emitted bytes.
            assert_eq!(parsed["status"], 422, "problem status must be the declared one");
            assert!(
                parsed["type"].is_string() && parsed["title"].is_string(),
                "problem must carry string type/title"
            );
        }
    }
});

fn expect_parsed(parsed: Result<Value, serde_json::Error>, out: &[u8]) -> Value {
    match parsed {
        Ok(v) => v,
        Err(e) => panic!(
            "problem encoder produced invalid JSON ({e}): {}",
            String::from_utf8_lossy(out)
        ),
    }
}
