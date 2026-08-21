//! Frozen benchmark schema corpus for M25-002-A/B.
//!
//! M25-002-A shapes mirror the M1 bridge fixtures (small object, nested
//! object, array of 100 records) so the strategy comparison stays comparable
//! with the committed bridge evidence. M25-002-B adds the payload matrix:
//! ~256B/1KB/16KB/64KB sized bodies, arrays of 1,000 records, an
//! optional/null-heavy object, and an RFC 9457 problem-shaped payload.

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
    vec![
        small_user(),
        nested_order(),
        records100(),
        records1000(),
        pad_256(),
        pad_1k(),
        pad_16k(),
        pad_64k(),
        opt_null(),
        problem_shape(),
    ]
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

fn record_item_ir() -> SchemaIr {
    SchemaIr::Object {
        properties: bmap(&[
            ("active", SchemaIr::Boolean),
            ("id", s_int(Some(0), None)),
            ("name", s_string(Some(1), Some(64))),
            ("qty", s_int(Some(0), None)),
        ]),
        required: vec!["active".into(), "id".into(), "name".into(), "qty".into()],
    }
}

fn records_n(name: &'static str, n: usize) -> BenchSchema {
    let ir = SchemaIr::Array {
        items: Box::new(record_item_ir()),
        min_items: Some(1),
        max_items: None,
    };
    let valid = Value::Array(
        (0..n)
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
    BenchSchema { name, ir, valid }
}

fn records100() -> BenchSchema {
    records_n("records100", 100)
}

fn records1000() -> BenchSchema {
    records_n("records1000", 1000)
}

/// Object with an exactly-sized `blob` string so the serialized fixture lands
/// near `target` bytes (recorded exactly via `inBytes` in the raw evidence).
fn sized(name: &'static str, target: u64) -> BenchSchema {
    let blob_len = target.max(64) - 48;
    let ir = SchemaIr::Object {
        properties: bmap(&[
            ("blob", s_string(Some(blob_len), Some(blob_len))),
            ("id", s_int(Some(0), None)),
            ("label", s_string(Some(1), Some(32))),
        ]),
        required: vec!["blob".into(), "id".into(), "label".into()],
    };
    let valid = json!({
        "blob": "x".repeat(blob_len as usize),
        "id": 7,
        "label": "payload",
    });
    BenchSchema { name, ir, valid }
}

fn pad_256() -> BenchSchema {
    sized("pad_256", 256)
}

fn pad_1k() -> BenchSchema {
    sized("pad_1k", 1024)
}

fn pad_16k() -> BenchSchema {
    sized("pad_16k", 16_384)
}

fn pad_64k() -> BenchSchema {
    sized("pad_64k", 65_536)
}

/// Optional/null-heavy object: four optional strings (half with defaults),
/// two nullable scalars, one optional-nullable string, one required int.
/// The fixture exercises present, absent, null-with-default, and null paths.
fn opt_null() -> BenchSchema {
    let opt_str = |default: Option<Value>| SchemaIr::Optional {
        inner: Box::new(s_string(None, Some(8))),
        default,
    };
    let ir = SchemaIr::Object {
        properties: bmap(&[
            ("a1", opt_str(None)),
            ("a2", opt_str(None)),
            ("b1", opt_str(Some(json!("bd1")))),
            ("b2", opt_str(Some(json!("bd2")))),
            ("id", s_int(Some(0), None)),
            (
                "n1",
                SchemaIr::Nullable {
                    inner: Box::new(s_int(None, None)),
                },
            ),
            (
                "n2",
                SchemaIr::Nullable {
                    inner: Box::new(SchemaIr::Boolean),
                },
            ),
            (
                "tag",
                SchemaIr::Optional {
                    inner: Box::new(SchemaIr::Nullable {
                        inner: Box::new(s_string(None, Some(4))),
                    }),
                    default: None,
                },
            ),
        ]),
        required: vec!["id".into()],
    };
    let valid = json!({
        "a1": "aa",       // present, no default
        "b1": null,       // optional null → default "bd1"
        "id": 9,
        "n1": null,       // nullable null
        "n2": false,      // nullable present
        // a2 absent (no default → omitted), b2 absent (default → inserted),
        // tag absent (optional, no default → omitted)
    });
    BenchSchema {
        name: "opt_null",
        ir,
        valid,
    }
}

/// RFC 9457 problem-shaped input payload (plain object IR: the Problem IR
/// node itself stays outside the generated-decoder subset by design).
fn problem_shape() -> BenchSchema {
    let ir = SchemaIr::Object {
        properties: bmap(&[
            (
                "detail",
                SchemaIr::Optional {
                    inner: Box::new(s_string(Some(1), Some(256))),
                    default: None,
                },
            ),
            (
                "instance",
                SchemaIr::Optional {
                    inner: Box::new(s_string(None, Some(256))),
                    default: None,
                },
            ),
            ("status", s_int(Some(400), Some(599))),
            ("title", s_string(Some(1), Some(128))),
            ("type", s_string(Some(1), Some(128))),
        ]),
        required: vec!["status".into(), "title".into(), "type".into()],
    };
    let valid = json!({
        "detail": "The requested resource does not exist.",
        "instance": "/orders/42",
        "status": 404,
        "title": "Not Found",
        "type": "https://example.com/problems/not-found",
    });
    BenchSchema {
        name: "problem_shape",
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
        "records1000" => vec![
            ("empty-array", json!([])),
            (
                "bad-item-type",
                Value::Array({
                    let mut v: Vec<Value> = (0..1000)
                        .map(|i| json!({"active": true, "id": i, "name": format!("item{i}"), "qty": i}))
                        .collect();
                    v[500]["qty"] = json!("9");
                    v
                }),
            ),
        ],
        "pad_256" => vec![
            (
                "blob-too-short",
                json!({"blob": "x", "id": 7, "label": "payload"}),
            ),
            (
                "blob-unknown-field",
                json!({"blob": "x".repeat(208), "id": 7, "label": "payload", "extra": true}),
            ),
        ],
        "pad_1k" => vec![(
            "wrong-type-id",
            json!({"blob": "x".repeat(976), "id": "7", "label": "payload"}),
        )],
        "pad_16k" => vec![("missing-label", json!({"blob": "x".repeat(16336), "id": 7}))],
        "pad_64k" => vec![(
            "blob-too-long",
            json!({"blob": "x".repeat(65_489), "id": 7, "label": "payload"}),
        )],
        "opt_null" => vec![
            ("nullable-wrong-type", json!({"id": 9, "n1": "5"})),
            ("optional-bad-inner", json!({"a1": "123456789", "id": 9})),
            ("required-missing", json!({"a1": "aa"})),
            ("optional-nullable-bad", json!({"id": 9, "tag": 3})),
        ],
        "problem_shape" => vec![
            (
                "status-out-of-range",
                json!({"status": 600, "title": "Not Found", "type": "t"}),
            ),
            ("missing-title", json!({"status": 404, "type": "t"})),
            (
                "bad-detail-len",
                json!({"detail": "d".repeat(300), "status": 404, "title": "Not Found", "type": "t"}),
            ),
        ],
        _ => Vec::new(),
    }
}
