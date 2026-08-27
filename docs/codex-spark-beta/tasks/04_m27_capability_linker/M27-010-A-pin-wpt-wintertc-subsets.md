---
task_id: M27-010-A
parent_task: M27-010
milestone: M27
priority: P1
mode: IMPLEMENT
status: PASS
context_card: context/milestones/M27.md
commit_required: true
---

# M27-010-A — Pin WPT/WinterTC subsets

## Atomic goal

Pin WPT/WinterTC subsets.

## Parent intent

Separate standards compatibility from internal framework tests.

## Dependencies

- `M27-005-Z` — `tasks/04_m27_capability_linker/M27-005-Z-package-evidence-for-implement-url-and-urlsearchparams.md`
- `M27-006-Z` — `tasks/04_m27_capability_linker/M27-006-Z-package-evidence-for-implement-textencoder-and-textdecoder.md`
- `M27-007-Z` — `tasks/04_m27_capability_linker/M27-007-Z-package-evidence-for-implement-abortcontroller-and-abortsignal.md`
- `M27-008-Z` — `tasks/04_m27_capability_linker/M27-008-Z-package-evidence-for-implement-crypto-random-subset.md`

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
5. Implement exactly this deliverable: Pin WPT/WinterTC subsets.
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
m27-010-a: pin wpt wintertc subsets
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M27-010-A) — PASS

- Date: 2026-08-27
- Branch/PR: m27-010-a (squash-merged; see git log for final hash)
- Closes: #294

### Changed files
- `conformance/web-api/wpt-manifest.json` (new): Pinned Web Platform Tests (WPT) and WinterTC Minimal Common Web Platform API test subset manifest declaring exact input/expected vectors across all four M27 Web API capabilities (`url`, `text_encoding`, `abort`, `crypto`).
- `conformance/web-api/web-api.conformance.test.ts` (new): Automated TypeScript standards conformance suite executing against the pinned manifest.
- `crates/q-capabilities/tests/wpt_wintertc_conformance.rs` (new): Rust integration test suite verifying native capability models against the pinned vectors.
- `docs/reports/m27-010-wpt-wintertc-conformance.md` (new): Web API conformance baseline report detailing standards alignment, manifest vectors, test suites, and acceptance guardrails.

### Pinned Subsets & Vector Counts
- `url`: 15 vectors (relative URL resolution, origin/port/host normalization, IPv6, URLSearchParams mutations/sorting/iteration)
- `text_encoding`: 9 vectors (UTF-8 multi-byte encoding, decode BOM handling, replacement U+FFFD mode, fatal mode)
- `abort`: 4 vectors (AbortController abort reason, AbortSignal.abort, AbortSignal.timeout)
- `crypto`: 6 vectors (getRandomValues buffer types/quota, randomUUID RFC 4122 v4 pattern and uniqueness)
Total: 34 pinned vectors across 4 capabilities.

### Command results
- `cargo test -p q-capabilities` → 107 unit tests + 7 integration tests passed
- `cargo test -p q-engine-quickjs` → 14+97 passed
- `cargo test -p q-pack` → 96+2 passed
- `cargo test -p velqu-runtime` → 31 passed
- `bun test` → 211 pass / 0 fail (27 files)
- `bun run typecheck` → clean
- `cargo fmt --check` → clean
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `./scripts/verify` → **ALL PASS (exit 0)**

### Disclosures (standing)
- CI fails with zero executed steps on every PR since ~#714 (infrastructure-side); disclosed per PR. Local evidence above is complete.
