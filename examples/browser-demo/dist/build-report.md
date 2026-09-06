# Build report — app

## Routes

| Route | Method | Path | Stage | Policy | Caps | Validation | Response |
|---|---|---|---|---|---|---|---|
| hello.get | GET | /hello/:name | engine | — | — | native | native |
| echo.post | POST | /echo | engine | — | — | native | native |

## Strategies

- validation: native default for representable Schema IR v2 (M25-002 evidence: 20–40% lower latency on small/nested shapes, ~6x lower on 16–64 KB payloads)
- responses: native serialization default (ADR-0015, M25-002-C: zero bridge crossings, no JS stringify jitter)
- fallback: explicit per-route fallback with visible reason and estimated overhead (SCHEMA-005, ADR-0009)
- JS fallbacks used: **none** (SCHEMA-005)

## Artifacts

- app.qpack: 9333 B
- route-manifest.json: 978 B
- schema-manifest.json: 1082 B
- capability-manifest.json: 470 B
- contract.json: 1603 B
- contract.d.ts: 669 B
- contract.meta.json: 413 B
- openapi.json: 2702 B
- build-report.json: 4960 B
- app.qpack.sources.json: 33218 B
- published-manifest.json: 783 B
