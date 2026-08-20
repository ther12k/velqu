---
type: Product Gate
title: Velqu Public Beta Definition
status: draft
tags:
- beta
- scope
- release-gate

---

# Public Beta Definition

## Target

The target is **Velqu `0.1.0-beta.1`**.

A beta-ready Velqu is installable and usable by external developers for evaluation, staging, internal tools, and non-critical services. It is not a production-ready GA release and carries no SLA.

## Required beta capabilities

- Rust + QuickJS-NG production runtime with verified numeric QPack execution.
- Native compiled routing, lazy request bridge, schema-specialized validation/serialization, and binary QPack v2.
- Minimal Web runtime: console, timers, URL, encoding, AbortSignal, secure random, and outbound fetch.
- Serverless one-worker and multi-worker service profiles.
- Typed routes, status-aware Treaty, OpenAPI, contract lock, and semantic diff.
- Actual-runtime development loop, CLI, scaffolding, diagnostics, source maps, testing modes, and documentation.
- Optional first-party Postgres capability and JWT/auth reference package.
- Liveness/readiness, configuration, secrets redaction, observability, reverse-proxy and graceful drain semantics.
- Reproducible real-world benchmark harness, security baseline, soak evidence, clean installation, and self-verifying release packet.

## Working platform assumption

- Required working assumption: Linux x86_64 glibc.
- Linux ARM64 glibc is included when the CI/build environment is available.
- Other platforms are unsupported unless an owner decision expands the beta promise.

## Stability promise

- SemVer prerelease.
- Breaking changes may occur between beta releases.
- Every breaking change requires a migration note.
- Public contract hash and QPack/runtime ABI mismatches fail clearly.
- No long-term ABI guarantee until a later release-candidate program.

## Security and trust statement

- Application code inside the same QuickJS process is trusted.
- Beta does not provide hostile multi-tenant sandboxing.
- Network egress, fetch, artifact, policy, schema, and readiness errors fail closed.
- Known exploitable critical/high findings block the release.

## Performance statement

Beta may publish measured fixture-specific results. It must not claim universal superiority over Elysia, Bun, Node, or JIT runtimes. Bytecode improves startup; it is not native-machine-code JIT compilation.

## Beta versus later GA

| Area | Beta | Later GA |
|---|---|---|
| External installation | Required | Required |
| Real service proof | Required | Required |
| Core security baseline | Required | Deeper independent hardening |
| Soak | Two hours / one million requests minimum | Longer multi-platform qualification |
| Platforms | Narrow documented set | Formal support matrix |
| API/ABI | Prerelease, may change | Frozen SemVer policy |
| Signing/provenance | Checksums + SBOM; signatures when owner keys exist | Signed reproducible releases required |
| SLA/support | None | Owner-defined |
| “Production ready” wording | Forbidden | Requires later GA gate |
