//! Frozen benchmark schema corpus for M25-002-A.
//!
//! Three schema fixtures mirror the M1 bridge shapes (small object, nested
//! object, array of 100 records) so the strategy comparison stays comparable
//! with the committed bridge evidence. The payload matrix expansion
//! (256B/1KB/16KB/64KB, arrays 100/1,000, problems) is owned by M25-002-B.

use std::collections::BTreeMap;

use q_schema_runtime::SchemaIr;
use serde_json::{json, Value};

pub struct BenchSchema {
    /// stable schema name used in raw rows, summaries, and `generated::decode`
    pub name: &'static str,
    pub ir: SchemaIr,
    /// complete valid fixture used for the timed round trips
    pub valid: Value,
}

pub fn corpus() -> Vec<BenchSchema> {
    vec![small_user(), nested_order(), records100()]
}

fn bmap(entries: &[(&'static str, SchemaIr)]) -> BTreeMap<String, Box<SchemaIr>> {
    entries
        .iter()
        .map(|(k, v)| (k.to_string(), Box::new(v.clone())))
        .collect()
}

fn s_string(min: Option<u64>, max: Option<u64>) -> SchemaIr {
    SchemaIr::String {
        min_length: min,
        max_length: max,
        pattern: None,
        format: None,
    }
}

fn s_int(min: Option<i64>, max: Option<i64>) -> SchemaIr {
    SchemaIr::Integer {
        minimum: min,
        maximum: max,
    }
}

fn small_user() -> BenchSchema {
    let ir = SchemaIr::Object {
        properties: bmap(&[
            ("active", SchemaIr::Boolean),
            ("id", s_int(Some(0), None)),
            ("name", s_string(Some(1), Some(64))),
            (
                "nickname",
                SchemaIr::Optional {
                    inner: Box::new(SchemaIr::Nullable {
                        inner: Box::new(s_string(None, Some(32))),
                    }),
                    default: None,
                },
            ),
            (
                "tag",
                SchemaIr::Optional {
                    inner: Box::new(s_string(Some(1), Some(16))),
                    default: Some(json!("none")),
                },
            ),
        ]),
        required: vec!["active".into(), "id".into(), "name".into()],
    };
    let valid = json!({
        "active": true,
        "id": 42,
        "name": "Ada Lovelace",
        "nickname": null,
        "tag": "alpha",
    });
    BenchSchema {
        name: "small_user",
        ir,
        valid,
    }
}

fn nested_order() -> BenchSchema {
    let ir = SchemaIr::Object {
        properties: bmap(&[
            (
                "meta",
                SchemaIr::Object {
                    properties: bmap(&[
                        ("page", s_int(Some(0), None)),
                        ("total", s_int(Some(0), None)),
                    ]),
                    required: vec!["page".into(), "total".into()],
                },
            ),
            (
                "wrapper",
                SchemaIr::Object {
                    properties: bmap(&[(
                        "inner",
                        SchemaIr::Object {
                            properties: bmap(&[(
                                "list",
                                SchemaIr::Array {
                                    items: Box::new(SchemaIr::Object {
                                        properties: bmap(&[
                                            ("id", s_string(Some(1), Some(32))),
                                            ("qty", s_int(Some(0), Some(1000))),
                                        ]),
                                        required: vec!["id".into(), "qty".into()],
                                    }),
                                    min_items: Some(1),
                                    max_items: None,
                                },
                            )]),
                            required: vec!["list".into()],
                        },
                    )]),
                    required: vec!["inner".into()],
                },
            ),
        ]),
        required: vec!["meta".into(), "wrapper".into()],
    };
    let valid = json!({
        "meta": { "page": 1, "total": 2 },
        "wrapper": {
            "inner": {
                "list": [ { "id": "itm_1", "qty": 3 }, { "id": "itm_2", "qty": 5 } ]
            }
        }
    });
    BenchSchema {
        name: "nested_order",
        ir,
        valid,
    }
}

fn records100() -> BenchSchema {
    let ir = SchemaIr::Array {
        items: Box::new(SchemaIr::Object {
            properties: bmap(&[
                ("active", SchemaIr::Boolean),
                ("id", s_int(Some(0), None)),
                ("name", s_string(Some(1), Some(64))),
                ("qty", s_int(Some(0), None)),
            ]),
            required: vec!["active".into(), "id".into(), "name".into(), "qty".into()],
        }),
        min_items: Some(1),
        max_items: None,
    };
    let valid = Value::Array(
        (0..100)
            .map(|i| {
                json!({
                    "active": i % 2 == 0,
                    "id": i,
                    "name": format!("item{i}"),
                    "qty": i * 2,
                })
            })
            .collect(),
    );
    BenchSchema {
        name: "records100",
        ir,
        valid,
    }
}

/// Invalid mutations used by the differential tests: each is (label, value)
/// and must classify identically under `q_schema_runtime::validate` and the
/// generated decoder for the named schema.
#[cfg(test)]
pub fn invalid_cases(name: &str) -> Vec<(&'static str, Value)> {
    match name {
        "small_user" => vec![
            ("missing-required-name", json!({"active": true, "id": 42})),
            (
                "wrong-type-id",
                json!({"active": true, "id": "42", "name": "Ada"}),
            ),
            (
                "unknown-field",
                json!({"active": true, "id": 42, "name": "Ada", "extra": 1}),
            ),
            (
                "min-length-name",
                json!({"active": true, "id": 42, "name": ""}),
            ),
            (
                "nullable-wrong-type",
                json!({"active": true, "id": 42, "name": "Ada", "nickname": 7}),
            ),
            (
                "optional-bad-inner",
                json!({"active": true, "id": 42, "name": "Ada", "tag": ""}),
            ),
        ],
        "nested_order" => vec![
            (
                "bad-item-qty",
                json!({
                    "meta": { "page": 1, "total": 2 },
                    "wrapper": { "inner": { "list": [ { "id": "itm_1", "qty": "3" } ] } }
                }),
            ),
            (
                "min-items-list",
                json!({
                    "meta": { "page": 1, "total": 2 },
                    "wrapper": { "inner": { "list": [] } }
                }),
            ),
            (
                "missing-wrapper",
                json!({"meta": { "page": 1, "total": 2 }}),
            ),
            (
                "wrong-type-wrapper",
                json!({"meta": { "page": 1, "total": 2 }, "wrapper": 5}),
            ),
        ],
        "records100" => vec![
            ("not-array", json!({"id": 1})),
            (
                "item-unknown-field",
                Value::Array({
                    let mut v: Vec<Value> = (0..100).map(|i| json!({"active": true, "id": i, "name": format!("item{i}"), "qty": i})).collect();
                    v.push(json!({"active": true, "id": 100, "name": "x", "qty": 1, "zzz": 0}));
                    v
                }),
            ),
        ],
        _ => Vec::new(),
    }
}
