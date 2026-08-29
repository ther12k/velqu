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

/// M28-010-A: the M28-007/008 policy vector families (redirect limiter,
/// egress control, decompression bounds) execute against the compiled
/// policy — every manifest case drives the real API and every expected
/// error maps to its exact typed variant.
#[test]
fn fetch_m28_policy_manifest_vectors_execute_against_compiled_policy() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../conformance/web-api/wpt-manifest.json");
    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    let fetch = &manifest["capabilities"]["fetch"];
    let find = |id: &str| -> serde_json::Value {
        fetch["pinnedSubsets"]
            .as_array()
            .expect("fetch subsets")
            .iter()
            .find(|s| s["id"] == id)
            .unwrap_or_else(|| panic!("{id} subset missing"))
            .clone()
    };

    fn err_variant(err: &q_capabilities::FetchPolicyError) -> &'static str {
        use q_capabilities::FetchPolicyError as E;
        match err {
            E::SchemeNotAllowed { .. } => "SchemeNotAllowed",
            E::AddressDenied { .. } => "AddressDenied",
            E::HostnameDenied { .. } => "HostnameDenied",
            E::DowngradeRedirect { .. } => "DowngradeRedirect",
            E::TooManyRedirects { .. } => "TooManyRedirects",
            E::InvalidRedirectHops { .. } => "InvalidRedirectHops",
            E::InvalidDeadline { .. } => "InvalidDeadline",
            E::InvalidBodyLimit { .. } => "InvalidBodyLimit",
            E::BodyTooLarge { .. } => "BodyTooLarge",
            E::RedirectLoop { .. } => "RedirectLoop",
            E::DecompressedTooLarge { .. } => "DecompressedTooLarge",
            E::DecompressionBomb { .. } => "DecompressionBomb",
        }
    }

    use q_capabilities::{
        resolve_and_validate, DecompressionGuard, RedirectLimiter, RedirectOutcome,
        MAX_FETCH_RESPONSE_BODY_BYTES,
    };
    use std::net::IpAddr;

    let mut ran = 0usize;

    // ---- fetch-redirect-policy ----
    let redirect_subset = find("fetch-redirect-policy");
    let cases = redirect_subset["cases"].as_array().unwrap();
    for case in cases {
        let expect = case["expect"].as_str().unwrap();
        let mut lim = RedirectLimiter::new(q_capabilities::FetchPolicy::default());
        let outcome: Result<&'static str, String> = match case["op"].as_str().unwrap() {
            "follow" => {
                match lim.evaluate(case["from"].as_str().unwrap(), case["to"].as_str().unwrap()) {
                    Ok(RedirectOutcome::Follow) => Ok(""),
                    Ok(RedirectOutcome::Surface) => Err("unexpected surface".into()),
                    Err(e) => Err(err_variant(&e).to_string()),
                }
            }
            "ceiling" => {
                let hops = case["hops"].as_u64().unwrap() as u32;
                let mut result = Ok(());
                for hop in 1..=hops {
                    result = lim
                        .evaluate(
                            &format!("https://a.test/r{}", hop - 1),
                            &format!("https://a.test/r{hop}"),
                        )
                        .map(|_| ());
                    if result.is_err() {
                        break;
                    }
                }
                result.map(|_| "").map_err(|e| err_variant(&e).to_string())
            }
            "loop" => {
                let _ = lim.evaluate("https://a.test/x", "https://b.test/y");
                let _ = lim.evaluate("https://b.test/y", "https://a.test/x");
                lim.evaluate("https://a.test/x", "https://b.test/y")
                    .map(|_| "")
                    .map_err(|e| err_variant(&e).to_string())
            }
            "manual" => {
                let policy = q_capabilities::FetchPolicy::default()
                    .with_redirect_policy(q_capabilities::RedirectPolicy::Manual);
                let mut manual = RedirectLimiter::new(policy);
                match manual.evaluate("https://a.test/", "https://b.test/") {
                    Ok(RedirectOutcome::Surface) => Ok("surface"),
                    _ => Err("expected surface".into()),
                }
            }
            other => panic!("unknown redirect op: {other}"),
        };
        match (outcome.as_deref(), expect) {
            (Ok(""), "follow") | (Ok(""), "allow") => {}
            (Ok("surface"), "surface") => {}
            (Err(variant), e) if e.strip_prefix("deny:") == Some(variant) => {}
            (r, e) => panic!(
                "redirect vector mismatch: op {} expected {e}, got {:?}",
                case["op"].as_str().unwrap(),
                r
            ),
        }
        ran += 1;
    }

    // ---- fetch-egress-control ----
    let egress_subset = find("fetch-egress-control");
    let cases = egress_subset["cases"].as_array().unwrap();
    for case in cases {
        let expect = case["expect"].as_str().unwrap();
        let op = case["op"].as_str().unwrap();
        let result: Result<&'static str, String> = match op {
            "resolve_metadata_name" | "resolve_mixed" | "resolve_ok" | "resolve_empty" => {
                let policy = q_capabilities::FetchPolicy::default();
                let addrs: Vec<IpAddr> = case["addrs"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .map(|v| v.as_str().unwrap().parse().unwrap())
                            .collect()
                    })
                    .unwrap_or_default();
                let static_addrs: &'static [IpAddr] = Box::leak(addrs.into_boxed_slice());
                let mut resolve =
                    move |_host: &str| Ok::<Vec<IpAddr>, String>(static_addrs.to_vec());
                resolve_and_validate(&policy, case["host"].as_str().unwrap(), &mut resolve)
                    .map(|_| "")
                    .map_err(|e| err_variant(&e).to_string())
            }
            "resolve_ip_literal" => {
                let policy = q_capabilities::FetchPolicy::default();
                let mut resolve = |_host: &str| panic!("IP literals must not reach the resolver");
                resolve_and_validate(&policy, case["host"].as_str().unwrap(), &mut resolve)
                    .map(|_| "")
                    .map_err(|e| err_variant(&e).to_string())
            }
            "config_deny" | "config_allow" | "config_deny_wins" => {
                let mut policy = q_capabilities::FetchPolicy::default();
                if let Some(deny) = case["deny"].as_array() {
                    let owned: Vec<String> = deny
                        .iter()
                        .map(|v| v.as_str().unwrap().to_string())
                        .collect();
                    policy = policy.with_deny_hosts(owned);
                }
                if let Some(allow) = case["allow"].as_array() {
                    let owned: Vec<String> = allow
                        .iter()
                        .map(|v| v.as_str().unwrap().to_string())
                        .collect();
                    policy = policy.with_allow_hosts(owned);
                }
                policy
                    .check_host_config(case["host"].as_str().unwrap())
                    .map(|_| "")
                    .map_err(|e| err_variant(&e).to_string())
            }
            // The safe default composes BEFORE configuration: run the full
            // connect gate so allow-listing a metadata name still denies by
            // name (and the resolver is never consulted).
            "config_metadata_allow_cannot_reenable" => {
                let policy = q_capabilities::FetchPolicy::default()
                    .with_allow_hosts([case["allow"][0].as_str().unwrap()]);
                let called = std::cell::Cell::new(false);
                let resolve = |_host: &str| {
                    called.set(true);
                    Ok::<Vec<IpAddr>, String>(vec!["93.184.216.34".parse().unwrap()])
                };
                let out = resolve_and_validate(&policy, case["host"].as_str().unwrap(), resolve)
                    .map(|_| "");
                assert!(!called.get(), "metadata denial must precede resolution");
                out.map_err(|e| err_variant(&e).to_string())
            }
            other => panic!("unknown egress op: {other}"),
        };
        match (result.as_deref(), expect) {
            (Ok(""), "allow") => {}
            (Err(variant), e) if e.strip_prefix("deny:") == Some(variant) => {}
            (r, e) => panic!("egress vector mismatch: op {op} expected {e}, got {:?}", r),
        }
        ran += 1;
    }

    // ---- fetch-decompression-bounds ----
    let decompression_subset = find("fetch-decompression-bounds");
    let cases = decompression_subset["cases"].as_array().unwrap();
    for case in cases {
        let expect = case["expect"].as_str().unwrap();
        let op = case["op"].as_str().unwrap();
        let result: Result<&'static str, String> = match op {
            "decompress" => {
                let mut g = DecompressionGuard::new(MAX_FETCH_RESPONSE_BODY_BYTES);
                g.compressed_input(case["compressed"].as_u64().unwrap() as usize);
                g.decompressed_output(case["output"].as_u64().unwrap() as usize)
                    .map(|_| "")
                    .map_err(|e| err_variant(&e).to_string())
            }
            "proxy_mode" => {
                if q_capabilities::ProxyMode::Disabled
                    == q_capabilities::FetchPolicy::default().proxy_mode()
                {
                    Ok("")
                } else {
                    Err("proxy not disabled".into())
                }
            }
            other => panic!("unknown decompression op: {other}"),
        };
        match (result.as_deref(), expect) {
            (Ok(""), "allow") => {}
            (Ok(""), "disabled") => {}
            (Err(variant), e) if e.strip_prefix("deny:") == Some(variant) => {}
            (r, e) => panic!(
                "decompression vector mismatch: op {op} expected {e}, got {:?}",
                r
            ),
        }
        ran += 1;
    }

    // The manifest must have delivered every new vector to this executor.
    assert_eq!(ran, 21, "expected 21 M28 policy manifest vectors");
}
