---
task_id: M28-GATE
parent_task: M28-GATE
milestone: M28
priority: P0
mode: GATE_REVIEW
status: PASS
context_card: context/milestones/M28.md
commit_required: true
---

# M28-GATE — M2.8 — Native Outbound Fetch exit gate

## Atomic goal

Review and decide the M28 exit gate from source, tests, and evidence.

## Parent intent

Native Outbound Fetch

## Dependencies

- `M28-001-Z` — `tasks/05_m28_native_fetch/M28-001-Z-package-evidence-for-accept-fetch-tls-redirect-and-ssrf-security-adr.md`
- `M28-002-Z` — `tasks/05_m28_native_fetch/M28-002-Z-package-evidence-for-select-native-http-client-stack-from-evidence.md`
- `M28-003-Z` — `tasks/05_m28_native_fetch/M28-003-Z-package-evidence-for-implement-connection-pooling-dns-and-tls.md`
- `M28-004-Z` — `tasks/05_m28_native_fetch/M28-004-Z-package-evidence-for-implement-request-response-and-headers-subset.md`
- `M28-005-Z` — `tasks/05_m28_native_fetch/M28-005-Z-package-evidence-for-propagate-abortsignal-and-route-deadlines.md`
- `M28-006-Z` — `tasks/05_m28_native_fetch/M28-006-Z-package-evidence-for-implement-streaming-and-strict-backpressure.md`
- `M28-007-Z` — `tasks/05_m28_native_fetch/M28-007-Z-package-evidence-for-implement-redirect-and-compression-policy.md`
- `M28-008-Z` — `tasks/05_m28_native_fetch/M28-008-Z-package-evidence-for-implement-ssrf-and-network-egress-controls.md`
- `M28-009-Z` — `tasks/05_m28_native_fetch/M28-009-Z-package-evidence-for-integrate-lifecycle-observability-and-shutdown.md`
- `M28-010-Z` — `tasks/05_m28_native_fetch/M28-010-Z-package-evidence-for-complete-fetch-conformance-and-fault-testing.md`
- `M28-011-Z` — `tasks/05_m28_native_fetch/M28-011-Z-package-evidence-for-run-controlled-upstream-and-fan-out-benchmarks.md`

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

1. Freeze the candidate commit and confirm a clean working tree.
2. Review every dependency evidence packet against source and test reality.
3. Run the full verification commands from the exact candidate commit.
4. Check raw-to-report parity, index commit hashes, artifact hashes, and unresolved P0/P1 findings.
5. If any criterion fails, keep the gate IN_PROGRESS and list the exact blocking task; do not patch silently inside the gate review.
6. If all criteria pass, update the gate status and produce the milestone review packet, source archive, Git bundle, and checksum manifest.

## Parent acceptance guardrails

- Every parent task for M28 has a passing verification and evidence packet.
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
cargo test -p q-capabilities
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
m28-gate: m2 8 native outbound fetch exit gate
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Result (M28-GATE) — PASS

- Date: 2026-08-30
- Branch/PR: m28-gate (squash-merged; see git log for final hash)
- Candidate commit: 1cd63fc (clean tree at gate time)

### Gate decision: PASS
- All 11 parents (M28-001..M28-011) have V+Z packets PASS; ledger row updated.
- Full verification from the clean candidate commit: ./scripts/verify exit 0 (Rust: q-capabilities 192+7+1+4+9, q-engine-quickjs 18+101, velqu-runtime 12+5+44, q-http 4+6+1, q-bridge 11; TypeScript 219; fmt/clippy -D warnings clean; benchmark evidence current; binary b82960604ddf390d matches manifest).
- Review packet: docs/reports/m28-gate-review.md; indexes EVIDENCE_INDEX.json / REVIEW_INDEX.json updated to the M28 checkpoint (commit rewritten by scripts/release-packet at release time).
- Milestone report, source archive, Git bundle, and SHA-256 manifest produced via scripts/release-packet (release/ artifacts are untracked by design).
- No unresolved P0/P1 findings hidden or waived: the PACK_FORMAT_CURRENT owner decision remains tracked in REVIEW_INDEX openItems (carried from M26); the fastify/Node-fetch c=200 tail finding is a measured candidate limitation disclosed in M28-011-V, not a velqu-path defect.
