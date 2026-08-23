---
task_id: M26-006-A
parent_task: M26-006
milestone: M26
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-006-A — Hash required execution sections

## Atomic goal

Hash required execution sections.

## Parent intent

Protect pack corruption now and provide optional publisher signature verification for beta artifacts.

## Dependencies

- `M26-003-Z` — `tasks/03_m26_qpack_v2/M26-003-Z-package-evidence-for-encode-compiled-router-routeplans-schemas-policies-and-func.md`
- `M26-004-Z` — `tasks/03_m26_qpack_v2/M26-004-Z-package-evidence-for-embed-raw-quickjs-bytecode-without-base64.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`
- `packages/auth-jwt/ (create if absent)`
- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `conformance/security/security.conformance.test.ts`
- `scripts/package`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Hash required execution sections.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Digest detects corruption.
- Signature verifies publisher when configured.
- Unsigned production policy is explicit.
- No docs conflate digest and authenticity.

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

## Required evidence for this microtask

- Integrity/signature tests.
- Key rotation notes.
- Threat-model update.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m26-006-a: hash required execution sections
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record — M26-006-A (PASS)

Deliverable: an aggregate CONTENT digest over exactly the REQUIRED
execution sections — the integrity value that out-of-band publisher
signatures (M26-006-B hook) commit to.

Change (`crates/q-pack/src/lib.rs` only):

- `qpack2::reader::required_sections_digest(&[(DirEntry, &[u8])])`:
  for each REQUIRED catalog id (§6) ascending, the section's sha256 is
  RECOMPUTED from the section bytes (never taken from directory
  claims), and the (id, recomputed hash) pairs are hashed together.
  Layout artifacts (offsets, padding, alignment) are deliberately
  excluded: re-laying out identical required content yields the same
  digest. Optional sections (BUNDLE_BYTECODE) are excluded — the
  digest pins the execution graph, not deploy-mode extras.
- ADR-0026 boundary documented on the function: integrity, not
  authenticity.

Test-fix found during this packet (honesty note): the shared test
helper `repair_section_hash` wrote the repaired content hash at entry
offset +32; the field lives at +24 (spec §3). The M26-004-A bytecode
test's `validate(&repaired).is_err()` assertion therefore passed for
the WRONG reason (garbage bytes, not semantics). Fixed: helper writes
at +24 for both header layouts, and the bytecode test now asserts the
truthful ADR-0026 behavior — a self-consistently rewritten pack PASSES
per-section integrity and only the M26-003-D execution-integrity
binding rejects it. (M26-003-D's own binding test already used +24
inline and needed no change.)

Tests (81 total):

- `required_sections_digest_detects_corruption_and_ignores_optional`:
  optional-section presence leaves the digest unchanged; any
  required-section byte change changes it EVEN WITH the directory hash
  repaired (self-consistent rewrites are detectable by recomputation);
  different required content changes it.
- `bytecode_section_in_bound_file_and_tamper_rejected` updated to the
  truthful self-consistent-rewrite semantics.

Commands and results:

- `cargo test -p q-pack` — 81 passed + 2, 0 failed.
- `cargo test -p velqu-runtime` — 28 passed.
- `cargo test -p q-engine-quickjs` — 1 + 97 passed.
- `bun test` — 83 pass / 0 fail / 487 expect().
- `bun run typecheck` — clean.
- `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `./scripts/verify` — green except the pre-existing documented
  `validate-benchmark-evidence` scoped failure (flagged follow-up from
  M26-002-A).

Guardrails: digest detects corruption (recomputation proof); no
docs/signature conflation (ADR-0026 boundary on the function; the
signature hook itself is M26-006-B).
