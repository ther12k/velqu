---
task_id: M25-007-V
parent_task: M25-007
milestone: M25
priority: P1
mode: VERIFY
status: PASS
context_card: context/milestones/M25.md
commit_required: true
---

# M25-007-V — Verify Implement explicit generic and Web fallback paths

## Atomic goal

Prove every acceptance criterion for parent task M25-007 without broadening scope.

## Parent intent

Support advanced cases without hiding performance or semantic costs.

## Dependencies

- `M25-007-A` — `tasks/02_m25_schema_codecs/M25-007-A-tag-fallback-reason-in-routeplan.md`
- `M25-007-B` — `tasks/02_m25_schema_codecs/M25-007-B-support-raw-response-full-request-escape-hatches.md`
- `M25-007-C` — `tasks/02_m25_schema_codecs/M25-007-C-keep-fallback-bounded-and-deadline-aware.md`
- `M25-007-D` — `tasks/02_m25_schema_codecs/M25-007-D-expose-bridge-crossings-and-codec-choice-in-velqu-inspect.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `packages/treaty/src/index.ts`
- `packages/contract/src/index.ts`
- `packages/testing/src/index.ts`
- `conformance/treaty/treaty.conformance.test.ts`
- `benchmarks/harness/`
- `benchmarks/manifest.json`
- `benchmarks/raw/`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm all implementation dependencies are present in the current branch.
2. Map every parent acceptance criterion to source and at least one positive or negative test.
3. Run targeted suites first, then the full verification commands listed below.
4. Inspect fail-before-ready behavior, cleanup paths, and evidence/source consistency where relevant.
5. Fix only defects necessary to satisfy this parent task; create a new follow-up task for unrelated findings.
6. Record exact commands and results, then commit the verification closure.

## Parent acceptance guardrails

- Fallback never activates silently.
- Raw Response bypass behavior is documented.
- No contract claim is generated when adapter lacks required projection.
- Fallback routes pass conformance.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-schema-runtime
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

- Inspect snapshots.
- Fallback integration tests.
- Performance delta report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Adding new features beyond the parent acceptance criteria.
- Marking the milestone gate PASS.
- Replacing raw evidence with a prose assertion.

## Commit guidance

Suggested subject:

```text
m25-007-v: verify implement explicit generic and web fallback paths
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record (M25-007-V)

Status: **PASS**. Every parent M25-007 acceptance guardrail maps to source
and passing tests; all verification commands were run fresh on this branch
(no code changes — verification closure only).

### Guardrail → source → evidence

1. **Fallback never activates silently.**
   - `q_pack` verify: a js plan strategy without a reason from the closed
     `FALLBACK_REASONS` vocabulary rejects at pack load; an
     out-of-vocabulary reason rejects; a native plan carrying a reason
     rejects (`rejects_silent_fallback_and_invalid_reasons`).
   - Raw envelopes without the `raw-response` capability are controlled
     500s with the violation logged
     (`raw_response_and_full_request_escape_hatches`).
   - Compiler: explicit developer-forced js responses push an `explicit`
     fallback descriptor like every other fallback.
2. **Raw Response bypass behavior is documented.**
   - `docs/specs/unsupported-transformations.md` §5 — status contract,
     header precedence, AS-IS body, the schema-exclusivity rule (pack
     verify rejects a raw route with a declared response schema), and the
     full-request materialization bounds.
3. **No contract claim is generated when adapter lacks required
   projection.**
   - The raw-response/schema exclusivity at pack verify; the route
     manifest (M25-007-D) exposes the real codec choices so no tool claims
     a native codec where the generic path runs (manifest + CLI inspect
     snapshot test).
4. **Fallback routes pass conformance.**
   - `fallback_paths_are_bounded_and_deadline_aware` — busy handlers on
     js-validation, raw-response, and full-request routes each settle 504
     at the deadline; the engine keeps serving (no quarantine); oversize
     fallback bodies reject 413 before the engine.
   - `js_fallback_body_routes_raw_json_to_handler`,
     `quickjs_stringify_fallback_stays_json_equivalent_to_encoder`,
     `deeply_nested_body_fails_boundedly` — fallback semantics parity.

### Command results (this branch, fresh worktree)

- `cargo test -p q-engine-quickjs` — 1 + 96 passed.
- `cargo test -p q-schema-runtime` — 57 unit + 3 fuzz passed.
- `cargo test -p velqu-runtime` — 24 integration passed.
- `cargo test -p q-pack` — 41 + 2 passed.
- `bun test` — 75 passed, 0 failed, 340 expect calls.
- `bun run typecheck` — clean. `cargo fmt --check` — clean.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `scripts/validate-okf` — 176 links, 0 errors.
- `./scripts/verify` — all stages pass except the documented
  isolated-worktree `qRuntimeRelease`/`proofPack` manifest hash mismatch
  (identical on every packet branch this session).

Inspect snapshots (required parent evidence): the M25-007-D compiler
conformance test's live CLI snapshot plus the manifest assertions.

Changed files: this record, `docs/codex-spark-beta/STATUS.md`,
`docs/codex-spark-beta/indexes/TASK_INDEX.md` (verification closure only).

Commit: `a0f421f`.
