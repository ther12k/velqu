---
task_id: M28-010-A
parent_task: M28-010
milestone: M28
priority: P0
mode: IMPLEMENT
status: TODO
context_card: context/milestones/M28.md
commit_required: true
---

# M28-010-A — Run selected WPT cases

## Atomic goal

Run selected WPT cases.

## Parent intent

Prove the beta subset across success and failure modes.

## Dependencies

- `M28-004-Z` — `tasks/05_m28_native_fetch/M28-004-Z-package-evidence-for-implement-request-response-and-headers-subset.md`
- `M28-005-Z` — `tasks/05_m28_native_fetch/M28-005-Z-package-evidence-for-propagate-abortsignal-and-route-deadlines.md`
- `M28-006-Z` — `tasks/05_m28_native_fetch/M28-006-Z-package-evidence-for-implement-streaming-and-strict-backpressure.md`
- `M28-007-Z` — `tasks/05_m28_native_fetch/M28-007-Z-package-evidence-for-implement-redirect-and-compression-policy.md`
- `M28-008-Z` — `tasks/05_m28_native_fetch/M28-008-Z-package-evidence-for-implement-ssrf-and-network-egress-controls.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M28.md`
- `context/components/engine-scheduler.md`
- `context/components/ingress-bridge.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `crates/q-runtime/src/serve.rs`
- `crates/q-http/src/lib.rs`
- `Cargo.toml`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Run selected WPT cases.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Documented subset passes.
- No panic/hang/unbounded work.
- All failures map predictably.
- Skips are explicit.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-http
```
```bash
cargo test -p q-capabilities
```
```bash
cargo test -p velqu-runtime
```

## Required evidence for this microtask

- Conformance report.
- Fixture inventory.
- Fuzz report.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m28-010-a: run selected wpt cases
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
