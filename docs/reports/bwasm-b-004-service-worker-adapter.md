# BWASM-B-004 — Service Worker Adapter and Static-Host Bootstrap

## Result

**PASS** — a built Velqu browser application serves on static hosting
without a Velqu application server: a scope-guarded, classified,
content-addressed Service Worker adapter plus a page bootstrap whose
unsupported/failed registrations land in a structured injected-fetch
fallback. 20 new tests (package **89/89**), typecheck clean.

## Design

- **Scope guard (pure)**: only same-origin requests under the explicit
  scope prefix are intercepted; scope escapes, cross-origin traffic,
  and unparseable URLs are passthrough — the SW never calls
  `respondWith` for them (tested; ADR-0038 §4). Editor/auth/
  model-gateway path prefixes (`/__velqu_editor__`, `/auth/`, `/oauth/`,
  `/model-gateway/`) are passthrough by classification.
- **Classification (support matrix)**: navigation (mode or accept
  header), asset (paths/extensions), api (default), passthrough;
  forms (POST urlencoded) = api; redirects = the runtime's redirect
  Response served status-as-is (all fixture-locked).
- **Cache plan (content-addressed)**: precache list derived from the
  B-002 manifest — every artifact under `/app/` with its sha256;
  cache name `velqu:<appId>:<buildId>`; a cache hit implies the
  verified hash was checked at install (loader semantics, B-002).
- **Fetch-event core**: scope guard → classify → behavior. api +
  navigation route to the kernel-backed runtime fetch; assets are
  cache-first with a typed 504 offline problem when absent; navigation
  with a dead runtime and a cached build yields the deterministic 503
  `offline` problem (never a hang); unhandled runtime failures
  propagate (never silent success).
- **Bootstrap + fallback**: `bootstrapServiceWorker` returns a
  structured outcome — `service-worker` or `injected-fetch-fallback`
  with the reason — for unsupported environments and registration
  failures alike (tested: no `navigator.serviceWorker` ⇒ deterministic
  fallback).
- **Update decision (pure)**: different buildId ⇒
  `apply-on-next-reload` (never mid-request); same ⇒ `no-change`
  (ADR-0039 §4; rollback = redeploy the prior build).

## Test evidence (20 new; package 89/89)

Scope guard (4: in-scope, escape, cross-origin, unparseable),
classification (6: navigation×2, assets×2, api, passthrough prefixes,
forms, redirects), cache plan (coverage + naming), fetch-event core
(6: api routing, cache-first asset, scope escape NOT intercepted,
cross-origin NOT intercepted, offline navigation 503 problem, api
failure surfaces), bootstrap fallback + update decision (3).

## Honest boundaries

- Real-browser SW lifecycle (actual registration, Cache Storage
  integration, Chromium/Firefox/WebKit lanes per the frozen matrix) is
  BWASM-Q-002 — the injected-factory/double pattern tested here is the
  deterministic core those lanes will drive.
- Static-deployment recording and the external cleanroom exercise are
  BWASM-Q-007; B-005's preview command will serve this adapter locally.
- The Worker bootstrap (R-004) and the SW here are separate realms by
  design: the SW serves cached artifacts and routes to the runtime; it
  does not execute handler code itself.

Standing CI disclosure applies (zero-step verify workflows since
~#714); local gates are the acceptance basis.
