---
type: Evidence Report
title: Security and Hardening Review
status: complete
milestone: M1–M2
---

# Security and hardening review (SEC-001..005)

## Overview

Security posture review for the Velqu M0–M2 implementation covering bytecode
trust, handle lifetime, secrets redaction, input limits, and capability boundaries.

## Checklist & Findings

| Area | Requirement | Evaluation & Controls | Status |
|---|---|---|---|
| Bytecode / Pack Integrity | SEC-001 | Application pack verified via SHA-256 digests of bundle & canonical routes. Engine name, version, and ABI must match exactly before ready. | PASS |
| Same-Process Trust | SEC-002 | Same-process QuickJS is documented throughout as for **trusted application code only**. It is NOT a hostile-code sandbox. Resource limits (heap 32MB, stack 512KB, deadline interrupt) are robustness controls. | PASS |
| Handle Lifetime & Memory | SEC-003 | Handles are `(slot, generation)` pairs. Settlement increments generation, invalidating stale wrappers. No raw pointers cross FFI. Single `unsafe` block in `q-engine-quickjs` reviewed. | PASS |
| Secret Redaction | SEC-004 | Thrown error messages (e.g. `secret-boom`) and stack traces are redacted from HTTP responses (500 internal problem). Original source mapped stack logged only to server stderr. Authorization headers excluded from completion logs. | PASS |
| Capability Seam | SEC-005 | Native operations (timer) are explicitly declared in the pack. Undeclared capabilities fail pack verification. Async operations enforce timeouts and cancellation. | PASS |

## FFI Audit

- Exactly one `unsafe` block in the workspace: `crates/q-engine-quickjs/src/worker.rs:784` (`copy_nonoverlapping` to pre-allocated Uint8Array).
- All JS values are confined to `ctx.with` blocks on a single dedicated OS thread; only Send-safe `'static` Rust types cross the worker channel.

## External Parsers — property-fuzz coverage

- Schema regex matching uses `regex` crate in Rust, executed safely within bounded length constraints.
- HTTP header parsing uses `httparse` via `hyper`.
- JSON parsing uses `serde_json` with depth and length bounds.

Property-based robustness tests run in every `cargo test` invocation
(deterministic PRNG, no external dependency):

| Parser | Test | Iterations | Result |
|---|---|---:|---|
| Application pack JSON + integrity | `q-pack` `fuzz_pack.rs` (random bytes, single-byte mutation of a valid pack) | 448 + 256 | PASS — never panics; >200/256 single-byte flips rejected by integrity |
| Query parser + percent-decoder | `q-http` `fuzz_parsers.rs` | 40,000 | PASS — never panics; always returns UTF-8; semantics invariants hold |
| Schema IR validator | `q-schema-runtime` `fuzz_validator.rs` (arbitrary JSON vs every IR kind, both sources) | 40,000 | PASS — deterministic classification, never panics |

CI (`.github/workflows/verify.yml`) runs `./scripts/verify` on
x86_64 and aarch64 Linux plus a fast OKF-only job. cargo-fuzz/ASan/TSan
coverage remains an open hardening item (disclosed, not claimed).
