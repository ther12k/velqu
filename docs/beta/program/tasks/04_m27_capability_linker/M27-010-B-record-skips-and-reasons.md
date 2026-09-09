---
task_id: M27-010-B
parent_task: M27-010
milestone: M27
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-010-B — Record skips and reasons

## Atomic goal

Record skips and reasons.

## Parent intent

Separate standards compatibility from internal framework tests.

## Dependencies

- `M27-010-A` — `tasks/04_m27_capability_linker/M27-010-A-pin-wpt-wintertc-subsets.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M27.md`
- `context/components/engine-scheduler.md`
- `context/components/capabilities-fetch.md`

### Source files

- `AGENTS.md`
- `crates/q-capabilities/src/lib.rs`
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Record skips and reasons.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- No unsupported API is advertised.
- Pass/fail/skip counts are reproducible.
- Behavioral regressions block relevant gate.
- Reports link to exact runtime build.

## Targeted commands

```bash
cargo test -p q-engine-quickjs
```
```bash
cargo test -p q-capabilities
```
```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Conformance report.
- Pinned test manifest.
- CI output.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
m27-010-b: record skips and reasons
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M27-010-B) — PASS

- Date: 2026-08-27
- Branch/PR: m27-010-b (squash-merged; see git log for final hash)
- Closes: #295

### Changed files
- `conformance/web-api/wpt-manifest.json`: Added structured `explicitSkips` array per capability with standard references, machine-readable reason codes (`BROWSER_ONLY_FEATURE`, `POSIX_RUNTIME_TARGET`, `WINTERTC_UTF8_ONLY`, `STREAMING_DEFERRED`, `ASYNC_COMBINATOR_DEFERRED`, `MINIMAL_EVENT_TARGET`, `UNSUPPORTED_CRYPTO_SUBTLE`, `SPEC_MANDATED_TYPE_ERROR`), detailed rationale, and deferred milestone targets.
- `conformance/web-api/web-api.conformance.test.ts`: Added automated verification asserting all 8 explicit skips are declared with valid standard references and reasons.
- `docs/reports/m27-010-wpt-wintertc-conformance.md`: Added dedicated Explicit Skips & Rationale section documenting skip inventory, reason codes, and deferred roadmap targets.

### Explicit Skips Breakdown (8 total)
1. `wpt-url-blob-scheme` — `BROWSER_ONLY_FEATURE` (OUT_OF_SCOPE: server-side runtime lacks Blob store)
2. `wpt-url-file-windows-drive` — `POSIX_RUNTIME_TARGET` (OUT_OF_SCOPE: Linux/POSIX server scope)
3. `wpt-encoding-legacy-labels` — `WINTERTC_UTF8_ONLY` (OUT_OF_SCOPE: WinterTC minimal profile specifies UTF-8 only)
4. `wpt-encoding-streaming` — `STREAMING_DEFERRED` (POST_M27: chunked streaming decoders deferred)
5. `wpt-abort-signal-any` — `ASYNC_COMBINATOR_DEFERRED` (M28: multi-signal composition deferred to M28 native fetch)
6. `wpt-abort-event-bubbling` — `MINIMAL_EVENT_TARGET` (OUT_OF_SCOPE: simple single-target dispatch only)
7. `wpt-crypto-subtle` — `UNSUPPORTED_CRYPTO_SUBTLE` (GA_TRACK: ADR-0018 / M27-008-D no complex crypto)
8. `wpt-crypto-float-typedarray` — `SPEC_MANDATED_TYPE_ERROR` (OUT_OF_SCOPE: spec mandates TypeError)

### Command results
- `cargo test -p q-capabilities` → 107 unit tests + 7 integration tests passed
- `cargo test -p q-engine-quickjs` → 14+97 passed
- `cargo test -p q-pack` → 96+2 passed
- `cargo test -p velqu-runtime` → 31 passed
- `bun test` → 212 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
