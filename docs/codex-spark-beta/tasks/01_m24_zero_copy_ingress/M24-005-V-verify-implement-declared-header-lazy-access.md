---
task_id: M24-005-V
parent_task: M24-005
milestone: M24
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-005-V — Verify Implement declared-header lazy access

## Atomic goal

Prove every acceptance criterion for parent task M24-005 without broadening scope.

## Parent intent

Expose only headers declared by route or policy without cloning the entire HeaderMap.

## Dependencies

- `M24-005-A` — `tasks/01_m24_zero_copy_ingress/M24-005-A-compile-header-name-ids-into-routeplan.md`
- `M24-005-B` — `tasks/01_m24_zero_copy_ingress/M24-005-B-read-header-values-by-id-on-demand.md`
- `M24-005-C` — `tasks/01_m24_zero_copy_ingress/M24-005-C-define-duplicate-header-behavior-and-byte-string-conversion.md`
- `M24-005-D` — `tasks/01_m24_zero_copy_ingress/M24-005-D-keep-full-headers-escape-hatch-explicit-and-costed.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `packages/compiler/src/emit.ts`
- `packages/cli/src/index.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Route declaring no headers copies none.
- Auth route reads only required headers.
- Duplicate/non-UTF8 behavior matches contract.
- Secret headers are redacted in diagnostics.

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

- Header access tests.
- Allocation profile.
- Security redaction tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Evidence

- `params_materialize_one_key_per_access` and `headers_are_declared_only_and_per_key_lazy` prove lazy per-key request access and zero unread-field materialization.
- `full_headers_escape_hatch_is_explicit_and_verified` proves full-Headers access requires explicit schema-less binding and sentinel authorization.
- `declared_header_value_joins_duplicates_and_is_lossy` proves duplicate joining and lossy UTF-8 conversion.
- Header values remain outside diagnostics/logging paths; secret redaction guardrail preserved.
- `cargo test -p q-engine-quickjs --test engine`: PASS.
- `cargo test -p q-http`: PASS.
- `cargo test -p q-bridge`: PASS.
- `cargo test -p velqu-runtime`: PASS (13 tests).
- `bun test`: PASS (35 pass, 0 fail).
- `bun run typecheck`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- Initial Bun run required `bun install --frozen-lockfile` and release proof/runtime artifacts; rerun passed. No source changes from setup.
- `./scripts/verify` benchmark stage retains known temporary-worktree binary hash limitation; canonical manifests unchanged.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m24-005-v: verify implement declared header lazy access
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
