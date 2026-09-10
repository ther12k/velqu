# Velqu Browser-WASM — All GitHub Issue Bodies

Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)

Each section below is also available as a standalone body file. Use `manifests/issues.json` for the exact GitHub title and labels.

## Contents

- [BWASM-EPIC — Velqu Browser-WASM runtime program](tasks/00_program/BWASM-EPIC-velqu-browser-wasm-runtime-program.md)
- [BWASM-D-001 — Freeze the Browser-WASM product and runtime contract](tasks/01_design/BWASM-D-001-freeze-the-browser-wasm-product-and-runtime-contract.md)
- [BWASM-D-002 — Produce the wasm32 portability baseline and dependency split map](tasks/01_design/BWASM-D-002-produce-the-wasm32-portability-baseline-and-dependency-split-map.md)
- [BWASM-D-003 — Define the browser execution threat model and isolation contract](tasks/01_design/BWASM-D-003-define-the-browser-execution-threat-model-and-isolation-contract.md)
- [BWASM-D-004 — Ratify support matrix, compatibility claims, and release budgets](tasks/01_design/BWASM-D-004-ratify-support-matrix-compatibility-claims-and-release-budgets.md)
- [BWASM-K-001 — Extract a portable runtime model crate](tasks/02_kernel/BWASM-K-001-extract-a-portable-runtime-model-crate.md)
- [BWASM-K-002 — Split byte-based QPack core from native loading and tooling](tasks/02_kernel/BWASM-K-002-split-byte-based-qpack-core-from-native-loading-and-tooling.md)
- [BWASM-K-003 — Extract a host-independent router core](tasks/02_kernel/BWASM-K-003-extract-a-host-independent-router-core.md)
- [BWASM-K-004 — Qualify the schema runtime for wasm32 and expose bounded validation](tasks/02_kernel/BWASM-K-004-qualify-the-schema-runtime-for-wasm32-and-expose-bounded-validation.md)
- [BWASM-K-005 — Implement the Rust Browser Kernel and wasm-bindgen ABI](tasks/02_kernel/BWASM-K-005-implement-the-rust-browser-kernel-and-wasm-bindgen-abi.md)
- [BWASM-K-006 — Verify and package portable-kernel evidence](tasks/02_kernel/BWASM-K-006-verify-and-package-portable-kernel-evidence.md)
- [BWASM-R-001 — Create @velqu/browser-runtime package and public runtime contract](tasks/03_runtime/BWASM-R-001-create-velqu-browser-runtime-package-and-public-runtime-contract.md)
- [BWASM-R-002 — Implement Fetch-compatible browser dispatcher](tasks/03_runtime/BWASM-R-002-implement-fetch-compatible-browser-dispatcher.md)
- [BWASM-R-003 — Define and emit the browser handler-bundle contract](tasks/03_runtime/BWASM-R-003-define-and-emit-the-browser-handler-bundle-contract.md)
- [BWASM-R-004 — Execute handlers in isolated Workers with cancellation and hard recovery](tasks/03_runtime/BWASM-R-004-execute-handlers-in-isolated-workers-with-cancellation-and-hard-recovery.md)
- [BWASM-R-005 — Integrate capability registry and Treaty with the browser runtime](tasks/03_runtime/BWASM-R-005-integrate-capability-registry-and-treaty-with-the-browser-runtime.md)
- [BWASM-R-006 — Verify and package browser-runtime evidence](tasks/03_runtime/BWASM-R-006-verify-and-package-browser-runtime-evidence.md)
- [BWASM-B-001 — Add compiler target browser-wasm](tasks/04_build_deploy/BWASM-B-001-add-compiler-target-browser-wasm.md)
- [BWASM-B-002 — Define content-addressed browser artifact manifest and loader](tasks/04_build_deploy/BWASM-B-002-define-content-addressed-browser-artifact-manifest-and-loader.md)
- [BWASM-B-003 — Enforce browser import policy with source-located diagnostics](tasks/04_build_deploy/BWASM-B-003-enforce-browser-import-policy-with-source-located-diagnostics.md)
- [BWASM-B-004 — Add Service Worker adapter and static-host bootstrap](tasks/04_build_deploy/BWASM-B-004-add-service-worker-adapter-and-static-host-bootstrap.md)
- [BWASM-B-005 — Add CLI build, preview, inspect, and export workflows](tasks/04_build_deploy/BWASM-B-005-add-cli-build-preview-inspect-and-export-workflows.md)
- [BWASM-B-006 — Verify cache activation, upgrades, rollback, and static deployment](tasks/04_build_deploy/BWASM-B-006-verify-cache-activation-upgrades-rollback-and-static-deployment.md)
- [BWASM-C-001 — Implement browser-safe timer, crypto, logging, and restricted fetch capabilities](tasks/05_capabilities/BWASM-C-001-implement-browser-safe-timer-crypto-logging-and-restricted-fetch-capabilities.md)
- [BWASM-C-002 — Make the Postgres capability contract asynchronous before browser freeze](tasks/05_capabilities/BWASM-C-002-make-the-postgres-capability-contract-asynchronous-before-browser-freeze.md)
- [BWASM-C-003 — Add optional PGlite-backed local SQL capability](tasks/05_capabilities/BWASM-C-003-add-optional-pglite-backed-local-sql-capability.md)
- [BWASM-C-004 — Add namespaced IndexedDB KV persistence capability](tasks/05_capabilities/BWASM-C-004-add-namespaced-indexeddb-kv-persistence-capability.md)
- [BWASM-C-005 — Fail closed for deployment-required and unavailable capabilities](tasks/05_capabilities/BWASM-C-005-fail-closed-for-deployment-required-and-unavailable-capabilities.md)
- [BWASM-Q-001 — Build shared native-versus-browser conformance and differential suites](tasks/06_quality_release/BWASM-Q-001-build-shared-native-versus-browser-conformance-and-differential-suites.md)
- [BWASM-Q-002 — Add real-browser CI lanes and supported-browser evidence](tasks/06_quality_release/BWASM-Q-002-add-real-browser-ci-lanes-and-supported-browser-evidence.md)
- [BWASM-Q-003 — Verify isolated preview-origin and untrusted-code security boundaries](tasks/06_quality_release/BWASM-Q-003-verify-isolated-preview-origin-and-untrusted-code-security-boundaries.md)
- [BWASM-Q-004 — Add Browser-WASM observability and developer diagnostics](tasks/06_quality_release/BWASM-Q-004-add-browser-wasm-observability-and-developer-diagnostics.md)
- [BWASM-Q-005 — Set and enforce WASM size, startup, latency, and leak budgets](tasks/06_quality_release/BWASM-Q-005-set-and-enforce-wasm-size-startup-latency-and-leak-budgets.md)
- [BWASM-Q-006 — Publish Browser-WASM documentation, limitations, and migration guide](tasks/06_quality_release/BWASM-Q-006-publish-browser-wasm-documentation-limitations-and-migration-guide.md)
- [BWASM-Q-007 — Run an external cleanroom static deployment and offline exercise](tasks/06_quality_release/BWASM-Q-007-run-an-external-cleanroom-static-deployment-and-offline-exercise.md)
- [BWASM-Q-008 — Assemble release evidence, SBOM, checksums, provenance, and candidate packet](tasks/06_quality_release/BWASM-Q-008-assemble-release-evidence-sbom-checksums-provenance-and-candidate-packet.md)
- [BWASM-X-001 — Spike QuickJS-NG-in-WASM engine parity and record GO or NO-GO](tasks/07_optional_parity/BWASM-X-001-spike-quickjs-ng-in-wasm-engine-parity-and-record-go-or-no-go.md)
- [BWASM-GATE — Browser-WASM beta readiness GO or NO-GO](gates/BWASM-GATE-browser-wasm-beta-readiness-go-or-no-go.md)

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/00_program/BWASM-EPIC-velqu-browser-wasm-runtime-program.md`  
Program: `BWASM`  
Phase: `00_program` — Program  
Mode: `GATE` — Coordinate dependencies and decisions; do not implement child work here.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-EPIC — Velqu Browser-WASM runtime program

## Atomic goal

Track the complete program that makes a Velqu application buildable as static assets and executable in an ordinary browser with a meaningful Rust/WASM kernel and no Velqu application server.

## Parent intent

Coordinate the complete Browser-WASM program and keep product claims tied to completed evidence.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- None; this issue can be opened immediately.

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `AGENTS.md`
- `README.md`
- `Cargo.toml`
- `package.json`
- `scripts/verify`

## Steps

1. Ratify the target architecture and forbidden claims before implementation.
2. Keep a live checklist linking every design, implementation, verification, evidence, optional parity, and gate issue.
3. Record owner decisions and accepted residual risks in the program decision log.
4. Close only after BWASM-GATE records GO against an exact candidate.

## Acceptance criteria

- [ ] The epic distinguishes static hosting from an application server.
- [ ] It states that the MVP is Rust/WASM kernel plus isolated browser Worker handlers.
- [ ] It states that exact QuickJS-NG-in-WASM parity is separately gated and optional by default.
- [ ] It prohibits unsupported claims about hostile-code sandboxing, production secrets, shared persistence, and native performance parity.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Run the packet validator.
- Dry-run issue registration and inspect titles, labels, dependencies, and body paths.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Registered issue index.
- [ ] Owner decision log.
- [ ] Final BWASM-GATE link and outcome.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Implementing code in the epic.
- Closing child work from self-attestation alone.
- Treating optional work as an implicit release blocker.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-epic:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/01_design/BWASM-D-001-freeze-the-browser-wasm-product-and-runtime-contract.md`  
Program: `BWASM`  
Phase: `01_design` — Architecture and decisions  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-D-001 — Freeze the Browser-WASM product and runtime contract

## Atomic goal

Write and ratify the architecture decision record for the Browser-WASM target.

## Parent intent

Freeze boundaries before implementation so the program does not drift into a full port of native q-runtime or a JavaScript-only mock.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- None; this issue can be opened immediately.

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `AGENTS.md`
- `README.md`
- `Cargo.toml`
- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-runtime/src/lib.rs`
- `crates/q-engine/src/lib.rs`
- `crates/q-engine-quickjs/src/lib.rs`

## Steps

1. Define the canonical browser interface as `fetch(Request): Promise<Response>`.
2. Choose the required MVP profile: Rust/WASM owns artifact verification, routing, request/response schema validation, compatibility checks, capability authorization, and problem mapping; generated handlers run in an isolated Worker.
3. Define an experimental `quickjs-wasm` profile behind a separate decision gate.
4. Define request lifecycle, artifact lifecycle, capability model, persistence model, update model, and package/crate boundaries.
5. Classify semantics as identical, adapted, unsupported, deployment-required, or explicitly simulated.
6. Record non-goals and forbidden product claims.

## Acceptance criteria

- [ ] ADR includes architecture and sequence diagrams plus explicit ownership before and after handler execution.
- [ ] Compiler execution may remain native/Bun for MVP; deployed runtime execution is browser-local.
- [ ] Service Worker is explicitly either mandatory for beta or an adapter over the canonical dispatcher.
- [ ] Default Worker handlers are explicitly not exact QuickJS-NG engine parity.
- [ ] Owner acceptance is recorded before K-phase implementation merges.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Architecture review against current workspace/package dependency graph.
- A proof sketch of Request → WASM plan → Worker handler → WASM completion → Response.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Accepted ADR with owner/date.
- [ ] Decision table.
- [ ] Rejected alternatives and rationale.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Implementing runtime code.
- Compiling all of q-runtime unchanged.
- Promising all handler logic executes in WASM in the default profile.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-d-001:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/01_design/BWASM-D-002-produce-the-wasm32-portability-baseline-and-dependency-split-map.md`  
Program: `BWASM`  
Phase: `01_design` — Architecture and decisions  
Mode: `VERIFY_OR_FIX` — Verify first, fix defects within this issue's bounded area, and preserve before/after evidence.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-D-002 — Produce the wasm32 portability baseline and dependency split map

## Atomic goal

Measure current `wasm32-unknown-unknown` compatibility and classify every relevant crate/package.

## Parent intent

Freeze boundaries before implementation so the program does not drift into a full port of native q-runtime or a JavaScript-only mock.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-D-001` — Freeze the Browser-WASM product and runtime contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `AGENTS.md`
- `README.md`
- `Cargo.toml`
- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-runtime/src/lib.rs`
- `crates/q-http/Cargo.toml`
- `crates/q-engine/Cargo.toml`
- `crates/q-engine-quickjs/Cargo.toml`
- `crates/q-pack/Cargo.toml`
- `crates/q-router/Cargo.toml`
- `crates/q-schema-runtime/Cargo.toml`

## Steps

1. Create a machine-readable inventory with `portable`, `split-required`, `native-only`, and `browser-only` classifications.
2. Run targeted wasm32 checks and retain exact failures rather than inferring compatibility.
3. Inspect transitive dependencies with `cargo tree` and classify blockers by API, build script, platform intrinsic, or architecture coupling.
4. Identify portable source that should move instead of being duplicated.
5. Propose the smallest dependency cuts for the K-phase.

## Acceptance criteria

- [ ] Every workspace member and package involved in compile, contracts, tests, or runtime dispatch is classified.
- [ ] Each non-portable item names its blocking API/dependency and proposed disposition.
- [ ] Baseline can be reproduced from a clean checkout and is bound to a commit/toolchain manifest.
- [ ] WASI/server-side WASM is not conflated with ordinary browser wasm32.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- `rustup target add wasm32-unknown-unknown`.
- `cargo check --target wasm32-unknown-unknown -p q-schema-runtime`.
- Targeted q-router/q-pack/q-engine checks with failures retained.
- `cargo tree` reports; `bun test`; `bun run typecheck`.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] `portability-inventory.json`.
- [ ] `wasm32-baseline.md`.
- [ ] Dependency-cut graph.
- [ ] Exact toolchain/commit manifest.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Refactoring crates in this audit.
- Suppressing compiler failures.
- Declaring compatibility from source inspection alone.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-d-002:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/01_design/BWASM-D-003-define-the-browser-execution-threat-model-and-isolation-contract.md`  
Program: `BWASM`  
Phase: `01_design` — Architecture and decisions  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-D-003 — Define the browser execution threat model and isolation contract

## Atomic goal

Define the security boundary for browser-deployed Velqu code, including AI-generated or user-authored applications.

## Parent intent

Freeze boundaries before implementation so the program does not drift into a full port of native q-runtime or a JavaScript-only mock.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-D-001` — Freeze the Browser-WASM product and runtime contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `AGENTS.md`
- `README.md`
- `Cargo.toml`
- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-runtime/src/lib.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `packages/compiler/src/index.ts`

## Steps

1. Model actors, assets, trust boundaries, deployment modes, and abuse cases.
2. Specify separate preview origin, sandboxed iframe, Worker, CSP, permissions/referrer policies, and validated messaging requirements.
3. Specify input/output/log/capability-call bounds, network defaults, credential handling, storage/cache protections, and recovery.
4. Define trusted-code versus untrusted-preview modes and forbidden sandbox claims.
5. Create a malicious-app test matrix referenced by downstream tasks.

## Acceptance criteria

- [ ] Threat model covers origin confusion, XSS, credential leakage, postMessage spoofing, cache poisoning, capability escalation, infinite loops, oversized data, storage exhaustion, and browser-fetch exfiltration.
- [ ] Provider keys, production secrets, and remote DB credentials never enter generated browser artifacts.
- [ ] Untrusted mode uses default-deny or explicit allowlisting for outbound network access.
- [ ] Worker termination is documented as deadline recovery, not a hard heap or certified hostile-code sandbox.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Review at least three malicious fixtures.
- Validate proposed CSP/iframe policy in a minimal two-origin deployment.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Threat model.
- [ ] Security invariants.
- [ ] Abuse-case matrix.
- [ ] Owner acceptance or exact unresolved decisions.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Claiming formal sandbox security.
- Implementing the runtime.
- Same-origin untrusted preview as the recommended design.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-d-003:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/01_design/BWASM-D-004-ratify-support-matrix-compatibility-claims-and-release-budgets.md`  
Program: `BWASM`  
Phase: `01_design` — Architecture and decisions  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-D-004 — Ratify support matrix, compatibility claims, and release budgets

## Atomic goal

Set evidence-bound browser support, compatibility, size, startup, latency, memory, offline, and update targets.

## Parent intent

Freeze boundaries before implementation so the program does not drift into a full port of native q-runtime or a JavaScript-only mock.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-D-001` — Freeze the Browser-WASM product and runtime contract
- `BWASM-D-002` — Produce the wasm32 portability baseline and dependency split map
- `BWASM-D-003` — Define the browser execution threat model and isolation contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `AGENTS.md`
- `README.md`
- `Cargo.toml`
- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-runtime/src/lib.rs`

## Steps

1. Classify browser/OS/device lanes as tested, supported, experimental, or out of scope.
2. Define budgets for base WASM, JS glue, handler bundle, optional capability bundles, and total initial transfer.
3. Define cold/warm startup, request latency, memory-growth, and repeated-lifecycle measurement procedures.
4. Define offline, cache activation, update/rollback, and artifact-version compatibility expectations.
5. List allowed intentional native/browser differences.

## Acceptance criteria

- [ ] Every budget names a representative environment, sample procedure, and noise/waiver policy.
- [ ] No universal browser/mobile support is claimed without lanes.
- [ ] No native/competitor performance comparison is allowed without matched methodology.
- [ ] Budget/support changes require evidence and owner approval.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Run a small measurability prototype only; do not treat it as final performance evidence.
- Review all targets against D-002 and D-003.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Support matrix.
- [ ] Machine-readable budgets.
- [ ] Known-limitations baseline.
- [ ] Owner decision record.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Optimization work.
- Arbitrary marketing numbers.
- Using one laptop/browser result as a support claim.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-d-004:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/02_kernel/BWASM-K-001-extract-a-portable-runtime-model-crate.md`  
Program: `BWASM`  
Phase: `02_kernel` — Portable Rust/WASM kernel  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-K-001 — Extract a portable runtime model crate

## Atomic goal

Move cross-target IDs, route/contract models, invocation/result structures, and problem types out of native engine ownership.

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

- `BWASM-D-001` — Freeze the Browser-WASM product and runtime contract
- `BWASM-D-002` — Produce the wasm32 portability baseline and dependency split map

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `Cargo.toml`
- `crates/q-engine/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-runtime/src/lib.rs`

## Steps

1. Inventory model-only types in q-engine/q-pack/q-router.
2. Create a dependency-light portable crate with versioned deterministic serialization.
3. Keep Tokio traits/synchronization, `Instant`, host handles, native byte buffers, and engine lifecycle outside.
4. Add explicit conversion layers for native adapters.

## Acceptance criteria

- [ ] Portable model crate compiles natively and for wasm32.
- [ ] Normal dependency tree excludes Tokio, Hyper, rquickjs, memmap2, filesystem/process/socket, and native Postgres.
- [ ] Existing native behavior and identifiers do not drift.
- [ ] Round-trip fixtures are deterministic and versioned.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Native and wasm32 cargo checks.
- q-engine/q-router/q-pack regression tests.
- `cargo tree` audit.
- `./scripts/verify`.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Moved-type map.
- [ ] Before/after dependency graph.
- [ ] Serialization fixtures and hashes.
- [ ] Exact command results.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Porting the engine trait.
- Adding browser APIs to the model crate.
- Changing TypeScript public contracts.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-k-001:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/02_kernel/BWASM-K-002-split-byte-based-qpack-core-from-native-loading-and-tooling.md`  
Program: `BWASM`  
Phase: `02_kernel` — Portable Rust/WASM kernel  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-K-002 — Split byte-based QPack core from native loading and tooling

## Atomic goal

Separate bounded in-memory pack parsing/verification from filesystem, mmap, signing/authoring, CLI, and native loading.

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

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `Cargo.toml`
- `crates/q-engine/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-runtime/src/lib.rs`
- `crates/q-pack/Cargo.toml`
- `crates/q-bytecode-tool/`

## Steps

1. Create `q-pack-core` or an accepted equivalent that operates on bytes and portable models only.
2. Move native loading/tooling behind a native crate or feature excluded from the browser kernel.
3. Preserve format/version/hash/signature decisions through shared fixtures.
4. Add explicit byte, section, depth, allocation, and error limits for untrusted browser artifacts.

## Acceptance criteria

- [ ] Portable pack core compiles for wasm32 without native loader imports.
- [ ] Native and WASM produce equivalent results for valid/invalid fixtures.
- [ ] Malformed, truncated, swapped, and oversized inputs never panic or allocate without bounds.
- [ ] No filesystem path is reachable from the browser-facing API.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Native q-pack tests.
- wasm-bindgen valid/invalid fixture tests.
- Fuzz/property tests.
- `cargo tree --target wasm32-unknown-unknown`.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Format compatibility report.
- [ ] Fixture hashes.
- [ ] Parser-boundary/fuzz report.
- [ ] Dependency audit.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Building packs in the browser for MVP.
- Filesystem emulation.
- Weakening integrity checks.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-k-002:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/02_kernel/BWASM-K-003-extract-a-host-independent-router-core.md`  
Program: `BWASM`  
Phase: `02_kernel` — Portable Rust/WASM kernel  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-K-003 — Extract a host-independent router core

## Atomic goal

Refactor route matching and method resolution into one deterministic native/WASM implementation.

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

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `Cargo.toml`
- `crates/q-engine/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-runtime/src/lib.rs`
- `crates/q-router/Cargo.toml`
- `conformance/`

## Steps

1. Define portable compiled-route input models.
2. Remove q-engine/q-pack host coupling from route algorithms.
3. Preserve native adapter behavior through conversions.
4. Specify base path, percent decoding, query exclusion, trailing slash, malformed URL, ambiguity, duplicate route, and method semantics.

## Acceptance criteria

- [ ] Router core compiles for wasm32 without host runtime dependencies.
- [ ] Native and browser tests consume the same route fixtures.
- [ ] Precedence and decoded params match the native baseline.
- [ ] Invalid/ambiguous tables fail deterministically at initialization.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- q-router native tests.
- wasm-bindgen route fixture tests.
- Property tests for patterns/encoding.
- `./scripts/verify`.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Shared route corpus.
- [ ] Native-vs-WASM diff.
- [ ] Dependency tree.
- [ ] Regression results.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- HTTP listening.
- Handler execution.
- A second JavaScript router.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-k-003:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/02_kernel/BWASM-K-004-qualify-the-schema-runtime-for-wasm32-and-expose-bounded-validation.md`  
Program: `BWASM`  
Phase: `02_kernel` — Portable Rust/WASM kernel  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-K-004 — Qualify the schema runtime for wasm32 and expose bounded validation

## Atomic goal

Make Velqu schema IR validation a first-class deterministic browser-WASM component.

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

- `BWASM-D-002` — Produce the wasm32 portability baseline and dependency split map

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `Cargo.toml`
- `crates/q-engine/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-runtime/src/lib.rs`
- `crates/q-schema-runtime/Cargo.toml`
- `packages/schema/src/index.ts`
- `packages/contract/src/index.ts`

## Steps

1. Make wasm32 an explicit supported build for q-schema-runtime.
2. Define stable request/response validation inputs and outputs for the kernel.
3. Add depth, collection, string, regex/work, error-count, and output limits.
4. Share canonicalization and error-order fixtures across targets.

## Acceptance criteria

- [ ] Native and wasm32 schema tests pass with equivalent codes, paths, and ordering.
- [ ] Limit violations return typed problems without panic or browser hangs.
- [ ] No JavaScript schema fallback silently changes semantics.
- [ ] Schema WASM size contribution is measured.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Native schema tests.
- wasm-bindgen browser tests.
- Boundary and fuzz/property tests.
- `./scripts/verify`.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Cross-target fixtures.
- [ ] Budget results.
- [ ] Artifact-size report.
- [ ] Commit-bound verification log.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Changing schema language.
- Compiling TypeScript schema source in Rust WASM.
- Ignoring regex/resource risk.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-k-004:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/02_kernel/BWASM-K-005-implement-the-rust-browser-kernel-and-wasm-bindgen-abi.md`  
Program: `BWASM`  
Phase: `02_kernel` — Portable Rust/WASM kernel  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-K-005 — Implement the Rust Browser Kernel and wasm-bindgen ABI

## Atomic goal

Create the real Rust/WASM request kernel that initializes verified artifacts, plans invocations, authorizes capabilities, and validates/maps handler results.

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

- `BWASM-K-002` — Split byte-based QPack core from native loading and tooling
- `BWASM-K-003` — Extract a host-independent router core
- `BWASM-K-004` — Qualify the schema runtime for wasm32 and expose bounded validation
- `BWASM-D-003` — Define the browser execution threat model and isolation contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `Cargo.toml`
- `crates/q-engine/src/lib.rs`
- `crates/q-pack/src/lib.rs`
- `crates/q-router/src/lib.rs`
- `crates/q-schema-runtime/src/lib.rs`
- `crates/q-runtime/src/lib.rs`

## Steps

1. Create `q-browser-kernel` (`cdylib`/`rlib`) and a versioned wasm-bindgen boundary.
2. Initialize from content-addressed browser manifest/pack bytes with size, integrity, and compatibility checks.
3. Implement `plan_request`: route, decode, validate, normalize context, authorize declared capabilities, or return a complete problem.
4. Implement `complete_invocation`: validate declared status/headers/body and normalize Response/problem data.
5. Reject ABI mismatch, missing/extra handlers, undeclared capability/status, malformed artifacts, and invalid output.
6. Use bounded serialization and benchmark candidate ABI encodings before freeze.

## Acceptance criteria

- [ ] A real Request path crosses JS → WASM plan → handler → WASM completion → Response with no API server.
- [ ] Kernel import table excludes sockets, fs, process, signals, native threads, Hyper, Tokio networking, rquickjs, and native Postgres.
- [ ] Unknown route/method, invalid request/response, undeclared status/capability, and ABI mismatch have stable problems.
- [ ] Initialization/disposal behavior is explicit and panic paths never become success.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- wasm32 cargo check.
- wasm-bindgen lifecycle and negative tests.
- ABI compatibility fixtures.
- WASM import audit.
- `./scripts/verify`.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] WASM/JS glue hashes.
- [ ] ABI specification.
- [ ] Import table.
- [ ] End-to-end browser trace.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Listening HTTP.
- Executing handlers inside Rust for the default profile.
- Calling browser APIs from portable core crates.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-k-005:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/02_kernel/BWASM-K-006-verify-and-package-portable-kernel-evidence.md`  
Program: `BWASM`  
Phase: `02_kernel` — Portable Rust/WASM kernel  
Mode: `EVIDENCE` — Package evidence from one exact candidate; do not mix implementation and attestation.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

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
```

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/03_runtime/BWASM-R-001-create-velqu-browser-runtime-package-and-public-runtime-contract.md`  
Program: `BWASM`  
Phase: `03_runtime` — Browser runtime and Worker execution  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-R-001 — Create @velqu/browser-runtime package and public runtime contract

## Atomic goal

Create the browser-safe package boundary that owns Velqu's in-browser Request-to-Response runtime.

## Parent intent

Provide a real browser runtime over Request/Response, isolated handler execution, and Treaty without a listening application server.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-K-005` — Implement the Rust Browser Kernel and wasm-bindgen ABI
- `BWASM-D-001` — Freeze the Browser-WASM product and runtime contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `packages/compiler/src/index.ts`
- `crates/q-browser-kernel/`

## Steps

1. Add packages/browser-runtime with browser-only exports and no Bun/Node/native imports.
2. Define BrowserRuntime, BrowserRuntimeOptions, BrowserRuntimeError, and lifecycle types.
3. Expose createBrowserRuntime() and a Fetch-compatible fetch(Request) entry point.
4. Keep authoring/core and Treaty types reusable without importing testing or native runtime packages.
5. Add package build, typecheck, API-surface tests, and browser bundler smoke fixtures.

## Acceptance criteria

- [ ] A clean consumer can import @velqu/browser-runtime from a browser bundle.
- [ ] The public entry point accepts Request and resolves Response.
- [ ] The package dependency graph contains no Bun.*, node:*, native addon, q-http, or q-runtime dependency.
- [ ] Exports and TypeScript declarations are explicit and tested.
- [ ] Runtime lifecycle errors are structured rather than console-only.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- `bun run typecheck`
- `bun test packages/browser-runtime`
- Build the package for browser target.
- Bundle and execute a clean consumer fixture in a real browser.
- Static forbidden-import scan.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Package manifest and exports snapshot.
- [ ] Dependency/import audit.
- [ ] Clean browser-consumer log.
- [ ] Public API snapshot.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Service Worker interception.
- QuickJS execution.
- Persistence adapters.
- Native runtime changes not required by the shared contract.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-r-001:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/03_runtime/BWASM-R-002-implement-fetch-compatible-browser-dispatcher.md`  
Program: `BWASM`  
Phase: `03_runtime` — Browser runtime and Worker execution  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-R-002 — Implement Fetch-compatible browser dispatcher

## Atomic goal

Dispatch a browser Request through the WASM kernel and return a standards-compliant Response.

## Parent intent

Provide a real browser runtime over Request/Response, isolated handler execution, and Treaty without a listening application server.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-R-001` — Create @velqu/browser-runtime package and public runtime contract
- `BWASM-K-005` — Implement the Rust Browser Kernel and wasm-bindgen ABI

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `packages/compiler/src/index.ts`
- `crates/q-browser-kernel/`

## Steps

1. Normalize URL, method, headers, query, body, and abort signal at the JS/WASM boundary.
2. Use the Rust/WASM kernel for route selection, parameter extraction, request validation, capability checks, and response validation.
3. Map route misses, method mismatches, malformed input, and internal failures to Velqu problem responses.
4. Support bounded text, JSON, URL-encoded, multipart metadata, and binary body handling according to the frozen support matrix.
5. Preserve deterministic header/status behavior and define unsupported streaming behavior explicitly.

## Acceptance criteria

- [ ] Static and parameterized routes dispatch with production-equivalent precedence.
- [ ] 405/Allow, OPTIONS behavior, HEAD behavior, trailing-slash policy, and duplicate-header policy are fixture-locked.
- [ ] Request and response schema failures use the canonical problem shape.
- [ ] Abort before and during dispatch returns the documented cancellation result.
- [ ] No request path can bypass kernel validation through a JavaScript-only fast path.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Browser unit tests for every supported body/status/header form.
- Shared route corpus against native and browser targets.
- Malformed and oversized request corpus.
- Abort/cancellation tests.
- Real-browser fetch smoke.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Dispatcher conformance report.
- [ ] Native/browser fixture diff.
- [ ] Unsupported-semantics inventory.
- [ ] Raw browser logs.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Opening a TCP listener.
- Pretending Service Worker transport is real network conformance.
- Native Hyper backpressure parity where browsers provide no equivalent.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-r-002:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/03_runtime/BWASM-R-003-define-and-emit-the-browser-handler-bundle-contract.md`  
Program: `BWASM`  
Phase: `03_runtime` — Browser runtime and Worker execution  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-R-003 — Define and emit the browser handler-bundle contract

## Atomic goal

Define the deterministic contract connecting compiled application handlers to the browser runtime.

## Parent intent

Provide a real browser runtime over Request/Response, isolated handler execution, and Treaty without a listening application server.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-R-001` — Create @velqu/browser-runtime package and public runtime contract
- `BWASM-D-001` — Freeze the Browser-WASM product and runtime contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `packages/compiler/src/index.ts`
- `crates/q-browser-kernel/`

## Steps

1. Define stable route-ID-to-handler module mapping and bundle metadata.
2. Specify handler invocation input/output, context shape, status declarations, and error serialization.
3. Require generated handler modules to register through a narrow runtime API instead of ambient globals.
4. Version the handler ABI independently from package version where appropriate.
5. Provide deterministic bundle ordering, source maps, and source-location metadata.

## Acceptance criteria

- [ ] The same source produces byte-stable metadata under a normalized environment.
- [ ] Unknown ABI versions fail closed with an actionable diagnostic.
- [ ] Duplicate or missing handler IDs are rejected before execution.
- [ ] Source locations survive into runtime errors without exposing private host paths.
- [ ] Handlers cannot silently register undeclared routes or statuses.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Compiler fixture snapshots.
- Duplicate/missing/unknown ABI negative tests.
- Reproducible-build test.
- Source-map browser smoke.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Handler ABI specification.
- [ ] Golden bundle fixtures.
- [ ] Reproducibility hashes.
- [ ] Diagnostic snapshots.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Choosing a hostile-code security claim.
- Embedding QuickJS in the MVP.
- Supporting arbitrary dynamic imports.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-r-003:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/03_runtime/BWASM-R-004-execute-handlers-in-isolated-workers-with-cancellation-and-hard-recovery.md`  
Program: `BWASM`  
Phase: `03_runtime` — Browser runtime and Worker execution  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-R-004 — Execute handlers in isolated Workers with cancellation and hard recovery

## Atomic goal

Run generated handlers outside the editor/UI realm and provide deterministic cancellation and recovery.

## Parent intent

Provide a real browser runtime over Request/Response, isolated handler execution, and Treaty without a listening application server.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-R-003` — Define and emit the browser handler-bundle contract
- `BWASM-D-003` — Define the browser execution threat model and isolation contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `packages/compiler/src/index.ts`
- `crates/q-browser-kernel/`

## Steps

1. Create a dedicated Worker execution host and validated message protocol.
2. Use one-shot or pooled workers according to the threat model and document the chosen lifecycle.
3. Propagate request deadlines and AbortSignal state.
4. Terminate and replace workers that exceed time, message, log, or output budgets.
5. Redact stack paths and bound console/log forwarding.
6. Ensure runtime state cannot leak across projects unless an adapter explicitly declares persistence.

## Acceptance criteria

- [ ] Infinite loops are stopped by worker termination and the runtime remains usable afterward.
- [ ] Late messages from a terminated/stale worker are ignored.
- [ ] Cross-project invocation IDs cannot collide or receive another project's result.
- [ ] Uncloneable/oversized payloads become structured errors.
- [ ] No parent DOM, editor token, or provider credential is reachable through the execution protocol.
- [ ] Documentation states that Worker isolation is not by itself a proven hostile-code sandbox.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Infinite-loop and high-log-volume adversarial tests.
- Abort race tests.
- Worker crash/restart tests.
- Cross-project leakage tests.
- Real-browser CSP/isolation smoke.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Threat-model test matrix.
- [ ] Worker protocol schema.
- [ ] Crash/timeout raw logs.
- [ ] Isolation screenshots or browser traces.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Claiming hard heap enforcement when the browser cannot prove it.
- Executing generated code on the editor origin.
- Passing secrets into the preview worker.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-r-004:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/03_runtime/BWASM-R-005-integrate-capability-registry-and-treaty-with-the-browser-runtime.md`  
Program: `BWASM`  
Phase: `03_runtime` — Browser runtime and Worker execution  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-R-005 — Integrate capability registry and Treaty with the browser runtime

## Atomic goal

Make capability injection and typed Treaty calls work through the same browser runtime boundary.

## Parent intent

Provide a real browser runtime over Request/Response, isolated handler execution, and Treaty without a listening application server.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-R-002` — Implement Fetch-compatible browser dispatcher
- `BWASM-R-004` — Execute handlers in isolated Workers with cancellation and hard recovery
- `BWASM-K-005` — Implement the Rust Browser Kernel and wasm-bindgen ABI

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `packages/compiler/src/index.ts`
- `crates/q-browser-kernel/`

## Steps

1. Define a browser capability registry keyed by declared capability IDs and versions.
2. Pass only declared capability handles into each handler context.
3. Add a Treaty transport/dispatch adapter backed by BrowserRuntime.fetch or direct typed dispatch without semantic bypass.
4. Preserve declared status narrowing and canonical problem decoding.
5. Reject missing, incompatible, undeclared, or deployment-only capability use before side effects.

## Acceptance criteria

- [ ] Treaty clients call browser routes with the same route IDs and status typing as native builds.
- [ ] Capability authorization happens before handler side effects.
- [ ] A route cannot access a capability omitted from its compiled declaration.
- [ ] Version mismatch and unavailable capability failures are machine-readable.
- [ ] Direct Treaty mode and Request/Response mode share validation and routing semantics.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Treaty compile-time fixtures.
- Runtime capability allow/deny matrix.
- Side-effect-before-authorization regression test.
- Direct-vs-fetch differential tests.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Treaty consumer fixture.
- [ ] Capability registry manifest.
- [ ] Negative-test logs.
- [ ] Native/browser behavior diff.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Implementing every capability adapter.
- Using @velqu/testing as the production browser runtime.
- Silently mocking production-only integrations.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-r-005:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/03_runtime/BWASM-R-006-verify-and-package-browser-runtime-evidence.md`  
Program: `BWASM`  
Phase: `03_runtime` — Browser runtime and Worker execution  
Mode: `EVIDENCE` — Package evidence from one exact candidate; do not mix implementation and attestation.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-R-006 — Verify and package browser-runtime evidence

## Atomic goal

Independently verify the JavaScript/Worker browser runtime at one exact commit.

## Parent intent

Provide a real browser runtime over Request/Response, isolated handler execution, and Treaty without a listening application server.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-R-001` — Create @velqu/browser-runtime package and public runtime contract
- `BWASM-R-002` — Implement Fetch-compatible browser dispatcher
- `BWASM-R-003` — Define and emit the browser handler-bundle contract
- `BWASM-R-004` — Execute handlers in isolated Workers with cancellation and hard recovery
- `BWASM-R-005` — Integrate capability registry and Treaty with the browser runtime
- `BWASM-K-006` — Verify and package portable-kernel evidence

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/treaty/src/index.ts`
- `packages/testing/src/index.ts`
- `packages/compiler/src/index.ts`
- `crates/q-browser-kernel/`

## Steps

1. Freeze a clean candidate and install packages from produced tarballs in a clean fixture.
2. Run dispatcher, Worker, Treaty, capability, abort, and negative-path browser suites.
3. Audit bundles for forbidden imports and accidental secret/global access.
4. Inspect public exports, ABI versions, source maps, and package metadata.
5. Package raw logs, browser/toolchain versions, hashes, and reviewer conclusions.

## Acceptance criteria

- [ ] All R-phase criteria are independently demonstrated.
- [ ] Evidence is generated from package bytes intended for release.
- [ ] Browser execution uses the WASM kernel rather than an unverified JavaScript matcher/validator.
- [ ] No unresolved runtime P0 remains.
- [ ] Every known semantic difference is documented and linked to owner acceptance or a blocking issue.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Clean tarball consumer.
- Real-browser suite.
- Bundle/import audit.
- Adversarial Worker suite.
- Full repository verify.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Runtime verification report.
- [ ] Raw browser logs/traces.
- [ ] Tarball and bundle hashes.
- [ ] Browser/OS/toolchain manifest.
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

- Implementing missing features during evidence work except evidence-harness defects.
- Mixing commits or locally modified package bytes.
- Waiving failures without a linked decision.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-r-006:`.
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

---

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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/04_build_deploy/BWASM-B-002-define-content-addressed-browser-artifact-manifest-and-loader.md`  
Program: `BWASM`  
Phase: `04_build_deploy` — Compiler, artifacts, and static deployment  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-B-002 — Define content-addressed browser artifact manifest and loader

## Atomic goal

Make browser-WASM artifacts integrity-bound, cacheable, and safely activatable.

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

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/compiler/src/index.ts`
- `packages/compiler/src/extract.ts`
- `packages/compiler/src/emit.ts`
- `packages/cli/src/index.ts`
- `packages/browser-runtime/`
- `.github/workflows/verify.yml`

## Steps

1. Define canonical browser artifact manifest fields, versions, URLs, media types, sizes, and SHA-256 digests.
2. Implement loader verification before WASM instantiation or handler import.
3. Bind handler bundle, kernel, schemas, contract, capability declarations, and source-map metadata to one build ID.
4. Reject partial/mixed-version deployments.
5. Support relative URLs for static hosting under a configured base path.

## Acceptance criteria

- [ ] Tampered, truncated, missing, cross-build, or unsupported-version artifacts fail closed.
- [ ] The loader verifies bytes before activation.
- [ ] A build can be hosted at root or a non-root static base path.
- [ ] Manifest canonicalization is covered by cross-language golden vectors.
- [ ] Error reporting identifies the failing artifact without dumping sensitive content.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Golden manifest vectors.
- Tamper/truncation/mix-and-match tests.
- Root and subpath deployment smoke.
- Cache reload tests.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Manifest schema.
- [ ] Golden vectors.
- [ ] Tamper-test logs.
- [ ] Artifact inventory and hashes.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Code signing beyond the frozen threat model unless separately approved.
- Assuming TLS alone replaces artifact binding.
- Loading unverified modules first and checking later.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-b-002:`.
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

---

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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/04_build_deploy/BWASM-B-004-add-service-worker-adapter-and-static-host-bootstrap.md`  
Program: `BWASM`  
Phase: `04_build_deploy` — Compiler, artifacts, and static deployment  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-B-004 — Add Service Worker adapter and static-host bootstrap

## Atomic goal

Expose a built Velqu browser application through scoped Service Worker fetch interception on static hosting.

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

- `BWASM-R-002` — Implement Fetch-compatible browser dispatcher
- `BWASM-B-002` — Define content-addressed browser artifact manifest and loader
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

1. Provide generated bootstrap code that registers, installs, verifies, and activates the runtime under an explicit scope.
2. Route only owned application requests to BrowserRuntime.fetch.
3. Define navigation, API, asset, form, redirect, cache, and offline behavior.
4. Avoid intercepting editor/auth/model-gateway traffic outside the preview scope.
5. Provide a Worker-only/injected-fetch fallback for unsupported or embedded environments.

## Acceptance criteria

- [ ] A static HTTPS deployment serves application routes without a Velqu application server.
- [ ] Scope escape and unrelated-origin requests are not intercepted.
- [ ] First install, reload, update, offline reload, and unregister behaviors are deterministic.
- [ ] Redirects and forms behave according to the support matrix.
- [ ] Failure to register Service Worker produces an actionable fallback/error, not a hanging preview.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Chromium/Firefox/WebKit registration and fetch tests according to the frozen matrix.
- Root/subpath/static-host fixtures.
- Offline/reload/update tests.
- Scope-escape tests.
- Worker-only fallback smoke.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Static deployment recording.
- [ ] Service Worker lifecycle logs.
- [ ] Scope test results.
- [ ] Fallback-mode evidence.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Claiming that static hosting means zero hosting.
- Intercepting provider gateway or editor control-plane requests.
- Using Service Worker as the only test path.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-b-004:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/04_build_deploy/BWASM-B-005-add-cli-build-preview-inspect-and-export-workflows.md`  
Program: `BWASM`  
Phase: `04_build_deploy` — Compiler, artifacts, and static deployment  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-B-005 — Add CLI build, preview, inspect, and export workflows

## Atomic goal

Give developers a coherent CLI path from Velqu source to a browser-WASM static deployment.

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
- `BWASM-B-002` — Define content-addressed browser artifact manifest and loader
- `BWASM-B-004` — Add Service Worker adapter and static-host bootstrap

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/compiler/src/index.ts`
- `packages/compiler/src/extract.ts`
- `packages/compiler/src/emit.ts`
- `packages/cli/src/index.ts`
- `packages/browser-runtime/`
- `.github/workflows/verify.yml`

## Steps

1. Add or extend commands for build --target browser-wasm, preview, inspect, and export.
2. Show target compatibility, artifact sizes, declared capabilities, deployment-only requirements, ABI versions, and digests.
3. Make preview serve only static generated bytes plus development diagnostics.
4. Support machine-readable JSON output for CI.
5. Document clean-build, cache, base-path, and source-map controls.

## Acceptance criteria

- [ ] A clean sample follows one documented command sequence to build and preview.
- [ ] Inspect detects tampered or mixed artifact sets.
- [ ] JSON output is schema-versioned and fixture-tested.
- [ ] CLI exits nonzero for unsupported imports/capabilities or failed integrity checks.
- [ ] Preview mode is not required in production and does not hide external server dependencies.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- CLI integration tests.
- Clean scaffold fixture.
- JSON snapshot tests.
- Tampered artifact inspect test.
- Static export deployment smoke.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] CLI transcript.
- [ ] Generated artifact inventory.
- [ ] JSON output fixtures.
- [ ] Clean-consumer log.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Building a hosted control plane.
- Bundling provider credentials.
- Treating the development static server as production Velqu runtime.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-b-005:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/04_build_deploy/BWASM-B-006-verify-cache-activation-upgrades-rollback-and-static-deployment.md`  
Program: `BWASM`  
Phase: `04_build_deploy` — Compiler, artifacts, and static deployment  
Mode: `VERIFY_OR_FIX` — Verify first, fix defects within this issue's bounded area, and preserve before/after evidence.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-B-006 — Verify cache activation, upgrades, rollback, and static deployment

## Atomic goal

Prove browser-WASM builds activate atomically and can upgrade or roll back on static hosts without mixed application state.

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

- `BWASM-B-004` — Add Service Worker adapter and static-host bootstrap
- `BWASM-B-005` — Add CLI build, preview, inspect, and export workflows

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/compiler/src/index.ts`
- `packages/compiler/src/extract.ts`
- `packages/compiler/src/emit.ts`
- `packages/cli/src/index.ts`
- `packages/browser-runtime/`
- `.github/workflows/verify.yml`

## Steps

1. Create N, N+1, and rollback fixtures with incompatible route/schema/handler changes.
2. Exercise cold install, warm reload, multiple tabs, interrupted download, partial CDN propagation, and offline state.
3. Ensure a build becomes active only after all bound artifacts verify.
4. Define client notification/reload behavior when an update is ready or incompatible.
5. Test at least the frozen supported static host shapes and base paths.

## Acceptance criteria

- [ ] No request executes against a mixed N/N+1 artifact set.
- [ ] Interrupted or corrupt updates leave the last known-good build usable where policy permits.
- [ ] Rollback restores a fully coherent build.
- [ ] Multiple tabs converge according to the documented activation policy.
- [ ] Cache cleanup does not remove artifacts still required by an active client.
- [ ] All discovered defects are fixed or linked as blockers with reproductions.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Playwright multi-context lifecycle suite.
- CDN partial-propagation simulator.
- Offline/interrupted update tests.
- Rollback rehearsal.
- Static host matrix.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Upgrade/rollback report.
- [ ] Browser traces.
- [ ] Artifact/cache inventories before and after each scenario.
- [ ] Known-residual-risk register.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Application data migrations beyond documented adapter hooks.
- Hand-waving eventual cache consistency.
- Declaring success from a single clean first load.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-b-006:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/05_capabilities/BWASM-C-001-implement-browser-safe-timer-crypto-logging-and-restricted-fetch-capabilities.md`  
Program: `BWASM`  
Phase: `05_capabilities` — Browser capabilities and persistence  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-C-001 — Implement browser-safe timer, crypto, logging, and restricted fetch capabilities

## Atomic goal

Provide the mandatory browser capability baseline with explicit security and resource policy.

## Parent intent

Expose only browser-safe capabilities, support explicit local persistence, and fail closed for production-only requirements.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-R-005` — Integrate capability registry and Treaty with the browser runtime
- `BWASM-D-003` — Define the browser execution threat model and isolation contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-capabilities/`
- `crates/q-capability-postgres/`
- `packages/browser-runtime/`

## Steps

1. Implement timer using browser scheduling with deadline and cancellation propagation.
2. Implement Web Crypto-backed random/digest primitives only where semantics match the native contract.
3. Implement bounded structured logging with redaction, levels, correlation IDs, and host forwarding.
4. Implement outbound fetch with default-deny policy, origin/method/header/body/response limits, timeout, redirect, and credential controls.
5. Version and declare each adapter in the browser artifact manifest.

## Acceptance criteria

- [ ] No adapter exposes editor credentials, ambient cookies, storage, DOM, or unrestricted network access.
- [ ] Timer and fetch stop or discard work after cancellation according to the contract.
- [ ] Crypto mismatch with native algorithms is rejected or documented; it is not silently substituted.
- [ ] Log and response floods are bounded and produce structured limit errors.
- [ ] Fetch credentials default to omit and redirects cannot escape policy.
- [ ] Capability availability is introspectable before handler execution.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Capability unit tests.
- Cancellation and timeout races.
- Network allow/deny/redirect/credential matrix.
- Log-flood and oversized-response tests.
- Native/browser contract fixtures where semantics overlap.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Capability conformance matrix.
- [ ] Network-policy traces.
- [ ] Limit/cancellation logs.
- [ ] Adapter manifest examples.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Giving previews unrestricted internet access.
- Implementing server secrets in browser.
- Claiming cryptographic equivalence without shared vectors.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-c-001:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/05_capabilities/BWASM-C-002-make-the-postgres-capability-contract-asynchronous-before-browser-freeze.md`  
Program: `BWASM`  
Phase: `05_capabilities` — Browser capabilities and persistence  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-C-002 — Make the Postgres capability contract asynchronous before browser freeze

## Atomic goal

Change the public Postgres capability to an async contract that can be implemented safely by both native Postgres and browser-local adapters.

## Parent intent

Expose only browser-safe capabilities, support explicit local persistence, and fail closed for production-only requirements.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-D-001` — Freeze the Browser-WASM product and runtime contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-capabilities/`
- `crates/q-capability-postgres/`
- `packages/browser-runtime/`

## Steps

1. Change sql/query operations to return Promise-based results in TypeScript authoring types.
2. Update compiler analysis, generated types/contracts, native bridge, testing helpers, examples, and documentation.
3. Define cancellation, deadline, transaction, result-value, error, and row-count semantics.
4. Provide migration diagnostics or a codemod where synchronous-looking use was previously accepted.
5. Lock the API before browser capability adapters depend on it.

## Acceptance criteria

- [ ] All official examples use await and compile against the new contract.
- [ ] Native Postgres behavior and errors remain covered by integration tests.
- [ ] Generated handler code cannot accidentally serialize an unresolved Promise.
- [ ] Migration guidance identifies every affected public API.
- [ ] A beta API snapshot records the async contract.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Typecheck all packages/examples.
- Compiler fixtures.
- Native Postgres integration tests.
- Migration fixture from old to new API.
- Public API snapshot.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] API diff.
- [ ] Migration guide/codemod transcript.
- [ ] Native integration logs.
- [ ] Updated contract fixtures.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Adding browser PGlite itself.
- Hiding the break through unsafe any types.
- Freezing two incompatible sync/async database APIs.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-c-002:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/05_capabilities/BWASM-C-003-add-optional-pglite-backed-local-sql-capability.md`  
Program: `BWASM`  
Phase: `05_capabilities` — Browser capabilities and persistence  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P1`  
Optional: `YES — excluded from the MVP release gate unless an owner decision promotes it before candidate freeze.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-C-003 — Add optional PGlite-backed local SQL capability

## Atomic goal

Offer an opt-in browser-local SQL adapter for prototype applications without pretending it is production PostgreSQL infrastructure.

## Parent intent

Expose only browser-safe capabilities, support explicit local persistence, and fail closed for production-only requirements.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-C-002` — Make the Postgres capability contract asynchronous before browser freeze
- `BWASM-R-005` — Integrate capability registry and Treaty with the browser runtime
- `BWASM-B-001` — Add compiler target browser-wasm

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-capabilities/`
- `crates/q-capability-postgres/`
- `packages/browser-runtime/`

## Steps

1. Create an optional package/adapter backed by PGlite or an owner-approved equivalent.
2. Support memory and explicitly enabled IndexedDB persistence modes.
3. Map the frozen Velqu Postgres capability subset to the browser adapter.
4. Define unsupported SQL/extensions/concurrency/transaction behavior.
5. Expose database reset/export/import hooks for preview UX and tests.
6. Lazy-load the database WASM/assets so projects without SQL do not pay the payload cost.

## Acceptance criteria

- [ ] Supported SQL fixtures behave according to the documented capability subset.
- [ ] Unsupported operations fail with stable, actionable codes.
- [ ] Persistence is isolated by project and origin namespace.
- [ ] The adapter never claims multi-user durability, production availability, or native Postgres performance.
- [ ] Projects without the capability do not download or instantiate database assets.
- [ ] Database bytes and versions are integrity-bound to the browser build.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- SQL compatibility fixture corpus.
- Memory and IndexedDB persistence tests.
- Project-isolation tests.
- Lazy-load/network trace.
- Export/import/reset tests.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] SQL support matrix.
- [ ] Payload/network measurements.
- [ ] Persistence/isolation logs.
- [ ] Compatibility test results.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Full PostgreSQL parity.
- Shared multi-user database.
- Server-side secrets or remote database credentials.
- Making PGlite mandatory for the core Browser-WASM runtime.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-c-003:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/05_capabilities/BWASM-C-004-add-namespaced-indexeddb-kv-persistence-capability.md`  
Program: `BWASM`  
Phase: `05_capabilities` — Browser capabilities and persistence  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-C-004 — Add namespaced IndexedDB KV persistence capability

## Atomic goal

Provide the small mandatory local-persistence primitive for browser previews without requiring a SQL engine.

## Parent intent

Expose only browser-safe capabilities, support explicit local persistence, and fail closed for production-only requirements.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-R-005` — Integrate capability registry and Treaty with the browser runtime

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-capabilities/`
- `crates/q-capability-postgres/`
- `packages/browser-runtime/`

## Steps

1. Define a versioned async KV capability with get, set, delete, list/prefix, clear, and optional transaction/batch semantics.
2. Implement a memory adapter and a namespaced IndexedDB adapter.
3. Define serialization, quotas, schema/version migration, cancellation, and error behavior.
4. Namespace all data by application/build/project identity according to the product contract.
5. Provide explicit export, reset, and garbage-collection controls.

## Acceptance criteria

- [ ] Memory and IndexedDB adapters pass one shared contract suite.
- [ ] One project cannot enumerate or read another project's keys.
- [ ] Quota, serialization, migration, and blocked-database failures are structured.
- [ ] Upgrading an application does not silently erase data outside declared migration policy.
- [ ] Private/incognito or unavailable IndexedDB conditions have a documented fallback/error.
- [ ] No preview data is represented as production-durable or multi-user.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Shared adapter contract tests.
- Real-browser IndexedDB tests.
- Cross-project namespace tests.
- Quota/migration/blocked-upgrade tests.
- Export/reset/GC tests.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] KV API specification.
- [ ] Browser traces.
- [ ] Migration fixtures.
- [ ] Isolation and quota results.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Distributed consistency.
- Cross-device synchronization.
- Using localStorage for unbounded application state.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-c-004:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/05_capabilities/BWASM-C-005-fail-closed-for-deployment-required-and-unavailable-capabilities.md`  
Program: `BWASM`  
Phase: `05_capabilities` — Browser capabilities and persistence  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-C-005 — Fail closed for deployment-required and unavailable capabilities

## Atomic goal

Turn unsupported production integrations into explicit build/runtime outcomes that can drive deployment UX and pricing.

## Parent intent

Expose only browser-safe capabilities, support explicit local persistence, and fail closed for production-only requirements.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-R-005` — Integrate capability registry and Treaty with the browser runtime
- `BWASM-B-003` — Enforce browser import policy with source-located diagnostics
- `BWASM-C-001` — Implement browser-safe timer, crypto, logging, and restricted fetch capabilities
- `BWASM-C-002` — Make the Postgres capability contract asynchronous before browser freeze
- `BWASM-C-004` — Add namespaced IndexedDB KV persistence capability

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `packages/core/src/index.ts`
- `packages/testing/src/index.ts`
- `crates/q-capabilities/`
- `crates/q-capability-postgres/`
- `packages/browser-runtime/`

## Steps

1. Define capability portability states: browser, browser-and-native, simulated, deployment-required, and forbidden.
2. Record states and required configuration in compiler artifacts.
3. Fail at build time when capability usage is statically known and impossible for the selected target.
4. Fail before side effects at runtime when availability depends on host configuration.
5. Return a stable deployment-required problem shape containing capability ID, route ID, reason code, and safe remediation metadata.
6. Expose a machine-readable deployment-requirements summary for CLI and app-builder UI.

## Acceptance criteria

- [ ] Secrets, real remote Postgres credentials, payments, email delivery, public webhooks, cron, and durable queues are never silently mocked unless an explicit simulation profile is selected.
- [ ] Deployment-required responses contain no secret values or provider-specific private data.
- [ ] Build, inspect, runtime, and Treaty surfaces agree on the capability classification.
- [ ] The app-builder can determine whether deployment is required without executing the route.
- [ ] Capability checks happen before handler or adapter side effects.
- [ ] Unknown classifications fail closed.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Capability classification golden fixtures.
- Build-time and runtime negative tests.
- Side-effect ordering tests.
- Treaty problem decoding tests.
- CLI JSON requirements snapshot.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Capability portability registry.
- [ ] Problem schema and examples.
- [ ] Build/runtime consistency report.
- [ ] No-side-effect proof logs.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Implementing billing or pricing logic in Velqu core.
- Returning fake successful payment/email/database operations by default.
- Treating every unavailable capability as a generic 500.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-c-005:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-001-build-shared-native-versus-browser-conformance-and-differential-suites.md`  
Program: `BWASM`  
Phase: `06_quality_release` — Conformance, security, DevEx, and release qualification  
Mode: `VERIFY_OR_FIX` — Verify first, fix defects within this issue's bounded area, and preserve before/after evidence.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-Q-001 — Build shared native-versus-browser conformance and differential suites

## Atomic goal

Prove that compatibility-critical behavior is shared or explicitly classified across native Velqu and Browser-WASM.

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

- `BWASM-K-006` — Verify and package portable-kernel evidence
- `BWASM-R-006` — Verify and package browser-runtime evidence
- `BWASM-B-006` — Verify cache activation, upgrades, rollback, and static deployment
- `BWASM-C-005` — Fail closed for deployment-required and unavailable capabilities

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `scripts/verify`
- `.github/workflows/verify.yml`
- `conformance/`
- `packages/browser-runtime/`
- `crates/q-browser-kernel/`
- `docs/`

## Steps

1. Create a single fixture corpus for routes, methods, params, query, bodies, headers, schema validation, status declarations, problem responses, Treaty calls, and capability authorization.
2. Run each applicable fixture through native runtime and browser runtime.
3. Canonicalize only approved nondeterministic fields before comparison.
4. Classify each fixture as exact parity, equivalent-by-contract, browser-only, native-only, or unsupported.
5. Fail CI on unreviewed drift.

## Acceptance criteria

- [ ] Every public Browser-WASM behavior has at least one conformance fixture.
- [ ] Route and schema compatibility-critical paths use the Rust/WASM kernel.
- [ ] Differences are linked to a frozen support-matrix entry and owner decision.
- [ ] The suite detects intentional mutation of routing, validation, status, or problem semantics.
- [ ] Results include exact source commit, native binary hash, WASM hash, and browser versions.
- [ ] No broad snapshot update can approve unrelated drift silently.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Native/browser differential runner.
- Mutation/sensitivity tests.
- Contract and Treaty type fixtures.
- Full repository verification.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Machine-readable conformance matrix.
- [ ] Raw native/browser outputs.
- [ ] Mutation-test report.
- [ ] Artifact/toolchain hashes.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Requiring parity for transport features browsers cannot expose.
- Normalizing away substantive semantic differences.
- Using only happy-path routes.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-q-001:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-002-add-real-browser-ci-lanes-and-supported-browser-evidence.md`  
Program: `BWASM`  
Phase: `06_quality_release` — Conformance, security, DevEx, and release qualification  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-Q-002 — Add real-browser CI lanes and supported-browser evidence

## Atomic goal

Run Browser-WASM tests in real browsers and bind support claims to maintained CI evidence.

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
- `BWASM-B-006` — Verify cache activation, upgrades, rollback, and static deployment
- `BWASM-D-004` — Ratify support matrix, compatibility claims, and release budgets

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `scripts/verify`
- `.github/workflows/verify.yml`
- `conformance/`
- `packages/browser-runtime/`
- `crates/q-browser-kernel/`
- `docs/`

## Steps

1. Add wasm-bindgen/browser and end-to-end lanes for the browsers/platforms selected in BWASM-D-004.
2. Run kernel, dispatcher, Worker, Service Worker, IndexedDB, cache/update, and static-host smoke suites.
3. Separate required lanes from allowed-failure experimental lanes.
4. Capture browser engine/version, OS/architecture, feature flags, and artifacts.
5. Add scheduled or release-candidate coverage where the normal PR matrix is intentionally smaller.

## Acceptance criteria

- [ ] Every claimed supported browser has a blocking evidence lane.
- [ ] A browser absent from evidence is marked unverified/unsupported rather than implicitly supported.
- [ ] CI tests actual emitted release-like artifacts, not development source imports only.
- [ ] Failures upload enough logs/traces/artifacts for diagnosis.
- [ ] Experimental lanes cannot satisfy a release gate.
- [ ] Matrix ownership and update cadence are documented.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Real-browser kernel tests.
- Playwright/WebDriver end-to-end suite.
- Static-host and Service Worker suite.
- Scheduled compatibility run.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] CI workflow definitions.
- [ ] Representative green run links/logs.
- [ ] Browser/OS matrix manifest.
- [ ] Failure artifact example.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Browser support claims based solely on standards documentation.
- Headless-only claims where headed behavior materially differs.
- Making every browser/version blocking without an owner decision.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-q-002:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-003-verify-isolated-preview-origin-and-untrusted-code-security-boundaries.md`  
Program: `BWASM`  
Phase: `06_quality_release` — Conformance, security, DevEx, and release qualification  
Mode: `VERIFY_OR_FIX` — Verify first, fix defects within this issue's bounded area, and preserve before/after evidence.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-Q-003 — Verify isolated preview-origin and untrusted-code security boundaries

## Atomic goal

Demonstrate that generated preview code cannot cross the documented editor, credential, network, storage, or project boundaries.

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

- `BWASM-R-004` — Execute handlers in isolated Workers with cancellation and hard recovery
- `BWASM-B-004` — Add Service Worker adapter and static-host bootstrap
- `BWASM-C-001` — Implement browser-safe timer, crypto, logging, and restricted fetch capabilities
- `BWASM-D-003` — Define the browser execution threat model and isolation contract

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `scripts/verify`
- `.github/workflows/verify.yml`
- `conformance/`
- `packages/browser-runtime/`
- `crates/q-browser-kernel/`
- `docs/`

## Steps

1. Build a production-shaped two-origin fixture: editor/control plane and isolated preview origin.
2. Apply sandboxed iframe policy, strict CSP, Permissions Policy, COOP/COEP only where required, and validated postMessage schemas.
3. Attempt DOM escape, parent access, credential/cookie access, provider-key theft, network exfiltration, import bypass, storage crossover, Service Worker scope escape, and message confusion.
4. Test malicious logs, stack traces, redirects, URLs, headers, HTML, and oversized messages.
5. Commission an independent review of the implemented threat model and claims.

## Acceptance criteria

- [ ] Preview code cannot read editor-origin DOM, storage, authentication material, or provider secrets.
- [ ] Default network policy blocks unapproved exfiltration paths.
- [ ] Service Worker scope cannot control the editor/control-plane origin.
- [ ] All cross-origin messages are origin-, schema-, project-, and invocation-validated.
- [ ] Known browser limitations and residual risks are explicit.
- [ ] No document claims a hostile-code sandbox unless the independent review supports that exact claim.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Adversarial browser suite.
- CSP/Permissions Policy reporting tests.
- Cross-origin storage/cookie tests.
- Service Worker scope attacks.
- Dependency/security scan.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Threat-model verification report.
- [ ] Independent reviewer findings and disposition.
- [ ] CSP/network traces.
- [ ] Residual-risk register.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Assuming WebAssembly or Workers are automatically secure sandboxes.
- Testing only same-origin development mode.
- Suppressing exploit evidence after a fix.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-q-003:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-004-add-browser-wasm-observability-and-developer-diagnostics.md`  
Program: `BWASM`  
Phase: `06_quality_release` — Conformance, security, DevEx, and release qualification  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P1`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-Q-004 — Add Browser-WASM observability and developer diagnostics

## Atomic goal

Make browser runtime failures understandable without exposing sensitive data or requiring native-runtime debugging tools.

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

- `BWASM-R-006` — Verify and package browser-runtime evidence
- `BWASM-B-005` — Add CLI build, preview, inspect, and export workflows

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `scripts/verify`
- `.github/workflows/verify.yml`
- `conformance/`
- `packages/browser-runtime/`
- `crates/q-browser-kernel/`
- `docs/`

## Steps

1. Define structured lifecycle events for load, verify, instantiate, route, validate, invoke, capability, persist, cache, update, cancel, and fail.
2. Add stable diagnostic codes and correlation IDs across loader, kernel, runtime, Worker, Service Worker, Treaty, and CLI.
3. Provide a bounded developer event stream and optional inspector panel adapter.
4. Map generated errors back to source locations and route IDs.
5. Add redaction and production-preview logging defaults.

## Acceptance criteria

- [ ] A developer can distinguish integrity, compatibility, route, schema, capability, handler, timeout, persistence, cache, and deployment-required failures.
- [ ] Diagnostics correlate one request across Worker and Service Worker boundaries.
- [ ] Secrets, authorization headers, cookies, SQL values, and arbitrary bodies are not logged by default.
- [ ] Logs and traces are bounded and can be exported for issue evidence.
- [ ] Diagnostic codes are documented and snapshot-tested.
- [ ] Observability can be disabled or reduced for shipped static applications.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Diagnostic-code snapshot tests.
- Redaction corpus.
- Cross-boundary correlation tests.
- Log-flood limit tests.
- Source-map error fixture.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Diagnostic catalog.
- [ ] Example exported trace.
- [ ] Redaction test report.
- [ ] Inspector screenshots.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- A hosted telemetry service.
- Collecting user application data by default.
- Using free-form console text as the only diagnostic contract.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-q-004:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-005-set-and-enforce-wasm-size-startup-latency-and-leak-budgets.md`  
Program: `BWASM`  
Phase: `06_quality_release` — Conformance, security, DevEx, and release qualification  
Mode: `VERIFY_OR_FIX` — Verify first, fix defects within this issue's bounded area, and preserve before/after evidence.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-Q-005 — Set and enforce WASM size, startup, latency, and leak budgets

## Atomic goal

Turn browser feasibility into measurable release budgets rather than an unbounded payload/performance claim.

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

- `BWASM-K-006` — Verify and package portable-kernel evidence
- `BWASM-R-006` — Verify and package browser-runtime evidence
- `BWASM-B-006` — Verify cache activation, upgrades, rollback, and static deployment
- `BWASM-D-004` — Ratify support matrix, compatibility claims, and release budgets

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `scripts/verify`
- `.github/workflows/verify.yml`
- `conformance/`
- `packages/browser-runtime/`
- `crates/q-browser-kernel/`
- `docs/`

## Steps

1. Measure compressed/uncompressed kernel, glue, handler, and optional-capability sizes separately.
2. Measure cold/warm load, verification, compilation/instantiation, first request, steady request, Worker restart, and update activation.
3. Measure memory growth across repeated requests, failures, aborts, worker restarts, and route/schema corpora.
4. Run on the device/browser tiers selected in BWASM-D-004.
5. Add blocking budgets and a documented process for intentional budget changes.

## Acceptance criteria

- [ ] Core projects do not download optional SQL or parity-engine assets.
- [ ] Every blocking metric has a command, raw sample set, percentile/statistic definition, environment, and threshold.
- [ ] CI or candidate verification detects material size/startup regressions.
- [ ] No unbounded memory growth remains in the defined soak scenario.
- [ ] Results are not represented as native-runtime throughput benchmarks.
- [ ] Budget exceptions require an owner decision and before/after evidence.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Artifact size gate.
- Cold/warm browser benchmark harness.
- Repeated-request and Worker-restart soak.
- Memory/leak instrumentation.
- Optional-capability lazy-load trace.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Raw samples and statistics.
- [ ] Environment/device/browser manifest.
- [ ] Artifact size inventory.
- [ ] Regression-gate output.
- [ ] Accepted budget-change decisions.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Marketing benchmarks without reproducible raw data.
- Comparing browser-local requests directly to network-server throughput.
- One high-end desktop as the only device tier.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-q-005:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-006-publish-browser-wasm-documentation-limitations-and-migration-guide.md`  
Program: `BWASM`  
Phase: `06_quality_release` — Conformance, security, DevEx, and release qualification  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-Q-006 — Publish Browser-WASM documentation, limitations, and migration guide

## Atomic goal

Give external users an accurate end-to-end path and prevent Browser-WASM preview from being mistaken for native production equivalence.

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

- `BWASM-B-005` — Add CLI build, preview, inspect, and export workflows
- `BWASM-C-005` — Fail closed for deployment-required and unavailable capabilities
- `BWASM-Q-004` — Add Browser-WASM observability and developer diagnostics

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `scripts/verify`
- `.github/workflows/verify.yml`
- `conformance/`
- `packages/browser-runtime/`
- `crates/q-browser-kernel/`
- `docs/`

## Steps

1. Document architecture, build/deploy flow, static-host requirements, base paths, HTTPS/Service Worker requirements, and fallback mode.
2. Document supported APIs, bodies, streaming, headers, cookies, persistence, capabilities, browser matrix, limits, and diagnostics.
3. Document browser preview versus native deployment semantics and evidence classes.
4. Provide migration guidance for async Postgres and deployment-required capabilities.
5. Add quickstart, troubleshooting, security model, upgrade/rollback, and app-builder integration examples.
6. Add an explicit unsupported/experimental section, including QuickJS-WASM status.

## Acceptance criteria

- [ ] A new user can build, statically host, exercise, inspect, update, and reset a sample without private repository knowledge.
- [ ] No doc says 'serverless', 'zero server', 'sandbox', 'Postgres compatible', or 'production parity' without precise qualification.
- [ ] Every public diagnostic code and capability state is documented.
- [ ] Examples are executed in CI from published/generated artifacts.
- [ ] Native deployment remains the documented path for production-only capabilities.
- [ ] Support claims exactly match BWASM-D-004 and candidate evidence.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Documentation link/checker.
- Executable quickstart.
- Migration fixture.
- Terminology/claim lint.
- Clean-reader usability run.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Rendered documentation output.
- [ ] Quickstart transcript.
- [ ] Migration diff.
- [ ] Claim-audit report.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Promising unsupported browser/runtime behavior.
- Using internal design history as a prerequisite.
- Hiding limitations in a separate obscure document.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-q-006:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/06_quality_release/BWASM-Q-007-run-an-external-cleanroom-static-deployment-and-offline-exercise.md`  
Program: `BWASM`  
Phase: `06_quality_release` — Conformance, security, DevEx, and release qualification  
Mode: `EVIDENCE` — Package evidence from one exact candidate; do not mix implementation and attestation.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-Q-007 — Run an external cleanroom static deployment and offline exercise

## Atomic goal

Prove that an external consumer can build and deploy a useful Velqu Browser-WASM application using only public artifacts and documentation.

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

- `BWASM-Q-002` — Add real-browser CI lanes and supported-browser evidence
- `BWASM-Q-003` — Verify isolated preview-origin and untrusted-code security boundaries
- `BWASM-Q-005` — Set and enforce WASM size, startup, latency, and leak budgets
- `BWASM-Q-006` — Publish Browser-WASM documentation, limitations, and migration guide

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `scripts/verify`
- `.github/workflows/verify.yml`
- `conformance/`
- `packages/browser-runtime/`
- `crates/q-browser-kernel/`
- `docs/`

## Steps

1. Use a fresh external repository and participant/agent not involved in implementation.
2. Install release-candidate packages/artifacts without workspace links or source-path fallbacks.
3. Build a bounded CRUD-style app with route params, schema validation, Treaty, timer/logging, local KV, and at least one deployment-required capability.
4. Deploy to a generic static HTTPS host under a non-root base path.
5. Exercise first load, forms/fetch, persistence, offline reload, update, rollback, reset/export, and deployment-required UX.
6. Record setup failures, docs gaps, iterations, ambiguity, and framework defects separately.

## Acceptance criteria

- [ ] The app works from registry/candidate artifacts only.
- [ ] No Velqu application server is running after static deployment.
- [ ] The participant can distinguish browser-local behavior from native production behavior.
- [ ] Local data persists according to the documented policy and remains project-isolated.
- [ ] Deployment-required behavior is explicit and machine-readable.
- [ ] Every blocking defect is fixed and re-proven or leaves the candidate NO-GO.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Clean clone/install/build transcript.
- Static host network/process inventory.
- Browser user-journey suite.
- Offline/update/rollback rehearsal.
- External usability notes.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] External repository commit.
- [ ] Package/artifact lock and hashes.
- [ ] Deployment recording and network trace.
- [ ] Participant report and defect disposition.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Using monorepo workspaces or unpublished package sources.
- Treating implementer familiarity as usability evidence.
- Replacing failed journeys with manual claims.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-q-007:`.
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

---

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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/07_optional_parity/BWASM-X-001-spike-quickjs-ng-in-wasm-engine-parity-and-record-go-or-no-go.md`  
Program: `BWASM`  
Phase: `07_optional_parity` — Optional QuickJS-NG WASM parity  
Mode: `VERIFY_OR_FIX` — Verify first, fix defects within this issue's bounded area, and preserve before/after evidence.  
Priority: `P1`  
Optional: `YES — excluded from the MVP release gate unless an owner decision promotes it before candidate freeze.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-X-001 — Spike QuickJS-NG-in-WASM engine parity and record GO or NO-GO

## Atomic goal

Determine whether executing handlers in QuickJS-NG compiled to browser WASM materially improves Velqu parity enough to justify its cost.

## Parent intent

Investigate closer QuickJS-NG engine parity without silently making it a prerequisite for the practical MVP.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-K-005` — Implement the Rust Browser Kernel and wasm-bindgen ABI
- `BWASM-R-004` — Execute handlers in isolated Workers with cancellation and hard recovery
- `BWASM-D-004` — Ratify support matrix, compatibility claims, and release budgets

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `crates/q-engine/src/lib.rs`
- `crates/q-engine-quickjs/src/lib.rs`
- `crates/q-engine-quickjs/Cargo.toml`
- `packages/browser-runtime/`

## Steps

1. Identify a maintainable QuickJS-NG browser-WASM build path and exact engine version strategy.
2. Implement a bounded prototype that loads one compiled handler bundle through the frozen handler ABI.
3. Compare JavaScript semantics, startup, payload, memory, cancellation, debugging, CSP requirements, and maintenance risk against native browser Worker execution.
4. Test interoperability with the Rust/WASM kernel and browser capability bridge.
5. Record blockers in rquickjs/upstream/toolchain integration without hiding them behind a custom fork.
6. Produce a scored GO/NO-GO decision and an adoption plan only if thresholds are met.

## Acceptance criteria

- [ ] The spike uses reproducible source/toolchain references and does not masquerade as production support.
- [ ] Engine-version mismatch with native Velqu is measured and explicitly classified.
- [ ] Payload/startup/memory costs are compared using raw evidence.
- [ ] Infinite loop/cancellation/recovery behavior is demonstrated.
- [ ] A GO decision identifies ownership, update cadence, security review, release budget, and fallback behavior.
- [ ] A NO-GO decision leaves the default Worker-based Browser-WASM target unaffected.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Engine semantic fixture corpus.
- Cold/warm benchmark.
- Loop/cancellation/recovery tests.
- CSP and browser matrix smoke.
- Prototype reproducibility build.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Prototype source and artifact hashes.
- [ ] Version/toolchain inventory.
- [ ] Comparative benchmark and semantic report.
- [ ] GO/NO-GO decision record.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Making QuickJS-WASM a hidden dependency of the MVP.
- Claiming same-engine parity with a different QuickJS-NG version.
- Maintaining an unreviewed permanent fork by default.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-x-001:`.
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

---

Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/gates/BWASM-GATE-browser-wasm-beta-readiness-go-or-no-go.md`  
Program: `BWASM`  
Phase: `08_gate` — Release gate  
Mode: `GATE_REVIEW` — Review an exact candidate and issue a binary GO/NO-GO verdict.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `TODO`

---

# BWASM-GATE — Browser-WASM beta readiness GO or NO-GO

## Atomic goal

Review one exact Browser-WASM candidate and record an unambiguous GO or NO-GO against the frozen product contract.

## Parent intent

Review one exact candidate and make an unambiguous GO or NO-GO decision.

## Architecture invariant

This work targets a **hybrid Browser-WASM runtime**:

- compatibility-critical routing, schema validation, manifest/QPack verification, capability authorization, and problem mapping run through Rust compiled to WebAssembly;
- generated TypeScript handlers run in an isolated browser Worker for the MVP;
- the public runtime boundary is `Request -> Promise<Response>`;
- production deployment remains the native Velqu runtime for native-only capabilities;
- QuickJS-NG-in-WASM is optional unless a recorded owner decision changes the release contract.

## Dependencies

- `BWASM-D-004` — Ratify support matrix, compatibility claims, and release budgets
- `BWASM-K-006` — Verify and package portable-kernel evidence
- `BWASM-R-006` — Verify and package browser-runtime evidence
- `BWASM-B-006` — Verify cache activation, upgrades, rollback, and static deployment
- `BWASM-C-005` — Fail closed for deployment-required and unavailable capabilities
- `BWASM-Q-001` — Build shared native-versus-browser conformance and differential suites
- `BWASM-Q-002` — Add real-browser CI lanes and supported-browser evidence
- `BWASM-Q-003` — Verify isolated preview-origin and untrusted-code security boundaries
- `BWASM-Q-005` — Set and enforce WASM size, startup, latency, and leak budgets
- `BWASM-Q-006` — Publish Browser-WASM documentation, limitations, and migration guide
- `BWASM-Q-007` — Run an external cleanroom static deployment and offline exercise
- `BWASM-Q-008` — Assemble release evidence, SBOM, checksums, provenance, and candidate packet

Do not begin implementation while a mandatory dependency that defines this issue's contract is unresolved.

## Read first

- `AGENTS.md`
- `README.md`
- `scripts/verify`
- `.github/workflows/verify.yml`
- `docs/codex-spark-browser-wasm/`

## Steps

1. Verify exact candidate commit, artifact inventory, checksums, SBOM, provenance, package bytes, and evidence index.
2. Review all mandatory child issues and dependency closures.
3. Confirm supported-browser lanes, conformance classes, security review, performance budgets, external cleanroom, static deployment, update/rollback, docs, and residual risks.
4. Confirm native Velqu remains green and its public/runtime behavior has not regressed outside approved migration.
5. Classify every open P0/P1 and obtain explicit owner acceptance where policy allows.
6. Record the final decision, release channel, rollback path, monitoring plan, and claims allowed after the decision.

## Acceptance criteria

- [ ] All mandatory dependencies are closed with evidence from the exact candidate.
- [ ] Zero unresolved P0 blockers exist.
- [ ] Every unresolved P1 is either a documented NO-GO blocker or explicitly accepted with owner, rationale, scope, and expiry.
- [ ] Static deployment works without a Velqu application server on every claimed browser/host shape.
- [ ] Security language does not overclaim hostile-code sandboxing.
- [ ] Optional BWASM-C-003 and BWASM-X-001 do not block MVP unless an owner decision made them mandatory before candidate freeze.
- [ ] The issue ends with exactly one prominent verdict: GO or NO-GO.

## Targeted tests and commands

The assignee must discover the exact repository commands at implementation time and preserve them in evidence. At minimum, run or add coverage equivalent to:

- Re-run candidate packet validator.
- Verify all hashes and evidence links.
- Spot-check clean install/static deploy.
- Review required CI and security findings.
- Run final full-repository verification.

Always run the repository's canonical full verification command before handoff when the change touches executable code or release artifacts.

## Required evidence

- [ ] Signed/recorded gate review.
- [ ] Candidate SHA and artifact hashes.
- [ ] Dependency closure table.
- [ ] P0/P1 and residual-risk disposition.
- [ ] GO release instructions or NO-GO blocker list.

Evidence must include the exact source commit and, where artifacts are involved, the exact artifact hashes.

## Guardrails

- Preserve native Velqu behavior unless this issue explicitly freezes and tests a migration.
- Do not replace Rust/WASM compatibility logic with an unverified JavaScript-only implementation.
- Do not equate “no Velqu application server” with “no static hosting”.
- Do not expose provider credentials, production secrets, or ambient editor-origin authority to browser handlers.
- Do not claim hostile-code sandboxing, PostgreSQL parity, or native-runtime performance parity without the separately required evidence.
- Do not close an evidence or gate issue using self-authored implementation claims alone.

## Out of scope

- Fixing implementation inside the gate review.
- Using percentage-complete language instead of a verdict.
- Changing support claims without rerunning affected evidence.
- Closing the epic before GO is recorded.

## Commit / PR guidance

- Use a focused branch and one logically bounded PR.
- Suggested commit prefix: `bwasm-gate:`.
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
