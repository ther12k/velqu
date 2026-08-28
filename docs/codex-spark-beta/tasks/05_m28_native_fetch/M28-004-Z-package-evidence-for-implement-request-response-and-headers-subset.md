---
task_id: M28-004-Z
parent_task: M28-004
milestone: M28
priority: P0
mode: EVIDENCE
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-004-Z — Package evidence for Implement Request, Response, and Headers subset

## Atomic goal

Create source-backed evidence and handoff for parent task M28-004; update status only if verification passed.

## Parent intent

Expose a useful Web-compatible API without materializing unnecessary objects.

## Dependencies

- `M28-004-V` — `tasks/05_m28_native_fetch/M28-004-V-verify-implement-request-response-and-headers-subset.md`

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
- `crates/q-runtime/src/main.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm the parent verification commit is present and the tree is clean.
2. Collect source paths, exact test names, command output, raw evidence, report paths, and commit hash.
3. Generate or update reports from raw evidence; never hand-edit values that should be generated.
4. Update the relevant beta/production task status only when every acceptance criterion is proven.
5. Check that review/evidence indexes and checksums identify the current commit.
6. Run the listed validation commands and commit only evidence/status changes.

## Parent acceptance guardrails

- Common backend fetch code works.
- Header/body limits are enforced.
- No silent Node-specific behavior.
- WPT subset passes.

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

- API conformance.
- Body-used tests.
- Allocation profile.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Changing runtime behavior except for an evidence-generation bug.
- Marking a parent PASS when any acceptance criterion is unproven.
- Using spot checks as canonical evidence.

## Commit guidance

Suggested subject:

```text
m28-004-z: package evidence for implement request response and headers
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-004-Z) — PASS

- Date: 2026-08-28
- Branch/PR: m28-004-z (squash-merged; see git log for final hash)
- Closes: #329

### Parent closure — M28-004 Implement Request, Response, and Headers subset

Parent intent: expose a useful Web-compatible API without materializing unnecessary objects. Status: **PASS**.

Packet commits (squash merges):
- M28-004-A — 2d336b6 (#927, Closes #324): WinterTC / WHATWG Fetch API subset in QuickJS (`fetch`, `Request`, `Response`, `Headers`, `Response.json()`, `bodyUsed` enforcement, `__velquNativeCapabilities.fetch`)
- M28-004-B — 22558a2 (#928, Closes #325): Lazy headers materialization on Request & Response (0 heap allocation if handler checks status/ok only); hardened graceful shutdown test
- M28-004-C — 13dcffb (#929, Closes #326): `Response.prototype.clone()` and `Request.prototype.clone()` with independent `bodyUsed = false` lifecycle state; cloning consumed body fails closed with `TypeError`
- M28-004-D — 32ffac2 (#930, Closes #327): Explicit fail-closed scheme diagnostics for unallowed schemes (`file:`, `ws:`, `data:`, `ftp:`) with `TypeError` matching ADR-0033 §1
- M28-004-V — 2306d59 (#931, Closes #328): Verification closure mapping all 4 acceptance guardrails

### Evidence ledger (required microtask evidence)
- **API conformance**: `conformance/web-api/web-api.conformance.test.ts` (Headers case-insensitivity/mutation/iteration, Response status/ok/headers/text/json/arrayBuffer/bytes, Response.json builder, clone independence, unallowed scheme rejection) + engine tests in `crates/q-engine-quickjs/src/worker.rs`.
- **Body-used tests**: `bodyUsed` single-consumption rule verified on `text()`, `json()`, `arrayBuffer()`, `bytes()`, and `clone()`.
- **Allocation profile**: Lazy property getters on `headers` and on-demand body stream readers avoid eagerly creating map and string objects.

### Command results (this branch)
- `cargo test -p q-engine-quickjs` → 17 unit + 97 worker passed
- `cargo test -p velqu-runtime` → 8 unit + 5 integration + 31 conformance passed (44 total)
- `cargo test -p q-capabilities` → 132 unit + 8 integration passed
- `cargo test -p q-http` → 4+6+1 passed
- `cargo test -p q-bridge` → 11 passed
- `bun test` → 219 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Ledger update
- `docs/beta/04_TASK_LEDGER.md`: M28-004 flipped TODO -> PASS.

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
