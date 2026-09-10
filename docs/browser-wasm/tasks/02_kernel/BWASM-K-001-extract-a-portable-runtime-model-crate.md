Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/02_kernel/BWASM-K-001-extract-a-portable-runtime-model-crate.md`  
Program: `BWASM`  
Phase: `02_kernel` — Portable Rust/WASM kernel  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `PASS`

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

## Result (2026-09-05)

- Issue: BWASM-K-001 (#1229)
- Candidate commit: see PR; report `docs/reports/bwasm-k-001-portable-runtime-model.md`
- Files changed: `Cargo.toml`, `Cargo.lock`, `crates/q-engine/Cargo.toml`, `crates/q-engine/src/lib.rs`, new `crates/q-runtime-model/`, report
- Commands/tests: native + wasm32 model checks; 4 fixture tests; q-engine/q-router/q-pack/q-bridge regressions; validate-okf; verify ALL PASS
- Acceptance: all four criteria PASS; no unresolved in-scope P0
- Follow-ups: K-002 (#1230), K-003 (#1231), K-004 (#1232), K-005 (#1233), K-006 (#1234)
- Standing CI disclosure: local gates are the acceptance basis (`verify` workflows have zero executed steps since ~#714).
