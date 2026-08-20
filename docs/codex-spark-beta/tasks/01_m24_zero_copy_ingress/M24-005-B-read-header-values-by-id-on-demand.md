---
task_id: M24-005-B
parent_task: M24-005
milestone: M24
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-005-B — Read header values by ID on demand

## Atomic goal

Read header values by ID on demand.

## Parent intent

Expose only headers declared by route or policy without cloning the entire HeaderMap.

## Dependencies

- `M24-005-A` — `tasks/01_m24_zero_copy_ingress/M24-005-A-compile-header-name-ids-into-routeplan.md`

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
5. Implement exactly this deliverable: Read header values by ID on demand.
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
m24-005-b: read header values by id on demand
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: header values read by ID on demand. `serve.rs` admission now copies ONLY the header names this route's plan declares (M24-005-A's `header_name_ids` → `pack.header_name_table`), looking each name up on the native HeaderMap (first value, lossy-UTF-8 — the same semantics `materialize_headers` had) instead of cloning the entire map. JavaScript gets per-key lazy access mirroring M24-004-D: new natives `__velquReqHeaderNames` (declared keys, 0 data cost) and `__velquReqHeader(slot, gen, key)` (materializes exactly one value, charging its exact byte length), with the prelude building `ctx.headers` as per-key getters (`__velquMakeLazyHeaders`). Undeclared headers do not exist as keys.
- Changed files:
  - `crates/q-runtime/src/serve.rs` (declared-only header copy by plan ids; `materialize_headers` import removed)
  - `crates/q-engine-quickjs/src/worker.rs` (`__velquReqHeaderNames`, `__velquReqHeader` with exact byte accounting)
  - `crates/q-engine-quickjs/src/prelude.rs` (`__velquMakeLazyHeaders` per-key getters)
  - `crates/q-engine-quickjs/tests/engine.rs` (`headers.lazy` handler + declared-only/per-key proof)
  - `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-005-B-read-header-values-by-id-on-demand.md`, `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`
- Tests: new `headers_are_declared_only_and_per_key_lazy` — request carries only `authorization`; handler sees `keys: "authorization"`, `"content-type" in headers === false`, one value access = 1 materialized field / 14 bytes, slot settles to 0. Route-declares-no-headers copies none (existing `field_free_invocation_skips_request_store_slot`, `lazy_ctx_touches_nothing` — 0 host calls). Auth flow end-to-end: runtime policy conformance (401 without header / 200 with) and full HTTP suites pass.
- Verification: `cargo test -p q-pack` PASS (35 + 2 fuzz); `cargo test -p q-engine-quickjs` PASS (1 + 92); `cargo test -p q-http` PASS (2 + 3); `cargo test -p q-bridge` PASS (9); `cargo test -p q-schema-runtime` PASS; `cargo test -p q-router` PASS (15); `cargo test -p velqu-runtime` PASS (13); `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS; `bun run typecheck` PASS; `bun test` PASS (35/0 after proof-pack + release-binary rebuild). Raw logs: `/tmp/m24-005-b-rust.log`, `/tmp/m24-005-b-bun.log`.
- Guardrail status: duplicate-header joining and the explicit full-Headers escape hatch are M24-005-C/D; secret-header redaction in diagnostics is unchanged from the existing redaction rules (header values never logged — SEC-004).
- Next dependency-ready task: M24-005-C (define duplicate header behavior and byte/string conversion).

