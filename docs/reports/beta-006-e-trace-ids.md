# BETA-006-E — Optional Trace IDs

Status: **IMPLEMENTED** (bounded optional trace-id propagation).

## Design: trace IDs (the "or" arm — no tracing-system integration)

- Inbound `x-trace-id` or W3C `traceparent` (trace-id segment) is
  extracted, **validated and bounded**: printable ASCII only (spaces,
  control characters rejected), ≤ 128 chars, empty rejected.
- The id is emitted as `traceId` in the structured `request.complete`
  log; absent headers omit the field (never null).
- Strictly optional: requests without trace headers log exactly as
  before (request id only). No tracing backend, exporter, or sampling
  configuration exists — integration with a tracing system is an
  operator-side post-processing concern for the beta.

## Redaction audit

The trace id is bounded printable ASCII supplied by the caller — no
PII gate needed beyond the shape bound; no token/secret ever flows
through this path.

## Tests (3 new, in q-http)

`x-trace-id` and W3C `traceparent` extraction; absent → None;
rejection of spaces/control chars/oversized/empty ids; 128-char
boundary accepted. Full gates green (fmt/clippy/verify; bun test 434+).
