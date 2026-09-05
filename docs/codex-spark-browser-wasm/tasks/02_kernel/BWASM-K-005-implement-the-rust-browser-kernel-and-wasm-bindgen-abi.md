Atomic Browser-WASM packet: `docs/codex-spark-browser-wasm/tasks/02_kernel/BWASM-K-005-implement-the-rust-browser-kernel-and-wasm-bindgen-abi.md`  
Program: `BWASM`  
Phase: `02_kernel` — Portable Rust/WASM kernel  
Mode: `IMPLEMENT` — Implement the bounded change and its targeted tests.  
Priority: `P0`  
Optional: `NO — mandatory for the Browser-WASM MVP.`  
Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04)  
Status: `PASS`

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

## Result (2026-09-05)

- Issue: BWASM-K-005 (#1233)
- Candidate commit: see PR; report `docs/reports/bwasm-k-005-browser-kernel-abi.md`
- q-browser-kernel: init (bounded, integrity-verified, source-bundle policy), plan_request (routing + validation + capability authz, stable problems), complete_invocation (declared-status + response-schema enforcement), authorize_capability; wasm-bindgen ABI (schema-pinned =0.2.108).
- Tests: native 15/15; ON-TARGET 14/14 (wasm32-wasip1); import audit CLEAN (2 bindgen shims only); dep audit 0 host crates.
- Artifacts (hashed in evidence/kernel/): wasm 1,731,509 raw / 572,711 gzip-9 (sha256 db72b8e8…); JS glue 2,830 gzip-9 (sha256 900618ab…).
- ⚠ BUDGET FINDING recorded (not waived): composed kernel exceeds the ≤500 KiB ratified budget; baseline for the Q-005 size packet.
- Native-only test scoping documented (Node WASI 16 MiB allocation limitation).
- Follow-ups: K-006 (#1234) packages the kernel evidence; R-phase builds the JS runtime on this ABI.
```
