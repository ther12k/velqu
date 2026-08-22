//! Property-based robustness tests for the native schema validator:
//! arbitrary JSON-ish values against every IR kind must classify (Ok/Err)
//! without panicking, and classification must be deterministic.

use q_schema_runtime::{validate, SchemaIr, Source};
use serde_json::{json, Value};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

fn random_json(rng: &mut Rng, depth: u8) -> Value {
    use serde_json::Number;
    match rng.next() % 7 {
        0 => Value::Null,
        1 => Value::Bool(rng.next().is_multiple_of(2)),
        2 => Value::Number(Number::from((rng.next() % 1000) as i64)),
        3 => Value::Number(Number::from_f64((rng.next() % 1000) as f64 / 7.0).unwrap()),
        4 => Value::String(format!("s{}", rng.next() % 100)),
        5 if depth > 0 => {
            let n = (rng.next() % 5) as usize;
            Value::Array((0..n).map(|_| random_json(rng, depth - 1)).collect())
        }
        6 if depth > 0 => {
            let n = (rng.next() % 5) as usize;
            Value::Object(
                (0..n)
                    .map(|i| (format!("k{i}"), random_json(rng, depth - 1)))
                    .collect(),
            )
        }
        _ => Value::String(String::new()),
    }
}

#[test]
fn validator_never_panics_and_is_deterministic() {
    let irs: Vec<SchemaIr> = vec![
        SchemaIr::String {
            min_length: Some(1),
            max_length: Some(10),
            pattern: None,
            format: None,
        },
        SchemaIr::String {
            min_length: None,
            max_length: None,
            pattern: Some("^usr_[0-9]+$".into()),
            format: None,
        },
        SchemaIr::String {
            min_length: None,
            max_length: None,
            pattern: None,
            format: Some("email".into()),
        },
        SchemaIr::Integer {
            minimum: Some(0),
            maximum: Some(50),
        },
        SchemaIr::Number {
            minimum: None,
            maximum: None,
        },
        SchemaIr::Boolean,
        SchemaIr::Enum {
            values: vec![json!("a"), json!("b")],
        },
        SchemaIr::Nullable {
            inner: Box::new(SchemaIr::Integer {
                minimum: None,
                maximum: None,
            }),
        },
        SchemaIr::Array {
            items: Box::new(SchemaIr::Integer {
                minimum: None,
                maximum: None,
            }),
            min_items: Some(1),
            max_items: Some(3),
        },
        SchemaIr::Object {
            properties: [
                (
                    "name".to_string(),
                    Box::new(SchemaIr::String {
                        min_length: Some(1),
                        max_length: None,
                        pattern: None,
                        format: None,
                    }),
                ),
                (
                    "n".to_string(),
                    Box::new(SchemaIr::Optional {
                        inner: Box::new(SchemaIr::Integer {
                            minimum: None,
                            maximum: None,
                        }),
                        default: Some(json!(10)),
                    }),
                ),
            ]
            .into_iter()
            .collect(),
            required: vec!["name".into()],
        },
        SchemaIr::Union {
            members: vec![
                Box::new(SchemaIr::Integer {
                    minimum: None,
                    maximum: None,
                }),
                Box::new(SchemaIr::String {
                    min_length: None,
                    max_length: None,
                    pattern: None,
                    format: None,
                }),
            ],
        },
        // IR v2 nodes: must classify (typed unsupported error) without panicking
        SchemaIr::Transform {
            input: Box::new(SchemaIr::String {
                min_length: None,
                max_length: None,
                pattern: None,
                format: None,
            }),
            output: Box::new(SchemaIr::Integer {
                minimum: None,
                maximum: None,
            }),
            name: "parse-count".into(),
        },
        SchemaIr::File {
            content_type: Some("text/csv".into()),
            max_bytes: 1024,
        },
        SchemaIr::File {
            content_type: None,
            max_bytes: 1,
        },
        SchemaIr::Problem {
            type_uri: Some("https://example.com/probs/oos".into()),
            title: "Out of stock".into(),
            status: 409,
            detail: Some(Box::new(SchemaIr::String {
                min_length: None,
                max_length: None,
                pattern: None,
                format: None,
            })),
        },
        SchemaIr::Problem {
            type_uri: None,
            title: "Boom".into(),
            status: 500,
            detail: None,
        },
        // M25-001-B fallback markers: must classify without panicking
        SchemaIr::Fallback {
            reason: "unsupported-transform".into(),
            inner: Some(Box::new(SchemaIr::Integer {
                minimum: None,
                maximum: None,
            })),
        },
        SchemaIr::Fallback {
            reason: "measured".into(),
            inner: None,
        },
        SchemaIr::Fallback {
            reason: "unrepresentable".into(),
            inner: Some(Box::new(SchemaIr::String {
                min_length: None,
                max_length: None,
                pattern: None,
                format: None,
            })),
        },
    ];
    let mut rng = Rng(0xf00dfeedfaceb00c);
    for _ in 0..40_000 {
        let ir = &irs[(rng.next() as usize) % irs.len()];
        let v = random_json(&mut rng, 3);
        let a = validate(ir, &v, Source::Body);
        let b = validate(ir, &v, Source::Body);
        assert_eq!(a.is_ok(), b.is_ok(), "validator must be deterministic");
        let aq = validate(ir, &v, Source::Query);
        assert!(aq.is_ok() || aq.is_err()); // must merely terminate
    }
}

#[test]
fn path_source_coerces_strings_but_body_does_not() {
    let int_ir = SchemaIr::Integer {
        minimum: None,
        maximum: None,
    };
    // path/query: "7" coerces
    assert!(validate(&int_ir, &json!("7"), Source::Path).is_ok());
    // body: "7" is a type error
    assert!(validate(&int_ir, &json!("7"), Source::Body).is_err());
}

#[test]
fn direct_decoder_programs_never_panic_and_are_deterministic() {
    use q_schema_runtime::DecoderProgram;
    use std::collections::BTreeMap;

    let test_ir = SchemaIr::Object {
        properties: BTreeMap::from([
            (
                "id".to_string(),
                Box::new(SchemaIr::Integer {
                    minimum: Some(1),
                    maximum: Some(500),
                }),
            ),
            (
                "name".to_string(),
                Box::new(SchemaIr::String {
                    min_length: Some(1),
                    max_length: Some(20),
                    pattern: None,
                    format: None,
                }),
            ),
            (
                "active".to_string(),
                Box::new(SchemaIr::Optional {
                    inner: Box::new(SchemaIr::Boolean),
                    default: Some(json!(true)),
                }),
            ),
            (
                "score".to_string(),
                Box::new(SchemaIr::Optional {
                    inner: Box::new(SchemaIr::Number {
                        minimum: Some(0.0),
                        maximum: Some(100.0),
                    }),
                    default: None,
                }),
            ),
        ]),
        required: vec!["id".into(), "name".into()],
    };

    let prog = DecoderProgram::compile(&test_ir, Source::Path);
    let mut rng = Rng(0xbadc0ffeed000001);

    for _ in 0..20_000 {
        let id_val = format!("{}", (rng.next() % 1000) as i64 - 200);
        let name_val = format!("usr_{}", rng.next() % 100);
        let active_val = if rng.next().is_multiple_of(2) {
            "true"
        } else {
            "invalid"
        };
        let score_val = format!("{:.2}", (rng.next() % 200) as f64 - 50.0);

        let params: Vec<(&str, &[u8])> = vec![
            ("id", id_val.as_bytes()),
            ("name", name_val.as_bytes()),
            ("active", active_val.as_bytes()),
            ("score", score_val.as_bytes()),
        ];

        let r1 = prog.decode_params_bytes(&params);
        let r2 = prog.decode_params_bytes(&params);
        assert_eq!(
            r1.is_ok(),
            r2.is_ok(),
            "direct decoder must be deterministic"
        );
    }
}

/// M25-009-A: round-trip fuzz over the generated codecs. For every
/// (representable object schema, random value) pair:
///
/// - the direct ENCODER accepts exactly when the reference validator
///   accepts (never a silent divergence in either direction);
/// - on acceptance, encoder bytes equal serde_json serialization of the
///   reference-normalized output AND parse back to that output;
/// - the direct BODY DECODER re-accepts the encoded bytes (full
///   decode -> encode -> decode round-trip parity).
///
/// No panic, no hang, no unbounded output.
#[test]
fn encoded_decoded_round_trip_matches_reference() {
    use q_schema_runtime::{DecoderTable, EncoderTable};

    let str_spec = |min, max| SchemaIr::String {
        min_length: min,
        max_length: max,
        pattern: None,
        format: None,
    };
    let int_spec = SchemaIr::Integer {
        minimum: Some(-3),
        maximum: Some(50),
    };
    let schemas: Vec<SchemaIr> = vec![
        // flat mixed scalars with bounds
        SchemaIr::Object {
            properties: [
                ("name".to_string(), Box::new(str_spec(Some(1), Some(8)))),
                ("count".to_string(), Box::new(int_spec.clone())),
                (
                    "active".to_string(),
                    Box::new(SchemaIr::Optional {
                        inner: Box::new(SchemaIr::Boolean),
                        default: Some(json!(false)),
                    }),
                ),
            ]
            .into_iter()
            .collect(),
            required: vec!["name".into(), "count".into()],
        },
        // arrays of scalars with item bounds
        SchemaIr::Object {
            properties: [(
                "tags".to_string(),
                Box::new(SchemaIr::Array {
                    items: Box::new(str_spec(None, Some(5))),
                    min_items: Some(1),
                    max_items: Some(4),
                }),
            )]
            .into_iter()
            .collect(),
            required: vec!["tags".into()],
        },
        // nullable + union members + enum + literal + fallback-with-inner
        SchemaIr::Object {
            properties: [
                (
                    "note".to_string(),
                    Box::new(SchemaIr::Nullable {
                        inner: Box::new(str_spec(None, None)),
                    }),
                ),
                (
                    "u".to_string(),
                    Box::new(SchemaIr::Union {
                        members: vec![Box::new(int_spec.clone()), Box::new(SchemaIr::Boolean)],
                    }),
                ),
                (
                    "grade".to_string(),
                    Box::new(SchemaIr::Enum {
                        values: vec![json!("a"), json!("b")],
                    }),
                ),
                (
                    "kind".to_string(),
                    Box::new(SchemaIr::Literal {
                        value: json!("item"),
                    }),
                ),
                (
                    "fb".to_string(),
                    Box::new(SchemaIr::Fallback {
                        reason: "explicit".into(),
                        inner: Some(Box::new(int_spec.clone())),
                    }),
                ),
            ]
            .into_iter()
            .collect(),
            required: vec![],
        },
    ];

    let encoder_table = EncoderTable::from_schemas(&schemas);
    let decoder_table = DecoderTable::from_schemas(&schemas);

    let mut rng = Rng(0xC0DEC0DE);
    let mut accepted = 0usize;
    let mut rejected = 0usize;
    for iteration in 0..20_000 {
        let sid = (rng.next() % schemas.len() as u64) as u32;
        let ir = &schemas[sid as usize];
        // values biased toward the schema's shape: shallow random JSON over
        // the declared property names (plus occasional noise keys)
        // half the corpus: likely-valid randomized objects over the
        // declared keys (exercising acceptance deeply); half: arbitrary
        // random JSON (exercising rejection paths and wrong types)
        let value = if rng.next().is_multiple_of(2) {
            // likely-valid values for EXACTLY the picked schema's declared
            // keys (occasionally dropping optional ones)
            let mut v = serde_json::Map::new();
            let name_len = (rng.next() % 8 + 1) as usize;
            match sid {
                0 => {
                    v.insert("name".into(), json!("n".repeat(name_len)));
                    v.insert("count".into(), json!(((rng.next() % 54) as i64) - 3));
                    if rng.next().is_multiple_of(2) {
                        v.insert("active".into(), json!(rng.next().is_multiple_of(2)));
                    }
                }
                1 => {
                    let n = (rng.next() % 4 + 1) as usize;
                    v.insert(
                        "tags".into(),
                        json!((0..n)
                            .map(|i| "t".repeat((rng.next() % 5 + 1) as usize) + &i.to_string())
                            .collect::<Vec<_>>()),
                    );
                }
                _ => {
                    if rng.next().is_multiple_of(2) {
                        v.insert(
                            "note".into(),
                            if rng.next().is_multiple_of(2) {
                                Value::Null
                            } else {
                                json!("nn")
                            },
                        );
                    }
                    if rng.next().is_multiple_of(2) {
                        v.insert(
                            "u".into(),
                            if rng.next().is_multiple_of(2) {
                                json!(rng.next() % 54)
                            } else {
                                json!(rng.next().is_multiple_of(2))
                            },
                        );
                    }
                    if rng.next().is_multiple_of(2) {
                        v.insert(
                            "grade".into(),
                            if rng.next().is_multiple_of(2) {
                                json!("a")
                            } else {
                                json!("b")
                            },
                        );
                    }
                    if rng.next().is_multiple_of(2) {
                        v.insert("kind".into(), json!("item"));
                    }
                    if rng.next().is_multiple_of(2) {
                        v.insert("fb".into(), json!(((rng.next() % 54) as i64) - 3));
                    }
                }
            }
            Value::Object(v)
        } else {
            let v = random_json(&mut rng, 3);
            match v {
                Value::Object(mut m) => {
                    // randomize presence of declared keys to hit
                    // required-missing and noise keys
                    if rng.next().is_multiple_of(3) {
                        m.remove("name");
                    }
                    if rng.next().is_multiple_of(5) {
                        m.insert("noise".into(), json!(rng.next() % 7));
                    }
                    Value::Object(m)
                }
                other => other,
            }
        };

        let reference = validate(ir, &value, Source::Body);
        let program = encoder_table
            .get(sid)
            .expect("representable schemas compile");

        match reference {
            Ok(expected) => {
                accepted += 1;
                let mut out = Vec::new();
                program
                    .encode(&value, &mut out)
                    .unwrap_or_else(|e| panic!("encoder rejected what reference accepted (iter {iteration}): {e:?}\nvalue: {value}"));
                // byte parity with the reference serialization
                let expected_bytes =
                    serde_json::to_vec(&expected).expect("reference output serializes");
                assert_eq!(
                    out, expected_bytes,
                    "byte drift at iter {iteration} for {value}"
                );
                // parses back to the normalized output (bounded: shallow corpus)
                let reparsed: Value = serde_json::from_slice(&out).expect("encoder bytes parse");
                assert_eq!(
                    reparsed, expected,
                    "round-trip parse drift at iter {iteration}"
                );
                // decoder re-accepts (decode -> encode -> decode parity)
                let decoded = decoder_table
                    .decode_body_value(sid, &reparsed)
                    .unwrap_or_else(|e| {
                        panic!("decoder rejected encoded bytes (iter {iteration}): {e:?}")
                    });
                assert_eq!(decoded, expected);
            }
            Err(_) => {
                rejected += 1;
                let mut out = Vec::new();
                assert!(
                    program.encode(&value, &mut out).is_err(),
                    "encoder accepted what reference rejected (iter {iteration}): {value}"
                );
            }
        }
    }
    // the corpus must exercise both sides meaningfully
    assert!(
        accepted > 1_000,
        "corpus too rejection-heavy: {accepted} accepted"
    );
    assert!(
        rejected > 1_000,
        "corpus too acceptance-heavy: {rejected} rejected"
    );
}
