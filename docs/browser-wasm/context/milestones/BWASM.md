# BWASM — Browser-WASM Program Context

Baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93`  
Status: planning packet; implementation not started by this artifact

## Mission

Make supported Velqu applications deployable as static browser artifacts with a mandatory Rust/WASM compatibility kernel and no running Velqu application server.

## Frozen-by-default target

```text
Rust/WASM:
  manifest/artifact verification
  routing and params
  request/response schema validation
  capability authorization
  canonical problem mapping

Browser JavaScript:
  isolated Worker handler execution
  Fetch/Request/Response adaptation
  browser capability adapters
  Service Worker deployment integration

Native Velqu:
  production-only capabilities
  native ingress/lifecycle
  QuickJS-NG production handler engine
```

## Non-negotiable invariants

1. Do not port all of native `q-runtime`.
2. Do not ship a JavaScript-only fake under a WASM label.
3. Do not expose secrets or editor-origin authority to preview code.
4. Do not claim a hostile-code sandbox without independent evidence.
5. Do not silently mock production side effects.
6. Do not mix artifact versions during activation/update.
7. Do not expand browser support claims beyond blocking evidence.
8. Keep native Velqu green.
9. Keep PGlite and QuickJS-WASM optional unless owner-promoted.
10. Gate one exact candidate with a binary GO/NO-GO decision.

## Agent entry point

1. Read `MASTER_PLAN.md`.
2. Read `OWNER_DECISIONS.md`.
3. Open the assigned issue body.
4. Verify dependencies are accepted/closed.
5. Re-check source paths against current master.
6. Run targeted tests and canonical repository verification.
7. Commit exact evidence and hand off using the issue template.
8. Stop at decision/evidence boundaries; do not invent owner approval.

## Completion boundary

Only `BWASM-GATE` can declare the mandatory program ready. The epic closes only after a GO verdict.
