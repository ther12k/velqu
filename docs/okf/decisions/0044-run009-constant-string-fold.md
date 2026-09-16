---
type: Decision Record
id: ADR-0044
title: Extend native liveness (RUN-009) to provably constant string responses; schema-driven fold; engine-bound benchmark class
status: accepted
date: 2026-09-16
deciders: project owner (benchmark-review session)
---

# ADR-0044 — Native Liveness for Constant Strings; Schema-Driven Fold; E1 Class

## Context

The C1-A stage breakdown (#1375) proved the canonical C1↔C2 throughput gap
is not a response-encoding difference: canonical `/js-json` never executes
QuickJS (`handler_calls: 0` — build-time constant response via RUN-009 native
liveness), while `/js-text` pays the full engine round-trip (~9.8 µs measured).
RUN-009 excluded string literals because their content type was "unknown" —
but the route's declared response schema (`response: { 200: s.string() }`)
states the wire kind, so the exclusion is not forced by missing information.

## Decision

1. **Extend RUN-009 to statically provable string responses.** A no-ctx
   handler returning a string literal (or no-substitution template) folds to
   a native constant response with `text/plain; charset=utf-8` **iff** the
   route declares a status-200 response whose schema is a plain
   constraint-free string. Fold is schema-driven and fail-closed:
   constrained strings, kind mismatches, parameterized handlers, and
   non-literal expressions keep the engine path.
2. **Make the object fold schema-driven too (correctness tightening).** The
   pre-existing object fold bypassed the declared contract: a constant that
   contradicted the declared schema (e.g. `() => ({ok:true})` under
   `s.string()`) was served 200 natively where the engine would have failed
   the contract. The fold now requires the declared 200 kind to match the
   folded expression kind; mismatches keep the engine path and fail their
   contract like any dynamic value.
3. **Reclassify the benchmark taxonomy.** After the fold, C1 and C2 are
   "AOT-foldable constant response" benchmarks (text and JSON) — they are
   NOT JS-engine throughput evidence (C2 already was not). A new
   supplemental class **E1 / JS-TEXT-DYNAMIC** provides the explicit
   JS-boundary plaintext benchmark: identical wire bytes to C1, a
   deliberately non-literal handler, and a BEHAVIORAL engine-bound
   precondition — the probe harness verifies handler invocations > 0 against
   the instrumented runtime and fails if a future optimizer folds E1 too.
4. **Wire semantics are frozen; execution strategy is implementation.** The
   frozen fixture contracts (exact bytes, statuses, content types) are
   unchanged; how Velqu produces the bytes is not part of the contract.

## Consequences

- Constant text responses (`() => "plain"` under `s.string()`) gain the same
  AOT path as constant JSON — measured handler_calls drop to 0.
- Constrained constant strings stay on the engine path, where the C1-B
  direct text plan validates them cheaply (borrowed, no allocation).
- The build report's nativeStages now includes folded text routes; the
  per-route `nativeStage` field already discloses `native-liveness` vs
  `engine`.
- Benchmarks and reports must stop citing C1/C2 as QuickJS throughput; E1
  (and, if needed, an E2 JSON variant) is the engine-bound evidence.
- A pre-existing soundness gap closes: constants contradicting their
  declared schema no longer bypass contract validation via the fold.

## Alternatives considered

- Keep strings engine-bound so C1 remains a JS benchmark — rejected: that
  preserves a benchmark at the cost of a real product optimization for
  exactly the response kind the contract already describes.
- Fold arbitrary statically-evaluable expressions (array indexing,
  constants) — rejected for now: expand the trusted-literal subset only as
  needed; E1's behavioral precondition guards the boundary.
- Build-time constraint validation for constrained constant strings —
  deferred; they keep the engine path until then.
