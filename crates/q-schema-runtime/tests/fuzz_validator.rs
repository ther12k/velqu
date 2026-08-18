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
