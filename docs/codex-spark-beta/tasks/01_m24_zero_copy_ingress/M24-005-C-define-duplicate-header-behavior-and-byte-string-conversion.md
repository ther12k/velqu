---
task_id: M24-005-C
parent_task: M24-005
milestone: M24
priority: P0
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-005-C — Define duplicate header behavior and byte/string conversion

## Atomic goal

Define duplicate header behavior and byte/string conversion.

## Parent intent

Expose only headers declared by route or policy without cloning the entire HeaderMap.

## Dependencies

- `M24-005-B` — `tasks/01_m24_zero_copy_ingress/M24-005-B-read-header-values-by-id-on-demand.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M24.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `crates/q-http/src/lib.rs`
- `crates/q-bridge/src/lib.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`
- `package.json`
- `.github/workflows/verify.yml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Define duplicate header behavior and byte/string conversion.
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
```bash
bun test
```
```bash
bun run typecheck
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
m24-005-c: define duplicate header behavior and byte string conversion
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Completion record

- Status: **PASS**
- Deliverable: the duplicate-header and byte/string conversion contract, frozen and tested. `q_http::declared_header_value(map, name)` is the single implementation of the contract: repeated values for a declared name join with `", "` in arrival order (HTTP list semantics); header bytes convert lossily to UTF-8 (invalid sequences become U+FFFD — never a panic, never a rejection); a declared-but-absent name yields `None` and is omitted. `serve.rs` admission now reads every plan-declared header through this helper (replacing the first-value `headers.get` lookup M24-005-B used), so policy/auth header access observes exactly this contract.
- Changed files:
  - `crates/q-http/src/lib.rs` (`declared_header_value` + duplicate/non-UTF8/absent contract tests)
  - `crates/q-runtime/src/serve.rs` (declared-header copy uses the contract helper)
  - `docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/M24-005-C-define-duplicate-header-behavior-and-byte-string-conversion.md`, `docs/codex-spark-beta/STATUS.md`, `docs/codex-spark-beta/indexes/TASK_INDEX.md`
- Tests: new `declared_header_value_joins_duplicates_and_is_lossy` (q-http: duplicate `authorization` values join in arrival order; declared-absent → None; `[0x41,0xff,0xfe,0x42]` converts lossily with U+FFFD and no panic). Auth flow end-to-end unchanged: runtime policy conformance (401/200) and all suites green.
- Verification: `cargo test -p q-pack` PASS (35 + 2 fuzz); `cargo test -p q-engine-quickjs` PASS (1 + 92); `cargo test -p q-http` PASS (3 unit + 3 parser fuzz); `cargo test -p q-bridge` PASS (9); `cargo test -p q-schema-runtime` PASS; `cargo test -p q-router` PASS (15); `cargo test -p velqu-runtime` PASS (13 — `graceful_shutdown_exits_zero` flaked once under full parallel matrix load, same known SIGTERM-race flake recorded in M24-002-Z; isolated rerun and full-suite rerun both pass); `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets -- -D warnings` PASS; `bun run typecheck` PASS; `bun test` PASS (35/0). Raw logs: `/tmp/m24-005-c-rust.log`, `/tmp/m24-005-c-bun.log`.
- Guardrail status: secret-header redaction in diagnostics unchanged (header values never logged — SEC-004); the explicit full-Headers escape hatch is M24-005-D.
- Next dependency-ready task: M24-005-D (keep full Headers escape hatch explicit and costed).

