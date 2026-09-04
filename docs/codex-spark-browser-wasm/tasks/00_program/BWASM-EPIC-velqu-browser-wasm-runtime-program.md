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
