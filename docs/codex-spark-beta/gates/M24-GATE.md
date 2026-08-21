---
task_id: M24-GATE
parent_task: M24-GATE
milestone: M24
priority: P0
mode: GATE_REVIEW
status: PASS
context_card: context/milestones/M24.md
commit_required: true
---

# M24-GATE — M2.4 — Zero-Copy Ingress and Worker-Local Request Bridge exit gate

## Atomic goal

Review and decide the M24 exit gate from source, tests, and evidence.

## Parent intent

Zero-Copy Ingress and Worker-Local Request Bridge

## Dependencies

- `M24-001-Z` — `tasks/01_m24_zero_copy_ingress/M24-001-Z-package-evidence-for-freeze-ingress-ownership-and-backpressure-design.md`
- `M24-002-Z` — `tasks/01_m24_zero_copy_ingress/M24-002-Z-package-evidence-for-route-before-request-materialization.md`
- `M24-003-Z` — `tasks/01_m24_zero_copy_ingress/M24-003-Z-package-evidence-for-implement-worker-local-generation-checked-request-slab.md`
- `M24-004-Z` — `tasks/01_m24_zero_copy_ingress/M24-004-Z-package-evidence-for-capture-path-parameters-as-byte-ranges.md`
- `M24-005-Z` — `tasks/01_m24_zero_copy_ingress/M24-005-Z-package-evidence-for-implement-declared-header-lazy-access.md`
- `M24-006-Z` — `tasks/01_m24_zero_copy_ingress/M24-006-Z-package-evidence-for-implement-lazy-query-and-cookie-decoding.md`
- `M24-007-Z` — `tasks/01_m24_zero_copy_ingress/M24-007-Z-package-evidence-for-implement-bounded-read-once-body-admission.md`
- `M24-008-Z` — `tasks/01_m24_zero_copy_ingress/M24-008-Z-package-evidence-for-replace-per-request-js-closures-with-native-backed-prototyp.md`
- `M24-009-Z` — `tasks/01_m24_zero_copy_ingress/M24-009-Z-package-evidence-for-add-ingress-and-bridge-observability.md`
- `M24-010-Z` — `tasks/01_m24_zero_copy_ingress/M24-010-Z-package-evidence-for-complete-ingress-bridge-fuzzing-and-conformance.md`

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
- `crates/q-engine-quickjs/src/prelude.rs`
- `crates/q-engine-quickjs/tests/engine.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-runtime/tests/runtime_conformance.rs`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Freeze the candidate commit and confirm a clean working tree.
2. Review every dependency evidence packet against source and test reality.
3. Run the full verification commands from the exact candidate commit.
4. Check raw-to-report parity, index commit hashes, artifact hashes, and unresolved P0/P1 findings.
5. If any criterion fails, keep the gate IN_PROGRESS and list the exact blocking task; do not patch silently inside the gate review.
6. If all criteria pass, update the gate status and produce the milestone review packet, source archive, Git bundle, and checksum manifest.

## Parent acceptance guardrails

- Every parent task for M24 has a passing verification and evidence packet.
- Full project verification passes from a clean tree.
- Evidence indexes identify the exact commit and artifacts.
- No unresolved P0/P1 criterion is hidden or waived without owner/reviewer approval.

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
m24-gate: m2 4 zero copy ingress and worker local request bridge exit
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Gate review record

- Candidate commit: `75bda51f6872524d363a0f5eadc2460cf6bc8678`; clean working tree before packaging.
- All dependency packages `M24-001-Z` through `M24-010-Z`: `PASS`.
- `./scripts/verify`: PASS; all Rust, TypeScript, OKF, report, and benchmark artifact checks passed.
- Required release artifacts generated and checksum-verified: source ZIP, Git bundle, `SOURCE-COMMIT.txt`, `REVIEW_INDEX.json`, `EVIDENCE_INDEX.json`, `BENCHMARK_MANIFEST.json`, and `SHA256SUMS.txt`.
- Decision: M24 gate PASS; no unresolved P0/P1 criterion.
