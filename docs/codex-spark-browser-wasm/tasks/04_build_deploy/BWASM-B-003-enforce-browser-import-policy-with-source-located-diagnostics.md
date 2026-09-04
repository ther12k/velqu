Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/04_build_deploy/BWASM-B-003-enforce-browser-import-policy-with-source-located-diagnostics.md`  
Program: `BWASM`  
Phase: `04_build_deploy` — Compiler, artifacts, and static deployment  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-B-003 — Enforce browser import policy with source-located diagnostics

## Atomic goal

Prevent server-only or unsafe dependencies from entering browser-WASM builds.

## Parent intent

Turn the runtime into deterministic static artifacts that can be built, inspected, updated, rolled back, and hosted generically.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-B-001` — Add compiler target browser-wasm
- `BWASM-D-003` — Define the browser execution threat model and isolation contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/compiler/src/index.ts`
- `packages/compiler/src/extract.ts`
- `packages/compiler/src/emit.ts`
- `packages/cli/src/index.ts`
- `packages/browser-runtime/`
- `.github/workflows/verify.yml`

## Steps

1. Define allow/deny rules for node:*, Bun.*, native addons, dynamic code loading, filesystem, raw sockets, child processes, and unsupported globals.
2. Resolve aliases and re-exports so forbidden imports cannot hide behind a local wrapper.
3. Classify imports as browser-safe, simulated, deployment-required, or forbidden.
4. Produce diagnostic codes, source ranges, remediation text, and docs links.
5. Add an explicit escape hatch only if owner-approved and auditable; default must fail closed.

## Acceptance criteria

- [ ] Direct, transitive, aliased, re-exported, and dynamic forbidden imports are caught.
- [ ] False positives for approved browser-safe packages are fixture-tested.
- [ ] Diagnostics identify the import chain and suggested capability alternative.
- [ ] No unresolved dependency is silently externalized into production preview.
- [ ] Policy version is recorded in the build manifest.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Compiler policy corpus.
- Transitive/alias negative fixtures.
- Browser-safe positive fixtures.
- Bundle post-scan.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Import policy document.
- [ ] Diagnostic snapshots.
- [ ] Positive/negative fixture results.
- [ ] Final bundle audit.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Attempting to polyfill arbitrary Node APIs.
- Relying solely on a string grep of source files.
- Allowing remote imports without integrity and policy.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-b-003:`.
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
