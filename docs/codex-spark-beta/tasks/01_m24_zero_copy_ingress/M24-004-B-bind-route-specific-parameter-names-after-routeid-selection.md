---
task_id: M24-004-B
parent_task: M24-004
milestone: M24
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-004-B — Bind route-specific parameter names after RouteId selection

## Atomic goal

Bind route-specific parameter names after RouteId selection.

## Parent intent

Avoid allocating parameter strings until validation or JavaScript access requires them.

## Dependencies

- `M24-004-A` — `tasks/01_m24_zero_copy_ingress/M24-004-A-store-capture-start-end-ranges-against-the-uri-path.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `packages/compiler/src/emit.ts`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `crates/q-engine-quickjs/src/convert.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Bind route-specific parameter names after RouteId selection.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Parameterized routes preserve exact names and values.
- No owned parameter string on an unread path.
- Percent-decoding policy is explicit and tested.
- Invalid encodings fail consistently.

## Targeted commands

```bash
cargo test -p q-pack
```
```bash
cargo test -p q-router
```
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

## Required evidence for this microtask

- Allocation test.
- Reference router parity.
- Encoding edge-case corpus.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m24-004-b: bind route specific parameter names after routeid selection
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: route-specific parameter names bind after RouteId selection through an interned table. `CompiledRoute.param_names: Vec<String>` (a per-route clone built at router construction) is replaced by `param_name_ids: Vec<u32>` into a single Router-owned `param_name_table` shared by all routes — equal names dedupe to one entry across `build`, `from_pack`, and `from_serialized`. `Router::param_names(route_index) -> Vec<&str>` borrows the RouteId-selected route's names from the table with zero allocation; `materialize_params` resolves names through the same ids, so the owned pair is created only at materialization. The same path shape under different methods binds each RouteId's OWN names (GET `/users/:id` vs POST `/users/:userId`), and repeated names within one route bind positionally per capture.
- Changed files:
  - `crates/q-router/src/lib.rs` (interned name table + dense ids across all three constructors; borrowed `param_names` accessor; id-based `materialize_params`; RouteId-binding test)
  - `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-004-B-bind-route-specific-parameter-names-after-routeid-selection.md`, `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`
- Tests: new `param_names_bind_after_routeid_selection_and_are_borrowed` (method-disambiguated name binding, repeated-name positional binding, out-of-range index fails closed with no names). All M24-004-A proofs remain green unchanged (ranges corpus, reference parity property suite, laziness). End-to-end unchanged: engine 90, runtime conformance 13, Bun 35/0.
- Verification: `cargo test -p q-router` PASS (15); `cargo test -p q-engine-quickjs` PASS (1 + 90); `cargo test -p q-http` PASS (2 + 3); `cargo test -p q-bridge` PASS (9); `cargo test -p q-schema-runtime` PASS (unit + fuzz); `cargo test -p velqu-runtime` PASS (13); `bun run typecheck` PASS; `bun test` PASS (35/0 after proof-pack build); `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS. Raw log: `/tmp/m24-004-b-bun.log`.
- Acceptance criteria proven: parameterized routes preserve exact names and values (RouteId binding test + parity suite); no owned parameter string on an unread path (borrowed `param_names`, ids-only CompiledRoute, materialization still gated by M24-004-A's needs check); percent-decoding policy unchanged and tested (M24-004-A corpus); invalid encodings fail consistently (unchanged raw-bytes behavior).
- Remaining risk / deferred by design: numeric/UUID byte-level validation is M24-004-C; lazy JS string materialization through the bridge is M24-004-D.
- Next dependency-ready task: M24-004-C (validate numeric/UUID formats directly from bytes where possible).

