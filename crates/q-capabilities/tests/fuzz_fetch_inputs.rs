//! Property-based robustness tests for fetch inputs (M28-010-C): URLs,
//! header names, host configuration entries, and decompression sequences.
//! Deterministic xorshift PRNG — no external dependency; runs in every
//! `cargo test` invocation. Properties assert more than no-panic: the
//! security invariants (credential stripping, dialable pins, scheme
//! allowlist) must hold for EVERY input.

use q_capabilities::fetch_policy::{
    check_body_helper_size, headers_surviving_redirect, is_credential_header,
    is_cross_origin_redirect, url_origin, BodyHelper, CREDENTIAL_REDIRECT_HEADERS,
    MAX_EGRESS_HOST_ENTRIES,
};
use q_capabilities::{
    resolve_and_validate, DecompressionGuard, FetchPolicy, FetchPolicyError, ParsedUrl,
    RedirectLimiter, RedirectOutcome, MAX_FETCH_RESPONSE_BODY_BYTES, MAX_REDIRECT_HOPS,
};
use std::net::IpAddr;

/// xorshift PRNG — deterministic, no deps.
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
    /// Pseudo-random printable string (may contain URL/header metachars).
    fn text(&mut self, n: usize) -> String {
        const ALPHABET: &[u8] = b"abcXYZ019:/.?#[]@!$&'()*+,;=~-_^% \t\r\n<>\"\\|{}";
        (0..n)
            .map(|_| ALPHABET[(self.next() as usize) % ALPHABET.len()] as char)
            .collect()
    }
    fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[(self.next() as usize) % items.len()]
    }
}

const ITERATIONS: usize = 512;

/// Seed corpus of interesting hosts/URLs (mixed with fuzzed text).
const HOST_SEEDS: &[&str] = &[
    "example.test",
    "127.0.0.1",
    "169.254.169.254",
    "[::1]",
    "metadata.google.internal",
    "192.168.1.1",
    "[fd00::ec2:1234]",
    "evil.test.evil",
    "",
];

#[test]
fn fuzzed_urls_never_panic_and_malformed_never_pass_scheme_gate() {
    let mut rng = Rng(0x243f6a8885a308d3);
    let policy = FetchPolicy::default();
    for _ in 0..ITERATIONS {
        let len = (rng.next() % 64) as usize;
        let junk = rng.text(len);
        // Parsing never panics: Ok or Err, always.
        let parsed = ParsedUrl::parse(&junk, Some("https://base.test/"));
        if let Ok(u) = parsed {
            // A successful parse must still fail the scheme gate unless the
            // scheme is exactly http/https (case-insensitive).
            let scheme = u
                .href
                .split("://")
                .next()
                .unwrap_or("")
                .to_ascii_lowercase();
            if scheme != "http" && scheme != "https" {
                assert!(policy.check_scheme(&scheme).is_err());
            } else {
                assert!(policy.check_scheme(&scheme).is_ok());
            }
        }
        // Helper extraction never panics and is consistent: a URL without
        // a scheme delimiter or host yields None.
        let s = junk.split_once("://").map(|(s, _)| s).unwrap_or("");
        let o = url_origin(&junk);
        if s.is_empty() {
            assert!(o.is_none());
        }
    }
}

#[test]
fn fuzzed_header_names_never_leak_credentials_cross_origin() {
    let mut rng = Rng(0x13198a2e03707344);
    for _ in 0..ITERATIONS {
        let len = (rng.next() % 48) as usize;
        let name = rng.text(len);
        // Cross-origin: a header survives ONLY if it is not a credential
        // header (per the closed set). This must hold for arbitrary names.
        let survived =
            headers_surviving_redirect("https://a.test/", "https://b.test/", [name.as_str()]);
        let is_cred = is_credential_header(&name);
        assert_eq!(
            survived.is_empty(),
            is_cred,
            "name {name:?}: survival must equal non-credential"
        );
        // Same-origin: nothing is ever stripped.
        let same =
            headers_surviving_redirect("https://a.test/x", "https://a.test/y", [name.as_str()]);
        assert_eq!(same.len(), 1, "same-origin keeps {name:?}");
        // The decision is stable under origin equivalence: port-443 vs none.
        assert!(!is_cross_origin_redirect(
            "https://a.test:443/",
            "https://a.test/"
        ));
    }
}

#[test]
fn credential_set_is_exhaustively_stripped_regardless_of_fuzzed_origins() {
    let mut rng = Rng(0xa4093822299f31d0);
    let origins = [
        ("https://a.test/", "https://b.test/"),
        ("http://a.test/", "http://a.test:8080/"),
        ("https://a.test/", "http://a.test/"),
        ("garbage", "https://b.test/"),
        ("https://a.test/", "not-a-url"),
    ];
    for _ in 0..ITERATIONS {
        let (from, to) = *rng.pick(&origins);
        for name in CREDENTIAL_REDIRECT_HEADERS {
            let survived = headers_surviving_redirect(from, to, [*name, "x-ok"]);
            if is_cross_origin_redirect(from, to) {
                assert_eq!(survived, ["x-ok"], "{name} must be stripped {from} -> {to}");
            }
        }
    }
}

#[test]
fn fuzzed_hosts_never_panic_the_egress_gate_and_ok_implies_dialable() {
    let mut rng = Rng(0xbf597fc7beef0ee4);
    let policy = FetchPolicy::default();
    let mut configured = FetchPolicy::default()
        .with_deny_hosts(["evil.test", ".internal"])
        .with_allow_hosts(["good.test"]);
    // Configuration itself is bounded.
    configured =
        configured.with_deny_hosts((0..MAX_EGRESS_HOST_ENTRIES + 10).map(|i| format!("h{i}")));
    assert!(configured.host_deny().len() <= MAX_EGRESS_HOST_ENTRIES);

    for _ in 0..ITERATIONS {
        let len = (rng.next() % 32) as usize;
        let host = if rng.next() & 1 == 0 {
            (*rng.pick(HOST_SEEDS)).to_string()
        } else {
            rng.text(len)
        };
        // Name gate never panics.
        let _ = configured.check_host_config(&host);
        // Full gate never panics; on Ok, EVERY pinned address is dialable.
        let answers: Vec<Vec<IpAddr>> = vec![
            vec![],
            vec!["127.0.0.1".parse().unwrap()],
            vec!["93.184.216.34".parse().unwrap()],
            vec![
                "93.184.216.34".parse().unwrap(),
                "10.9.9.9".parse().unwrap(),
            ],
        ];
        let addrs = rng.pick(&answers).clone();
        let static_addrs: &'static [IpAddr] = Box::leak(addrs.into_boxed_slice());
        if let Ok(pinned) = resolve_and_validate(&policy, &host, move |_h: &str| {
            Ok::<Vec<IpAddr>, String>(static_addrs.to_vec())
        }) {
            for addr in &pinned {
                assert!(
                    FetchPolicy::default().check_address(*addr).is_ok(),
                    "pinned address {addr} must be dialable"
                );
            }
        }
    }
}

#[test]
fn fuzzed_redirect_sequences_stay_bounded_and_typed() {
    let mut rng = Rng(0x27d4eb2f165667c5);
    let urls = [
        "https://a.test/x",
        "https://b.test/y",
        "http://c.test/z",
        "ftp://d.test/f",
        "not-a-url",
        "https://a.test:443/q",
    ];
    for _ in 0..ITERATIONS {
        let mut lim = RedirectLimiter::new(FetchPolicy::default());
        let mut from = (*rng.pick(&urls)).to_string();
        // A fuzzed walk can never exceed the hop ceiling with Ok results.
        for _ in 0..(MAX_REDIRECT_HOPS + 5) {
            let to = (*rng.pick(&urls)).to_string();
            match lim.evaluate(&from, &to) {
                Ok(RedirectOutcome::Follow) => {
                    assert!(lim.hops() <= MAX_REDIRECT_HOPS);
                    from = to;
                }
                Ok(RedirectOutcome::Surface) => break,
                Err(FetchPolicyError::RedirectLoop { .. }) => {}
                Err(FetchPolicyError::TooManyRedirects { .. }) => {
                    assert_eq!(lim.hops(), MAX_REDIRECT_HOPS);
                    break;
                }
                Err(_) => {}
            }
        }
        assert!(lim.hops() <= MAX_REDIRECT_HOPS);
    }
}

#[test]
fn fuzzed_decompression_sequences_never_exceed_the_bounds() {
    let mut rng = Rng(0x9e3779b97f4a7c15);
    for _ in 0..ITERATIONS {
        let mut g = DecompressionGuard::new(MAX_FETCH_RESPONSE_BODY_BYTES);
        let mut accepted_out: u64 = 0;
        let mut accepted_in: u64 = 0;
        for _ in 0..16 {
            let n = (rng.next() % 4096) as usize;
            let is_input = rng.next() & 1 == 0;
            if is_input {
                g.compressed_input(n);
                accepted_in += n as u64;
            } else if g.decompressed_output(n).is_ok() {
                accepted_out += n as u64;
            }
            // Accepted output can never exceed the cap.
            assert!(accepted_out <= MAX_FETCH_RESPONSE_BODY_BYTES);
            // Ratio invariant holds for every accepted state past threshold.
            if accepted_in >= q_capabilities::DECOMPRESSION_RATIO_THRESHOLD {
                assert!(
                    accepted_out
                        <= accepted_in.saturating_mul(q_capabilities::MAX_DECOMPRESSION_RATIO),
                    "accepted ratio {accepted_out}/{accepted_in} exceeded the ceiling"
                );
            }
        }
    }
}

#[test]
fn fuzzed_helper_sizes_fail_closed_monotonically() {
    let mut rng = Rng(0x6a09e667f3bcc908);
    let helpers = [
        BodyHelper::ResponseText,
        BodyHelper::ResponseJson,
        BodyHelper::ResponseArrayBuffer,
        BodyHelper::ResponseBytes,
    ];
    for _ in 0..ITERATIONS {
        let h = *rng.pick(&helpers);
        let n = ((rng.next() as usize) % (u32::MAX as usize)) & 0x1FF_FFFF;
        match check_body_helper_size(h, n) {
            Ok(()) => assert!(n <= h.max_bytes()),
            Err(FetchPolicyError::BodyTooLarge { .. }) => assert!(n > h.max_bytes()),
            Err(other) => panic!("unexpected error for size check: {other}"),
        }
    }
}
