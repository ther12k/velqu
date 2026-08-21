---
type: Final Report
title: Velqu (Project Q) M0–M2 Final Report
status: final
generated: 2026-08-18
format: master-agent-prompt §18
---

# Final Report — Velqu (Project Q) M0–M2

## Status

**COMPLETE** — all three authorized milestones implemented, measured, tested,
and verified. One performance BUDGET failed and is recorded honestly (below);
it is not a milestone gate.

## Authorized scope and exact stop point

Scope: OKF ingestion (Stage 0/1), M0 contracts + fair baselines, M1 Rust host
with one quickjs-ng worker, M2 static compiler + Treaty + proof app +
conformance + evidence. **Stop point: M2.** No M3, public alpha, database,
multi-worker, WebSocket/SSE, full fetch/crypto, or public-release work was
started (master prompt §16).

## Architecture verdict: pass / conditional pass / fail

**PASS** — the product thesis is supported by matched, reproducible evidence
from the M0–M2 gate runs on this host (historical comparator evidence, not the
current beta gate run): a statically compiled TypeScript contract on
Rust + QuickJS delivered C3/C4 cold-start p95 of 4.4/5.0 ms versus 152.0/149.9
ms for the matched Elysia 2 AOT fixture (1,680 fresh-process samples; table
below), and 6.2 MiB idle RSS p50 versus 83 MiB, while preserving
Treaty-quality end-to-end types and route governance. Public wording for these
numbers follows `docs/beta/governance/BENCHMARK_WORDING.md`; the current beta
gate cold evidence is Velqu-only
(`benchmarks/raw/cold-start/summary.json`).

## Requirements completed

44/44 traced P0 requirements PASS with code/test/evidence links in
`docs/m0-m2-traceability.md` (machine-readable:
`docs/reports/traceability.json`). Highlights: COMP-001..009 (compiler),
RUN-001..008 (runtime), SCHEMA-001..005, TRT-001..005, PERF-001..006,
SEC-001..005, OPS-001..002, DX-001..004, PR-001..005.

## Decisions and corrections made

- **ADR-0015** (measurement-driven): native Rust JSON adopted as default
  input AND response strategy — inputs 11–42% faster than engine JSON.parse
  on this host (reverses the design review's expectation; scope-limited to
  quickjs-ng 0.15.1 / rquickjs 0.12.2).
- **ADR-0016** (owner decision): product naming decided — Brand **Velqu**,
  descriptive **VelquJS**, CLI `velqu`, packages `@velqu/*`, runtime binary
  `velqu-runtime`.
- **ADR-0017** (measurement-driven): QuickJS module bytecode embedding via
  `velqu-bytecode` tool — saves 1.74 ms (−10.7%) on 1,000-route cold start.
- **ID-011**: Treaty route-id navigation (`api.hello.get({name}).get()`)
  instead of Eden-exact single-segment form (collision hazard on shared
  prefixes); owner may revisit ergonomics.
- serde_json `preserve_order` enabled so native serialization preserves
  business-object key order (byte-exact fixture bodies).

## Repository/files/modules added

- **Rust workspace** (`crates/`): q-pack, q-router, q-bridge, q-schema-runtime,
  q-engine (trait), q-engine-quickjs (single worker thread, prelude, timer
  capability, source mapping), q-http (hyper 1.11 + TokioTimer, limits,
  graceful shutdown), velqu-runtime (binary, 4.6 MiB stripped), q-bench-support
  (bridge microbench).
- **TypeScript** (`packages/`): @velqu/schema, @velqu/core, @velqu/contract, @velqu/treaty,
  @velqu/compiler (AST extraction + Bun.build + emitters), @velqu/cli (`q build /
  inspect / contract diff`), @velqu/testing (unitTreaty + runtimeTreaty).
- **Proof app** (`examples/proof/`): 9 frozen-contract routes; compiler emits
  9 deterministic artifacts incl. app.qpack, contract.d.ts, openapi.json,
  contract.lock.json, build-report.
- **Baselines** (`baselines/`): raw-rust (hyper), raw-bun, elysia2
  (2.0.0-beta.4 AOT) — all 27/27 on the canonical checker.
- **Conformance** (`conformance/`): compiler (6), treaty (3), routing,
  schema (6), bridge (2), lifecycle (1), security (2) — 21 total.
- **Evidence** (`benchmarks/raw/**`, `benchmarks/type-scale/`,
  `docs/reports/` 15 reports).

## Proof behavior demonstrated

health/live native (zero JS entry), JS text/JSON, validated path params
(422 + field), validated JSON body (201/422), policy session (401/200/404),
async native timer through a promise, cancellation safety, redacted 500 with
source-mapped stderr diagnostics. All via the ACTUAL release binary.

## Verification commands and exact results

```bash
./scripts/verify
```
→ OKF validation PASS (161 links) · cargo fmt/clippy(-D warnings) PASS ·
**cargo test 57/57** · release builds PASS · tsc PASS · proof build PASS
(9 routes, 579 ms) · **bun test 30/30** → `verify: ALL PASS (M0–M2 verified)`.

## Cold-start evidence

Historical M0–M2 comparator evidence (not the current beta gate run): 1,680
fresh-process samples, 0 failures. p50/p95 (ms):

| Class | velqu | raw-rust | raw-bun | elysia2 AOT |
|---|---|---|---|---|
| C3 validated | **2.9 / 4.4** | 2.1 / 2.7 | 14.2 / 18.3 | 132.6 / 152.0 |
| C4 policy | **3.3 / 5.0** | 2.2 / 3.8 | 14.3 / 21.2 | 133.3 / 149.9 |

Gate C3/C4 p95 ≤ 60% of Elysia: **PASS** (2.9% / 3.3% of Elysia's p95).

## Bridge strategy evidence

2000 samples/case, all correctness-asserted: input small −34%, nested −11%,
array100 −42% for native (B) vs engine (A); output array100 −23%; small
object ≈equal. → native default (ADR-0015).

## Warm performance and memory evidence

Warm (1000 req, c=10): velqu C0 69k req/s, C2 116k req/s (p50 85 μs), C3
35.7k req/s; 0 errors in 16 cells. Idle RSS p50: velqu **6.2 MiB** (budget
≤12 PASS; +2.8 MiB over raw-rust, budget ≤8 PASS) vs raw-bun 36 MiB, elysia
83 MiB. 1000-route RSS 11.1 MiB.

## TypeScript/Treaty evidence

tsc cold: 100/500/1000 routes = 1.9/1.7/2.0 s typical (500/1000 budgets
PASS; 100-route budget misses on the fixed tsc floor — recorded). Negative
types CAUGHT at all scales. Treaty: status narrowing, non-throwing errors,
network/abort distinction, published-contract mode driving the real binary
(runtime-local 7-call scenario), client isolation (zero server imports).

## Compiler/schema/contract evidence

Static extraction with trap tests (service factories never run; module side
effects never execute), source-located diagnostics for
collision/duplicate/dynamic/node:-imports, byte-identical rebuild hashes,
9 deterministic artifacts, OpenAPI 3.1 + semantic diff (breaking/compatible/
policy-sensitive), native liveness const-folding, capability detection.

## Security and FFI findings

One reviewed `unsafe` block (documented invariants); opaque generation
handles (expiry/wrong-owner rejected; slots freed at settle); pack SHA-256 +
exact engine/ABI match before ready; secrets redacted from responses and
stdout logs; source-mapped causes kept to stderr; same-process engine
documented as trusted-code-only (SEC-002).

## Failed, partial, unexecuted, or waived checks

1. **FAILED BUDGET (honest)**: 1000-route vs 25-route cold-start delta
   +409% (budget ≤20%); absolute 15.7 ms p50 still ~10× faster than matched
   Elysia. Remediation ideas recorded (binary pack, bytecode), not applied.
2. **FAILED BUDGET (honest)**: 100-route tsc budget 1.5 s vs ~1.9–4.8 s
   observed; cause is the fixed tsc process floor, not route scaling.
3. **UNEXECUTED**: QuickJS bytecode load spike (optional in M1); 10k-request
   retained-state soak; minified client byte-size measurement (source 5.5 KB
   recorded); warm high-concurrency saturation sweep (c=10 only).
4. **WAIVED**: none (no owner waivers exist).

## Known limitations

Single QuickJS worker (ADR-0008); route-id Treaty navigation (ID-011);
cookies/form/streams excluded (P1); dev server not built (P1, DX-005);
hyper header_read_timeout requires explicit TokioTimer (documented).

## Open owner decisions

OD-001..006 in `docs/open-decisions.md`: public name (Velqu is the working
name), package scope (@velqu/* placeholder), repository, license, platform
promise, release/governance. None decided by the implementation agent.

## Salvage/recommendation if a gate failed

Not applicable — all decision gates passed. For the two failed aspirational
budgets, M3 recommendations: binary/chunked pack + bytecode for route-count
scaling; editor-embedded type metrics instead of cold tsc for the 100-route
figure.

## Commit and clean-tree status

- HEAD: `08ec4d2` + this report's commit (see `git log -1`).
- Working tree: clean at handoff (`git status` empty).

## Archive path and SHA-256

`velqu-m0-m2-<stamp>.zip` + `.sha256` produced by `./scripts/package`
(re-run after the final commit for the definitive archive; the committed
hash pair is in the `.sha256` file beside the archive).
