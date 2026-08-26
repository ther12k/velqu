# M27-003 Context Profiles — Compatibility Report and Context Benchmark

Packet verification record for M27-003-A…D (V evidence).

## What exists now

- `ContextProfile::{Full, Web, Minimal}` (closed vocabulary,
  construction-time only) — `crates/q-engine-quickjs/src/lib.rs`,
  applied by `create_context` in worker.rs.
- `velqu-runtime --context-profile <name>` override; default Full;
  unknown names fail closed before ready.
- Compiler-emitted `intrinsicRequirement` + `reductionImpact`
  diagnostics in `capability-manifest.json` / `build-report.json`.
- Ready-line identity carries `"contextProfile"` — every startup
  self-describes the intrinsic set that produced its measurements
  (`crates/q-runtime/src/lib.rs`, pinned in
  `full_profile_retained_for_compatibility_testing`).

## Context benchmark (startup p50 / RSS p50, proof app)

Same host, same release binary, same pack; 5 fresh-process samples
per profile (`timeout`-killed after ready; nearest-rank p50):

| profile | ready?            | startup p50 ms | RSS p50 kB |
| ------- | ----------------- | -------------- | ---------- |
| full    | yes               | 3.929          | 7,144      |
| web     | yes               | 3.879          | 7,012      |
| minimal | **no** — rejected | n/a            | n/a        |

Raw startup ms — full: 2.44, 3.93, 6.23, 6.45, 3.25;
web: 3.88, 5.44, 3.61, 6.48, 3.48.

## Compatibility findings

1. **Web reduction is survivable today**: the proof bundle's only
   reductions-class usage is Map (bundled helper), which web keeps.
   Startup/RSS deltas vs full are inside noise at n=5.
2. **Minimal does not apply to this app**: the proof bundle
   references RegExp from top-level code, so bundle load fails
   before ready (`startup.rejected … RegExp are not supported`) —
   loud, never silent, exactly the fail-closed direction chosen in
   A/B/D. Lazy references behave differently (server starts; the
   touching route degrades per-request as redacted internal
   problems), pinned in `full_profile_retained_for_compatibility_
   testing`.
3. **Guardrail verdict — deferred**: no measurable startup/RSS
   benefit at this sample size, and no application currently ships
   a reduced profile. Per the parent guardrail ("measurable benefit
   OR feature is deferred"), context-profile SELECTION remains
   deferred to M27-011's matched measurement work; production
   serving stays on Full.

## Conformance

Full local gate suite passes unchanged on default Full:
q-pack 98, q-engine-quickjs 102 (+5 profile pins), velqu-runtime
conformance 31 (+1 compat test), bun 152/0. No separate Test262 run
was performed for reduced profiles; reduced-profile behavior is
pinned by targeted probes (absent globals, regex eval failure)
rather than claimed against a corpus. A subset suite remains future
work if selection ever moves forward.
