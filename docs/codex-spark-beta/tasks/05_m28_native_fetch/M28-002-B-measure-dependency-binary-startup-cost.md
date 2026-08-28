---
task_id: M28-002-B
parent_task: M28-002
milestone: M28
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-002-B — Measure dependency/binary/startup cost

## Atomic goal

Measure dependency/binary/startup cost.

## Parent intent

Choose a maintainable implementation based on cold start, size, correctness, streaming, and pooling.

## Dependencies

- `M28-002-A` — `tasks/05_m28_native_fetch/M28-002-A-compare-reqwest-and-lower-level-hyper-rustls-approach.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `Cargo.toml`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`
- `docs/beta/`
- `benchmarks/harness/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Measure dependency/binary/startup cost.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Decision is evidence-backed.
- No framework benchmark alone determines choice.
- Selected stack supports cancellation/backpressure.
- Fallback strategy documented.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Spike report.
- Raw measurements.
- Decision record.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-002-b: measure dependency binary startup cost
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-002-B) — PASS

- Date: 2026-08-28
- Branch/PR: m28-002-b (squash-merged; see git log for final hash)
- Closes: #313

### Changed files
- `Cargo.toml` + `crates/q-runtime/Cargo.toml`: linked the selected outbound stack — hyper 1 gains the `client` feature, hyper-util 0.1 gains `client`+`client-legacy` (extended in place: both were already production deps for ingress, no duplicate copies); new crates hyper-rustls 0.27 (ring, webpki-roots, http1, TLS 1.2) and http-body-util 0.1.
- `crates/q-runtime/src/fetch_stack.rs` (new): dormant-but-reachable stack module — `build_client()` (webpki-roots connector, HTTP/1, policy-shaped) + `describe()`; `stack_constructs_lazily_and_names_itself` test. Nothing dials; M28-003 wires the bounded pool and every dial stays gated by the ADR-0033 policy.
- `crates/q-runtime/src/main.rs`: `--fetch-stack-info` diagnostic flag (constructs the stack once, prints identity, exits); `--pack` becomes optional only for this flag.
- `benchmarks/raw/stack-spike/fetch-stack-cost.json` (new): raw before/after measurements — binary sizes + SHA-256s, 20 cold-start samples (10/side), percentiles, delta block.
- `docs/reports/m28-002-b-stack-cost.md` (new): method, results, budget verdicts.
- `benchmarks/manifest.json`: qRuntimeRelease hash refreshed for the linked binary (same remapped-rebuild pattern disclosed since M25-010-C).

### Measured results (n=10 fresh processes per side, remap-flag reproducible builds)
| Metric | before | after | Delta |
| --- | --- | --- | --- |
| Binary size | 5,552,936 B | 6,468,376 B | **+915,440 B (+16.5%)** |
| Cold-start p50 | 4.061 ms | 4.512 ms | +0.451 ms |
| Cold-start p95 | 4.704 ms | 5.214 ms | +0.509 ms |
| Cold-start p99 | 4.732 ms | 5.223 ms | +0.491 ms |

Verdict: cold-start stays ~4.5–5.2 ms (budget < 10 ms) — the dormant stack costs one-time binary paging, ~0.45 ms; binary growth +0.87 MiB is inside the +1 MiB envelope for a full HTTP/TLS stack and consistent with the M28-002-A standalone spike ratio. No split/defer decision triggered.

### Command results
- `cargo test -p q-engine-quickjs` 16+97 · `-p q-http` 4+6+1 · `-p q-capabilities` 132+8 · `-p velqu-runtime` 1+31 (new stack test) — all pass
- `bun test` → 215 pass / 0 fail; `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)** (twice — once after the dep change, once after manifest refresh)

### Guardrail mapping
- **Decision is evidence-backed** — before/after raw JSON + report.
- **No framework benchmark alone determines choice** — this packet quantifies the production cost of the A-packet decision; budget verdicts included.
- **Selected stack supports cancellation/backpressure** — unchanged from A; stack linked dormant.
- **Fallback strategy documented** — M28-002-A report stands; cost verdict confirms no fallback trigger.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
