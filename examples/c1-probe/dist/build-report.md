# Build report — app

## Routes

| Route | Method | Path | Stage | Policy | Caps | Validation | Response |
|---|---|---|---|---|---|---|---|
| diag.text.sync | GET | /diag/text-sync | native-liveness | — | — | native | native |
| diag.text.async | GET | /diag/text-async | native-liveness | — | — | native | native |
| diag.json.sync | GET | /diag/json-sync | native-liveness | — | — | native | native |
| diag.json.async | GET | /diag/json-async | native-liveness | — | — | native | native |
| diag.text.engine | GET | /diag/text-engine | engine | — | — | native | native |

## Strategies

- validation: native default for representable Schema IR v2 (M25-002 evidence: 20–40% lower latency on small/nested shapes, ~6x lower on 16–64 KB payloads)
- responses: native serialization default (ADR-0015, M25-002-C: zero bridge crossings, no JS stringify jitter)
- fallback: explicit per-route fallback with visible reason and estimated overhead (SCHEMA-005, ADR-0009)
- JS fallbacks used: **none** (SCHEMA-005)

## Artifacts

- app.qpack: 12986 B
- route-manifest.json: 2521 B
- schema-manifest.json: 707 B
- capability-manifest.json: 562 B
- contract.json: 1843 B
- contract.d.ts: 1168 B
- contract.meta.json: 887 B
- openapi.json: 2901 B
- build-report.json: 7376 B
- app.qpack.sources.json: 36092 B
- published-manifest.json: 784 B
