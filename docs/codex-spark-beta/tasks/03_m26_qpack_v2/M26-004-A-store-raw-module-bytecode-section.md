---
task_id: M26-004-A
parent_task: M26-004
milestone: M26
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-004-A — Store raw module bytecode section

## Atomic goal

Store raw module bytecode section.

## Parent intent

Remove base64 storage/decoding and duplicate production source by default.

## Dependencies

- `M26-002-Z` — `tasks/03_m26_qpack_v2/M26-002-Z-package-evidence-for-define-strict-runtime-and-bytecode-fingerprint.md`
- `M26-003-Z` — `tasks/03_m26_qpack_v2/M26-003-Z-package-evidence-for-encode-compiled-router-routeplans-schemas-policies-and-func.md`

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
5. Implement exactly this deliverable: Store raw module bytecode section.
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
m26-004-a: store raw module bytecode section
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-004-A (PASS)

Deliverable: store the raw QuickJS module bytecode as qpack2 section
0x0007 (spec §6, OPTIONAL) — raw bytes verbatim, engine-target metadata
as string refs; no base64 anywhere on the v2 path.

Changed files: `crates/q-pack/src/lib.rs` only.

Implementation (`qpack2::graph::bytecode_section`):

- `BytecodeMeta { quickjs, binding, endianness, target: Option<BytecodeTarget> }`
  — the v1 `BundleBytecode` fields minus the base64 `data`; reuses the
  crate-root `BytecodeTarget`.
- Payload layout: quickjs_ref u32, binding_ref u32, endianness_ref u32,
  has_target u8 [target: arch_ref u32, os_ref u32, pointer_width u8,
  endianness_ref u32], code_len u32, then the bytecode bytes verbatim.
  Fixed header size 30 bytes with target present.
- `encode(&meta, &code, &mut strings)` / `decode(bytes, strings)` with
  bounds-checked decode: truncation at every boundary, out-of-bounds
  string refs, target flag drift, pointer width not 4/8, code_len past
  the sane bound `MAX_CODE_BYTES = 1<<28` (constraint 11), code_len
  shorter/longer than remaining bytes, and trailing bytes all reject.

Tests (crates/q-pack/src/lib.rs):

- `bytecode_section_round_trips_raw_bytes` — payload mixing bytes outside
  the base64 alphabet survives byte-for-byte with and without target: if
  any base64 encode/decode happened on this path, those bytes could not
  round-trip (guardrail: no base64 decode at startup on the v2 path).
- `bytecode_section_rejects_drift_and_truncation` — every malformed
  shape listed above rejects.
- `bytecode_section_in_bound_file_and_tamper_rejected` — a bound v2 file
  (96-byte header) carrying section 0x0007 validates and decodes; a
  payload byte flip rejects via per-section sha256; with the content
  hash repaired, the M26-003-D execution-integrity binding still rejects.
- `bytecode_base64_vs_raw_size_report` — honest size evidence: 768 raw
  bytes → 1 026 bytes as base64-in-JSON vs 798 bytes as v2 section
  (30-byte fixed header); raw section strictly smaller (report printed
  under --nocapture).

Commands and results:

- `cargo test -p q-pack` — 71 passed + 2, 0 failed.
- `cargo test -p q-engine-quickjs` — 1 + 97 passed.
- `cargo test -p velqu-runtime` — 26 passed.
- `bun test` — 82 pass / 0 fail / 487 expect().
- `bun run typecheck` — clean.
- `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — green except the pre-existing documented
  `validate-benchmark-evidence` scoped failure (qRuntimeRelease +
  proofPack manifest hashes; flagged matched-evidence follow-up from
  M26-002-A, not altered here).

Guardrails: no base64 decode on the v2 bytecode path (round-trip proof);
tamper/incompatibility rejection (per-section sha256 + execution-integrity
binding + drift rejections); source-mode explicitness and load-exactly-once
are M26-004-C/B scope (v2 sections are not yet the production load path —
`current_mode_is_pinned_until_native_v2_lands`).
