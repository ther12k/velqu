Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/02_kernel/BWASM-K-004-qualify-the-schema-runtime-for-wasm32-and-expose-bounded-validation.md`  
Program: `BWASM`  
Phase: `02_kernel` — Portable Rust/WASM kernel  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `PASS`

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

## Result (2026-09-05)

- Issue: BWASM-K-004 (#1232)
- Candidate commit: see PR; report `docs/reports/bwasm-k-004-schema-wasm32-qualification.md`
- Qualification: full suite (67 tests) EXECUTED on-target (wasm32-wasip1, Node WASI runner committed at scripts/wasm-wasi-node-runner.mjs + .cargo/config.toml); test names and outcomes identical to native; fuzz validator ran 1.65 s of real work on-target.
- Bounded validation surface documented as the kernel contract (MAX_VALIDATE_DEPTH=64, typed problems, ordered finite errors); no-JS-fallback guardrail recorded for K-005.
- Size measured via committed probe: 1,216,002 B raw / 386,429 B gzip-9 (sha256 2b78355c…); proxy caveats (no wasm-opt/brotli on host) recorded against the ratified ≤500 KiB budget.
- Follow-ups: K-005 (#1233) must bind the kernel ABI to THIS crate (no JS validator substitution); K-006 (#1234) re-measures inside the real kernel build.
```
