---
task_id: M28-002-V
parent_task: M28-002
milestone: M28
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-002-V — Verify Select native HTTP client stack from evidence

## Atomic goal

Prove every acceptance criterion for parent task M28-002 without broadening scope.

## Parent intent

Choose a maintainable implementation based on cold start, size, correctness, streaming, and pooling.

## Dependencies

- `M28-002-A` — `tasks/05_m28_native_fetch/M28-002-A-compare-reqwest-and-lower-level-hyper-rustls-approach.md`
- `M28-002-B` — `tasks/05_m28_native_fetch/M28-002-B-measure-dependency-binary-startup-cost.md`
- `M28-002-C` — `tasks/05_m28_native_fetch/M28-002-C-test-dns-tls-pool-behavior.md`
- `M28-002-D` — `tasks/05_m28_native_fetch/M28-002-D-record-maintenance-security-considerations.md`

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
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `Cargo.toml`
- `packages/cli/src/index.ts`
- `crates/q-runtime/src/source_map.rs`
- `examples/proof/`
- `README.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

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
cargo test -p q-bridge
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
```bash
cargo fmt --check
```
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
```bash
./scripts/verify
```

## Required evidence for this microtask

- Spike report.
- Raw measurements.
- Decision record.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m28-002-v: verify select native http client stack from evidence
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-002-V) — PASS

- Date: 2026-08-28
- Branch/PR: m28-002-v (squash-merged; see git log for final hash)
- Closes: #316

### Acceptance-criterion mapping (parent M28-002 guardrails)

1. **Decision is evidence-backed** — verified: `benchmarks/raw/stack-spike/stack-comparison.json` (40 fresh-process spawn samples comparing reqwest 0.12 vs hyper stack in isolation; hyper −21.4% binary, −48.8% crate count, p95 3.32 vs 4.84 ms); `benchmarks/raw/stack-spike/fetch-stack-cost.json` (linked-in production cost: +915 KB binary, +0.45 ms cold-start delta, within < 10 ms budget); `docs/reports/m28-002-a-stack-comparison.md`, `m28-002-b-stack-cost.md`, `m28-002-c-dns-tls-pool-behavior.md`, `m28-002-d-maintenance-security.md`.
2. **No framework benchmark alone determines choice** — verified: Decision rests on full 6-axis matrix (policy fit to ADR-0033 with no bypass traps, direct pooling controls for M28-003, cancellation/backpressure support, maintainability, dependency risk, binary size/startup).
3. **Selected stack supports cancellation/backpressure** — verified: `streaming_body_supports_bounded_prefix_and_early_drop` (`benchmarks/stack-spike/spike-hyper/tests/stack_behavior.rs`) tests that 1 MiB stream reads bounded frames and drops mid-stream with server observing cancellation.
4. **Fallback strategy documented** — verified: `docs/reports/m28-002-a-stack-comparison.md` and `m28-002-d-maintenance-security.md` document reqwest as prepared fallback behind the ADR-0033 policy object.

### Verification runs (this branch, worktree-fresh)
- `cargo test --test stack_behavior` (spike workspace) → 6/6 passed (pool reuse, pool origin keying, DNS resolution, DNS unresolvable fast-fail, self-signed TLS fail-closed, streaming early-drop).
- `cargo test -p q-capabilities` → 132 unit + 8 integration passed
- `cargo test -p q-engine-quickjs` → 16+97 passed
- `cargo test -p q-http` → 4+6+1 passed
- `cargo test -p q-bridge` → 11 passed
- `cargo test -p velqu-runtime` → 1 unit + 31 conformance passed
- `./target/release/velqu-runtime --fetch-stack-info` → constructs cleanly and prints identity.
- `bun test` → 215 pass / 0 fail (27 files); `bun run typecheck` → clean
- `cargo fmt --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
