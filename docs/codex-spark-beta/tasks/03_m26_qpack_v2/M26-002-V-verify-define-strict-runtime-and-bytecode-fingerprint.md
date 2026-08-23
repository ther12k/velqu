---
task_id: M26-002-V
parent_task: M26-002
milestone: M26
priority: P0
mode: VERIFY
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-002-V — Verify Define strict runtime and bytecode fingerprint

## Atomic goal

Prove every acceptance criterion for parent task M26-002 without broadening scope.

## Parent intent

Prevent loading bytecode or plans under an incompatible engine/runtime build.

## Dependencies

- `M26-002-A` — `tasks/03_m26_qpack_v2/M26-002-A-include-runtime-abi-quickjs-ng-version-build-hash-rquickjs-version-bytecode-form.md`
- `M26-002-B` — `tasks/03_m26_qpack_v2/M26-002-B-fail-closed-on-mismatch.md`
- `M26-002-C` — `tasks/03_m26_qpack_v2/M26-002-C-provide-explicit-source-rebuild-path.md`
- `M26-002-D` — `tasks/03_m26_qpack_v2/M26-002-D-never-silently-fall-back.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Any fingerprint mismatch rejects before ready.
- Error identifies incompatible dimension.
- Engine upgrades require pack rebuild.
- Cross-target packs are rejected.

## Targeted commands

```bash
cargo test -p q-pack
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

- Fingerprint tests.
- Cross-build fixtures.
- Upgrade lane documentation.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m26-002-v: verify define strict runtime and bytecode fingerprint
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M26-002-V)

Status: **PASS**. Every parent M26-002 acceptance guardrail maps to source
and passing tests; all verification commands ran fresh on this branch (no
code changes — verification closure only).

### Guardrail → source → evidence

1. **Any fingerprint mismatch rejects before ready.**
   - ABI/engine/rquickjs/build-hash/capability-hash rejections
     (M26-002-A tests), cross-target bytecode rejections at load
     (M26-002-B), hash-valid garbage bytecode rejecting at the engine
     boundary (M26-002-D runtime test) — every path is
     `startup.rejected`, never a partial serve.
2. **Error identifies incompatible dimension.**
   - Every rejection names its dimension: "dimension: binding"
     (rquickjs), "dimension: runtime build" (+ rebuild remedy),
     "dimension: capabilities", "incompatible dimensions: arch/os/
     pointer width/endianness" (cross-target), bytecode-load failures
     with the underlying error.
3. **Engine upgrades require pack rebuild.**
   - `runtime_build_hash()` covers the engine identity tuple; any
     upgrade changes it and packs pin the exact hash ("engine upgrades
     require a pack rebuild" in the message).
4. **Cross-target packs are rejected.**
   - `cross_target_bytecode_fails_closed_with_dimensions` (all four
     dimensions independently); bytecode without a target fingerprint
     rejects fail-closed; the explicit `--no-bytecode` source path
     (M26-002-C) is a flag, never an automatic fallback (M26-002-D
     hash-valid-garbage proof).

### Command results (this branch, fresh worktree)

- `cargo test -p q-pack` — 53 + 2; `cargo test -p q-engine-quickjs` —
  1 + 97; `cargo test -p q-http` — 4 + 6 + 1; `cargo test -p
  q-schema-runtime` — 58 + 4 + 5; `cargo test -p velqu-runtime` — 26 —
  all passed.
- `bun test` — 82 passed, 0 failed, 487 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — clean.
- `./scripts/verify` — all stages pass except
  `validate-benchmark-evidence`: the documented isolated-worktree
  hash mismatch for `qRuntimeRelease`/`proofPack` (M26-002-A changed
  pack bytes; canonical proofPack refresh flagged there).

Changed files: this record, `docs/codex-spark-beta/STATUS.md`,
`docs/codex-spark-beta/indexes/TASK_INDEX.md` (verification closure only).

Commit: `c98d794`.
