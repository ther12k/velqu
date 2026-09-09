---
task_id: M28-002-A
parent_task: M28-002
milestone: M28
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-002-A — Compare reqwest and lower-level Hyper/Rustls approach

## Atomic goal

Compare reqwest and lower-level Hyper/Rustls approach.

## Parent intent

Choose a maintainable implementation based on cold start, size, correctness, streaming, and pooling.

## Dependencies

- `M28-001-Z` — `tasks/05_m28_native_fetch/M28-001-Z-package-evidence-for-accept-fetch-tls-redirect-and-ssrf-security-adr.md`

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
5. Implement exactly this deliverable: Compare reqwest and lower-level Hyper/Rustls approach.
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
m28-002-a: compare reqwest and lower level hyper rustls approach
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-002-A) — PASS

- Date: 2026-08-28
- Branch/PR: m28-002-a (squash-merged; see git log for final hash)
- Closes: #312

### Changed files
- `benchmarks/stack-spike/spike-reqwest/` (new): standalone spike crate (own workspace — production graph untouched) exercising reqwest 0.12 (`default-features = false`, `rustls-tls-webpki-roots` only) through lazy client construction, loopback plain-HTTP GET, bounded streaming prefix read, and early-drop cancellation.
- `benchmarks/stack-spike/spike-hyper/` (new): equivalent spike for hyper 1 (`http1`,`client`) + hyper-util 0.1 (`client`,`client-legacy`,`http1`,`tokio`) + hyper-rustls 0.27 (ring, webpki, TLS 1.2) — identical exercise, identical policy-shaped features.
- `benchmarks/raw/stack-spike/stack-comparison.json` (new): raw measurements — 40 fresh-process spawn samples (20 per candidate, all functional checks 20/20 pass), binary sizes, unique-crate counts, percentiles, production baseline.
- `docs/reports/m28-002-a-stack-comparison.md` (new): spike report + qualitative matrix + decision record + fallback strategy.

### Decision record (evidence-backed)
**Select: hyper 1 + hyper-util (client-legacy) + hyper-rustls (webpki-roots).**
Measured: −997,656 B binary (−21.4%), 43 vs 84 unique crates (−48.8%), spawn p95 3.318 ms vs 4.843 ms, p99 3.320 ms vs 6.242 ms; production already ships hyper+hyper-util for ingress, so outbound shares those crates. Qualitative: ADR-0033 policy applies directly with zero duplicated policy surfaces (reqwest conveniences are all "must disable" bypass traps), hyper-util exposes the pool knobs M28-003 needs (idle/active bounds, keepalive), both stacks drop-cancel and stream. No single benchmark decided — full matrix in the report. **Fallback**: reqwest with the same policy-shaped features, contained behind the policy object if the legacy client cannot be bounded as required.

### Command results
- `cargo test -p q-engine-quickjs` 16+97 · `-p q-http` 4+6+1 · `-p q-capabilities` 132+8 · `-p velqu-runtime` 31 — all pass
- `bun test` → 215 pass / 0 fail; `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0 (spikes live in standalone workspaces; production graph unchanged)
- `./scripts/verify` → **ALL PASS (exit 0)**

### Guardrail mapping
- **Decision is evidence-backed** — raw JSON + report above.
- **No framework benchmark alone determines choice** — qualitative matrix (policy fit, pooling control, cancellation, maintainability, dependency risk) carries the decision with measurements as one input.
- **Selected stack supports cancellation/backpressure** — proven functionally in both spikes (early drop mid-body); selected stack confirmed.
- **Fallback strategy documented** — reqwest behind the policy object, in the report.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
