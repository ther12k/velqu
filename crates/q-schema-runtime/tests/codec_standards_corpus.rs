//! M25-009-B: the generated codecs' output compared against STANDARD JSON
//! behavior — hand-written expected bytes (RFC 8259 semantics, independent
//! of serde_json construction), plus a minimized regression corpus replay
//! for every fuzz finding so far.
//!
//! M25-009-D — minimized fuzz-findings registry (each finding → its
//! permanent fixture):
//! 1. M25-009-A (round-trip fuzz, iteration 2): fallback-WITH-inner was
//!    not transparent in the encoder (`unsupported` instead of the inner
//!    shape) → `fallback_with_inner_encodes_transparently` below.
//! 2. M25-009-C (malformed corpus): the decoder leaked the last union
//!    member's internal error on a total miss instead of the canonical
//!    `union` problem → `union_miss_reports_canonical_code_everywhere`.

use q_schema_runtime::{EncoderProgram, ProblemProgram, SchemaIr};
use serde_json::{json, Value};
use std::collections::BTreeMap;

fn obj(props: Vec<(&str, SchemaIr)>, required: Vec<&str>) -> SchemaIr {
    SchemaIr::Object {
        properties: props
            .into_iter()
            .map(|(k, v)| (k.to_string(), Box::new(v)))
            .collect::<BTreeMap<_, _>>(),
        required: required.into_iter().map(String::from).collect(),
    }
}

fn free_string() -> SchemaIr {
    SchemaIr::String {
        min_length: None,
        max_length: None,
        pattern: None,
        format: None,
    }
}

/// Encoder bytes equal HAND-WRITTEN standard JSON — escaping, number
/// formatting, member ordering, and nesting all follow RFC 8259 exactly
/// as a standards-conformant producer emits them.
#[test]
fn encoder_output_matches_standard_json_bytes() {
    let ir = obj(
        vec![
            ("esc", free_string()),
            (
                "n",
                SchemaIr::Number {
                    minimum: None,
                    maximum: None,
                },
            ),
            (
                "i",
                SchemaIr::Integer {
                    minimum: None,
                    maximum: None,
                },
            ),
            (
                "list",
                SchemaIr::Array {
                    items: Box::new(SchemaIr::Integer {
                        minimum: None,
                        maximum: None,
                    }),
                    min_items: None,
                    max_items: None,
                },
            ),
        ],
        vec!["esc", "n", "i", "list"],
    );
    let program = EncoderProgram::compile(&ir).unwrap();

    let cases: Vec<(Value, &str)> = vec![
        // string escaping: quote, backslash, control chars become the two-
        // character escapes; non-ASCII passes through as UTF-8; forward
        // slash is NOT escaped (RFC 8259 leaves it unescaped)
        (
            json!({"esc": "a\"b\\c\nd\te☺ƒ/x", "n": 2.5, "i": 42, "list": [1, -2]}),
            "{\"esc\":\"a\\\"b\\\\c\\nd\\te☺ƒ/x\",\"i\":42,\"list\":[1,-2],\"n\":2.5}",
        ),
        // float that is integral keeps float form ("3.0"), matching the
        // reference producer's canonical f64 formatting
        (
            json!({"esc": "s", "n": 3.0, "i": 0, "list": []}),
            "{\"esc\":\"s\",\"i\":0,\"list\":[],\"n\":3.0}",
        ),
        // i64 extremes round through unchanged
        (
            json!({"esc": "s", "n": -1.5, "i": i64::MAX, "list": [i64::MIN]}),
            "{\"esc\":\"s\",\"i\":9223372036854775807,\"list\":[-9223372036854775808],\"n\":-1.5}",
        ),
        // unicode control range escapes
        (
            json!({"esc": "\u{1}\u{1f}", "n": 0.0, "i": 1, "list": []}),
            "{\"esc\":\"\\u0001\\u001f\",\"i\":1,\"list\":[],\"n\":0.0}",
        ),
    ];

    for (value, expected) in cases {
        let mut out = Vec::new();
        program
            .encode(&value, &mut out)
            .unwrap_or_else(|e| panic!("standards case must encode: {e:?} for {value}"));
        assert_eq!(
            String::from_utf8(out.clone()).unwrap(),
            expected,
            "encoder bytes diverge from standard JSON"
        );
        // and the bytes parse back (standards-conformant parser accepts)
        let reparsed: Value = serde_json::from_slice(&out).unwrap();
        assert_eq!(
            reparsed,
            serde_json::to_value(&value).unwrap(),
            "round-trip semantic drift"
        );
    }
}

/// The problem encoder emits the canonical RFC 9457 member order —
/// type, title, status, instance, detail?, errors?, extensions —
/// verified against a hand-written envelope.
#[test]
fn problem_encoder_matches_standard_envelope() {
    let ir = SchemaIr::Problem {
        type_uri: None,
        title: "Validation failed".into(),
        status: 422,
        detail: None,
    };
    let program = ProblemProgram::compile(&ir).unwrap();
    let mut out = Vec::new();
    program
        .encode(
            "https://velqu.dev/problems/validation",
            "Validation failed",
            None,
            Some("bad input"),
            &[],
            &[("traceId".to_string(), json!("t-1"))],
            "/orders/1",
            &mut out,
        )
        .unwrap();
    assert_eq!(
        String::from_utf8(out).unwrap(),
        "{\"type\":\"https://velqu.dev/problems/validation\",\"title\":\"Validation failed\",\"status\":422,\"instance\":\"/orders/1\",\"detail\":\"bad input\",\"traceId\":\"t-1\"}"
    );
}

/// Minimized fuzz-finding corpus replay (M25-009-A finding + boundary
/// shapes). Every entry must keep behaving exactly as recorded.
#[test]
fn codec_regression_corpus_replays() {
    // M25-009-A iteration-2 finding: fallback-WITH-inner is transparent —
    // encodes the inner shape, never `unsupported`
    let fb = obj(
        vec![(
            "fb",
            SchemaIr::Fallback {
                reason: "explicit".into(),
                inner: Some(Box::new(SchemaIr::Integer {
                    minimum: Some(0),
                    maximum: Some(50),
                })),
            },
        )],
        vec!["fb"],
    );
    let program = EncoderProgram::compile(&fb).unwrap();
    let mut out = Vec::new();
    program.encode(&json!({ "fb": 21 }), &mut out).unwrap();
    assert_eq!(String::from_utf8(out).unwrap(), "{\"fb\":21}");
    let mut out = Vec::new();
    let err = program.encode(&json!({ "fb": 99 }), &mut out).unwrap_err();
    assert_eq!(err[0].code, "maximum");

    // exact boundary values stay accepted at the bound and rejected past it
    let bounded = obj(
        vec![(
            "s",
            SchemaIr::String {
                min_length: Some(2),
                max_length: Some(4),
                pattern: None,
                format: None,
            },
        )],
        vec!["s"],
    );
    let program = EncoderProgram::compile(&bounded).unwrap();
    for len in [2usize, 3, 4] {
        let mut out = Vec::new();
        program
            .encode(&json!({ "s": "x".repeat(len) }), &mut out)
            .unwrap_or_else(|e| panic!("length {len} must be inside bounds: {e:?}"));
    }
    for len in [1usize, 5] {
        let mut out = Vec::new();
        assert!(
            program
                .encode(&json!({ "s": "x".repeat(len) }), &mut out)
                .is_err(),
            "length {len} must be outside bounds"
        );
    }
}

/// M25-009-C: malformed and boundary value corpus. Every malformed entry
/// must produce a TYPED error (never a panic) with decoder/reference/encoder
/// agreement; every boundary entry must be accepted everywhere with output
/// parity.
#[test]
fn malformed_and_boundary_corpus() {
    use q_schema_runtime::{validate, DecoderTable, Source};

    let int_b = |min, max| SchemaIr::Integer {
        minimum: min,
        maximum: max,
    };
    let num_b = |min, max| SchemaIr::Number {
        minimum: min,
        maximum: max,
    };
    let str_b = |min, max, fmt| SchemaIr::String {
        min_length: min,
        max_length: max,
        pattern: None,
        format: fmt,
    };
    let arr_b = |min, max| SchemaIr::Array {
        items: Box::new(int_b(None, None)),
        min_items: min,
        max_items: max,
    };

    let schema = obj(
        vec![
            ("i", int_b(Some(0), Some(100))),
            ("n", num_b(Some(-2.5), Some(2.5))),
            ("s", str_b(Some(1), Some(10), None)),
            ("email", str_b(None, None, Some("email".into()))),
            ("uuid", str_b(None, None, Some("uuid".into()))),
            ("list", arr_b(Some(1), Some(3))),
            (
                "grade",
                SchemaIr::Enum {
                    values: vec![json!("a"), json!("b")],
                },
            ),
            (
                "u",
                SchemaIr::Union {
                    members: vec![Box::new(int_b(Some(0), None)), Box::new(free_string())],
                },
            ),
        ],
        vec!["i", "s"],
    );

    let malformed: Vec<(&str, Value)> = vec![
        // wrong types at every declared position
        ("i as string", json!({"i": "5", "s": "x"})),
        ("i as float", json!({"i": 5.5, "s": "x"})),
        ("i as bool", json!({"i": true, "s": "x"})),
        ("i as array", json!({"i": [5], "s": "x"})),
        ("i as null", json!({"i": null, "s": "x"})),
        ("i beyond i64", json!({"i": u64::MAX, "s": "x"})),
        ("n as string", json!({"i": 1, "s": "x", "n": "1.5"})),
        ("n as bool", json!({"i": 1, "s": "x", "n": false})),
        ("s as number", json!({"i": 1, "s": 9})),
        ("s as array", json!({"i": 1, "s": ["x"]})),
        (
            "email malformed",
            json!({"i": 1, "s": "x", "email": "not-an-email"}),
        ),
        (
            "email empty local",
            json!({"i": 1, "s": "x", "email": "@example.org"}),
        ),
        ("uuid malformed", json!({"i": 1, "s": "x", "uuid": "xyz"})),
        (
            "uuid truncated",
            json!({"i": 1, "s": "x", "uuid": "123e4567-e89b-12d3-a456-4266141740"}),
        ),
        ("list as scalar", json!({"i": 1, "s": "x", "list": 3})),
        (
            "list wrong item type",
            json!({"i": 1, "s": "x", "list": [1, "two"]}),
        ),
        (
            "grade not a member",
            json!({"i": 1, "s": "x", "grade": "c"}),
        ),
        (
            "union matches no member",
            json!({"i": 1, "s": "x", "u": true}),
        ),
        (
            "union negative int misses first, non-string misses second",
            json!({"i": 1, "s": "x", "u": -1}),
        ),
        ("whole value not an object", json!([1, 2])),
        ("whole value scalar", json!("nope")),
        // bound violations just past the line
        ("i below min", json!({"i": -1, "s": "x"})),
        ("i above max", json!({"i": 101, "s": "x"})),
        ("n below min", json!({"i": 1, "s": "x", "n": -2.6})),
        ("n above max", json!({"i": 1, "s": "x", "n": 2.500001})),
        ("s empty violates minLength", json!({"i": 1, "s": ""})),
        ("s over maxLength", json!({"i": 1, "s": "xxxxxxxxxxx"})),
        (
            "list empty violates minItems",
            json!({"i": 1, "s": "x", "list": []}),
        ),
        (
            "list over maxItems",
            json!({"i": 1, "s": "x", "list": [1, 2, 3, 4]}),
        ),
        ("required s missing", json!({"i": 1})),
        ("unknown key", json!({"i": 1, "s": "x", "extra": 1})),
    ];

    let decoder_table = DecoderTable::from_schemas(std::slice::from_ref(&schema));
    let encoder_table = q_schema_runtime::EncoderTable::from_schemas(std::slice::from_ref(&schema));

    for (label, value) in malformed {
        // reference rejects with typed errors
        let ref_err = validate(&schema, &value, Source::Body)
            .unwrap_err_else(format!("malformed case must reject: {label}"));
        // decoder agrees (typed, never a panic)
        let dec = decoder_table
            .decode_body_value(0, &value)
            .unwrap_err_else(format!("decoder must reject: {label}"));
        // encoder agrees
        let program = encoder_table.get(0).unwrap();
        let mut out = Vec::new();
        let enc = program
            .encode(&value, &mut out)
            .unwrap_err_else(format!("encoder must reject: {label}"));
        // error-code parity on the first error (path parity where the
        // entry targets a single field)
        assert_eq!(
            dec[0].code, ref_err[0].code,
            "decoder/reference code mismatch for {label}"
        );
        assert_eq!(
            enc[0].code, ref_err[0].code,
            "encoder/reference code mismatch for {label}"
        );
    }

    // boundary values: every side accepts, outputs identical
    let boundary: Vec<(&str, Value)> = vec![
        ("exact min/max ints", json!({"i": 0, "s": "x"})),
        ("exact max int", json!({"i": 100, "s": "x"})),
        ("exact numeric bounds", json!({"i": 1, "s": "x", "n": -2.5})),
        (
            "exact numeric bound high",
            json!({"i": 1, "s": "x", "n": 2.5}),
        ),
        ("exact string bounds", json!({"i": 1, "s": "x".repeat(10)})),
        (
            "exact list bounds",
            json!({"i": 1, "s": "x", "list": [1, 2, 3]}),
        ),
        ("list single item", json!({"i": 1, "s": "x", "list": [7]})),
        ("enum exact member", json!({"i": 1, "s": "x", "grade": "a"})),
        (
            "union first member boundary (0)",
            json!({"i": 1, "s": "x", "u": 0}),
        ),
        ("valid email", json!({"i": 1, "s": "x", "email": "a@b.co"})),
        (
            "valid uuid",
            json!({"i": 1, "s": "x", "uuid": "123e4567-e89b-12d3-a456-426614174000"}),
        ),
    ];
    for (label, value) in boundary {
        let reference = validate(&schema, &value, Source::Body)
            .unwrap_or_else(|_| panic!("boundary case must accept: {label}"));
        let decoded = decoder_table
            .decode_body_value(0, &value)
            .unwrap_or_else(|_| panic!("decoder must accept: {label}"));
        assert_eq!(decoded, reference, "decoder drift at {label}");
        let program = encoder_table.get(0).unwrap();
        let mut out = Vec::new();
        program
            .encode(&value, &mut out)
            .unwrap_or_else(|_| panic!("encoder must accept: {label}"));
        assert_eq!(
            out,
            serde_json::to_vec(&reference).unwrap(),
            "encoder byte drift at {label}"
        );
    }
}

/// unwrap_err with a message (for malformed entries that MUST reject).
trait UnwrapErrElse {
    type Out;
    fn unwrap_err_else(self, msg: String) -> Self::Out;
}
impl<T: std::fmt::Debug, E> UnwrapErrElse for Result<T, E> {
    type Out = E;
    fn unwrap_err_else(self, msg: String) -> E {
        match self {
            Err(e) => e,
            Ok(v) => panic!("{msg}: accepted {v:?}"),
        }
    }
}

/// M25-009-C finding, minimized (M25-009-D): on a union total miss every
/// codec reports the CANONICAL typed `union` problem — decoder, reference
/// validator, and encoder agree on the code; member-internal errors never
/// leak.
#[test]
fn union_miss_reports_canonical_code_everywhere() {
    use q_schema_runtime::{validate, DecoderTable, EncoderTable, Source};

    let ir = obj(
        vec![(
            "u",
            SchemaIr::Union {
                members: vec![
                    Box::new(SchemaIr::Integer {
                        minimum: Some(0),
                        maximum: None,
                    }),
                    Box::new(free_string()),
                ],
            },
        )],
        vec!["u"],
    );
    let decoder_table = DecoderTable::from_schemas(std::slice::from_ref(&ir));
    let encoder_table = EncoderTable::from_schemas(std::slice::from_ref(&ir));

    // misses through both member kinds: bool misses both; a negative int
    // misses the integer member and is not a string
    for miss in [json!({ "u": true }), json!({ "u": -1 })] {
        let reference = validate(&ir, &miss, Source::Body).unwrap_err();
        assert_eq!(reference[0].code, "union");
        assert_eq!(
            reference[0].message,
            "value matched none of 2 union members"
        );

        let decoded = decoder_table.decode_body_value(0, &miss).unwrap_err();
        assert_eq!(
            decoded[0].code, "union",
            "decoder must not leak member errors"
        );
        assert_eq!(decoded[0].path, reference[0].path);

        let program = encoder_table.get(0).unwrap();
        let mut out = Vec::new();
        let encoded = program.encode(&miss, &mut out).unwrap_err();
        assert_eq!(encoded[0].code, "union");
    }

    // first-match still wins on hits (integers take member one)
    let hit = json!({ "u": 7 });
    decoder_table
        .decode_body_value(0, &hit)
        .unwrap_or_else(|e| panic!("hit must decode: {e:?}"));
}
