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
