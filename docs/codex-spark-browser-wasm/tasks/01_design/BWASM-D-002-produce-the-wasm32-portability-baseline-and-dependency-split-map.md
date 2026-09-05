Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/01_design/BWASM-D-002-produce-the-wasm32-portability-baseline-and-dependency-split-map.md`  
Program: `BWASM`  
Phase: `01_design` — Architecture and decisions  
Mode: `VERIFY_OR_FIX` — Verify first, fix defects within this issue's bounded area, and preserve before/after evidence.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `PASS`

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

## Result (2026-09-05)

- Status: PASS. Report: `docs/reports/bwasm-d-002-wasm32-portability-baseline.md`;
  machine-readable inventory:
  `docs/codex-spark-browser-wasm/evidence/wasm32-baseline.json`;
  exact retained compiler logs:
  `docs/codex-spark-browser-wasm/evidence/wasm32/check-<crate>.log`.
- Measured: `q-schema-runtime` compiles clean on wasm32 (portable);
  q-router/q-engine/q-bridge/q-pack fail only via the `q-engine -> tokio
  -> mio` edge (split-required); `q-capabilities` fails on `getrandom`
  (feature-gate native-only); q-http/q-engine-quickjs/velqu-runtime are
  native-only by architecture.
- Smallest K-phase cuts frozen: K-001 model-type extraction from
  q-engine; K-002 byte-core split (memmap2 confined to native loader) +
  getrandom gate; K-003 router/q-engine edge cut; K-004 on-target test
  qualification; re-measure bridge+pack after cuts.
- Candidate commit: see PR (bwasm-d-002). Gates: validate-okf pass;
  verify ALL PASS (post-setup).
