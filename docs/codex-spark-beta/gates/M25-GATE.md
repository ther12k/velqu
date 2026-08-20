---
task_id: M25-GATE
parent_task: M25-GATE
milestone: M25
priority: P0
mode: GATE_REVIEW
status: TODO
context_card: context/milestones/M25.md
commit_required: true
---

# M25-GATE — M2.5 — Schema-Specialized Input and JSON Output Pipeline exit gate

## Atomic goal

Review and decide the M25 exit gate from source, tests, and evidence.

## Parent intent

Schema-Specialized Validation and JSON Codecs

## Dependencies

- `M25-001-Z` — `tasks/02_m25_schema_codecs/M25-001-Z-package-evidence-for-define-canonical-schema-ir-v2.md`
- `M25-002-Z` — `tasks/02_m25_schema_codecs/M25-002-Z-package-evidence-for-build-reproducible-decoder-encoder-strategy-benchmark.md`
- `M25-003-Z` — `tasks/02_m25_schema_codecs/M25-003-Z-package-evidence-for-generate-params-query-header-decoders.md`
- `M25-004-Z` — `tasks/02_m25_schema_codecs/M25-004-Z-package-evidence-for-generate-json-body-decoders.md`
- `M25-005-Z` — `tasks/02_m25_schema_codecs/M25-005-Z-package-evidence-for-generate-status-specific-response-encoders.md`
- `M25-006-Z` — `tasks/02_m25_schema_codecs/M25-006-Z-package-evidence-for-generate-rfc-9457-problem-encoders.md`
- `M25-007-Z` — `tasks/02_m25_schema_codecs/M25-007-Z-package-evidence-for-implement-explicit-generic-and-web-fallback-paths.md`
- `M25-008-Z` — `tasks/02_m25_schema_codecs/M25-008-Z-package-evidence-for-unify-treaty-openapi-lock-and-runtime-schema-projection.md`
- `M25-009-Z` — `tasks/02_m25_schema_codecs/M25-009-Z-package-evidence-for-add-codec-fuzzing-and-differential-tests.md`
- `M25-010-Z` — `tasks/02_m25_schema_codecs/M25-010-Z-package-evidence-for-close-codec-performance-and-cold-start-evidence.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M25.md`
- `context/components/engine-scheduler.md`
- `context/components/schema-codecs.md`

### Source files

- `AGENTS.md`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/convert.rs`
- `packages/schema/src/index.ts`
- `packages/compiler/src/emit.ts`
- `crates/q-schema-runtime/tests/fuzz_validator.rs`
- `conformance/schema/schema.conformance.test.ts`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Freeze the candidate commit and confirm a clean working tree.
2. Review every dependency evidence packet against source and test reality.
3. Run the full verification commands from the exact candidate commit.
4. Check raw-to-report parity, index commit hashes, artifact hashes, and unresolved P0/P1 findings.
5. If any criterion fails, keep the gate IN_PROGRESS and list the exact blocking task; do not patch silently inside the gate review.
6. If all criteria pass, update the gate status and produce the milestone review packet, source archive, Git bundle, and checksum manifest.

## Parent acceptance guardrails

- Every parent task for M25 has a passing verification and evidence packet.
- Full project verification passes from a clean tree.
- Evidence indexes identify the exact commit and artifacts.
- No unresolved P0/P1 criterion is hidden or waived without owner/reviewer approval.

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

- Milestone report.
- Review index.
- Evidence index.
- Commit-named source archive and Git bundle.
- SHA-256 manifest.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Implementing missing milestone work inside the gate task.
- Waiving P0/P1 without explicit owner/reviewer approval.
- Calling a single benchmark run canonical when repeated evidence is required.

## Commit guidance

Suggested subject:

```text
m25-gate: m2 5 schema specialized input and json output pipeline exit
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
