---
type: Evidence Report
title: Benchmark Fairness Audit
status: complete
milestone: M0–M2
---

# Fairness audit (benchmark-methodology.md checklist)

| Check | Status | Notes |
|---|---|---|
| release builds for all | PASS | Rust `--release` (lto=thin, strip); Bun candidates run production JS (no dev mode); q-runtime stripped 4.6MB |
| idiomatic framework implementation | PASS | Elysia uses t.Object schemas + guards (its own validation), raw-bun zero-dep idiomatic handlers, raw-rust minimal hyper service |
| same payload and validation semantics | PASS | one frozen contract; single checker validates all candidates: 27/27 each (velqu 31/31 with exact problem checks) |
| same status/body incl. byte order | PASS | exact-bytes assertions for C0–C4 bodies; user JSON key order fixed in every candidate |
| same logging level | PASS | startup-line only everywhere; request logs off in baselines |
| same compression/TLS | PASS | none/none everywhere, HTTP/1.1 keep-alive |
| same eager/lazy deps | PASS | in-memory lazy user service in every candidate; no DB anywhere |
| no hidden pre-running server | PASS | fresh process per sample; harness kills each child |
| no static bypass as primary comparison | PASS | C0 static-liveness is reported as its own class; primary gates use C3/C4 (validated/policy routes through JS) |
| official Elysia best practices | PASS | pinned 2.0.0-beta.4 (`next`), `aot: true` default path; hook order + error mapping documented in `baselines/README.md` |
| benchmark-specific tuning disclosed | PASS | all tuning disclosed: Elysia ParseError 400→422 mapping; velqu pack strategy defaults (native inputs per bridge report) |
| randomized/interleaved run order | PASS | deterministic-shuffle interleave across candidate×class (cold-start), fixed order noted in route-count (sequential per candidate — disclosed) |
| raw data retained | PASS | `benchmarks/raw/**` JSONL + summaries committed |
| failures retained | PASS | failure counts + error details in raw lines (0 failures observed) |
| p50/p95/p99 (+mean/stdev in summaries) | PASS | summary.json |
| warm-load protocol | PARTIAL | warm benchmark suite is implemented (harness skeleton) but the full warm run is an M2 close-out item; marked UNEXECUTED where applicable |

## Candidate-specific disclosures

- **elysia2**: AOT compilation happens at `listen()` inside Elysia (their
  design); its cost is included in every cold sample — that is the product
  comparison, not a straw man. The 400→422 malformed-JSON mapping is a
  fixture-semantics alignment, disclosed.
- **raw-rust**: no validation library, no framework — transport lower bound
  only; never used to imply feature parity.
- **velqu**: C0 served natively (no JS) — this is a real product capability
  (RUN-009) and is disclosed via the `x-velqu-stage: native` header; the
  primary comparative gates (C3/C4) execute JavaScript handlers.
- Harness overhead: TCP-accept polling at 0.5ms interval adds ≤0.5ms
  quantization to ready timestamps; identical for all candidates.

## Cache treatment

Normal repeated process starts on a stable host (methodology's required
condition). No filesystem-cache clearing was performed for any candidate.
