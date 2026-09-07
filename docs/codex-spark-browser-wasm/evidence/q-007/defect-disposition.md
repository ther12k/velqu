# BWASM-Q-007 Defect Disposition

Round 1 (independent cleanroom evaluator, fresh agent, public artifacts only)
found 9 findings. Each is dispositioned below. Framework defects D4, D5, D7
were fixed and re-proven by the independent evaluator in real Chromium
(rounds 2–3, fresh app built from refreshed artifact tarballs). D4/D7
regression-swept clean in round 3.

| ID | Category | Disposition | Evidence |
|---|---|---|---|
| D1 | blocked → packaging | **Accepted limitation, disclosed.** The beta distribution is source-based (`docs/beta/INSTALL.md`: "@velqu/* npm packages are prepared but unpublished", Owner-gated per AGENTS.md constraint 13). Tarball layout (`package/` prefix, `workspace:*` deps) cannot be fixed without a publication decision that belongs to the Owner. Round-1/2/3 evaluators used manual extraction — the documented beta path. | defects-log round1 §D1 |
| D2 | docs-gap | **Fixed this packet.** `docs/beta/BROWSER_WASM.md` (BWASM-Q-006, merged #1291) now documents route/schema/handler authoring, project layout, capability usage, and probe flags. | #1291 |
| D3 | framework-defect (packaging) | **Fixed this packet.** `@velqu/browser-runtime` artifact set now ships the vendored, hash-pinned `kernel/` directory (kernel.json + wasm + glue); builds from artifacts alone succeed (rounds 2–3 used exactly this path). | round-2 verify.md |
| D4 | framework-defect | **Fixed + re-proven.** The emitted browser handler wrapper now maps the `@velqu/core` result protocol — `{__ok,status,value}` → declared status with `value` body; `{__problem,...}` → RFC-9457 problem at its declared status; `{__velquRaw,...}` → raw passthrough. Round 2: `POST /orders` → **201** with value body; `GET /orders/nope` → **404** problem envelope. | round-2 verify.md §D4 |
| D5 | framework-defect | **Fixed + re-proven (root cause was two-layer).** Layer 1 (kernel): plan-time authorization compared route GRANT names (`timer`) against inventory LINKED MODULE ids (`runtime:timers`) → every timer route 501; the kernel now accepts the same grant→module mapping the compiler's resolver applies (plan-time AND `authorizeCapability`), regression-tested (`plan_accepts_grant_name_when_linked_module_id_is_inventoried`). Layer 2 (worker): the invoke plan now carries the route's declared grants (`capabilities` field), and the Worker bootstrap exposes `ctx.native.<grant>` for exactly the declared grants — implementations are the page's policy-bounded C-001 capability graph, registered per session; no ambient authority (undeclared grants are absent from the view). Round 3: `GET /sys/tick` → 200 `{ms}`. | kernel test + round-3 verify.md §D5 |
| D6 | setup-friction / docs-gap | **Partially addressed, remainder is roadmap.** Probe flags documented (Q-006). A generated developer smoke page for arbitrary route exercise (bodies, custom headers) is a UX feature request, not a defect in the frozen MVP surface ("this page exposes NO ambient globals" is the documented security posture). Recorded as follow-up for the app-builder workstream. | follow-up note below |
| D7 | framework-defect | **Fixed + re-proven.** The Service Worker asset lane no longer fail-closes on a cache miss while the network is healthy: manifest artifacts stay cache-first with integrity; non-manifest assets deployed beside the app (custom pages, CSS, images) fall through to the network, and the 504 offline problem is reserved for actual network unreachability. Round 2: custom page 200 → offline reload 200 (cache boot) → server restart 200, no hang; round 3 regression sweep clean. | round-2 verify.md §D7 |
| D8 | docs-gap (+ dead code) | **Documented; flag semantics clarified.** `--kv` provisions the page-side KV adapter (used by the preview shell / developer tooling); handler-side persistence is declared per capability in the artifact manifest. BROWSER_WASM.md wording updated by Q-006. The namespace observation (`kv:items-crud` custom namespace) is the documented `${appId}:${namespace}` form with the app's own namespace choice. | #1291 |
| D9 | docs-gap (minor traps) | **Documented.** Static-liveness reclassification and the pinned toolchain are stated in BROWSER_WASM.md/INSTALL.md (Q-006); the toolchain error message itself was already verified clear by the evaluator. | #1291 |

## Not defects (verified working by the independent evaluator)

- Build-time Postgres refusal with source-located, actionable error (C-005).
- `inspect browser` integrity verification and capability inventory.
- First-load boot chain on Chromium: kernel WASM → pack verify → Worker → SW.
- Offline reload with full route execution from cache (`source: cache`).
- IndexedDB KV namespacing; persistence across reload and full server restart.
- Kernel validation 422s (minLength/minimum), 404/405 + `Allow`, RFC 9457 envelopes.

## Follow-up items (post-MVP roadmap, not blockers)

1. Generated developer smoke page for arbitrary in-browser route exercise (D6 remainder) — app-builder workstream.
2. npm publication / tarball layout — Owner-gated (AGENTS.md constraint 13); blocked on OD.
3. **#1292** — build-time guard for non-exported route bindings (silent-broken-bundle trap found in round 4; not a blocker for the candidate, the documented authoring pattern exports routes).
