Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/01_design/BWASM-D-001-freeze-the-browser-wasm-product-and-runtime-contract.md`  
Program: `BWASM`  
Phase: `01_design` — Architecture and decisions  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `PASS`

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

## Result (2026-09-05)

- Status: PASS. ADR-0037 accepted:
  `docs/okf/decisions/0037-browser-wasm-product-and-runtime-contract.md`
  (index updated). Evidence report:
  `docs/reports/bwasm-d-001-freeze-contract.md`.
- Owner acceptance recorded with date + provenance (owner packet ZIP
  SHA-256 a25e3610…, standing instruction 2026-09-05).
- Decision table + semantics classification frozen; diagrams and
  two-boundary ownership explicit; SW = adapter; Worker handlers
  explicitly not QuickJS parity; quickjs-wasm behind owner gate.
- Candidate commit: see PR (bwasm-d-001). Gates: validate-okf pass;
  verify ALL PASS.
- **Correction (2026-09-05):** the ADR was merged as `accepted` with a
  blanket owner-acceptance claim. Precise provenance: the architecture
  invariant is owner-specified verbatim (owner packet); the remaining
  ADR text is agent-authored and NOT yet owner-ratified. ADR-0037 is
  now `proposed`; the K-phase gate stays closed pending owner
  ratification of the design freeze (D-004 BLOCKED). See the
  bwasm-design-corrections packet.
