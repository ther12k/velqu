Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/04_build_deploy/BWASM-B-001-add-compiler-target-browser-wasm.md`  
Program: `BWASM`  
Phase: `04_build_deploy` — Compiler, artifacts, and static deployment  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-B-001 — Add compiler target browser-wasm

## Atomic goal

Teach the Velqu compiler to emit a deterministic browser-WASM application artifact set.

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

- `BWASM-K-005` — Implement the Rust Browser Kernel and wasm-bindgen ABI
- `BWASM-R-003` — Define and emit the browser handler-bundle contract
- `BWASM-D-001` — Freeze the Browser-WASM product and runtime contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/compiler/src/index.ts`
- `packages/compiler/src/extract.ts`
- `packages/compiler/src/emit.ts`
- `packages/cli/src/index.ts`
- `packages/browser-runtime/`
- `.github/workflows/verify.yml`

## Steps

1. Add an explicit browser-wasm target rather than overloading native QPack behavior.
2. Emit kernel WASM, JavaScript loader/glue, handler bundle, route/schema/capability manifest, contract, and source maps.
3. Carry target and ABI versions in every manifest.
4. Reject unsupported dynamic route/schema/capability declarations with source-located diagnostics.
5. Keep the compiler host on supported native/Bun tooling for MVP; only emitted output must run in browsers.

## Acceptance criteria

- [ ] A documented command builds a sample project into a self-contained browser artifact directory.
- [ ] Output is deterministic modulo explicitly normalized metadata.
- [ ] Native target output is unchanged unless covered by migration tests.
- [ ] Unsupported application constructs fail at build time, not as blank browser failures.
- [ ] No development workspace path leaks into emitted artifacts.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Compiler golden fixtures.
- Native target regression suite.
- Two-build reproducibility test.
- Clean static artifact smoke.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Compiler target specification.
- [ ] Artifact tree and hashes.
- [ ] Diagnostic snapshots.
- [ ] Native regression results.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Running the compiler itself in the browser.
- Replacing native QPack.
- Supporting arbitrary Node/Bun modules.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-b-001:`.
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
