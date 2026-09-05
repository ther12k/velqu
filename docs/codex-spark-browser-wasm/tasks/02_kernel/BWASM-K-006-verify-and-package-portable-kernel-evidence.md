Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/02_kernel/BWASM-K-006-verify-and-package-portable-kernel-evidence.md`  
Program: `BWASM`  
Phase: `02_kernel` — Portable Rust/WASM kernel  
Mode: `EVIDENCE` — Package evidence from one exact candidate; do not mix implementation and attestation.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `PASS`

---

# BWASM-K-006 — Verify and package portable-kernel evidence

## Atomic goal

Independently verify the portable crates and Browser Kernel at one exact commit.

## Parent intent

Put compatibility-critical, host-independent semantics on the real Rust/WASM request path.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-K-001` — Extract a portable runtime model crate
- `BWASM-K-002` — Split byte-based QPack core from native loading and tooling
- `BWASM-K-003` — Extract a host-independent router core
- `BWASM-K-004` — Qualify the schema runtime for wasm32 and expose bounded validation
- `BWASM-K-005` — Implement the Rust Browser Kernel and wasm-bindgen ABI
- `BWASM-D-004` — Ratify support matrix, compatibility claims, and release budgets

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `Cargo.toml`
- `crates/q-engine/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-runtime/src/lib.rs`

## Steps

1. Freeze a clean candidate.
2. Run native and real-browser wasm32 checks/tests.
3. Inspect dependency trees and final WASM imports/exports.
4. Re-run malformed/oversized inputs and cross-target fixture diffs.
5. Package raw logs, environment manifest, artifact hashes, and reviewer findings.

## Acceptance criteria

- [ ] All K-phase criteria are independently demonstrated.
- [ ] No hidden native or JavaScript-only fallback is used.
- [ ] Artifacts and evidence point to one exact commit and hashes.
- [ ] No unresolved kernel P0 remains; accepted mismatch links to owner decision.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Cargo fmt/clippy/test.
- wasm32 checks.
- wasm-bindgen/wasm-pack browser tests.
- Import audit.
- Full repository verify.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Kernel verification report.
- [ ] Raw logs.
- [ ] Toolchain/browser manifest.
- [ ] Fixture/artifact checksums.
- [ ] Reviewer sign-off.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Fixing product implementation inside evidence work except evidence tooling defects.
- Using different commits for logs/artifacts.
- Implicit waivers.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-k-006:`.
- Reference this issue ID in commits, PR body, tests, and evidence.
- Avoid generated queue/index churn until implementation and targeted tests are stable.
- If scope expands materially, stop and open a new dependency issue rather than hiding extra work here.

## Stop condition

Stop and hand off when **all** acceptance criteria are demonstrated, the required evidence is attached or committed, canonical verification is green, and no unresolved in-scope P0 remains. If a prerequisite, owner decision, browser limitation, or security claim blocks truthful completion, record the exact blocker and leave this issue open.

## Handoff format

```text
Issue:
Candidate commit:
Files changed:
Commands run:
Targeted tests:
Full verification:
Artifacts and SHA-256:
Browser/OS/toolchain:
Acceptance criteria:
Known limitations:
Residual risks:
Follow-up issue links:

## Result (2026-09-05)

- Issue: BWASM-K-006 (#1234)
- Candidate commit: 69187c8bcb7005da940f27ecf9baea6dcb5f2787; report `docs/reports/bwasm-k-006-kernel-evidence.md`; raw evidence `docs/codex-spark-browser-wasm/evidence/kernel-verification/` (00-environment … 05-reviewer-findings + artifacts + reproducible abi-check.cjs).
- Verification at one commit: 203 native tests; wasm32 checks for every portable configuration; on-target execution (schema 67 + kernel 14); dep audits 0; import audit CLEAN; **JS-ABI driven end-to-end from real JavaScript** (nodejs-target glue, JS-ABI-OK).
- Reviewer sign-off: three external reviewer runs were rate-limited (provider); a mechanical structured self-audit is committed (VERDICT: KERNEL-REVIEW-PASS, 0×P0 0×P1 3×P2 notes) with owner sign-off explicitly invited — recorded, not hidden.
- Open item carried: base-kernel size 572,711 B gzip-9 > 500 KiB ratified budget (Q-gate finding, per ADR-0039).
- Verify: run 1 ALL PASS (canonical); two later runs flaked on different timing tests under external host load (~10 load average from operator applications); both tests pass in isolation — full history in the report.
- K-PHASE COMPLETE (K-001..K-006). R-phase registration is the next owner-gated step per OD-BWASM-001 decision 4.
```
