//! #1318 / M6-002 — capability policy fuzz target (q-capabilities).
//!
//! Covers the fetch-parser and capability-metadata acceptance surface:
//! SSRF gate (`resolve_and_validate` — hostname denial, allow/deny
//! config, address-class trust mode), redirect limiter state machine,
//! scheme/address/host checks, capability identity parsing, inventory
//! canonicalization, and dependency-DAG resolution. Every adversarial
//! host/URL/id/DAG must produce a typed outcome — the SSRF gate must
//! NEVER grant a metadata/undialable target, whatever the bytes say.

#![no_main]

use libfuzzer_sys::fuzz_target;
use q_capabilities::fetch_policy::{
    is_metadata_hostname, resolve_and_validate, FetchPolicy, RedirectLimiter, RedirectPolicy,
    TrustMode,
};
use q_capabilities::identity::{CapabilityDescriptor, CapabilityId, CapabilityRequirement};
use q_capabilities::inventory::CapabilityInventory;
use q_capabilities::resolver::resolve_closure;
use std::net::IpAddr;

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }
    let lossy = String::from_utf8_lossy(data);
    let pieces: Vec<&str> = lossy.split('\n').collect();
    let host_field = pieces.first().copied().unwrap_or("");
    let url_a = pieces.get(1).copied().unwrap_or("https://example.test/");
    let url_b = pieces.get(2).copied().unwrap_or("https://example.test/next");

    // ---- 1. SSRF gate: resolve_and_validate with a fixed resolver ----
    // (Harness correction, 2026-09-12: the first draft asserted that ANY
    // Ok from this call with a metadata-returning fake resolver is a
    // breach. Wrong: IP-LITERAL hosts skip the injected resolver by
    // contract — the gate validates the literal itself. The first smoke
    // run "found" host "7::" this way; a harness defect, not a product
    // one. The invariants below are the ones the contract actually
    // makes. The 0000::/8 reserved-range classification nuance surfaced
    // by that run is recorded in fuzz/COVERAGE.md as an owner-decision
    // observation, not silently dropped.)
    let policy = FetchPolicy::default(); // deny-by-default trust mode
    let is_ip_literal = host_field.parse::<IpAddr>().is_ok();

    // Invariant 1a (name-resolved hosts): when the host is NOT an IP
    // literal, a resolver that returns ONLY the metadata address must
    // always yield a typed denial.
    if !is_ip_literal {
        let outcome = resolve_and_validate(&policy, host_field, |_| {
            Ok(vec![IpAddr::from([169, 254, 169, 254])])
        });
        assert!(
            outcome.is_err(),
            "SSRF gate granted a metadata-resolved host: {host_field:?}"
        );
    }

    // Invariant 1b (any host): on Ok, the returned pin set is non-empty
    // and EVERY address passes the policy's own address check — the
    // gate may never return an address it would refuse to dial.
    if let Ok(addrs) = resolve_and_validate(&policy, host_field, |h| {
        // Deterministic fake resolver: name-shape hosts resolve to a
        // public address, EXCEPT hosts ending in ".meta" which resolve
        // to the metadata endpoint (exercising the denial path).
        if h.ends_with(".meta") {
            Ok(vec![IpAddr::from([169, 254, 169, 254])])
        } else {
            Ok(vec![IpAddr::from([93, 184, 216, 34])])
        }
    }) {
        assert!(!addrs.is_empty(), "Ok with an empty pin set");
        for addr in &addrs {
            policy
                .check_address(*addr)
                .unwrap_or_else(|e| panic!("gate returned undialable {addr}: {e:?} (host {host_field:?})"));
        }
        assert!(
            !is_metadata_hostname(host_field),
            "Ok returned for a metadata hostname: {host_field:?}"
        );
    }

    // Metadata-by-name must always deny, byte-for-byte independent of
    // case/dot tricks the fuzzer tries.
    if is_metadata_hostname(host_field) {
        let outcome2 =
            resolve_and_validate(&policy, host_field, |_| Ok(vec![IpAddr::from([1, 1, 1, 1])]));
        assert!(
            outcome2.is_err(),
            "metadata hostname {host_field:?} slipped the name denial"
        );
    }

    // ---- 2. Redirect limiter: state machine totality + hop ceiling ----
    let mut limiter = RedirectLimiter::new(
        FetchPolicy::default().with_redirect_policy(RedirectPolicy::Follow { max_hops: 8 }),
    );
    let mut hops = 0u32;
    // Deterministic chain derived from the input: a->b repeatedly must
    // terminate in a typed outcome, never loop forever and never panic.
    for i in 0..64 {
        let (from, to) = if i % 2 == 0 {
            (url_a, url_b)
        } else {
            (url_b, url_a)
        };
        match limiter.evaluate(from, to) {
            Ok(q_capabilities::fetch_policy::RedirectOutcome::Follow) => hops += 1,
            Ok(q_capabilities::fetch_policy::RedirectOutcome::Surface) => break,
            Err(_) => break,
        }
    }
    assert!(
        hops <= 32,
        "redirect limiter followed {hops} hops — ceiling not enforced"
    );

    // ---- 3. Capability identity + inventory canonicalization ----
    if let Ok(id) = CapabilityId::parse(pieces.first().copied().unwrap_or("runtime:timers")) {
        // Round-trip: reparsing the canonical string yields the same id.
        let reparsed = CapabilityId::parse(id.as_str());
        assert_eq!(
            reparsed.as_ref().map(|r| r.as_str()),
            Ok(id.as_str()),
            "capability id round-trip unstable"
        );
    }
    let pairs: Vec<(String, u32)> = pieces
        .iter()
        .enumerate()
        .filter(|(_, s)| !s.is_empty())
        .map(|(i, s)| (format!("runtime:cap{i}"), (s.len() % 5) as u32 + 1))
        .collect();
    if let Ok(inv) = CapabilityInventory::from_pairs(&pairs) {
        // Canonical bytes are deterministic across rebuilds.
        let a = inv.sha256_hex();
        let again = CapabilityInventory::from_pairs(&pairs).expect("same pairs rebuild");
        assert_eq!(a, again.sha256_hex(), "inventory canonicalization nondeterministic");
    }

    // ---- 4. Dependency DAG: cycles and missing deps are typed ----
    let mk = |name: &str| CapabilityDescriptor {
        requirement: CapabilityRequirement {
            id: CapabilityId::parse(name).expect("fixed valid id"),
            version: q_capabilities::identity::CapabilityVersion(1),
        },
        dependencies: vec![],
    };
    let universe = vec![mk("runtime:timers"), mk("runtime:crypto"), mk("runtime:console")];
    let roots = pieces
        .iter()
        .filter_map(|s| CapabilityId::parse(s).ok())
        .map(|id| CapabilityRequirement {
            id,
            version: q_capabilities::identity::CapabilityVersion(1),
        })
        .take(8)
        .collect::<Vec<_>>();
    let _ = resolve_closure(&roots, &universe); // Ok or typed ResolveError — never panic
});
