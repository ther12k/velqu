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

- [x] **OD-001  — Meaning of Browser-WASM deployment.** Confirm that the claim means static HTTPS artifacts with no Velqu application server, not “zero hosting” or “offline by definition”. — RESOLVED via design freeze (2026-09-05, `evidence/design-freeze-owner-decision.md`): static-HTTPS-artifacts deployment, no Velqu application server (ADR-0037 §1).
- [x] **OD-002  — Primary use case.** Confirm whether beta is a local/prototype runtime, a supported static production runtime for bounded apps, or both. Claims and gate thresholds differ. — RESOLVED via design freeze: bounded static-production runtime for trusted apps; untrusted code is a deployment-posture concern (ADR-0037, ADR-0038).
- [x] **OD-003  — Native deployment relationship.** Confirm native Velqu remains required for production-only capabilities and remains the canonical runtime for native ingress/lifecycle behavior. — RESOLVED via design freeze: native Velqu remains canonical for native-only capabilities (ADR-0037 §5 capability classes).
- [x] **OD-004  — Handler engine.** Accept browser Worker JavaScript as the mandatory MVP handler engine; keep QuickJS-NG-in-WASM optional unless X-001 later passes and is promoted. — RESOLVED via design freeze: browser Worker JS is the mandatory MVP handler engine; QuickJS-in-WASM optional behind X-001 (now closed NO-GO, OD-054).
- [x] **OD-005  — Compiler location.** Accept native/Bun-hosted compilation for MVP; browser deployment does not imply browser compilation. — RESOLVED via design freeze: native/Bun-hosted compilation (ADR-0037 §4).
- [x] **OD-006  — Public API name.** Freeze `@velqu/browser-runtime`, CLI target name, artifact directory naming, and lifecycle API. — RESOLVED via design freeze: `@velqu/browser-runtime`, `browser-wasm` CLI target, `dist/browser` artifacts (B-001/B-005).

## Compatibility decisions

- [x] **OD-010  — Shared semantics.** Freeze which route, method, header, body, status, problem, schema, Treaty, and capability behaviors require exact parity versus equivalent-by-contract behavior. — RESOLVED via ADR-0039 ratification + differential conformance suite (Q-001): exact parity vs equivalent-by-contract classes frozen in ADR-0037 §7.
- [x] **OD-011  — Body support.** Decide JSON, text, URL-encoded, multipart/file metadata, binary, and streaming support/limits for beta. — RESOLVED via dispatcher contract (R-002): JSON/text/urlencoded in beta; multipart bounded metadata; binary/streaming unsupported-typed.
- [x] **OD-012  — Header and cookie semantics.** Decide duplicate headers, forbidden browser headers, cookie/session behavior, redirects, and credentials policy. — RESOLVED via ADR-0039 + dispatcher tests: platform-joined duplicate headers, forbidden-header policy, credentials omitted.
- [x] **OD-013  — HEAD/OPTIONS/405/trailing slash.** Freeze matching and method fallback behavior. — RESOLVED via dispatcher tests (R-002): kernel-side HEAD→GET, 405+Allow, trailing-slash identity frozen.
- [x] **OD-014  — WASM ABI versioning.** Freeze compatibility and rejection policy for kernel, manifest, handler, and capability ABI versions. — RESOLVED via B-002/R-003: ABI versions fail closed with actionable diagnostics (loader + handler ABI tests).
- [x] **OD-015  — Async Postgres break.** Approve the Promise-based Postgres capability contract and migration policy before API freeze. — RESOLVED via C-002: async (Promise) Postgres contract shipped pre-freeze; codemod transcript in `evidence/capabilities/c002/`.

## Security decisions

- [x] **OD-020  — Trust model.** Treat generated/user handlers as potentially untrusted for app-builder deployments. — RESOLVED via ADR-0038 (ratified as corrected): generated handlers treated as potentially untrusted; isolation honesty binding.
- [x] **OD-021  — Origin architecture.** Require a separate preview origin for untrusted handlers; define approved same-origin development exceptions. — RESOLVED via ADR-0038 §3: separate preview origin + sandboxed iframe; same-origin dev exceptions documented.
- [x] **OD-022  — Network policy.** Freeze default-deny behavior and how projects request/receive outbound-origin permissions. — RESOLVED via C-001 fetch capability: default-deny, declared allowlist, method policy (policy summary introspectable pre-execution).
- [x] **OD-023  — Credentials.** Confirm no provider key, production secret, editor credential, or ambient authenticated cookie is passed to the preview runtime. — RESOLVED via C-001 tests: credentials forced omit, credential headers stripped, no ambient authority.
- [x] **OD-024  — Isolation claim.** Prohibit “secure sandbox” wording until an independent review supports a precise claim. — RESOLVED via ADR-0038 §5 correction + R-004 bootstrap honesty statement: no sandbox wording without independent review.
- [x] **OD-025  — Cross-origin isolation.** Decide whether SharedArrayBuffer/COOP/COEP is needed; document embed and browser implications. — RESOLVED via C-003/E7: cross-origin isolation required only for the local-SQL evidence surface (`/__velqu_editor__/` paths); main shell deployment contract unchanged; engine shipped as its own dist layout.
- [x] **OD-026  — Source maps and diagnostics.** Decide what is shipped in production static builds and what paths/data must be redacted. — RESOLVED via Q-004: production diagnostics disabled/reducible; source locations sanitized; secrets redacted (tested).

## Browser and deployment decisions

- [x] **OD-030  — Browser matrix.** Choose blocking beta browsers and versions; mark all others experimental or unsupported. — RESOLVED via ADR-0039 + Q-002: Chromium desktop = required/tested lane; Firefox/WebKit = experimental allowed-failure lanes (now running green in CI — green CI does NOT promote classification; promotion stays an owner ADR amendment).
- [x] **OD-031  — Device tiers.** Choose desktop/mobile tiers for size/startup/memory evidence. — RESOLVED via D-004 budgets: mid-tier 2022 Android-class environment as the evidence target.
- [x] **OD-032  — Static host matrix.** Choose required root/subpath and host shapes. — RESOLVED via B-002/Q-007: root and non-root base paths verified in the external cleanroom exercise.
- [x] **OD-033  — Service Worker fallback.** Decide whether injected Fetch/Worker mode is supported when Service Worker is unavailable. — RESOLVED via B-004: injected-fetch fallback supported when Service Worker is unavailable (tested).
- [x] **OD-034  — Offline claim.** Decide what is cached, what still needs a gateway/network, and what “offline” means. — RESOLVED via Q-007 + KNOWN-LIMITATIONS: offline = cached verified artifacts; network-dependent routes answer declared unavailable problems; no offline guarantee.
- [x] **OD-035  — Update policy.** Freeze activation, multi-tab convergence, last-known-good retention, rollback, and data-migration hooks. — RESOLVED via B-006: content-addressed activation, last-known-good retention, bounded cache retention (tested).

## Capability and persistence decisions

- [x] **OD-040  — Mandatory local persistence.** Accept namespaced IndexedDB KV as baseline. — RESOLVED via C-004: namespaced IndexedDB KV shipped as the baseline persistence surface (fail-closed; explicit memory fallback only when flagged).
- [x] **OD-041  — PGlite status.** Keep PGlite optional for beta unless a product requirement explicitly promotes it. — RESOLVED via C-003: PGlite shipped as an optional opt-in package (`@velqu/browser-pglite`), excluded from the release gate; promotion stays an owner decision.
- [x] **OD-042  — Simulation policy.** Decide which deployment-only capabilities may offer explicit simulations and how simulations are visually/machine-readably distinguished. — RESOLVED via ADR-0037 §7 / C-005: the `explicitly simulated` class remains empty by design; deployment-required fails closed with typed problems.
- [x] **OD-043  — Deployment-required problem.** Freeze type/code/fields/status semantics and pre-side-effect enforcement. — RESOLVED via C-005: stable RFC-9457 deployment-required problem, pre-side-effect enforcement (tested).
- [x] **OD-044  — Data lifecycle.** Freeze project namespace, reset/export/import, upgrade migration, garbage collection, and user-deletion behavior. — RESOLVED via C-004 + C-003: namespaced stores, export/import/reset lifecycle, bounded retention, explicit gc control (tested).
- [x] **OD-045  — Quotas.** Set request, response, message, log, storage, fetch, and execution limits. — RESOLVED via C-001/C-004/C-003 enforced ceilings: body/log/fetch/storage/op-deadline quotas are enforced limits, not prose.

## Release decisions

- [x] **OD-050 — Beta release channel.** Choose version/tag/package publication policy. — RESOLVED BY OWNER ACTION (2026-09-09): npm org `velqu`; all seven packages published as `0.1.0-beta.1` under the `beta` dist-tag; `latest` intentionally withheld; record in `docs/open-decisions.md` (OD-010) and runbook in `docs/beta/PUBLISHING.md`.
- [x] **OD-051  — Performance budgets.** Freeze core/optional payload, cold/warm startup, first request, steady request, Worker restart, and memory-growth thresholds. — RESOLVED via design freeze (as ratified targets): `evidence/budgets.json` numeric ceilings; measurement procedures executed by Q-005.
- [ ] **OD-052 — Evidence ownership.** Assign independent reviewers for kernel/runtime/security/cleanroom/gate packets.
- [ ] **OD-053 — Residual risk acceptance.** Define who can accept P1 risk, required metadata, and expiry.
- [x] **OD-054 — QuickJS promotion rule.** Define the exact X-001 thresholds that would justify making QuickJS-WASM supported or mandatory. — RESOLVED (2026-09-11): NO-GO verdict rendered in `docs/browser-wasm/evidence/x001/06-go-no-go-decision.md`. Evaluated across 6 dimensions; scored 9 / 30 (threshold >= 24/30). Fails due to 3-minor-version skew (0.12.1 vs 0.15.1, broken bytecode parity), +242 KB brotli payload bloat exceeding glue budget by 4.7x, 310x latency penalty, and 1,475x cold-start penalty. Hybrid Worker architecture retained for the current contract (revisit rule: x001 decision record §4).
