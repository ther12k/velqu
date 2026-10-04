# canary-shadow self-test transcript (#1419)

Captured 2026-10-04T06:59:47Z. Harness correctness proof only — NOT canary evidence (Phase 0 requires the owner-named baseline/candidate pair per CANARY_PROGRAM §1; assignment = #1321 item 2).

## Identity test (two velqu-runtime instances, same proof pack, ephemeral ports)

```
canary-shadow: 14 comparisons, 0 divergences (0 unhandled-5xx) over 2 pass(es)
IDENTITY_EXIT=0
```

Manifest covers 200 liveness, param routes, authorized 200, typed 401, typed 404 problem, and a POST — the RFC 9457 instance-member mask verified on problem bodies.

## Divergence test (candidate = stub returning 200 {\"stub\":true})

```
DIVERGENCE GET /users/usr_1: status 401 vs 200; ctype application/problem+json vs application/json; body {…instance:"*"…} vs {"stub":true}
canary-shadow: 1 comparisons, 1 divergences (0 unhandled-5xx) over 1 pass(es)
DIVERGENCE_EXIT=1
```

Dead-candidate case (stub crashed pre-listen) was also observed live: reported as transport-error divergence, exit 1 — the fail-closed abort CANARY_PROGRAM §4 requires.
