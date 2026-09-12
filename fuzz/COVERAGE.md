# M6-002 Coverage Map — issue acceptance → campaign target

The #1318 acceptance names more boundaries than crates with Rust APIs.
This map accounts for every named surface; nothing was silently
dropped. If a surface has no target here, the issue cannot close.

## Acceptance surface → evidence

| #1318 acceptance surface | Evidence | Kind |
|---|---|---|
| HTTP admission (query/percent decoders, header bounds) | `fuzz_targets/http_decode` | libFuzzer (sustained) |
| Router (method/path resolution) | `fuzz_targets/router_match` | libFuzzer (sustained) |
| QPack (verifier, mixed-mode rejection, bytecode policies) | `fuzz_targets/pack_verify` | libFuzzer (sustained) |
| Schema codecs (validation, deterministic classification, backtracking) | `fuzz_targets/schema_validate` | libFuzzer (sustained) |
| Bridge handles (generation checks, stale/foreign access) | `fuzz_targets/bridge_handles` | libFuzzer (sustained) |
| **Treaty encoders** (path interpolation, query serialization, response mapping) | `scripts/ts-fuzz-campaign.ts` → `benchmarks/raw/ga-m6-fuzz/ts-treaty-ledger.json` | seeded property-fuzz over the published `treaty()` API (Treaty is TypeScript — outside cargo-fuzz's reach); wired into `scripts/fuzz-campaign.sh` and fail-closed into its verdict |
| Response/problem encoders (M25-005/006 — the bytes Treaty decodes) | `fuzz_targets/codec_encoders` | libFuzzer (sustained) |
| **Fetch parsers** (SSRF gate, URL/host/address checks, redirect limiter, metadata denial) | `fuzz_targets/capabilities_policy` | libFuzzer (sustained) |
| **Capability metadata** (identity parse, inventory canonicalization, dependency DAG) | `fuzz_targets/capabilities_policy` | libFuzzer (sustained) |
| Sanitizers | `scripts/fuzz-campaign.sh` S1 (ASan workspace) + S2 (UBSan over the quickjs-ng C FFI via clang — the boundary Miri cannot execute) | recorded per-stage in the campaign ledger |
| Concurrency/property suites | existing deterministic suites (`cargo test` per crate; beta-009-a) + Miri over the FFI-free crates (`scripts/miri-campaign.sh`) | Miri + tests |
| **Explicit unsafe/FFI audit** | `scripts/unsafe-audit.py` → `docs/production/evidence/m6-unsafe-audit.md` (fail-closed: new unclassified unsafe blocks break the gate) | generated audit artifact |

## Findings and observations ledger (2026-09-12 smoke runs)

1. **Harness defect (fixed)** — `capabilities_policy` v1 asserted that
   any `Ok` from `resolve_and_validate` with a metadata-returning fake
   resolver is a breach. Wrong: IP-literal hosts skip the injected
   resolver **by contract** (the literal itself is validated). First
   smoke run surfaced host `"7::"` through this false invariant.
   Harness corrected to the contract's actual invariants: (a)
   name-resolved hosts must deny a metadata-only resolver; (b) any `Ok`
   returns a non-empty pin set whose every address passes
   `FetchPolicy::check_address`; (c) `Ok` never for a metadata hostname.

2. **Product finding (fixed + regression test)** — the same smoke run
   exposed a real panic in `FetchPolicy::is_metadata_hostname`
   (q-capabilities): `String::truncate(253)` panics when byte 253
   splits a multi-byte char, so a hostile multi-byte `Host`/URL string
   could panic the SSRF gate (DoS class, fail-open adjacent). Fixed
   with an ASCII fast-path (all metadata endpoint names are ASCII —
   non-ASCII can never match). Regression test:
   `fetch_policy::tests::metadata_hostname_never_panics_on_multibyte_hosts`.

3. **Owner decision (2026-09-12): HARDEN BEFORE GA — RESOLVED.** The
   observation that IP literal `7::` (0000::/8) classified as
   `AddressClass::Public`/dialable was escalated to the owner and
   decided: default fetch trust permits only **globally reachable**
   public destinations; IETF-reserved and special-purpose IPv6
   destinations that are not globally reachable (0000::/8,
   2001:db8::/32 documentation, 5f00::/16 SRv6 SID — forwardable but
   not globally reachable, deliberately not called "reserved") deny by
   default. Implemented as the semantic rule in ADR-0033 §2's amendment
   with the IANA registry snapshot documented at
   `is_globally_reachable_v6`; regression fixtures pin both directions
   (`7::`, `2001:db8::1`, `5f00::1` deny; `2001:4860:4860::8888`,
   `2620:fe::fe`, `2001:1::1` stay Public). std's nightly-only
   `is_global()` is not used — Velqu owns the classifier.
