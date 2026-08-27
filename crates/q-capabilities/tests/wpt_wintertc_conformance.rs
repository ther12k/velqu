//! WPT / WinterTC Conformance Integration Suite (M27-010-A, M28-001-C).
//!
//! Evaluates `q-capabilities` models directly against the pinned WPT/WinterTC
//! test subset defined in `conformance/web-api/wpt-manifest.json`.

use std::net::IpAddr;
use std::path::Path;

use q_capabilities::abort::{AbortControllerModel, AbortSignalModel};
use q_capabilities::crypto::{CryptoRandom, MAX_RANDOM_BYTES_LEN};
use q_capabilities::fetch_policy::FetchPolicy;
use q_capabilities::text_encoding::{TextDecoderModel, TextDecoderOptions, TextEncoderModel};
use q_capabilities::url_model::{ParsedSearchParams, ParsedUrl};

#[test]
fn wpt_url_relative_resolution_pinned_subset() {
    let base = "https://example.com/a/b/c";
    let vectors = [
        ("../d", "https://example.com/a/d"),
        ("../../d", "https://example.com/d"),
        ("./d", "https://example.com/a/b/d"),
        ("/root", "https://example.com/root"),
        ("?query=1", "https://example.com/a/b/c?query=1"),
        ("#hash", "https://example.com/a/b/c#hash"),
    ];

    for (input, expected) in vectors {
        let u = ParsedUrl::parse(input, Some(base)).expect("valid resolution");
        assert_eq!(u.href, expected, "resolution failed for input {input}");
    }
}

#[test]
fn wpt_url_normalization_pinned_subset() {
    let u1 = ParsedUrl::parse("http://example.com:80/path", None).unwrap();
    assert_eq!(u1.origin, "http://example.com");
    assert_eq!(u1.port, "");

    let u2 = ParsedUrl::parse("https://example.com:443/path", None).unwrap();
    assert_eq!(u2.origin, "https://example.com");
    assert_eq!(u2.port, "");

    let u3 = ParsedUrl::parse("http://[::1]:8080/path", None).unwrap();
    assert_eq!(u3.host, "[::1]:8080");
    assert_eq!(u3.hostname, "[::1]");

    let u4 = ParsedUrl::parse("https://example.com/path with spaces/", None).unwrap();
    assert_eq!(u4.pathname, "/path%20with%20spaces/");
}

#[test]
fn wintertc_urlsearchparams_pinned_subset() {
    let mut sp = ParsedSearchParams::parse("a=1&b=2&a=3");
    assert_eq!(sp.get("a"), Some("1"));
    assert_eq!(sp.get_all("a"), vec!["1", "3"]);
    assert!(sp.has("b", None));

    sp.append("key", "val");
    assert!(sp.has("key", None));

    sp.sort();
    let s = sp.to_query_string();
    assert!(s.starts_with("a=1&a=3&b=2&key=val"), "got {s}");

    sp.delete("a", None);
    assert_eq!(sp.get("a"), None);
}

#[test]
fn wpt_textencoder_utf8_pinned_subset() {
    let vectors: [(&str, &[u8]); 4] = [
        ("hello", &[104, 101, 108, 108, 111]),
        ("café", &[99, 97, 102, 195, 169]),
        (
            "こんにちは",
            &[
                227, 129, 147, 227, 130, 147, 227, 129, 171, 227, 129, 161, 227, 129, 175,
            ],
        ),
        ("🚀", &[240, 159, 154, 128]),
    ];

    for (input, expected) in vectors {
        let bytes = TextEncoderModel::encode(input).unwrap();
        assert_eq!(bytes, expected, "encode mismatch for '{input}'");
    }
}

#[test]
fn wpt_textdecoder_utf8_pinned_subset() {
    // Lossless
    let decoder = TextDecoderModel::new(None, TextDecoderOptions::default()).unwrap();
    let s = decoder.decode(&[104, 101, 108, 108, 111]).unwrap();
    assert_eq!(s, "hello");

    // BOM strip vs preserve
    let bom_bytes = &[239, 187, 191, 104, 105];
    let dec_strip = TextDecoderModel::new(
        None,
        TextDecoderOptions {
            fatal: false,
            ignore_bom: false,
        },
    )
    .unwrap();
    let s_strip = dec_strip.decode(bom_bytes).unwrap();
    assert_eq!(s_strip, "hi");

    let dec_preserve = TextDecoderModel::new(
        None,
        TextDecoderOptions {
            fatal: false,
            ignore_bom: true,
        },
    )
    .unwrap();
    let s_preserve = dec_preserve.decode(bom_bytes).unwrap();
    assert_eq!(s_preserve, "\u{FEFF}hi");

    // Replacement mode vs Fatal mode
    let invalid_bytes = &[104, 255, 105];
    let dec_replace = TextDecoderModel::new(
        None,
        TextDecoderOptions {
            fatal: false,
            ignore_bom: false,
        },
    )
    .unwrap();
    let s_replace = dec_replace.decode(invalid_bytes).unwrap();
    assert_eq!(s_replace, "h\u{FFFD}i");

    let dec_fatal = TextDecoderModel::new(
        None,
        TextDecoderOptions {
            fatal: true,
            ignore_bom: false,
        },
    )
    .unwrap();
    let err_fatal = dec_fatal.decode(invalid_bytes);
    assert!(err_fatal.is_err(), "fatal mode must reject invalid UTF-8");
}

#[test]
fn wpt_abortcontroller_and_signal_pinned_subset() {
    let ctrl = AbortControllerModel::new();
    assert!(!ctrl.signal().is_aborted());

    assert!(ctrl.abort(Some("custom-reason")));
    assert!(ctrl.signal().is_aborted());
    assert_eq!(ctrl.signal().reason().as_deref(), Some("custom-reason"));

    // Idempotent abort
    assert!(!ctrl.abort(Some("second-abort")));
    assert_eq!(ctrl.signal().reason().as_deref(), Some("custom-reason"));

    // Pre-aborted factory
    let sig = AbortSignalModel::aborted_with("pre-abort");
    assert!(sig.is_aborted());
    assert_eq!(sig.reason().as_deref(), Some("pre-abort"));
}

#[test]
fn wpt_crypto_random_pinned_subset() {
    // getRandomValues quota
    let mut buf = vec![0u8; 16];
    CryptoRandom::get_random_values(&mut buf).unwrap();
    assert!(
        buf.iter().any(|&b| b != 0),
        "entropy fill must produce non-zero values"
    );

    assert_eq!(MAX_RANDOM_BYTES_LEN, 65536);

    // randomUUID format
    let uuid = CryptoRandom::random_uuid().unwrap();
    assert_eq!(uuid.len(), 36);
    let parts: Vec<&str> = uuid.split('-').collect();
    assert_eq!(parts.len(), 5);
    assert_eq!(parts[0].len(), 8);
    assert_eq!(parts[1].len(), 4);
    assert_eq!(parts[2].len(), 4);
    assert_eq!(parts[3].len(), 4);
    assert_eq!(parts[4].len(), 12);
    assert!(parts[2].starts_with('4'), "version nibble must be 4");
    let variant_first = parts[3].chars().next().unwrap();
    assert!(
        matches!(variant_first, '8' | '9' | 'a' | 'b'),
        "variant bits must be 10xx"
    );
}

/// M28-001-C: execute the pinned fetch security-policy vectors from
/// `conformance/web-api/wpt-manifest.json` against the compiled policy.
#[test]
fn fetch_policy_manifest_vectors_execute_against_compiled_policy() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../conformance/web-api/wpt-manifest.json");
    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    let fetch = manifest["capabilities"]["fetch"]
        .as_object()
        .expect("fetch capability pinned in manifest");
    let subset = fetch["pinnedSubsets"]
        .as_array()
        .expect("fetch subsets")
        .iter()
        .find(|s| s["id"] == "fetch-policy-security")
        .expect("fetch-policy-security subset");

    let policy = FetchPolicy::default();
    let mut allowed = 0usize;
    let mut denied = 0usize;
    for case in subset["cases"].as_array().expect("cases") {
        let expect = case["expect"].as_str().expect("expect");
        let result = match case["check"].as_str().expect("check") {
            "scheme" => policy.check_scheme(case["input"].as_str().expect("input")),
            "address" => {
                let addr: IpAddr = case["input"].as_str().expect("input").parse().expect("ip");
                policy.check_address(addr)
            }
            "redirect" => policy.check_redirect_hop(
                case["from"].as_str().expect("from"),
                case["to"].as_str().expect("to"),
                case["hop"].as_u64().expect("hop") as u32,
            ),
            other => panic!("unknown fetch policy check kind: {other}"),
        };
        match (result, expect) {
            (Ok(()), "allow") => allowed += 1,
            (Err(_), "deny") => denied += 1,
            (r, e) => panic!(
                "fetch policy vector mismatch: check {} input {:?} expected {e}, got {:?}",
                case["check"],
                case["input"],
                r.map_err(|err| err.to_string())
            ),
        }
    }
    // The subset must exercise both directions — a one-sided matrix proves
    // nothing.
    assert!(allowed >= 4, "expected allow vectors, got {allowed}");
    assert!(denied >= 15, "expected deny vectors, got {denied}");
    assert_eq!(allowed + denied, subset["cases"].as_array().unwrap().len());
}
