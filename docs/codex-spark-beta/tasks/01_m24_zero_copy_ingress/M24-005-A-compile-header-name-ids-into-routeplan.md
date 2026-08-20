---
task_id: M24-005-A
parent_task: M24-005
milestone: M24
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-005-A — Compile header-name IDs into RoutePlan

## Atomic goal

Compile header-name IDs into RoutePlan.

## Parent intent

Expose only headers declared by route or policy without cloning the entire HeaderMap.

## Dependencies

- `M24-003-Z` — `tasks/01_m24_zero_copy_ingress/M24-003-Z-package-evidence-for-implement-worker-local-generation-checked-request-slab.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Compile header-name IDs into RoutePlan.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

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

## Required evidence for this microtask

- Header access tests.
- Allocation profile.
- Security redaction tests.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m24-005-a: compile header name ids into routeplan
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: header-name IDs compiled into RoutePlan. `RoutePlanDecl` gains `header_name_ids: Vec<u32>` and `QPack` gains the canonical `header_name_table` (sorted, deduped union across routes). The compiler (`packages/compiler/src/emit.ts`) derives each route's declared names — security scheme headers (or the `authorization` header a policy implies, mirroring the existing security projection) plus headers-binding schema properties — emits the per-plan ids, and attaches the table to the pack. `QPack::verify` enforces the invariant end-to-end: each plan's ids must be exactly the names its route declares, `fieldNeeds.headers` requires at least one declared name, and the pack table must equal the derived union. Because plans serialize inside `RouteEntry`, the ids are covered by the execution-graph (routes) hash; tampering the table or ids is rejected at load (`header_name_table_and_ids_are_verified`). Verified end-to-end: the compiled proof pack emits `headerNameTable: ["authorization"]` with `headerNameIds: [0]` on exactly the policy route, loads and serves through the release binary.
- Changed files:
  - `crates/q-pack/src/lib.rs` (`header_name_ids`, `header_name_table`, verify enforcement, tamper test)
  - `packages/compiler/src/emit.ts` (table construction, per-plan ids, pack field)
  - `crates/q-runtime/tests/runtime_conformance.rs` (fixture `finalize_numeric` derives table+ids like the compiler)
  - `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-005-A-compile-header-name-ids-into-routeplan.md`, `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`
- Tests: new `header_name_table_and_ids_are_verified` (q-pack: consistent pack verifies; tampered table entry, tampered ids, and `fieldNeeds.headers` with no names are each rejected with the specific error). Emission proven by the rebuilt proof pack + full HTTP conformance (policy 401/200 flow) and engine suites.
- Verification: `cargo test -p q-pack` PASS (35 + 2 fuzz); `cargo test -p q-engine-quickjs` PASS (1 + 91); `cargo test -p q-http` PASS (2 + 3); `cargo test -p q-bridge` PASS (9); `cargo test -p q-schema-runtime` PASS; `cargo test -p q-router` PASS (15); `cargo test -p velqu-runtime` PASS (13); `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS; `bun run typecheck` PASS; `bun test` PASS (35/0 after proof-pack + release-binary rebuild). Raw logs: `/tmp/m24-005-a-rust.log`, `/tmp/m24-005-a-bun.log`, `/tmp/m24-005-a-proof.log`.
- Guardrail status: header values are still read through the full-headers path — per-ID on-demand reading is M24-005-B; duplicate/non-UTF8 header semantics and secret redaction land with B/C per the packet sequence. This packet delivers the compiled ids + verified table.
- Next dependency-ready task: M24-005-B (read header values by ID on demand).

