---
task_id: M27-001-B
parent_task: M27-001
milestone: M27
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-001-B — Define CapabilityId/version/dependencies

## Atomic goal

Define CapabilityId/version/dependencies.

## Parent intent

Specify install, lazy init, invocation ownership, cancellation, drain, shutdown, versioning, and errors for native capabilities.

## Dependencies

- `M27-001-A` — `tasks/04_m27_capability_linker/M27-001-A-accept-adr.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-runtime/src/main.rs`
- `docs/beta/`
- `examples/proof/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Define CapabilityId/version/dependencies.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No capability can start work outside allowed phase.
- Every op is physically cancellable or explicitly non-cancellable.
- Version conflicts fail before ready.
- Shutdown reaches quiescence or fails closed.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
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

- Lifecycle state tests.
- Capability author guide draft.
- Threat review.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m27-001-b: define capabilityid version dependencies
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M27-001-B (PASS)

Deliverable: CapabilityId/version/dependencies defined (ADR-0029).

### Changed files

- `docs/okf/decisions/0029-capability-identity-versioning-and-requirements.md`
  — new ADR: validated `namespace:name` grammar with a closed
  namespace vocabulary (`runtime` only), charset/length bounds,
  integer versions compared **exactly** (no implicit compatibility
  until M27-009-D), `CapabilityRequirement`/`CapabilityDescriptor`
  shapes, deterministic resolution, fail-before-ready integration
  with ADR-0028, threat review.
- `docs/okf/decisions/index.md` — ADR-0029 entry.
- `crates/q-capabilities/src/identity.rs` — new module:
  `CapabilityId::parse` (fail-closed typed errors, nothing
  repaired), `CapabilityVersion` newtype, requirement/descriptor
  types, `resolve_requirement` (exact match; `Missing` /
  `VersionConflict` with both versions), `resolve_and_install`
  (conflict routes the lifecycle to `Failed` before `Ready`).
- `crates/q-capabilities/src/lib.rs` — `pub mod identity` + re-exports.
- `docs/beta/CAPABILITY_AUTHORS.md` — "Identity and versions"
  section added to the draft guide.
- `docs/codex-spark-beta/STATUS.md`, `indexes/TASK_INDEX.md` — this
  packet PASS.

### Tests

`cargo test -p q-capabilities` — 16 passed (7 lifecycle from A + 9
identity): `ids_parse_and_round_trip`,
`malformed_ids_fail_closed_with_typed_errors` (empty / missing
separator / empty namespace / empty name / unknown namespace
`node:fs` / uppercase / underscore / over-length name / over-length
id / extra separator), `exact_version_match_satisfies_requirement`,
`version_mismatch_conflicts_with_both_versions_named` (message
carries required + linked), `unlinked_capability_is_missing`,
`resolve_and_install_installs_on_success`,
`resolve_and_install_conflict_fails_lifecycle_before_ready`
(guardrail 3: Failed, activate rejected terminal),
`resolve_and_install_missing_fails_lifecycle_before_ready`,
`descriptors_carry_validated_dependencies`.

### Commands (fresh worktree on M27-001-A HEAD aca4552)

- `cargo test -p q-pack` 96 · `-p q-engine-quickjs` 98 ·
  `-p q-capabilities` 16 · `-p velqu-runtime` 30 — pass.
- `bun test` 125 pass / 0 fail; `bun run typecheck` clean.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets --
  -D warnings` — clean.

### Notes

- Dependency-graph construction and cycle rejection stay with the
  compiler resolver (M27-002-B); this packet defines only the data
  shapes and single-requirement resolution.
- Linked-set uniqueness is upstream of this scan (M27-002-C
  inventory hash), as documented in ADR-0029 §4.
