---
task_id: M26-GATE
parent_task: M26-GATE
milestone: M26
priority: P0
mode: GATE_REVIEW
status: PASS
context_card: context/milestones/M26.md
commit_required: true
---

# M26-GATE — M2.6 — Binary QPack v2 and Reproducible Artifact ABI exit gate

## Atomic goal

Review and decide the M26 exit gate from source, tests, and evidence.

## Parent intent

Binary QPack v2 and Reproducible Artifact ABI

## Dependencies

- `M26-001-Z` — `tasks/03_m26_qpack_v2/M26-001-Z-package-evidence-for-accept-qpack-v2-format-and-compatibility-adr.md`
- `M26-002-Z` — `tasks/03_m26_qpack_v2/M26-002-Z-package-evidence-for-define-strict-runtime-and-bytecode-fingerprint.md`
- `M26-003-Z` — `tasks/03_m26_qpack_v2/M26-003-Z-package-evidence-for-encode-compiled-router-routeplans-schemas-policies-and-func.md`
- `M26-004-Z` — `tasks/03_m26_qpack_v2/M26-004-Z-package-evidence-for-embed-raw-quickjs-bytecode-without-base64.md`
- `M26-005-Z` — `tasks/03_m26_qpack_v2/M26-005-Z-package-evidence-for-implement-zero-copy-or-bounded-copy-pack-reader.md`
- `M26-006-Z` — `tasks/03_m26_qpack_v2/M26-006-Z-package-evidence-for-implement-execution-integrity-and-authenticity-hooks.md`
- `M26-007-Z` — `tasks/03_m26_qpack_v2/M26-007-Z-package-evidence-for-guarantee-reproducible-release-packs.md`
- `M26-008-Z` — `tasks/03_m26_qpack_v2/M26-008-Z-package-evidence-for-provide-explicit-v1-compatibility-and-migration-tool.md`
- `M26-009-Z` — `tasks/03_m26_qpack_v2/M26-009-Z-package-evidence-for-build-shared-runtime-and-standalone-deployment-artifacts.md`
- `M26-010-Z` — `tasks/03_m26_qpack_v2/M26-010-Z-package-evidence-for-close-route-count-cold-start-evidence.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/M26.md`
- `context/components/qpack-router.md`

### Source files

- `AGENTS.md`
- `crates/q-pack/src/lib.rs`
- `crates/q-bytecode-tool/src/main.rs`
- `crates/q-runtime/src/main.rs`
- `packages/compiler/src/emit.ts`
- `crates/q-pack/tests/fuzz_pack.rs`
- `docs/specs/pack-format-v1.md`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Freeze the candidate commit and confirm a clean working tree.
2. Review every dependency evidence packet against source and test reality.
3. Run the full verification commands from the exact candidate commit.
4. Check raw-to-report parity, index commit hashes, artifact hashes, and unresolved P0/P1 findings.
5. If any criterion fails, keep the gate IN_PROGRESS and list the exact blocking task; do not patch silently inside the gate review.
6. If all criteria pass, update the gate status and produce the milestone review packet, source archive, Git bundle, and checksum manifest.

## Parent acceptance guardrails

- Every parent task for M26 has a passing verification and evidence packet.
- Full project verification passes from a clean tree.
- Evidence indexes identify the exact commit and artifacts.
- No unresolved P0/P1 criterion is hidden or waived without owner/reviewer approval.

## Targeted commands

```bash
cargo test -p q-pack
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
m26-gate: m2 6 binary qpack v2 and reproducible artifact abi exit gate
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.

## Gate decision record (M26-GATE)

Decision: **PASS** — M2.6 (Binary QPack v2 and Reproducible
Artifact ABI) exits the gate at candidate commit `9aa16be` (reviewed
implementation) with the release packet binding this gate commit.

### Dependency closure

`M26-001-Z` … `M26-010-Z` all **PASS** (ledger rows M26-001…M26-010
PASS; merged commits a61e6c4, 70ada0d, 6d3860c, 742c5d1, 5f55113,
36419e4, a548568, 3f7fd2c, bcae745, 0f2eb01). Each parent's V packet
mapped its acceptance criteria to source and named tests; each Z
packet packaged the raw evidence.

### Verification (fresh, candidate commit 9aa16be)

- `cargo test -p q-pack` 96 · `-p velqu-runtime` 30 — pass.
  `bun test` 125 passed / 0 failed. `cargo fmt --check`, clippy
  `-D warnings`, `bun run typecheck` — clean.
- `./scripts/verify` — ALL PASS, exit 0 (fmt, clippy, workspace
  tests, release builds, typecheck, proof build, conformance, OKF,
  benchmark-evidence validation).

### Parity re-check (fresh at this gate)

- Route-count: 20/20 cells × 5 metrics recomputed exactly from the
  2,000-row canonical JSONL (nearest-rank); zero failures; 1,000
  stage-trace rows.
- Manifest: 12/12 artifact hashes match files. Summary: 10/10 pack
  hashes match. The summary binary hash self-identifies the
  benchmark-run build (documented division; manifest stays the
  canonical release record).

### Carried-forward limitation (disclosed, not waived)

`PACK_FORMAT_CURRENT` stays pinned to v1: the production default
path still parses JSON packs, so the M25-escalated cold-start
limitation is unmitigated in the default path (canonical ladder:
25-route p50 6.143 ms preserved; 10,000-route p50 926.3 ms,
pack.load ~97%). ADR-0024 rule 2 anticipated the flip with M26-003;
the adapter landed, the flip did not, and no M26 packet required
it. Resolution needs an owner-authorized follow-up packet; recorded
in REVIEW_INDEX/EVIDENCE_INDEX open items and
`docs/reports/m26-gate-review.md`. Not hidden, not waived by this
gate.

### Release packet

Generated by `scripts/release-packet` from a clean tree at this
commit: `SOURCE-COMMIT.txt`, Git bundle, source archive,
`BENCHMARK_MANIFEST.json`, indexes, `SHA256SUMS.txt`
(`sha256sum -c release/SHA256SUMS.txt` passes).
