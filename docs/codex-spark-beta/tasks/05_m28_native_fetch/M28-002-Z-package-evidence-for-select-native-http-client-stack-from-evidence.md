---
task_id: M28-002-Z
parent_task: M28-002
milestone: M28
priority: P1
mode: EVIDENCE
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-002-Z — Package evidence for Select native HTTP client stack from evidence

## Atomic goal

Create source-backed evidence and handoff for parent task M28-002; update status only if verification passed.

## Parent intent

Choose a maintainable implementation based on cold start, size, correctness, streaming, and pooling.

## Dependencies

- `M28-002-V` — `tasks/05_m28_native_fetch/M28-002-V-verify-select-native-http-client-stack-from-evidence.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

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

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m28-002-z: package evidence for select native http client stack from ev
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-002-Z) — PASS

- Date: 2026-08-28
- Branch/PR: m28-002-z (squash-merged; see git log for final hash)
- Closes: #317

### Parent closure — M28-002 Select native HTTP client stack from evidence

Parent intent: choose a maintainable implementation based on cold start, size, correctness, streaming, and pooling. Status: **PASS**.

Packet commits (squash merges):
- M28-002-A — e351c26 (#915, Closes #312): Compared reqwest 0.12 vs hyper stack in standalone spike crates; selected hyper 1 + hyper-util (client-legacy) + hyper-rustls (webpki-roots); raw data `benchmarks/raw/stack-spike/stack-comparison.json`, report `docs/reports/m28-002-a-stack-comparison.md`
- M28-002-B — c6cedcd (#916, Closes #313): Linked selected stack into production binary as dormant module `crates/q-runtime/src/fetch_stack.rs` + `--fetch-stack-info` diagnostic flag; measured real linked cost (+915 KB binary, +0.45 ms cold-start delta, <10 ms budget passes); raw data `benchmarks/raw/stack-spike/fetch-stack-cost.json`, report `docs/reports/m28-002-b-stack-cost.md`
- M28-002-C — e046d4f (#917, Closes #314): Behavioral verification — 6/6 probes passed (pool keepalive reuse, origin keying, DNS resolution, fast fail on unresolvable host, self-signed TLS fail-closed with no bypass knob, streaming prefix with early-drop cancellation); report `docs/reports/m28-002-c-dns-tls-pool-behavior.md`
- M28-002-D — 79be340 (#918, Closes #315): Maintenance and security considerations — ownership boundaries, pinned ring-only dependencies, upgrade packet rules, RUSTSEC review checklist, fail-closed webpki refresh policy, maintenance checklist (`docs/reports/m28-002-d-maintenance-security.md`)
- M28-002-V — 306350d (#919, Closes #316): Verification closure mapping all 4 acceptance guardrails

### Evidence ledger (required microtask evidence)
- **Spike report**: `docs/reports/m28-002-a-stack-comparison.md`, `m28-002-b-stack-cost.md`, `m28-002-c-dns-tls-pool-behavior.md`, `m28-002-d-maintenance-security.md`.
- **Raw measurements**: `benchmarks/raw/stack-spike/stack-comparison.json` (40 fresh-process spawn samples) + `benchmarks/raw/stack-spike/fetch-stack-cost.json` (20 before/after production cold-start samples).
- **Decision record**: `docs/reports/m28-002-a-stack-comparison.md` §Decision record (hyper stack selected; fallback strategy documented).

### Command results (this branch)
- `cargo test --test stack_behavior` (spike) → 6 passed / 0 failed
- `cargo test -p q-capabilities` → 132 unit + 8 integration passed
- `cargo test -p q-engine-quickjs` → 16 unit + 97 worker passed
- `cargo test -p q-http` → 4+6+1 passed
- `cargo test -p velqu-runtime` → 1 unit + 31 conformance passed
- `./target/release/velqu-runtime --fetch-stack-info` → constructs cleanly and prints identity
- `bun test` → 215 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M28-002 flipped TODO -> PASS.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
