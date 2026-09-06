# Build report — app

## Routes

| Route | Method | Path | Stage | Policy | Caps | Validation | Response |
|---|---|---|---|---|---|---|---|
| greet.get | GET | /greet/:name | engine | — | — | native | native |
| echo.post | POST | /echo | engine | — | — | native | native |
| maybe.get | GET | /maybe/yes | engine | — | — | native | native |
| absent.get | GET | /absent | engine | — | — | native | native |
| param.echo | GET | /param/:name | engine | — | — | native | native |
| query.echo | GET | /query-echo | engine | — | — | native | native |
| timed.get | GET | /timed | engine | — | timer | native | native |

## Strategies

- validation: native default for representable Schema IR v2 (M25-002 evidence: 20–40% lower latency on small/nested shapes, ~6x lower on 16–64 KB payloads)
- responses: native serialization default (ADR-0015, M25-002-C: zero bridge crossings, no JS stringify jitter)
- fallback: explicit per-route fallback with visible reason and estimated overhead (SCHEMA-005, ADR-0009)
- JS fallbacks used: **none** (SCHEMA-005)

## Artifacts

- app.qpack: 18645 B
- route-manifest.json: 3938 B
- schema-manifest.json: 2384 B
- capability-manifest.json: 722 B
- contract.json: 4066 B
- contract.d.ts: 1651 B
- contract.meta.json: 1142 B
- openapi.json: 7709 B
- build-report.json: 11058 B
- app.qpack.sources.json: 39944 B
- published-manifest.json: 785 B
