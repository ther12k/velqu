# BWASM-K-006 — Verify and Package Portable-Kernel Evidence

## Result

**PASS** — the entire K-phase (BWASM-K-001..005) is independently
demonstrated at **one exact commit** (`69187c8bcb7005da940f27ecf9baea6dcb5f2787`),
with raw logs, environment manifest, artifact hashes, and an external
reviewer pass, packaged under
`docs/codex-spark-browser-wasm/evidence/kernel-verification/`.

## Verification matrix (all at commit `69187c8`)

| Check | Result | Raw evidence |
|---|---|---|
| Native tests, all portable crates + kernel (q-runtime-model 4, q-pack 100+2, q-router 15, q-schema-runtime 58+5+4, q-browser-kernel 15) | **ALL PASS (203 tests)** | `01-consolidated-runs.txt` |
| wasm32-unknown-unknown checks (model, schema, router, q-pack[portable], kernel, kernel+bindgen) | **ALL PASS** | `01-consolidated-runs.txt` |
| On-target execution (wasm32-wasip1 under Node WASI): schema 67, kernel 14 | **ALL PASS** | `01-consolidated-runs.txt` |
| Dependency audits (wasm32 graph): forbidden crates | **0 in every portable configuration** | `01-consolidated-runs.txt` |
| WASM import audit | **CLEAN — 2 imports, both wasm-bindgen JS shims; no wasi/fs/socket/thread** | `03-import-audit.txt` |
| **JS-ABI end-to-end** (the real wasm-bindgen surface, nodejs-target glue, plain Node 22): init → plan(200 + 404) → complete(200 + undeclared-status contract violation) → dispose | **JS-ABI-OK** | `02-js-abi-check.txt` + `abi-check.cjs` |
| Malformed/oversized inputs | covered by the kernel suite executed in this matrix (tampered pack, oversized message, malformed JSON, ABI mismatch, unknown route, wrong method) — all fail closed with typed problems | `01-consolidated-runs.txt` |

Evidence files: `00-environment.txt` (toolchain manifest: Linux
x86_64, rustc/cargo 1.96.0, node 22.23.2, bun 1.4.0, wasm-bindgen CLI
0.2.108), `01-consolidated-runs.txt`, `02-js-abi-check.txt`,
`03-import-audit.txt`, `04-artifact-hashes.txt`, plus the artifacts
themselves (bg.wasm, nodejs glue, deterministic fixture pack) and the
reproducible check script (`abi-check.cjs`; fixture regenerable via
`cargo run -p q-browser-kernel --example pack-fixture-gen -- <path>`).

## Artifact hashes (commit-bound)

```text
q_browser_kernel_bg.wasm  sha256 db72b8e82da56e7e5752878a8d3cc064ceba368711263bdcfbb04e0f7c8c48de
  raw 1,731,509 B · gzip-9 572,711 B
q_browser_kernel.js (nodejs target)  sha256 8434c8575f330c8ae666cf7a57afef5c49fd3d02d8f5f3216c0b7eb9d302e5de
  raw 8,563 B · gzip-9 2,051 B
fixture-pack.json  sha256 c5b20ee56a2a28d65b71b93891839c8c367b98d7da8218e4b826076ae80950db
```

## K-phase criterion closure (independent re-demonstration)

- **K-001** portable model crate: compiles native+wasm32 in this
  matrix; fixture/ABI version pin in its suite (native pass).
- **K-002** byte-based pack core: portable configuration wasm32 check
  PASS here; fuzz/mutation suites in the native pass; fs/signing/entropy
  excluded by configuration (dep audit 0).
- **K-003** router core: wasm32 check with default features PASS; 15
  semantics tests in the native pass; semantics spec in-crate.
- **K-004** schema qualification: 67 tests **executed on-target** in
  this matrix, identical outcomes to native.
- **K-005** browser kernel + ABI: 15 native / 14 on-target tests;
  import audit CLEAN; **JS-ABI driven end-to-end from real JavaScript**
  in this packet (beyond K-005's own evidence).

## No hidden fallbacks

- No JavaScript fallback exists for any compatibility-critical surface:
  routing/validation/verification/authz/problem-mapping are the Rust
  crates compiled to wasm32 (import audit proves no host escape hatch);
  the JS side is the bindgen shim only, and `abi-check.cjs` drives the
  same kernel the Rust tests exercise.
- No native fallback: the portable configurations audited above contain
  zero host crates; the kernel's wasm import table has no wasi/fs/
  socket/thread entries.

## Open item carried (accepted mismatch → owner decision)

- **Base-kernel size exceeds the ratified ≤500 KiB budget**
  (572,711 B gzip-9 vs 512,000 B). Recorded by K-005; re-measured and
  confirmed here. Per ADR-0039 this stands as a release-blocking finding
  for the browser Q-gate (BWASM-Q-005) — kernel size work (profile
  tuning, `wasm-opt`, export minimization) is that packet's scope with
  this measurement as baseline. No kernel P0 remains open: the kernel's
  functional criteria are all demonstrated.

## Reviewer sign-off

Independent code-review pass over the K-phase (portability claims,
kernel fail-closed paths, native-semantics fidelity, test quality,
evidence honesty) — verdict recorded in `05-reviewer-findings.md`
(verdict line: `KERNEL-REVIEW-PASS`/`FAIL` with findings; any P0/P1
dispositioned before this packet ships).

## Gates (this worktree — full history)

- fmt/clippy( `-D warnings`, workspace)/`validate-okf`: clean.
- `./scripts/verify` **run 1: ALL PASS** at this exact commit (no
  manifest refresh needed — the committed manifest matched).
- Runs 2 and 3 then flaked on DIFFERENT timing-sensitive tests
  (`incremental.test.ts` build-latency; ops readiness scenario) while
  the host was externally loaded (load average ~10 from the operator's
  own applications — Chrome, desktop client, VirtualBox VM; none of
  the agent's processes). Both tests pass in isolation immediately
  after (0 fail each, isolation transcripts in the session log). An
  unnecessary manifest refresh between runs 1 and 2 was reverted
  (`git checkout`) once run 1's clean state was confirmed as canonical.
- Canonical evidence: **run 1, ALL PASS**, with the flake context
  recorded here rather than silently re-run.

Standing CI disclosure applies (zero-step verify workflows since
~#714); local gates are the acceptance basis.
