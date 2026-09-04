# BETA-009-D — Chaos Tests for Upstream, Database, and Worker Poison

## Scope and result

This packet re-runs the existing deterministic fault-injection evidence across
network upstreams, database leases, and QuickJS workers. No runtime behavior
was changed.

### Worker poison / cancellation

`docs/reports/m3-010-b-chaos.md` and its committed raw
`benchmarks/raw/worker-scaling/soak-summary.json` record the 901-second
`velqu-soak-v2` chaos run:

- 14 worker poison/rebuild cycles (7 per worker), each replacement initialized
  and served in approximately 3–11 ms.
- 1,703,012 dispatched; 1,685,983 completed/verified; 8,518 expected injected
  disconnects; 8,511 expected injected timeouts; 0 unexplained errors.
- Exact accounting: completed + expected disconnects + expected timeouts equals
  dispatched; no lost request, mismatch, or panic.
- Final per-worker heaps 202,104 / 201,880 B; RSS fell 5,772 → 5,560 KiB.
- Graceful drain ended with all slots quiesced and all cancellations accounted.

The supporting q-engine-quickjs, q-bridge, q-runtime conformance, and drain
suites also pass in the BETA-009-A/V evidence runs.

### Upstream fault injection

`cargo test -p velqu-runtime --test fetch_fixture_conformance` covers:

- DNS rebinding table with a private answer — host validation rejects the
  poisoned address set.
- Slow upstream body — explicit 500 ms read budget cuts a ~1.2 s four-chunk
  body in under 3 s.
- Immediate-close and garbage TLS handshakes — both fail closed within the
  bounded 10 s dial budget.
- Redirect chain and pool concurrency — every hop is policy-checked and the
  active-connection bound serializes fixture traffic.

`fetch_proxy_cancellation_conformance` additionally poisons every ambient
proxy variable; the request does not traverse the poison listener, confirming
ambient proxy env is ignored.

### Database fault injection

`q-capability-postgres` deterministic mock connector tests cover:

- delayed connect → typed `ConnectTimeout`, no hanging acquire;
- rejected connect → typed `ConnectRejected`;
- capacity wait → typed bounded `AtCapacity`;
- error/timeout lease discard → poisoned connections close instead of returning
  to idle; `discarded_error` counter increments;
- shutdown → new acquires rejected and in-flight release closes connections;
- stale/dead idle connections discarded and replaced.

## Triage

No new crash, panic, orphan invocation, unbounded wait, connection reuse after
error, or unexplained chaos failure was found. All injected failures are
classified expected outcomes with bounded cleanup. The existing M3-010 chaos
report remains the raw soak source; this report links it rather than
fabricating a new multi-hour run.

## Companion evidence and known limitations

- `docs/reports/m3-010-b-chaos.md`
- `benchmarks/raw/worker-scaling/soak-summary.json`
- `docs/reports/beta-009-a-fuzz-suites.md`
- `docs/reports/beta-009-c-threat-model-review.md`
- `docs/beta/LIMITS-AND-NON-GOALS.md`
- `docs/reports/m28-002-d-maintenance-security.md`

The checked-in chaos run is deterministic fixture/soak evidence, not a claim
of production-scale availability or a hostile-code sandbox. External database
credentials and public upstreams are not required for the mock/fixture tests.
