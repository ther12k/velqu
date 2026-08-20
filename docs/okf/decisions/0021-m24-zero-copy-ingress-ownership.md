# ADR-0021 — M2.4 Zero-Copy Ingress and Worker-Local Request Ownership

- **Status:** Accepted (2026-08-20)
- **Deciders:** Antigravity Engineering, Architecture Review
- **Consulted:** ADR-0005 (native routing, lazy bridge), ADR-0008 (one runtime per worker), ADR-0018 (M2.4 authorization), AGENTS.md constraints 2/3/7/8/11
- **Informs:** M24-001 … M24-010 (`docs/codex-spark-beta/tasks/01_m24_zero_copy_ingress/`), M3 multi-worker

## Context

M23R2 closed the numeric artifact/router gate: routing is native, fail-before-ready,
and identity-driven (`RouteId`/`PolicyId`/`SchemaId`). The remaining per-request
host overheads live in ingress and the request bridge:

1. `q-http` materializes a `RequestContext` eagerly — `method`/`path` as `String`,
   decoded query pairs, cloned header pairs, and a fully buffered body — **before**
   routing decides whether any of it is needed.
2. `serve.rs` **clones** query/headers/body again into `q_bridge::RequestMeta` when
   inserting into the store: two full copies of data the handler may never read.
3. `q_bridge::RequestStore` is process-wide (`Mutex<Vec<Slot>>` shared by all
   tasks). With one worker this is pure overhead; with M3 multi-worker it becomes
   a contention point and a cross-worker hazard.
4. Client disconnect before response completion has no defined ownership path:
   the ingress future is dropped, but the invocation, slot, and any buffered body
   rely on the engine's timeout to eventually settle.

M2.4 (ADR-0018) authorizes: contract-driven field admission (`FieldNeeds`),
zero-copy header/query borrowing, a worker-local request slab eliminating global
mutexes, and numeric request IDs. This ADR freezes the **ownership model** that
those packets implement. Detailed sub-specs live in the M24-001-B/C/D packets
(body ownership & queue admission; no-copy/bounded-copy boundaries; overload
responses & metrics); this ADR is the frame they elaborate.

## Decision

### D1 — Ownership spine

A request is owned by exactly one party at every moment, and ownership only
moves forward:

```
┌────────────────────────────────────────────────────────────────────────────┐
│ (1) INGRESS — tokio connection task owns the native request                │
│     hyper::Request<Incoming> → method, uri (path+query bytes),             │
│     HeaderMap, body stream — NOTHING decoded, NO RequestContext built      │
│     queue permit acquired BEFORE body download (max_queue semaphore)       │
└───────────────┬────────────────────────────────────────────────────────────┘
                │  route by (method, path) — Rust, before any JS (AGENTS.md #2)
                ▼
        ┌───────────────────┐   404/405/native liveness/quarantine/ready
        │  router.resolve   │──────────────────────────────► response,
        └───────┬───────────┘                                native parts dropped,
                │ Found(RouteId, capture ranges)             permit released
                ▼
┌────────────────────────────────────────────────────────────────────────────┐
│ (2) ADMISSION — bounded transfer into the worker's queue                   │
│     body downloaded ONLY if RoutePlan FieldNeeds wants a body, bounded by  │
│     binding.limit_bytes, read exactly once; oversize → 413, queue full →   │
│     503 BEFORE any body byte is read                                       │
│     ownership of the needed native parts MOVES into the queued job         │
└───────────────┬────────────────────────────────────────────────────────────┘
                ▼
┌────────────────────────────────────────────────────────────────────────────┐
│ (3) WORKER — owns the request slab (no process-wide store)                 │
│     slab entry = slot index + generation + native parts + decoded cache    │
│     JS sees ONLY opaque (slot, generation); materialization happens inside │
│     store.access-style closures that cannot outlive the slot lock          │
│     unread fields are never materialized (counters prove it)               │
└───────────────┬────────────────────────────────────────────────────────────┘
                │  handler settles / timeout / disconnect / quarantine / shutdown
                ▼
┌────────────────────────────────────────────────────────────────────────────┐
│ (4) SETTLEMENT — exactly one owner, idempotent                            │
│     generation += 1 → all outstanding handles expire deterministically;    │
│     native parts dropped; slot returns to the free list; permit released;  │
│     response bytes (owned by the pipeline future) are written or dropped   │
└────────────────────────────────────────────────────────────────────────────┘
```

Key ownership rules:

- **R1 (ingress ownership).** The connection task owns all native request parts
  until routing. No `RequestContext` is built for unrouted requests; 404/405/
  native-liveness responses allocate only the response.
- **R2 (move, don't borrow).** Transfer into the queue and into the slab is by
  **move**. No `&Request`/`&[u8]` crosses an `.await` point or a task boundary.
  Where JavaScript needs a field, a bounded copy is made inside the worker
  (the only sanctioned copies are listed in M24-001-C's no-copy/bounded-copy
  boundary table).
- **R3 (worker-local slab).** The slab is a field of the worker, not a global.
  Handles are `(slot, generation)` interpreted **per worker**; a handle created
  by worker A is meaningless to worker B and must be rejected deterministically
  (M24-003-D). With one worker (M1/M2, AGENTS.md #3) this is structural; M3
  adds the worker-id dimension without changing the rule.
- **R4 (single settlement owner).** Every terminal condition — handler
  completion, deadline timeout, client disconnect, worker quarantine, shutdown —
  funnels to ONE settle routine on the worker. Settlement is idempotent under
  the generation check; the second arriver is a no-op. This extends the proven
  `settle()`/generation design in `q-bridge` today.
- **R5 (response ownership).** The pipeline future owns response bytes once the
  engine hands them over. Client disconnect drops that future; bytes are freed
  with it. The slab slot is settled independently by R4 — response failure
  never resurrects a slot.

### D2 — Terminal invariants (acceptance guardrails)

These are invariants of the M24-010 gate; each names its proof artifact.

- **INV-1 — No request data is borrowed beyond its owner lifetime.**
  Native parts live in exactly one place (ingress task → queued job → slab
  entry); JS-visible access happens through generation-checked closures that
  cannot capture the borrow. Proof: compile-time move semantics + bridge tests
  (`settlement_expires_handle_and_reuse_is_isolated`, M24-003-C/D tests) +
  counters showing zero materialization for unread fields.
- **INV-2 — Queue/body limits are explicit and enforced before work.**
  `max_queue` permits gate admission before body download; `limit_bytes` bounds
  the read-once body; header count/size and URI length limits stay as today;
  slab capacity equals the queue bound so admission is the only growth path.
  Proof: `queue_limit_returns_503_when_saturated`, `body_and_header_limits_reject_oversize`
  (extended in M24-007 to prove 503 precedes any body read).
- **INV-3 — Cancellation has one owner.**
  All terminal paths converge on the worker's settle routine; double-settle is
  a checked no-op (generation). Disconnect, timeout, and completion race to it
  safely (extends `completion_wins_abort_race_without_double_count`,
  `abort_actually_wins_completion_race`, `double_settle_is_idempotent`).
- **INV-4 — The design supports one and multiple workers.**
  Nothing in the spine assumes a single worker: the slab, handles, settlement,
  and queue admission are per-worker; the process-wide mutex disappears. M3
  instantiates N workers over the same rules. Proof: structural review here +
  M24-003-D cross-worker rejection tests (initially exercised at unit level
  with a foreign slab instance).

### D3 — Request slot state machine

```
                 insert (queue admitted, permit held)
    ┌─────────┐ ----------------------------------► ┌─────────┐
    │  FREE   │                                    │ ACTIVE  │◄─────────┐
    └─────────┘ ◄---------------------------------- └─────────┘          │
        ▲          settle(): generation += 1,             │ access(slot,  │
        │          drop native parts + decoded cache      │ generation)   │
        └─────────────── all terminals ──────────────────┤  (lazy,       │
                          handler done                   │   bounded)    │
                          deadline timeout               │               │
                          client disconnect              │ microtask/    │
                          quarantine                     │ native op     │
                          shutdown                       │ still ACTIVE  │
                                                        └───────────────┘
```

Notable points: access never changes state (materialization is read-only under
the slot lock); every terminal path lands in the same settle; slot reuse
allocates a fresh generation so stale handles fail closed (`BridgeError::Expired`).

### D4 — State-machine tests plan

The plan freezes which tests must exist before M24-010 closes; most extend
suites that already pass today.

| # | Transition / property | Test (suite) |
|---|---|---|
| T1 | insert → access → settle → access fails | `settlement_expires_handle_and_reuse_is_isolated` (q-bridge, exists) |
| T2 | access materializes and counts; unread costs zero | `access_materializes_and_counts`, `unread_request_costs_nothing` (q-bridge, exist) |
| T3 | double settle idempotent, live slot count exact | `double_settle_is_idempotent` (q-bridge, exists) |
| T4 | completion vs abort race settles exactly once | `completion_wins_abort_race_without_double_count`, `abort_actually_wins_completion_race` (engine, exist) |
| T5 | timeout settles slot, cancels floating ops | deadline/timeout suite (engine, exists) + M24-003-C slot assertion |
| T6 | disconnect mid-flight settles without double-free | new M24-007/010 conformance test (drop response future, assert 0 live slots) |
| T7 | quarantine settles ALL active slots | extends `cleanup_poison_aborts_all_native_ops_and_zeroes_pending_ops` (engine) |
| T8 | shutdown settles everything, exits clean | `graceful_shutdown_exits_zero` (runtime, exists) + live-slot assertion |
| T9 | queue full → 503 before body read | `queue_limit_returns_503_when_saturated` (runtime, exists) + byte-read assertion M24-007 |
| T10 | body over limit → 413, stream aborted | `body_and_header_limits_reject_oversize` (runtime, exists) + read-once assertion |
| T11 | stale generation on reused slot denied | T1 covers; M24-003-D adds foreign-slab (cross-worker) rejection |
| T12 | slot capacity == queue bound; no growth path besides admission | M24-003 unit test on slab construction from `Limits` |

### D5 — Threat / ownership review

| Threat | Vector | Control (this ADR / packet) |
|---|---|---|
| Use-after-free of request data | JS retains handle past settlement | generation bump at settle; expired access fails closed deterministically (R4, INV-1) |
| Cross-request leakage | slot reuse serves stale data | fresh generation per insert; per-slot decoded cache dropped at settle (D3) |
| Unbounded memory | many bodies/requests queued | permit BEFORE body download; `limit_bytes`; slab capacity == `max_queue` (INV-2) |
| Cancellation race → double free / lost wakeup | disconnect + timeout + completion simultaneously | single settle owner, generation idempotence (R4, INV-3) |
| Cross-worker handle forgery/replay | M3: handle from worker A used on B | per-worker handle interpretation; foreign handles rejected (R3, M24-003-D) |
| Slowloris / header abuse | stalled ingress | existing `header_read_timeout`, header count/size limits retained (INV-2) |
| Permit leak | panicked/dropped ingress future | permit guard is drop-based; settle releases it on every terminal path (D2) |
| Overload opacity | 503s happen silently | overload responses/metrics specified in M24-001-D |

## Consequences

- `q-http`'s eager `RequestContext` construction is retired from the dynamic
  path; `q_bridge::RequestStore`'s process-wide mutex is replaced by a
  worker-local slab (both land in M24-002/M24-003, behavior-preserving for
  conformance tests).
- The bridge's proven generation/expiry semantics and laziness counters carry
  over unchanged; tests T1–T4 already pin them.
- AGENTS.md constraint 11 (everything bounded) gains a sharper form: queue
  permits precede body downloads, and slab capacity is derived from the queue
  bound.
- M3 multi-worker requires no re-design of the bridge: it instantiates the
  same per-worker rules (INV-4).
- Out of scope (unchanged): response-schema slow path (M2.5), binary pack
  (M2.6), WebSockets/SSE remain unauthorized.
