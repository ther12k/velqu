Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-008-assemble-release-evidence-sbom-checksums-provenance-and-candidate-packet.md`  
Program: `BWASM`  
Phase: `06_quality_release` — Conformance, security, DevEx, and release qualification  
Mode: `EVIDENCE` — Package evidence from one exact candidate; do not mix implementation and attestation.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-Q-008 — Assemble release evidence, SBOM, checksums, provenance, and candidate packet

## Atomic goal

Bind all Browser-WASM release claims, bytes, verification, and residual risks to one exact candidate.

## Parent intent

Prove cross-target semantics, browser support, security, performance, documentation, and clean external usability.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-Q-001` — Build shared native-versus-browser conformance and differential suites
- `BWASM-Q-007` — Run an external cleanroom static deployment and offline exercise
- `BWASM-K-006` — Verify and package portable-kernel evidence
- `BWASM-R-006` — Verify and package browser-runtime evidence
- `BWASM-B-006` — Verify cache activation, upgrades, rollback, and static deployment

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `scripts/verify`
- `.github/workflows/verify.yml`
- `conformance/`
- `packages/browser-runtime/`
- `crates/q-browser-kernel/`
- `docs/`

## Steps

1. Freeze exact source commit, lockfiles, toolchains, packages, native comparator, WASM artifacts, handler bundles, manifests, and docs.
2. Generate inventory, SHA-256 checksums, package/WASM SBOMs, license report, and available provenance attestations.
3. Run the complete required matrix against candidate bytes.
4. Collect design decisions, conformance, security, performance, browser, cleanroom, upgrade/rollback, and docs evidence.
5. Publish a machine-readable candidate index with claim-to-evidence mapping.
6. Record all open P0/P1 and accepted residual risks without silently waiving them.

## Acceptance criteria

- [ ] Every release claim maps to evidence produced from the exact candidate.
- [ ] All distributed files appear in inventory, checksums, and applicable SBOM/provenance records.
- [ ] Rebuilding/verifying from the candidate instructions reproduces accepted artifacts or documented deterministic digests.
- [ ] No evidence references a different commit or locally altered bytes.
- [ ] P0 blockers make the packet NO-GO automatically.
- [ ] The packet is sufficient for an independent gate reviewer to decide without private context.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Full candidate battery.
- Checksum verification.
- SBOM/license scan.
- Clean artifact re-install.
- Evidence-link and exact-SHA validator.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Candidate index.
- [ ] Checksums/SBOM/provenance.
- [ ] All raw and summarized reports.
- [ ] Open-risk register.
- [ ] Reproduction transcript.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Creating evidence before final candidate bytes.
- Using passing logs from older commits.
- Publishing a green summary while hiding failed required lanes.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-q-008:`.
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
```
