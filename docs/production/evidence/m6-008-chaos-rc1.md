# M6-008 — Chaos and fault-injection freshness rerun at RC1 (2026-10-03)

Candidate `8b3dce23` (RC1, OD-011). Freshness rerun required by the GA
reconciliation ("rerun at RC candidate for freshness"); no evidence
transferred from the b8fee349-era runs. Protocol identical to
`docs/reports/m3-010-b-chaos.md`: q-soak chaos mode, 2 independent
QuickJS workers behind the bounded dispatcher, continuous closed-loop
verified load.

## Command (RC q-soak sha256 `24bfc207008ac6cc…`, built at the RC SHA)

```
q-soak --workers 2 --duration-secs 900 --window-secs 30 \
  --chaos-secs 60 --disconnect-permille 5 --timeout-permille 5
```

Raw evidence: `benchmarks/raw/ga-m6-008-chaos-rc1/` (soak.jsonl,
soak-summary.json, stdout/stderr logs). Ran on the qualification host
alongside the fuzz campaign; co-tenant load noted for throughput
context only — the verdict criteria are correctness-classification
criteria, not throughput.

## Results

- **901 s run, 2,246,811 dispatched, 2,224,328 completed-verified**
  (completionRate 0.9900 — exactly the expectation for 0.5 %
  disconnect + 0.5 % timeout injection).
- **Error classes contain ONLY the two injected classes**, at exactly
  their configured rates: `injected_disconnect` 11,240 (0.500 %),
  `injected_timeout` 11,243 (0.500 %). **Zero unexpected errors.**
- **14 worker poison/replacement cycles** (every 60 s, alternating
  workers; engine re-init 3.4–3.8 ms per replacement) under live
  traffic — engine replacement held the verified-load contract.
- Leak analysis (harness conclusion): no monotonic retention —
  per-worker heap deltas 648/712 bytes over the run; process RSS drift
  +608 KiB characterized as bounded allocator retention (max
  window-to-window step 280 KiB).
- stderr 0 bytes; natural-expiry completion (summary written by the
  harness's own terminal path), graceful drain included.

## Verdict

**PASS for the RC1 freshness rerun of M6-008's chaos/fault-injection
program, for this candidate, this configuration, and this duration.**
Diagnostic evidence per AGENTS constraint 12; the acceptance mapping to
the ledger row (deadlock/auth-bypass/readiness/rollback behaviors) is
the reconciliation pass's work when all RC1 evidence is in.
