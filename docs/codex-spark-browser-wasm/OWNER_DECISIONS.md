# Browser-WASM Owner Decisions

Resolve these decisions in `BWASM-D-001` through `BWASM-D-004`. Record the decision, owner, date, rationale, rejected alternatives, compatibility impact, and issues affected.

## Decision log template

```text
Decision ID:
Status: proposed | accepted | rejected | superseded
Owner:
Date:
Decision:
Rationale:
Alternatives rejected:
Compatibility impact:
Security impact:
Performance/payload impact:
Affected issues:
Revisit trigger:
```

## Product and claim decisions

- [ ] **OD-001 — Meaning of Browser-WASM deployment.** Confirm that the claim means static HTTPS artifacts with no Velqu application server, not “zero hosting” or “offline by definition”.
- [ ] **OD-002 — Primary use case.** Confirm whether beta is a local/prototype runtime, a supported static production runtime for bounded apps, or both. Claims and gate thresholds differ.
- [ ] **OD-003 — Native deployment relationship.** Confirm native Velqu remains required for production-only capabilities and remains the canonical runtime for native ingress/lifecycle behavior.
- [ ] **OD-004 — Handler engine.** Accept browser Worker JavaScript as the mandatory MVP handler engine; keep QuickJS-NG-in-WASM optional unless X-001 later passes and is promoted.
- [ ] **OD-005 — Compiler location.** Accept native/Bun-hosted compilation for MVP; browser deployment does not imply browser compilation.
- [ ] **OD-006 — Public API name.** Freeze `@velqu/browser-runtime`, CLI target name, artifact directory naming, and lifecycle API.

## Compatibility decisions

- [ ] **OD-010 — Shared semantics.** Freeze which route, method, header, body, status, problem, schema, Treaty, and capability behaviors require exact parity versus equivalent-by-contract behavior.
- [ ] **OD-011 — Body support.** Decide JSON, text, URL-encoded, multipart/file metadata, binary, and streaming support/limits for beta.
- [ ] **OD-012 — Header and cookie semantics.** Decide duplicate headers, forbidden browser headers, cookie/session behavior, redirects, and credentials policy.
- [ ] **OD-013 — HEAD/OPTIONS/405/trailing slash.** Freeze matching and method fallback behavior.
- [ ] **OD-014 — WASM ABI versioning.** Freeze compatibility and rejection policy for kernel, manifest, handler, and capability ABI versions.
- [ ] **OD-015 — Async Postgres break.** Approve the Promise-based Postgres capability contract and migration policy before API freeze.

## Security decisions

- [ ] **OD-020 — Trust model.** Treat generated/user handlers as potentially untrusted for app-builder deployments.
- [ ] **OD-021 — Origin architecture.** Require a separate preview origin for untrusted handlers; define approved same-origin development exceptions.
- [ ] **OD-022 — Network policy.** Freeze default-deny behavior and how projects request/receive outbound-origin permissions.
- [ ] **OD-023 — Credentials.** Confirm no provider key, production secret, editor credential, or ambient authenticated cookie is passed to the preview runtime.
- [ ] **OD-024 — Isolation claim.** Prohibit “secure sandbox” wording until an independent review supports a precise claim.
- [ ] **OD-025 — Cross-origin isolation.** Decide whether SharedArrayBuffer/COOP/COEP is needed; document embed and browser implications.
- [ ] **OD-026 — Source maps and diagnostics.** Decide what is shipped in production static builds and what paths/data must be redacted.

## Browser and deployment decisions

- [ ] **OD-030 — Browser matrix.** Choose blocking beta browsers and versions; mark all others experimental or unsupported.
- [ ] **OD-031 — Device tiers.** Choose desktop/mobile tiers for size/startup/memory evidence.
- [ ] **OD-032 — Static host matrix.** Choose required root/subpath and host shapes.
- [ ] **OD-033 — Service Worker fallback.** Decide whether injected Fetch/Worker mode is supported when Service Worker is unavailable.
- [ ] **OD-034 — Offline claim.** Decide what is cached, what still needs a gateway/network, and what “offline” means.
- [ ] **OD-035 — Update policy.** Freeze activation, multi-tab convergence, last-known-good retention, rollback, and data-migration hooks.

## Capability and persistence decisions

- [ ] **OD-040 — Mandatory local persistence.** Accept namespaced IndexedDB KV as baseline.
- [ ] **OD-041 — PGlite status.** Keep PGlite optional for beta unless a product requirement explicitly promotes it.
- [ ] **OD-042 — Simulation policy.** Decide which deployment-only capabilities may offer explicit simulations and how simulations are visually/machine-readably distinguished.
- [ ] **OD-043 — Deployment-required problem.** Freeze type/code/fields/status semantics and pre-side-effect enforcement.
- [ ] **OD-044 — Data lifecycle.** Freeze project namespace, reset/export/import, upgrade migration, garbage collection, and user-deletion behavior.
- [ ] **OD-045 — Quotas.** Set request, response, message, log, storage, fetch, and execution limits.

## Release decisions

- [ ] **OD-050 — Beta release channel.** Choose version/tag/package publication policy.
- [ ] **OD-051 — Performance budgets.** Freeze core/optional payload, cold/warm startup, first request, steady request, Worker restart, and memory-growth thresholds.
- [ ] **OD-052 — Evidence ownership.** Assign independent reviewers for kernel/runtime/security/cleanroom/gate packets.
- [ ] **OD-053 — Residual risk acceptance.** Define who can accept P1 risk, required metadata, and expiry.
- [ ] **OD-054 — QuickJS promotion rule.** Define the exact X-001 thresholds that would justify making QuickJS-WASM supported or mandatory.
