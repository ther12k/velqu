---
type: Architecture Decision Record
title: ADR-0038 Browser Execution Threat Model and Isolation Contract
status: proposed
date: 2026-09-05
implements: BWASM-D-003 (isolation contract), ADR-0037 (browser-wasm product contract), ADR-0026 (integrity is not authenticity), ADR-0035 (same-process trusted code assumption)
owner-acceptance: PENDING. The Browser-WASM packet and the owner's 2026-09-05 instruction authorized preparing these design documents; neither specifies nor ratifies this threat model's specifics (CSP baselines, preview-origin policy, bounds values). This ADR is agent-authored under that authorization and is NOT owner-ratified. This frontmatter previously recorded blanket "owner acceptance"; corrected 2026-09-05.
---

# ADR-0038: Browser Execution Threat Model and Isolation Contract

## Context

The browser runtime (ADR-0037) executes application handler code that
may be AI-generated or user-authored, inside the user's browser, on an
origin the operator controls. The native product's security posture
(trusted pack code, network as adversary, ADR-0035) must be re-stated
for an environment where the "process interior" is split across a WASM
kernel and a Worker, and where the *browser*, not Velqu, owns the real
isolation primitives. This ADR freezes what the runtime guarantees,
what it deliberately delegates to the platform, and what it refuses to
claim.

## 1. Actors

| Actor | Trust role |
|---|---|
| Application operator | Deploys the artifact; owns the serving origin; is accountable for handler code (including AI-generated code they ship) |
| End user | Data owner; browser enforces origin isolation on their behalf |
| Velqu browser runtime (kernel + Worker) | Mechanism: enforces the frozen in-runtime checks; holds no authority beyond its origin |
| Browser platform | Trusted computing base for origin isolation, CSP enforcement, storage partitioning |
| Artifact author/compiler | Produces the pack; authenticity is out-of-band (ADR-0026), not self-declared |
| Network adversary | Can serve stale/tampered artifacts; cannot break in-runtime integrity checks |

## 2. Assets and trust boundaries

**Assets**: user data reachable through declared capabilities (storage,
network allowlist); artifact integrity; the serving origin's authority
(cookies, DOM, storage of the host page); the operator's reputation.

**Boundaries (crossings are explicit and validated):**

1. **Origin boundary** (browser-enforced): the hard isolation boundary.
   Everything inside one origin — kernel, Worker, artifact — is one
   trust domain.
2. **Kernel ↔ Worker message boundary**: structured, validated messages
   only (no functions, no prototypes, no lazy handles); both directions
   schema-checked by the kernel; per-message bounds enforced.
3. **Artifact provenance boundary**: network → runtime; integrity
   verified in-band after load (fail-closed), authenticity delegated to
   deployment (served over the operator's origin; ADR-0026).
4. **Capability boundary**: every side-effecting operation is a
   declared capability call, authorized kernel-side against the
   artifact manifest before execution (ADR-0037 §5).

## 3. Deployment modes: trusted vs untrusted preview

| Mode | Definition | Isolation mechanism |
|---|---|---|
| **Trusted (production)** | The operator's own artifact on the operator's origin, including AI-generated code the operator chose to ship | Ordinary origin security; runtime integrity + capability checks |
| **Untrusted preview** | Rendering/running an artifact whose author is not the origin operator (AI-generated app being evaluated) | **Separate preview origin** (distinct registrable domain, not a subdomain of the app origin) + **sandboxed iframe** without `allow-same-origin`/`allow-scripts`-to-parent combinations, + its own storage partition |

The runtime never claims to sandbox untrusted code by itself; untrusted
preview safety is delegated to the platform's origin/iframe model and
the deployment posture above. In production mode, handler code is
**trusted application code** exactly as in the native product (ADR-0035
carried over): the network and the browser platform remain adversaries,
the handler is not.

**Forbidden claims (binding on all BWASM evidence and docs):** no
"hostile-code sandbox" claim; no "AI-generated code is safe because of
Velqu"; no claim that the Worker or WASM adds author-untrusted
isolation beyond what the browser origin model provides.

## 4. Platform requirements (deployment contract)

- **Preview origin**: separate registrable origin for any untrusted
  preview; production artifacts are never served from the preview
  origin.
- **Sandboxed iframe** for embeds/preview: `sandbox` without
  `allow-same-origin`; explicit `allow-scripts` only where the embedder
  intends execution; no parent-DOM reach.
- **CSP** (recommended baseline, enforced by serving origin):
  `script-src 'self' 'wasm-unsafe-eval'` (no `unsafe-eval`), `connect-src`
  limited to the artifact host + declared capability endpoints,
  `default-src 'self'`, `object-src 'none'`, `base-uri 'none'`,
  `frame-ancestors` set by the operator.
- **Permissions-Policy / Referrer-Policy**: restrictive defaults
  (`referrer: no-referrer`; powerful platform features denied unless a
  capability explicitly requires them).
- **Worker policy**: one dedicated Worker per runtime instance;
  Workers constructed from same-origin or hash-pinned blob URLs only;
  no remote-code Workers.

## 5. Bounds, defaults, and recovery (in-runtime)

- **Bounds**: the native bounded-everything posture carries over —
  request/response sizes, message sizes across the Worker boundary,
  capability-call argument bounds, handler deadline, and log volume are
  all finite and kernel-enforced (values ratified with the budgets in
  BWASM-D-004).
- **Network defaults**: no ambient fetch. The `fetch` capability is
  declared with an explicit allowlist; requests are made
  credentialless (`credentials: "omit"`), non-preflight-friendly, and
  the kernel rejects undeclared targets with typed problems before the
  Worker sees them.
- **Credentials**: handler code never receives cookies, tokens, or
  ambient editor-origin authority *from the runtime*; the capability
  bridge strips credential surfaces from declared capability calls; auth
  is the application's explicit contract (BETA-005 reference stays
  native-deployment guidance).
- **Ambient-API honesty (corrected 2026-09-05)**: in trusted
  (production) mode, the capability bridge mediates *declared* calls
  but **cannot prevent** handler code from calling browser APIs
  directly (a Worker exposes `fetch`, storage, and other platform
  surfaces). Trusted-handler non-use of ambient APIs is a **convention
  enforced by review and the compiler's import policy
  (BWASM-B-003), not a runtime guarantee**. Platform-enforceable
  backstops are origin isolation and CSP `connect-src`; storage
  partitioning is per-origin. Any wording implying the runtime *blocks*
  ambient access in trusted mode is wrong; untrusted-preview isolation
  is solely the deployment posture of §3 (separate origin + sandboxed
  iframe), never a runtime property.
- **Storage**: browser persistence only through the namespaced KV
  capability (per-app namespace derived from artifact identity); no
  shared/global storage; no ambient localStorage access from handlers.
- **Cache**: artifact caches are content-hash keyed (ADR-0023) and
  re-verified after load; a cache hit never skips integrity
  verification.
- **Recovery**: kernel-enforced deadline expiry → Worker terminated and
  cold-restarted (kill-and-replace, no in-place reload); repeated
  anomalies fail the runtime closed for the session; every recovery is
  observable in diagnostics (BWASM-Q-004 contract).

## 6. Abuse cases and dispositions

| Abuse case | Disposition |
|---|---|
| Tampered/stale artifact served by network | In-band integrity fail-closed after load (ADR-0026); authenticity = serve from operator origin |
| Handler exfiltrates data beyond declared endpoints | Capability allowlist enforced kernel-side + CSP `connect-src` backstop |
| Malicious structured message to Worker | Messages are schema-validated, size-bounded, prototype-free; Worker holds no authority |
| Worker DoS (infinite loop, unbounded recursion) | Kernel deadline → terminate + cold restart; no shared state to corrupt |
| Storage collision/overreach between artifacts | Per-artifact-identity namespacing; no global handles |
| Cache poisoning | Hash-keyed cache + verify-after-load (integrity never skipped) |
| Embedding abuse (clickjacking-style) | Operator-set `frame-ancestors`; sandboxed iframe requirements for embeds |
| Dependency/supply-chain compromise in the JS glue | Bundle content-hashing + SRI guidance in deployment docs (BWASM-B-004/Q-007 evidence) |

## Rejected alternatives

1. **In-runtime untrusted-code sandboxing claim** — rejected: the
   runtime executes operator-chosen code; claiming hostile-code
   isolation would be false (ADR-0035 extension) and would weaken the
   platform boundary that actually provides it.
2. **Subdomain-scoped preview isolation** — rejected: subdomains share
   the registrable domain's cookie/storage attack surface; previews get
   a separate origin.
3. **Ambient fetch with runtime-level URL filtering only** — rejected:
   CSP and credential-stripping are defense-in-depth, but authority
   must be declared and kernel-checked before the handler runs.
4. **In-place Worker code reload for updates** — rejected: kill-and-
   replace keeps the recovery model total and the artifact identity
   coherent.

## Consequences

- BWASM-R-004 (Worker execution), C-001 (capabilities), Q-003 (security
  verification) implement and test against this contract; deviations
  reopen this ADR.
- Documentation (BWASM-Q-006) must restate the forbidden claims
  verbatim.
- The preview-origin and CSP items are deployment contracts: they are
  documented requirements for operators, not runtime-enforceable
  guarantees, and docs must say so.
