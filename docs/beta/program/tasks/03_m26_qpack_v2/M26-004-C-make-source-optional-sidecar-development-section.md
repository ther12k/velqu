---
task_id: M26-004-C
parent_task: M26-004
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-004-C — Make source optional sidecar/development section

## Atomic goal

Make source optional sidecar/development section.

## Parent intent

Remove base64 storage/decoding and duplicate production source by default.

## Dependencies

- `M26-004-B` — `tasks/03_m26_qpack_v2/M26-004-B-load-exactly-once.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`

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

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Make source optional sidecar/development section.
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
m26-004-c: make source optional sidecar development section
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-004-C (PASS)

Deliverable: ADR-0027 producer flip — production packs embed no source
map; debug sources and maps move to the external
`<pack>.sources.json` sidecar (advisory tooling file, bound to the exact
pack bytes via packSha256) that the runtime never reads.

Changed files:

- `packages/compiler/src/emit.ts` — pack field `sourceMap` now `null`
  (debug-free production default; the map is still computed for the
  sidecar).
- `packages/compiler/src/index.ts` — build writes
  `app.qpack.sources.json` beside the pack: `{formatVersion: 1,
  packSha256: sha256(pack bytes), bundleSource, sourceMap, modules}`.
- `crates/q-pack/src/lib.rs` — `sources_sidecar` module:
  `SourcesSidecar`/`SidecarModule` types, `SIDECAR_FORMAT_VERSION = 1`,
  `verify_against(pack_sha256)` TOOL-side advisory binding check
  (unknown format versions fail closed for tooling; mismatch = wrong
  pack). Runtime confers nothing (ADR-0026 trust model).
- `crates/q-runtime/tests/runtime_conformance.rs` — test
  `source_sidecar_never_affects_serving`.
- `conformance/compiler/compiler.test.ts` — sidecar suite.

Tests:

- `source_sidecar_never_affects_serving` (velqu-runtime) — a
  present-but-garbage sidecar next to a valid pack: server serves
  normally (ADR-0027 compatibility matrix row 1).
- `sources_sidecar_binds_to_one_pack_and_tooling_checks_are_advisory`
  (q-pack) — sidecar JSON round trip; exact-hash verifies, drift and
  unknown format versions reject for tooling.
- Compiler conformance "production pack embeds no source map; sidecar
  carries sources bound to the pack hash" — pack.sourceMap is null,
  sidecar exists, packSha256 equals sha256(app.qpack), bundleSource
  contains the function manifest, modules non-empty.
- Pre-existing `verification_is_independent_of_debug_sidecars` unchanged
  and still green.

Commands and results:

- `cargo test -p q-pack` — 74 passed + 2.
- `cargo test -p velqu-runtime` — 27 passed.
- `cargo test -p q-engine-quickjs` — 1 + 97 passed.
- `bun test` — 83 pass / 0 fail / 178+ expects in the compiler suite
  alone (full suite 83 pass).
- `bun run typecheck`, `cargo fmt`, `cargo clippy --workspace
  --all-targets -- -D warnings` — clean.
- `./scripts/verify` — green except the pre-existing documented
  `validate-benchmark-evidence` scoped failure (flagged follow-up from
  M26-002-A).

Guardrails: no runtime read path to sidecars (pinned by test); production
packs debug-free by default; v1 compatibility preserved (legacy packs
with inline maps still verify and symbolize via `mapper_for`).
