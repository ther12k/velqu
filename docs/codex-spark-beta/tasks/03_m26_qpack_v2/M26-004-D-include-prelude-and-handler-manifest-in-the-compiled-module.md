---
task_id: M26-004-D
parent_task: M26-004
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-004-D — Include prelude and handler manifest in the compiled module

## Atomic goal

Include prelude and handler manifest in the compiled module.

## Parent intent

Remove base64 storage/decoding and duplicate production source by default.

## Dependencies

- `M26-004-C` — `tasks/03_m26_qpack_v2/M26-004-C-make-source-optional-sidecar-development-section.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-http/src/lib.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Include prelude and handler manifest in the compiled module.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No base64 decode at startup.
- No source parse in bytecode production mode.
- Tamper/incompatibility rejects.
- Small-app source mode remains explicit if measured faster.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
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

- Bytecode integration tests.
- Tamper tests.
- Pack size/startup evidence.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-004-d: include prelude and handler manifest in the compiled module
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-004-D (PASS)

Deliverable: the compiled module now contains the prelude and handler
manifest, so bytecode production startup evaluates zero prelude source
(guardrail: no source parse in bytecode production mode).

Changed files:

- `crates/q-engine-quickjs/src/lib.rs` — `pub mod prelude` (PRELUDE
  importable by the bytecode tool); `QuickJsConfig.embedded_prelude`
  (default false).
- `crates/q-engine-quickjs/src/worker.rs` — handle collection extracted
  into `collect_prelude_handles` (shared by both paths); spawn evaluates
  PRELUDE source only when NOT embedded; `load()` collects handles after
  the module evals (returned through the closure tuple, assigned to
  `self.prelude` after); embedded flag with no bytecode fails closed in
  the worker ("embedded-prelude pack must load bytecode").
- `crates/q-pack/src/lib.rs` — `QPack.bundle_prelude: Option<String>`
  (serde-omitted by default; closed vocabulary "embedded"); verify
  rejects embedded-without-bytecode and unknown values;
  `verify_without_bytecode` clears the marker with the bytecode (the
  source path always evaluates the host prelude — `--no-bytecode`
  recovery stays sanctioned for embedded packs).
- `crates/q-bytecode-tool/src/main.rs` + `Cargo.toml` — compiles
  `PRELUDE + bundle` (same-workspace, byte-identical prelude) and stamps
  `bundle_prelude: "embedded"`.
- `crates/q-runtime/src/main.rs` — config flag wired: marker present AND
  policy Enforce.
- `crates/q-runtime/tests/runtime_conformance.rs` — integration test.

Tests:

- `embedded_prelude_pack_serves_identically_and_source_recovery_works`
  (velqu-runtime, 28 total) — prelude+manifest module bytecode serves
  C0/C3/JS-JSON identically; `--no-bytecode` boots from source with the
  host prelude and serves.
- `bundle_prelude_marker_rules` (q-pack, 75 total) — marker rules and
  the source-path clearing.
- Legacy bytecode path unchanged and green
  (`bytecode_pack_serves_identically_and_mismatch_fails_before_ready`:
  host prelude + module bytecode without marker).

Commands and results:

- `cargo test -p q-pack` — 75 + 2; `cargo test -p q-engine-quickjs` —
  1 + 97; `cargo test -p velqu-runtime` — 28.
- `bun test` — 83 pass / 0 fail / 487 expect().
- `bun run typecheck`, `cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings` — clean.
- `./scripts/verify` — green except the pre-existing documented
  `validate-benchmark-evidence` scoped failure (flagged follow-up from
  M26-002-A).

Guardrails: no base64 decode at startup (M26-004-B cache still feeds the
handoff); no source parse in bytecode production mode (prelude source
eval removed from the embedded path — structural proof: handles can only
come from module globals); tamper/incompatibility rejects unchanged
(bytecode sha256 + fingerprint checks cover the prelude-including
module); source mode explicit (`--no-bytecode` recovery tested).
