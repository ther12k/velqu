# Browser-WASM support matrix (BWASM-Q-001)

Frozen classification of behaviors across the native Velqu runtime and
the Browser-WASM target. Every differential-suite classification links
here. Changes to this matrix require an owner decision (ADR-level) — the
differential suite fails on unreviewed drift.

## Classification vocabulary

`exact-parity` · `equivalent-by-contract` · `browser-only` ·
`native-only` · `unsupported`

## Matrix

| behavior | classification | note |
|---|---|---|
| Routing (method/path resolution, 404, 405) | exact-parity | same pack, same Rust router (native binary + wasm32 kernel) |
| Query schema validation (failure shape/status) | exact-parity | q-schema-runtime on both targets |
| Body schema validation (failure shape/status) | exact-parity | q-schema-runtime on both targets |
| Response schema contract (declared statuses enforced) | exact-parity | kernel completion + native handler settlement |
| Handler JSON body consumption (`ctx.body`) | exact-parity | normalized JSON body crosses the kernel verbatim |
| 200 responses over JSON bodies | exact-parity | canonical body equality |
| Problem envelope (validation / not-found / method) | equivalent-by-contract | envelope deltas: native carries `instance` + `detail` and omits body `allow` (the HTTP `Allow` header carries it); the kernel names problems via `problemId` and includes `allow`. Contract fields (status/type/title/errors) match exactly. |
| Handler params consumption (`ctx.params.x`) | native-only | browser MVP worker context exposes raw kernel fields; schema-driven params parsing is not wired into the worker context (follow-up: extend the worker context contract) |
| Handler query consumption (`ctx.query.x` as an object) | native-only | browser query crosses as pair arrays (dispatcher normalization); object-form query values are a worker-context follow-up |
| Handler `ctx.native.*` capability calls | native-only | worker-realm capability wiring is future work (C-001 installed the page-side graph; the worker callback seam is unimplemented) |
| Handler `status(n).value(...)` non-default returns | native-only | the emitted browser handler wrapper supports the declared-default status only (B-001 emitted contract); follow-up: extend the emitted handler contract |
| Native liveness routes (RUN-009) | native-only | refused at browser build time (B-001) |
| Deployment-required capabilities (e.g. `runtime:postgres`) | native-only | refused at browser build time (C-005); stable 501 problem shape at runtime |
| Forbidden capabilities (payments/email/webhooks/cron/queues) | unsupported | no capability exists; never simulated (C-005) |
| Service Worker offline/asset lane | browser-only | B-004/B-006; no native counterpart |
| SW cache activation/upgrade/rollback | browser-only | B-006 rehearsal lanes |

## Frozen classification counts (Q-001 corpus, 2026-09-06)

The committed differential suite pins: **4 exact-parity,
3 equivalent-by-contract, 4 native-only, 0 drift** across 11 fixtures.
Changing a count or a classification is reviewed drift — never an
automatic snapshot update.
