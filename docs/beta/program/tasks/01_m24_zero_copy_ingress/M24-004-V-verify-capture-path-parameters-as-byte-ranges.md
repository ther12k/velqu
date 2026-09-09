---
task_id: M24-004-V
parent_task: M24-004
milestone: M24
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-004-V — Verify Capture path parameters as byte ranges

## Atomic goal

Prove every acceptance criterion for parent task M24-004 without broadening scope.

## Parent intent

Avoid allocating parameter strings until validation or JavaScript access requires them.

## Dependencies

- `M24-004-A` — `tasks/01_m24_zero_copy_ingress/M24-004-A-store-capture-start-end-ranges-against-the-uri-path.md`
- `M24-004-B` — `tasks/01_m24_zero_copy_ingress/M24-004-B-bind-route-specific-parameter-names-after-routeid-selection.md`
- `M24-004-C` — `tasks/01_m24_zero_copy_ingress/M24-004-C-validate-numeric-uuid-formats-directly-from-bytes-where-possible.md`
- `M24-004-D` — `tasks/01_m24_zero_copy_ingress/M24-004-D-materialize-js-strings-lazily.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `conformance/schema/schema.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Parameterized routes preserve exact names and values.
- No owned parameter string on an unread path.
- Percent-decoding policy is explicit and tested.
- Invalid encodings fail consistently.

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
cargo test -p q-schema-runtime
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

- Allocation test.
- Reference router parity.
- Encoding edge-case corpus.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m24-004-v: verify capture path parameters as byte ranges
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: verification of the M24-004 A–D byte-range path-parameter implementation against the four parent guardrails. Implementation commits: `f6cb33f` (A, PR #654 — capture byte ranges), `e4e488b` (B, PR #655 — RouteId-bound interned names), `264fa1d` (C, PR #656 — byte-level numeric/UUID validation), `328696e` (D, PR #657 — per-key lazy JS strings).
- Correction to M24-004-D's record (found by this verification, fixed here): the D packet's `cargo test -p q-bridge PASS (9)` and clippy claims were stale — after the `ParamSpec` change, q-bridge's test helper still used the removed `params` field (compile failure masked by an `&&` chain), and the byte-charging `.map(|v| .. v)` tripped clippy `manual_inspect`. Both defects are fixed in this commit (helper → `param_specs: vec![]`; `.map` → `.inspect`); with the fixes the full matrix below is green. No production behavior changed by the fixes.
- Guardrail mapping (criterion → source → tests):
  - **Names and values preserved.** Ranges + interned names + per-key materialization; proven by `capture_ranges_defer_string_allocation_and_match_reference_values`, `param_names_bind_after_routeid_selection_and_are_borrowed`, the reference-parity property suite, `params_materialize_one_key_per_access`, and end-to-end HTTP conformance (`/hello/:name`, policy routes, `full_runtime_conformance`).
  - **No owned parameter string on an unread path.** Admission stores `ParamSpec` name+range pairs only; invalid values reject from bytes (`validate_params_bytes_*` tests); unread params allocate zero value strings (`lazy_ctx_touches_nothing` — 0 host calls; per-key proof — 1 field/2 bytes for exactly one key).
  - **Percent-decoding policy explicit and tested.** Raw-bytes policy; `capture_ranges_encoding_corpus_is_raw_and_panic_free` (percent/multibyte/emoji/slash corpus).
  - **Invalid encodings fail consistently.** Corpus + `validate_params_bytes_rejects_invalid_formats_from_bytes` (invalid UTF-8 integer bytes → typed `type` error); path slices stay on `/` char boundaries.
- Exact command results: `cargo test -p q-engine-quickjs` PASS (1 + 91); `cargo test -p q-http` PASS (2 + 3); `cargo test -p q-bridge` PASS (9); `cargo test -p q-schema-runtime` PASS (unit + fuzz); `cargo test -p velqu-runtime` PASS (13); `cargo test -p q-router` PASS (15); `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS; `bun run typecheck` PASS; `bun test` PASS (35/0). Raw logs: `/tmp/m24-004-v-rust.log` (stage markers ENG/HTTP/BRIDGE/SCHEMA/RUNTIME/ROUTER/FMT/CLIPPY all OK), `/tmp/m24-004-v-verify.log`, `/tmp/m24-004-v-bun.log`.
- Scoped verification limitation (unchanged, honestly recorded): `./scripts/verify` exits 1 on the single stage `validate-benchmark-evidence` — fresh-worktree stage ordering reports missing artifacts first; post-build the worktree `qRuntimeRelease` hash differs from the canonical manifest. No benchmark manifest or performance claim changed; all other verify stages passed.
- Next dependency-ready task: M24-004-Z (package evidence for Capture path parameters as byte ranges).

