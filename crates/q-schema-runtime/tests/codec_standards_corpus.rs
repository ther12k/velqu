//! M25-009-B: the generated codecs' output compared against STANDARD JSON
//! behavior — hand-written expected bytes (RFC 8259 semantics, independent
//! of serde_json construction), plus a minimized regression corpus replay
//! for every fuzz finding so far.

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
