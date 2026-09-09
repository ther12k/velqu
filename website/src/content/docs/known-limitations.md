---
title: "Known limitations"
description: "Recorded limitations and the honesty policy: trusted code only, no parity claims without evidence."
---
This is the canonical known-limitations inventory shipped with the beta
release packet. Scope and wording follow `docs/beta/LIMITS-AND-NON-GOALS.md`
and `docs/beta/01_BETA_DEFINITION.md`; each entry names where the limitation
is evidenced. This is a public beta: non-SLA, no production-readiness claim.

## Runtime and platform

1. **Platform support is Linux x86_64 glibc only.** ARM64 glibc is a
   conditional CI signal (BETA-010-B), not a shipped promise; Windows is
   unsupported; macOS is development-only.
2. **Trusted application code only.** Same-process QuickJS is not a
   hostile-code or multi-tenant sandbox.
3. **Pack↔runtime exact-match coupling.** A pack runs only on the exact
   runtime build it was compiled against (SEC-001); upgrading the runtime
   requires rebuilding and re-shipping the pack.
4. **PACK_FORMAT_CURRENT pinned to v1.** The v2 binary path exists behind
   tests but is not the default production load path, so the
   route-count-scaled cold-start limitation of the JSON pack load remains
   (owner decision pending; carried in both packet indexes).

## Performance boundaries (measured, fixture-specific)

5. **Warmed JIT floors can be lower.** In the ramp fixture Velqu's C0 steady
   p50 is 2.29× the class best and Velqu never overtook raw-rust within the
   recorded 100-request horizons (`benchmarks/raw/ramp/losses.json`).
6. **Bytecode is not JIT.** QPack bytecode removes startup parsing, not
   native-machine-code execution; heavy CPU loops favor JIT runtimes
   (`docs/beta/PERFORMANCE-METHODOLOGY.md`).
7. **No cloud cold-start claims.** Local process-exec measurements must not
   be extrapolated to platform provisioning times.
8. **Warm fixture coverage.** The warm matrix carries Velqu, raw Rust, raw
   Bun, and Elysia 2 at C0–C3 only; Fastify is pinned for the real-world
   matrix but has no warm-fixture row.

## Deployment and operations

9. **No native TLS/HTTP2 termination.** Reverse-proxy-first posture; public
   binds require explicit `proxyMode: "direct"` opt-in.
10. **Bounded fail-closed defaults.** Body 1 MiB, header 32 KiB, URI 8 KiB,
    queue 256, heap 32 MiB, stack 512 KiB, handler deadline 5 s, pending ops
    1024 — limit rejections are designed behavior.
11. **`defer` is in-memory best-effort**, not durable work.
12. **Forwarded headers are data, never identity**; cross-proxy identity
    requires signed application-layer tokens.
13. **Dynamic code execution is disabled** (`eval`, `new Function`) — typed
    `TypeError` by design.

## Packaging and publication

14. **npm packages are private.** All 9 `@velqu/*` tarballs are packed and
    checksummed but unpublished; the `beta`/`next` dist-tag flow is rehearsed
    (BETA-011-B) and Owner-gated.
15. **License selection is an open owner decision.** Workspace crates carry
    `UNLICENSED-BEFORE-OWNER-DECISION`; npm packages are `NOASSERTION` in the
    SBOM (`release/sbom.cdx.json` records the posture honestly).
16. **Advisory scanning unavailable in the build environment.** The SBOM
    covers dependency/license identification; cargo-audit/cargo-deny/
    osv-scanner were not installed (BETA-009-B disclosure).
17. **GitHub Release publication is Owner-gated.** The packet is
    self-verifying locally; publishing remains an owner action
    (`docs/beta/governance/RELEASE_AUTHORITY.md`).

## Browser-WASM preview target

19. **Browser-WASM is a preview and client-side target**, not an equivalent
    host for production-scale native workloads.
20. **Trusted application code only.** Same-origin Workers and WebAssembly
    are not hostile-code sandboxes. Malicious multi-tenant isolation requires
    origin-level separation (distinct subdomains) and sandboxed iframes.
21. **PostgreSQL requires native deployment.** Routes requiring `runtime:postgres`
    fail closed at build time; in-browser execution returns a stable 501
    `deployment-required` problem shape.
22. **QuickJS-WASM is optional and owner-gated.** The MVP runs handlers in a
    standard browser Worker; engine-in-WASM is not active by default.
23. **Browser support claims follow maintained CI evidence.** Only Chromium
    desktop is tested with blocking CI lanes; Firefox, Safari/WebKit, and
    mobile browsers remain experimental or untested.
24. **Local persistence is best-effort.** IndexedDB and Cache Storage are
    subject to browser eviction and do not represent durable multi-user storage.

## Evidence posture

18. **CI verify workflows stall** with zero executed steps at PR creation
    since roughly #714; every packet's acceptance basis is the full local
    gate battery, disclosed on each PR.
