---
task_id: G0-GATE
parent_task: G0-GATE
milestone: G0
priority: P0
mode: GATE_REVIEW
status: PASS
context_card: context/milestones/G0.md
commit_required: true
---

# G0-GATE — M23R2 Gate Closure — Trusted Numeric Artifact and Router exit gate

## Atomic goal

Review and decide the G0 exit gate from source, tests, and evidence.

## Parent intent

Trusted Numeric Graph and Evidence Truth

## Dependencies

- `G0-001-Z` — `tasks/00_g0_gate_close/G0-001-Z-package-evidence-for-freeze-and-reconcile-the-4e69049-release-baseline.md`
- `G0-002-Z` — `tasks/00_g0_gate_close/G0-002-Z-package-evidence-for-make-the-semantic-function-manifest-mandatory.md`
- `G0-003-Z` — `tasks/00_g0_gate_close/G0-003-Z-package-evidence-for-bind-router-and-schema-manifests-into-the-execution-graph-h.md`
- `G0-004-Z` — `tasks/00_g0_gate_close/G0-004-Z-package-evidence-for-load-the-serialized-router-directly.md`
- `G0-005-Z` — `tasks/00_g0_gate_close/G0-005-Z-package-evidence-for-complete-operational-routeid-policyid-and-schemaid-usage.md`
- `G0-006-Z` — `tasks/00_g0_gate_close/G0-006-Z-package-evidence-for-separate-and-verify-public-contract-identity.md`
- `G0-007-Z` — `tasks/00_g0_gate_close/G0-007-Z-package-evidence-for-remove-duplicate-legacy-state-from-current-packs.md`
- `G0-008-Z` — `tasks/00_g0_gate_close/G0-008-Z-package-evidence-for-close-canonical-performance-evidence.md`
- `G0-009-Z` — `tasks/00_g0_gate_close/G0-009-Z-package-evidence-for-create-self-verifying-milestone-and-evidence-indexes.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/G0.md`
- `context/components/qpack-router.md`
- `context/components/engine-scheduler.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-engine-quickjs/src/worker.rs`
- `packages/compiler/src/emit.ts`
- `conformance/routing/routing.conformance.test.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `docs/specs/pack-format-v1.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Freeze the candidate commit and confirm a clean working tree.
2. Review every dependency evidence packet against source and test reality.
3. Run the full verification commands from the exact candidate commit.
4. Check raw-to-report parity, index commit hashes, artifact hashes, allocator/startup profile status, and unresolved P0/P1 findings.
5. If any criterion fails, keep the gate IN_PROGRESS and list the exact blocking task; do not patch silently inside the gate review.
6. If all criteria pass, update the gate status and produce the milestone review packet, source archive, Git bundle, benchmark manifest, and checksum manifest.

## Parent acceptance guardrails

- Every parent task for G0 has a passing verification and evidence packet.
- Full project verification passes from a clean tree.
- Evidence indexes identify the exact commit and artifacts.
- No unresolved P0/P1 criterion is hidden or waived without owner/reviewer approval.

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
```bash
./scripts/validate-production-plan
```
```bash
./scripts/validate-okf
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
g0-gate: m23r2 gate closure trusted numeric artifact and router exit
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Decision

**PASS at evidence checkpoint `03cc48955c2f8b05c29cf6ca196572c67ed5dd2d`.**

- Numeric semantic manifests, serialized-router verification, RouteId/PolicyId/SchemaId operation, public/execution hash separation, and handler-table rejection are proven by the Rust/compiler tests listed in the root review index.
- Warm evidence contains five randomized repetitions and 240 cells; cold and route-count evidence contain the required fresh-process samples with zero failures.
- Startup and allocator evidence is captured in `benchmarks/raw/profiles/startup-10000.json` and `startup-10000.alloc.json`; Linux perf counters remain unavailable (`perf_event_paranoid=4`).
- Reports are generated and checked from raw evidence; `benchmarks/manifest.json` records run IDs, artifact hashes, and environment.
- The release packet binds the exact commit and passes `sha256sum -c`.

Next dependency-ready work is M24; no M24–BETA implementation packet is marked complete by this gate.
