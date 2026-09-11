//! #1318 / M6-002 — Ingress decoding fuzz target (q-http).
//!
//! Query strings and percent escapes cross the network boundary as
//! arbitrary bytes. Decoders must be total: any input maps to bounded,
//! structured output — never a panic, never unbounded allocation (the
//! "amplification" class of decoder bugs).

#![no_main]

use libfuzzer_sys::fuzz_target;
use q_http::{parse_query_with_policy, percent_decode_with_policy, RepeatedKeyPolicy};

fuzz_target!(|data: &[u8]| {
    let lossy = String::from_utf8_lossy(data);
    for policy in [RepeatedKeyPolicy::LastValueWins] {
        let pairs = parse_query_with_policy(&lossy, policy);
        // No-amplification bound: each emitted pair consumes at least
        // one input byte (keys/values are substrings of the input, so
        // total output characters cannot exceed 2x input here).
        let consumed: usize = pairs.iter().map(|(k, v)| k.len() + v.len()).sum();
        assert!(
            consumed <= lossy.len() * 2 + 8,
            "query decode amplified {} bytes to {}",
            lossy.len(),
            consumed
        );
    }
    let decoded = percent_decode_with_policy(&lossy, q_http::QUERY_INVALID_BYTE_POLICY);
    // Percent decoding can only shrink or replace bytes 1:1; the result
    // is at most the input length in chars-per-byte terms.
    assert!(
        decoded.len() <= lossy.len() * 3,
        "percent decode amplified {} bytes to {}",
        lossy.len(),
        decoded.len()
    );
});
