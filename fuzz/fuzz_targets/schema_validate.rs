//! #1318 / M6-002 — Schema validation fuzz target (q-schema-runtime).
//!
//! A representative nested schema IR (objects, arrays, scalars, unions,
//! optionals, literals) is fixed; arbitrary JSON is validated against
//! it. Validation must be a total, deterministic classification: every
//! input is `Ok` or a typed failure — never a panic, and the pattern
//! engine must not exhibit catastrophic backtracking on adversarial
//! pattern/string pairs.

#![no_main]

use libfuzzer_sys::fuzz_target;
use q_schema_runtime::{validate, Source, SchemaIr};
use std::collections::BTreeMap;
use std::sync::OnceLock;
use std::time::Instant;

fn boxed(ir: SchemaIr) -> Box<SchemaIr> {
    Box::new(ir)
}

fn corpus_ir() -> SchemaIr {
    SchemaIr::Object {
        properties: BTreeMap::from([
            (
                "name".into(),
                boxed(SchemaIr::String {
                    min_length: Some(1),
                    max_length: Some(60),
                    // Classic backtracking-bait pattern.
                    pattern: Some("^(a+)+$".into()),
                    format: None,
                }),
            ),
            ("count".into(), boxed(SchemaIr::Integer { minimum: Some(0), maximum: Some(1000) })),
            ("ratio".into(), boxed(SchemaIr::Number { minimum: Some(0.0), maximum: Some(1.0) })),
            ("active".into(), boxed(SchemaIr::Boolean)),
            (
                "kind".into(),
                boxed(SchemaIr::Enum { values: vec!["a".into(), "b".into(), 7.into()] }),
            ),
            (
                "nickname".into(),
                boxed(SchemaIr::Optional { inner: boxed(SchemaIr::String { min_length: None, max_length: Some(20), pattern: None, format: None }), default: None }),
            ),
            (
                "note".into(),
                boxed(SchemaIr::Nullable { inner: boxed(SchemaIr::String { min_length: None, max_length: Some(200), pattern: None, format: None }) }),
            ),
            (
                "tags".into(),
                boxed(SchemaIr::Array {
                    items: boxed(SchemaIr::String { min_length: Some(1), max_length: Some(10), pattern: None, format: None }),
                    min_items: Some(0),
                    max_items: Some(64),
                }),
            ),
            (
                "shape".into(),
                boxed(SchemaIr::Union {
                    members: vec![
                        boxed(SchemaIr::Literal { value: "point".into() }),
                        boxed(SchemaIr::Array {
                            items: boxed(SchemaIr::Number { minimum: None, maximum: None }),
                            min_items: Some(2),
                            max_items: Some(3),
                        }),
                    ],
                }),
            ),
        ]),
        required: vec!["name".into(), "count".into()],
    }
}

fn ir() -> &'static SchemaIr {
    static IR: OnceLock<SchemaIr> = OnceLock::new();
    IR.get_or_init(corpus_ir)
}

fuzz_target!(|data: &[u8]| {
    let lossy = String::from_utf8_lossy(data);
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&lossy) else {
        return; // not JSON: outside the validator's input domain
    };
    // Watchdog against catastrophic backtracking: a single validation
    // over a small input must be effectively instant. This bound is a
    // fuzz invariant, not a production timing claim.
    let start = Instant::now();
    let result = validate(ir(), &value, Source::Body);
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 1_000,
        "validation of a small input took {elapsed:?} — possible catastrophic backtracking"
    );
    // Determinism: same input, same outcome (Ok vs Err), twice.
    let second = validate(ir(), &value, Source::Body);
    assert_eq!(
        result.is_ok(),
        second.is_ok(),
        "validation is nondeterministic across identical inputs"
    );
});
