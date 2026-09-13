# RC Canary Program and Staged Rollout Protocol (M7-006)

This document defines the Release Candidate (RC) canary program, staged rollout procedure, upgrade/downgrade validation matrix, and automated abort criteria for Velqu production releases.

---

## 1. Objectives and Scope

The canary program provides empirical operational verification of RC artifacts before general promotion (GA):
- Validate real-world consumer workloads across the staged rollout stages.
- Prove non-disruptive rolling upgrades and emergency rollbacks without data corruption.
- Detect latent performance, memory, or protocol regressions under production-like traffic.

---

## 2. Staged Rollout Architecture

Canary deployment follows a four-phase progressive promotion sequence:

| Phase | Traffic Share | Duration | Scope and Gate |
|---|---|---|---|
| **Phase 0: Synthetic Shadow** | 0% (Shadow) | 2 hours | Internal mirrored traffic against live baseline. Validates HTTP compliance, response equivalence, and zero unhandled panic/crash. |
| **Phase 1: Internal Canary** | 1% – 5% | 12 hours | Internal services and opt-in non-critical workloads. Continuous monitoring of error rate, latency percentiles, and RSS bounds. |
| **Phase 2: Broad Canary** | 25% | 24 hours | Representative multi-region traffic across varied client profiles (browsers, HTTP/1.1 keep-alive connections). |
| **Phase 3: GA Promotion** | 100% | Permanent | Complete migration of all traffic after all exit criteria are satisfied. |

---

## 3. Upgrade and Downgrade Compatibility Matrix

Every release candidate must pass forward and backward compatibility verification across three architectural layers:

```
+-------------------------------------------------------------+
| Layer 1: QPack Bytecode and Engine Runtime Compatibility    |
| - QPack v2 engine fingerprint match (SEC-001)               |
| - Bytecode version compatibility check (fail-closed)       |
+-------------------------------------------------------------+
| Layer 2: Schema Contract and Route Graph Compatibility      |
| - Treaty client forward/backward contract compatibility     |
| - No unannounced route removals or breaking schema changes  |
+-------------------------------------------------------------+
| Layer 3: Capability ABI and Resource Boundaries             |
| - Native capability interface stability (timer, fetch, db)  |
| - Dispatcher queue and memory quotas preserved              |
+-------------------------------------------------------------+
```

### Compatibility Rules
1. **Rolling Upgrade (N → N+1)**:
   - Existing QPack files continue running on N+1 runtime when bytecode engine match is satisfied.
   - If QPack bytecode format increments, `q-bytecode-tool` recompiles packs without source changes.
   - Treaty clients on version N communicate with servers running N+1 without contract violation.
2. **Emergency Downgrade (N+1 → N)**:
   - Reverting binary `velqu-runtime` to version N immediately restores previous serving behavior.
   - Persistent stores (such as namespaced KV or PostgreSQL databases) do not perform irreversible state mutations during canary phases.

---

## 4. Automated Abort and Rollback Triggers

Any of the following signals automatically triggers immediate rollback to the stable baseline release:

| Signal | Threshold | Action |
|---|---|---|
| **5xx Error Rate** | > 0.05% of requests over a 5-minute rolling window | **Immediate Rollback** via reverse proxy drain |
| **P95 Latency Regression** | > 25% increase compared to baseline release | **Halt Promotion**; analyze route distribution |
| **Process RSS Drift** | Monotonic RSS increase > 5% per hour post-warmup | **Immediate Rollback** (potential memory retention) |
| **Crash / Panic** | > 0 unhandled panics or core dumps | **Immediate Rollback**; quarantine instance |
| **Worker Quarantine** | Worker restart frequency > 1 per 100,000 requests | **Halt Promotion** |

### Rollback Execution
1. Set canary weight to 0% at the reverse proxy (Nginx / load balancer).
2. Issue graceful drain (`/health/ready` returning 503 while allowing existing connections to finish within 15s).
3. Terminate canary instances and preserve logs/core dumps for postmortem analysis.

---

## 5. Promotion Exit Criteria

To exit the canary program and qualify for GA promotion:
- [ ] Completed Phase 2 (24-hour continuous run under 25% traffic share).
- [ ] Processed ≥ 10,000,000 cumulative requests across all canary nodes.
- [ ] Zero unhandled panics, segmentation faults, or worker quarantine loops.
- [ ] P50 and P95 latency remained within approved SLA bounds (see `SLOS_AND_ALERTS.md`).
- [ ] Memory footprint remained bounded with zero monotonic growth.
- [ ] Successful rehearsal of upgrade and downgrade rollback procedures.
