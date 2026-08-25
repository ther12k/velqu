# M26-009-A — Shared-mode deployment artifacts

Shared mode = `velqu-runtime` binary plus a separate `app.qpack`
(parent intent: small app updates without touching the runtime). This
packet formalizes the artifact story with repeatable smoke tests, an
install guide, and measured size/cold-start numbers.

## Smoke test

`scripts/artifact-smoke.sh [runtime] [pack] [port]` — deterministic,
CI-able, exits non-zero on any failure:

1. artifact existence checks (with byte sizes printed);
2. server becomes ready and answers real routes
   (`/health/live` → `{"status":"ok"}`, `/hello/:name`);
3. **mismatched-runtime rejection**: a pack copy claiming engine 9.9.9
   fails closed BEFORE ready with the actionable "engine mismatch"
   diagnostic (guardrail: shared mode rejects mismatched runtime);
4. cold-start sampling from the runtime's own `startupMs` ready-line
   telemetry (default 10 samples).

## Measured (this machine, release build, examples/proof)

| artifact | bytes |
|---|---|
| `velqu-runtime` (release) | 5,194,888 |
| `app.qpack` (proof app) | 24,414 |

Cold start (spawn → ready), 10 samples: **p50 ≈ 3.84 ms** at 2 routes
(raw range 3.20–7.23 ms). RSS and route-count scaling are covered by
the M25-010-C/D evidence; this packet pins the deployment-artifact view.
Standalone-mode deltas are M26-009-B's measurement obligation
("Startup/RSS differences are measured" across both modes).

## Install guide

`docs/beta/INSTALL.md` — prerequisites, the two files and their
producers, run command, fingerprint rule (pack runs only on its exact
runtime build; runtime upgrade = rebuild both), update matrix, limits,
and an explicit pointer that standalone mode is a separate deliverable.

## Guardrail status

- Both modes pass identical conformance — conformance suites drive the
  same compiled pack over HTTP (`bun test`); standalone mode inherits
  them in M26-009-B.
- Standalone contains no compiler toolchain — N/A here (shared mode
  ships no toolchain either; enforced for standalone in B).
- Shared mode rejects mismatched runtime — proven by smoke step 3.
- Startup/RSS differences are measured — shared-mode numbers above;
  cross-mode delta lands with M26-009-B's report.
