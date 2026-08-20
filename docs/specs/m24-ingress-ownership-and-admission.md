# M2.4 Ingress Ownership, Queue Admission, and Request-Slot Lifecycle

- **Status:** Accepted specification for M24-001-B/C (2026-08-20)
- **Parent ADR:** [ADR-0021 — M2.4 Zero-Copy Ingress and Worker-Local Request Ownership](../okf/decisions/0021-m24-zero-copy-ingress-ownership.md)
- **Scope:** body ownership, queue admission, disconnect cancellation, request-slot lifecycle, and no-copy/bounded-copy boundaries
- **Implementation packets:** M24-001-D, M24-002, M24-003, M24-004, M24-005, M24-006, M24-007, M24-010

This document makes ADR-0021's ownership spine executable as a sequence of
states and transfer rules. It is normative for M2.4; it specifies behavior but
does not claim that the current M2.3 implementation already provides it.

## 1. Owned request parts

The ingress task receives a Hyper request and initially owns this aggregate:

```text
IngressRequest {
  method: MethodView,          // borrowed/owned transport view
  uri: UriView,                // path and query bytes; query not decoded
  headers: HeaderMapView,      // no full String map
  body: IncomingBody,          // unread stream
  request_id: RequestId,       // numeric host identity; no string required on hot path
  cancel: CancellationSignal,  // notification only; not a second settlement owner
}
```

The aggregate has one owner. A field is either borrowed from this aggregate
for the duration of a synchronous native operation or moved into the next
owner. No reference to a request part crosses an `.await`, task boundary, or
worker boundary. A JavaScript wrapper never owns native request storage; it
owns only an opaque `(worker, slot, generation)` capability.

### 1.1 Ownership states

```text
INGRESS_OWNED
    │ native method/path inspection and bounded URI/header admission
    │ route decision (404/405/native route/Found(RouteId))
    ├──► REJECTED_NATIVE       response owns only its response bytes
    │     (414/431/404/405/503-ready etc.)
    └──► QUEUE_RESERVED         queue permit + queued job own the transfer
             │ worker accepts job
             ▼
         SLOT_RESERVED           worker owns all moved request parts
             │ request slot initialized and generation published
             ▼
         ACTIVE                  JS may request declared fields lazily
             │ one terminal wins
             ▼
         SETTLING                invalidate capability, cancel body/native ops,
             │                    drop parts/cache, release permit
             ▼
         FREE                    slot may be reused with a new generation
```

The `QUEUE_RESERVED` and `SLOT_RESERVED` states are deliberate. They prevent a
request from being ownerless between `try_acquire` and slab insertion, and they
make a panic/drop path observable in tests.

## 2. Queue admission order

Admission is ordered. Later steps must not happen after an earlier step rejects:

1. **Acquire native views.** Read method, path, and query byte ranges from the
   request head. Do not decode query pairs, copy all headers, poll the body, or
   construct `RequestContext`.
2. **Apply transport limits that do not require the body.** Reject URI overflow
   with 414 and header count/byte overflow with 431. The body stream remains
   unread and is dropped or protocol-drained according to the transport policy
   in §5.3.
3. **Route by method/path in Rust.** Resolve the serialized router before
   JavaScript and before body materialization. Return 404/405/Allow or native
   liveness/readiness directly. These responses never reserve a worker slot.
4. **Read the selected RoutePlan.** Obtain `FieldNeeds`, body limit, declared
   content type, deadline, and the worker assignment from verified numeric
   metadata. This is a vector/index operation; no route-name lookup occurs.
5. **Try the worker queue permit.** A failed non-blocking acquisition returns
   RFC 9457-compatible 503 with `Retry-After`; it occurs before any body poll.
   The permit is wrapped in the `QUEUE_RESERVED` job and is released exactly
   once by drop or settlement.
6. **Move the job to the worker.** The job contains only fields allowed by
   `FieldNeeds`: path capture ranges, selected header/query views, and the
   still-unread body stream when `body=true`. No full request clone is allowed.
7. **Create the worker slot.** The worker reserves a bounded slab entry,
   assigns a fresh generation, moves the job parts into it, and publishes the
   opaque capability. Only now can invocation begin.
8. **Admit the body once, if required.** The owner of the active slot performs
   the sole body read. It stops at `max_body_bytes + 1`; over-limit becomes
   413, drops the stream, and settles the slot. A body-free RoutePlan never
   polls or buffers the stream.

A queue permit is held from step 5 until the terminal settlement in step 7/8
and the response pipeline's completion. The slab capacity is bounded by the
worker's queue admission bound; no request path may grow it independently.

## 3. Body ownership and read-once state machine

The body stream is not a `Vec<u8>` until a verified RoutePlan requests it.

```text
NOT_NEEDED ───────────────────────────────────────────────► DROPPED
     │ FieldNeeds.body = true
     ▼
UNREAD ── worker starts sole read ──► READING
  │                                      │
  │ disconnect/timeout/quarantine       ├── EOF within limit ──► READY(bytes)
  └─────────────────────────────────────┤
                                         ├── limit+1 observed ─► REJECTED_413
                                         └── transport error ──► REJECTED_400
```

Rules:

- Only the worker that owns the slot may transition `UNREAD → READING`; a
  second caller receives a deterministic `BodyAlreadyConsumed`/settlement
  error rather than starting another read.
- `READY(bytes)` is bounded by the route's declared limit and may be parsed or
  copied into JS only when the handler calls `json()`, `text()`, or `bytes()`.
  Native validation may consume a bounded body into a native value, but that
  is an explicit RoutePlan admission, not an implicit bridge copy.
- `NOT_NEEDED` means the body is never materialized. If the connection must be
  kept reusable, the transport may bounded-drain/discard the stream without
  exposing bytes to the application; otherwise it closes the connection. This
  choice is transport policy, not application ownership, and is covered by the
  HTTP conformance test.
- On every non-READY terminal, the stream is dropped/aborted before the slot
  returns to `FREE`; no body task may retain it.
- A body limit is checked while reading, not after an unbounded collect.

## 4. Disconnect and cancellation ownership

Disconnect detection belongs to the ingress/response future, but settlement
belongs to the worker. This avoids two callers mutating a slot:

```text
Ingress observes write/caller cancellation
        │ sends Cancel(RequestId, Disconnect) once
        ▼
Worker cancellation arbiter (single owner)
        │ compare-and-set ACTIVE → SETTLING
        ├── winner: cancel body read + native operations; invalidate generation
        │           settle reply channel; drop parts/cache; release permit
        └── loser: observes SETTLING/FREE and does nothing
```

The same arbiter handles `HandlerComplete`, `Deadline`, `Disconnect`,
`Quarantine`, and `Shutdown`. A terminal reason is recorded once for metrics;
the client receives no second response. The ingress task never calls
`RequestStore::settle` directly and never frees worker-owned parts.

### 4.1 Race precedence

The observable winner is the first successful `ACTIVE → SETTLING` transition.
If completion wins, the response may be written; if disconnect wins, the
response future is dropped. In both cases slot invalidation, native-operation
cancellation, body-stream termination, and permit release are identical. A
late completion, timer, or disconnect can only observe the generation mismatch
and becomes a no-op.

### 4.2 Cancellation before worker activation

If disconnect occurs in `QUEUE_RESERVED` or `SLOT_RESERVED`, the queued job is
cancelled before JS invocation. The worker (or queue owner while transferring)
performs the same drop-and-release routine; no slot is published to JS. This
case must not leave a permit, body stream, or slab reservation live.

## 5. Request-slot lifecycle

Each worker owns a bounded slab. A slot contains native request parts, decoded
field cache, body state, request ID, and generation. It never contains a
borrowed reference to ingress memory.

| State | Owner | Allowed operations | Exit |
|---|---|---|---|
| `FREE` | worker slab | reserve only | `SLOT_RESERVED` |
| `SLOT_RESERVED` | worker transfer routine | move parts, initialize generation | `ACTIVE` or `FREE` on failed transfer |
| `ACTIVE` | worker invocation | lazy declared-field access, one body read, cancellation request | `SETTLING` |
| `SETTLING` | worker settlement routine | invalidate generation, abort operations, drop cache/parts, release permit | `FREE` |
| `FREE` | worker slab | no stale capability accepted | `SLOT_RESERVED` with fresh generation |

The capability is valid only when all three checks pass: worker identity (when
M3 is active), slot index, and generation. Generation increments before the
slot becomes reusable. `FREE` is not an accessible state. A stale capability
therefore fails before touching request bytes.

### 5.1 Settlement checklist

The single settlement routine must perform, in a bounded operation:

1. win `ACTIVE/SLOT_RESERVED → SETTLING` or return if another terminal won;
2. increment generation and mark the capability expired;
3. cancel the body read and all invocation-owned native operations;
4. clear decoded field cache and release body bytes;
5. drop remaining headers/query/path views and the transport body;
6. send at most one invocation outcome/cancellation notification;
7. release the queue permit and slab reservation;
8. transition to `FREE` and increment terminal metrics exactly once.

If cleanup itself exceeds its bounded budget, the worker follows the existing
quarantine policy; it does not extend the request lifetime or retry settlement
from another owner.

### 5.2 Request IDs

The hot path uses a numeric `RequestId` allocated by the host clock. A readable
string is created only for logs/problems that require one. Request IDs are not
used as map keys for routing, handler dispatch, slot lookup, or cancellation;
the worker's slot/generation capability is the ownership identity.

### 5.3 Early rejection and body disposition

| Rejection | Body polled? | Owner after rejection | Response |
|---|---:|---|---|
| URI too large | no | transport/connection policy | 414 |
| header count/bytes too large | no | transport/connection policy | 431 |
| no route | no | transport/connection policy | 404 |
| method not allowed | no | transport/connection policy | 405 + Allow |
| native liveness/readiness | no | transport/connection policy | native response |
| worker queue full | no | transport/connection policy | 503 + Retry-After |
| body not declared/needed | no application read | transport policy; optional bounded discard | route response |
| body over route limit | yes, bounded to limit+1 | worker until abort, then settlement | 413 |
| body transport failure | yes, once | worker until error, then settlement | 400 |

The transport policy must ensure an early response does not leave an unbounded
unread stream attached to a reusable connection. It may drain within a bounded
budget or close the connection; it must never copy the body into an application
request object merely to make the connection reusable.

## 6. Acceptance and evidence plan

M24-001-B is complete when the following artifacts and tests are present in the
implementation packets that consume this specification:

| ID | Required proof | Owning packet |
|---|---|---|
| B1 | Queue-full response occurs before the first body poll | M24-001-D, M24-007 |
| B2 | Body read is at most once and bounded at `limit+1` | M24-001-C, M24-007 |
| B3 | Body-free route never polls/materializes body | M24-002, M24-007 |
| B4 | Disconnect in queue, reading, and active-handler states settles once | M24-001-D, M24-003-C, M24-010 |
| B5 | Completion/disconnect/timeout race releases one permit and one slot | M24-003-C, M24-010 |
| B6 | Slot reservation failure cannot leak moved parts | M24-003-A, M24-003-V |
| B7 | Stale generation and foreign-worker capabilities fail closed | M24-003-B/D |
| B8 | Early response body disposition is bounded and conformance-tested | M24-007, M24-010 |
| B9 | Slab live count returns to zero after every terminal | M24-003-C, M24-010 |

This packet is a specification, not evidence that these future tests already
pass. Existing M2.3 tests listed in ADR-0021 remain regression coverage; the
M24 packets must add the ownership assertions above before the M24 gate.

## 7. Threat / ownership review

| Threat | Failure mode | Required control |
|---|---|---|
| Body read before routing | eager `collect()` buffers unused body | route and `FieldNeeds` precede every body poll |
| Queue bypass | body read while no worker capacity | permit acquired before transfer/read |
| Double body consumption | policy and handler both read stream | one owner/state transition; subsequent access fails |
| Disconnect leak | dropped response leaves slot/body live | worker arbiter settles on disconnect; live count/permit assertions |
| Double settlement | completion and timeout both free parts | one CAS/generation winner; losers are no-ops |
| Stale handle reads reused slot | JS continuation accesses next request | generation bump before reuse; worker ID check in M3 |
| Unbounded discard | early rejection drains unlimited attacker body | bounded drain or connection close; no application copy |
| Permit leak | cancellation/panic skips release | RAII permit guard plus settlement drop path |
| Cross-worker ownership | worker B accesses worker A's slot | capability includes worker identity or worker-local namespace |

## 8. No-copy and bounded-copy boundaries (M24-001-C)

Every movement of request data is classified exactly one way. "Zero-copy"
means a borrow or byte-range view inside one owner with no allocation;
"bounded copy" means a new allocation whose size is capped by a named limit
**before or during** the copy; anything else is forbidden.

### 8.1 Zero-copy (views and borrows)

| Transfer | View over | Lifetime rule |
|---|---|---|
| method/path bytes → router resolve | hyper request head | borrowed for the synchronous match only |
| path capture ranges (`start..end`) | URI path bytes | stored as ranges in the slot; bytes stay in the single owned buffer |
| declared header lookup by compiled header-name ID | `HeaderMap` | borrowed during validation/parse-from-bytes |
| query raw byte scan for declared keys | URI query bytes | borrowed during scan; no pair materialization |
| body stream ownership move (ingress → job → slot) | hyper `Incoming` | moved, never cloned; unread until §3 read |
| native JSON body → `serde_json::Value` (native strategy) | buffered body bytes | parsed once in place by the owning worker |
| response bytes engine → pipeline future | engine output | moved once; socket write consumes it |

### 8.2 Bounded copies (named bound per copy)

| Copy | Bound source | When it happens |
|---|---|---|
| path param bytes → JS string | `max_uri_bytes` | handler accesses `ctx.params` and `FieldNeeds.params` |
| query value percent-decode → JS string | `max_uri_bytes` | handler accesses a declared query key |
| header value bytes → JS string | `max_header_bytes` | handler accesses a declared header |
| cookie value decode | `max_header_bytes` | handler accesses a declared cookie |
| body stream → `Vec<u8>` buffer | route `limit_bytes` (checked at `limit+1`) | the single §3 read |
| body buffer → JS `text()`/`json()` string | route `limit_bytes` | handler calls `json()`/`text()` |
| body buffer → JS `bytes()` `Uint8Array` | route `limit_bytes` | handler calls `bytes()` |
| pre-validated params/query/body → JS object (native strategy) | schema-bounded by `limit_bytes` | invocation setup, only for declared fields |
| per-slot decoded-field cache entries | the copied field's own bound | first lazy access; dropped at settlement |
| log/problem request-ID string | fixed small constant | only when a log line or problem is produced |

Rules for bounded copies:

- **C1** — every copy names its bound; a copy without a named bound is a
  specification violation and must fail review.
- **C2** — bounds are enforced while copying (stream reads stop at
  `limit+1`; string materialization cannot exceed its source view's length,
  which is itself limited at admission).
- **C3** — a bounded copy happens only when a verified RoutePlan declared the
  field **and** the handler actually accessed it (laziness); unread fields
  copy zero bytes (counter-provable).
- **C4** — each slot's decoded cache is bounded by the sum of the *accessed*
  fields' bounds, never by the sum of all declared fields; it is dropped at
  settlement.
- **C5** — worst-case per-request allocation is therefore computable:
  `max_uri_bytes + max_header_bytes + limit_bytes + fixed overhead`, and the
  slab capacity (`max_queue`) bounds how many such requests exist at once.

### 8.3 Forbidden (must not exist on the M2.4 path)

- Full header map → `Vec<(String, String)>` clone for every request (current
  M2.3 ingress behavior; removed by M24-002/M24-005).
- Query pre-parse into pair vectors before `FieldNeeds` is known (current
  behavior; removed by M24-002/M24-006).
- `collect()` of the body before routing or without a route body binding
  (current behavior; removed by M24-002/M24-007).
- Cloning `RequestMeta` (query/headers/body) into the request store (current
  `serve.rs`; replaced by move in M24-003).
- Double body buffering (transport buffer plus application copy of the same
  bytes without an intervening drop).
- Any copy whose size is not bounded by a named admission limit.
- Copying request bytes across a worker boundary; M3 passes capabilities, not
  buffers.

### 8.4 Boundary test plan (extends §6)

| ID | Required proof | Owning packet |
|---|---|---|
| C-T1 | Unread param/query/header/body fields materialize zero bytes (`materialized_bytes` counters) | M24-002, M24-005, M24-007 |
| C-T2 | Path/query/header JS strings never exceed their source bounds | M24-004, M24-006 |
| C-T3 | Body buffer stops at `limit+1`; 413 before full read | M24-007 |
| C-T4 | Decoded cache exists only for accessed fields and is dropped at settle | M24-003-C, M24-006 |
| C-T5 | No full header-map clone on the request path (allocation counter) | M24-002, M24-005 |
| C-T6 | `RequestMeta` clone into store is gone; parts move once | M24-003 |
| C-T7 | Worst-case per-request allocation is documented and asserted in a stress profile | M24-010 |

## 9. Overload responses and metrics (M24-001-D)

Overload is an expected admission outcome, not an untyped engine failure. Every
rejection returns the declared RFC 9457-compatible problem shape and releases
any resource already acquired. The response must not disclose queue depth,
worker identity, filesystem paths, or internal exception details.

### 9.1 Response contract

| Condition | Status | Required headers | Problem identity | Body poll? | Permit/slot effect |
|---|---:|---|---|---:|---|
| URI exceeds `max_uri_bytes` | 414 | `content-type` | `limit` (`uri`) | no | no permit/slot acquired |
| Header count/bytes exceeds limit | 431 | `content-type` | `limit` (`headers`) | no | no permit/slot acquired |
| No matching route | 404 | `content-type` | `not-found` | no | no permit/slot acquired |
| Method unsupported | 405 | `content-type`, `allow` | `method` | no | no permit/slot acquired |
| Worker queue has no permit | 503 | `content-type`, `retry-after: 1` | `overload` | no | no permit/slot acquired |
| Engine quarantined/not ready | 503 | `content-type`, `retry-after: 1` where retryable | `internal` | no application read | no new slot; existing slots settle |
| Declared body exceeds route limit | 413 | `content-type` | `limit` (`body`) | yes, bounded to `limit+1` | active slot settles once |
| Body transport/read error | 400 | `content-type` | `body` | yes, once | active slot settles once |

The queue-full response is generated immediately after the non-blocking permit
attempt and **before the body stream is polled**. `Retry-After: 1` is a fixed,
conservative retry hint for the current single-worker service; future profiles
may select a different fixed policy, but it must remain declared and bounded.
No response may ask a client to retry an invalid request (414/431/404/405/413).

A response generated before slot ownership does not create a slot merely to
record metrics. A response after slot ownership must use the single settlement
routine from §5.1. The response body is bounded by the problem serializer's
fixed fields plus bounded field errors; internal details stay in logs only.

### 9.2 Metric vocabulary

The runtime exposes counters/gauges/histograms with a fixed name and label
vocabulary. Label values are enums or bounded numeric buckets; request IDs,
paths, header values, exception text, and arbitrary route names are forbidden
as metric labels.

| Metric | Type | Labels | Meaning |
|---|---|---|---|
| `velqu_ingress_admissions_total` | counter | `outcome={accepted,rejected}`, `stage={transport,routing,queue,body,worker}` | one increment for each admission decision |
| `velqu_ingress_rejections_total` | counter | `reason={uri_limit,header_limit,not_found,method_not_allowed,queue_full,not_ready,body_limit,body_read}`, `stage` | one increment per returned rejection; exactly one reason per request |
| `velqu_ingress_queue_permits_in_use` | gauge | `worker_bucket={single,worker_0..worker_15}` | permits currently held; must return to zero after drain |
| `velqu_ingress_queue_saturation_total` | counter | `worker_bucket` | failed non-blocking permit acquisitions |
| `velqu_ingress_body_bytes_total` | counter | `outcome={accepted,rejected,discarded}` | bounded bytes actually read/discarded, not declared limits |
| `velqu_ingress_body_reads_total` | counter | `outcome={started,completed,aborted,over_limit,error}` | proves read-once ownership and terminal result |
| `velqu_ingress_slots_in_use` | gauge | `worker_bucket` | active/reserved slab entries |
| `velqu_ingress_settlements_total` | counter | `reason={complete,disconnect,timeout,quarantine,shutdown,admission_error}` | exactly one terminal increment per slot |
| `velqu_ingress_materializations_total` | counter | `field={params,query,headers,body}`, `size_bucket={0,1..64,65..1024,1025..65536,65537+}` | lazy field materialization counts; bounded size bucket only |
| `velqu_ingress_admission_duration_seconds` | histogram | `stage={transport,routing,queue,body,worker}` | time spent in bounded admission stages |

The `worker_bucket` vocabulary is fixed to `single` for M1/M2 and at most
`worker_0` through `worker_15` for a future bounded M3 profile. A deployment
with more workers must aggregate into a configured bounded bucket set; it must
not create an unbounded label per worker.

Metrics are host-owned and may be sampled/exported asynchronously after the
request terminal. Export failure never changes the HTTP result and never keeps
a request slot, body, or permit alive. A metrics snapshot is best-effort; the
lifecycle counters used for correctness (`slots_in_use`, permits, settlements)
must be updated before the slot reaches `FREE`.

### 9.3 Backpressure and fairness rules

- Admission uses `try_acquire`, never an unbounded wait in the connection task.
- Queue capacity, body limits, slab capacity, and exporter buffers are finite
  configuration values validated before ready.
- One oversized or slow body cannot hold more than its one permit or extend the
  header/body deadline; disconnect aborts its read and settles its slot.
- Metrics and logs are not allowed to allocate a queue-sized batch per request.
- A queue-full response must be cheap enough to remain available during
  saturation; it cannot invoke JavaScript or acquire the engine mutex.
- The response status is stable under overload; metrics may distinguish the
  bounded reason/stage but must never become part of application behavior.

### 9.4 Overload and metrics test plan

| ID | Required proof | Owning packet |
|---|---|---|
| D-T1 | Queue saturation returns 503 + `Retry-After: 1` without polling a body | M24-007, M24-010 |
| D-T2 | URI/header/body limits return 414/431/413 with the declared problem and no leaked permit | M24-007 |
| D-T3 | Every rejection increments exactly one bounded reason/stage counter | M24-010 |
| D-T4 | `permits_in_use` and `slots_in_use` return to zero after completion, disconnect, timeout, and rejection | M24-003-C, M24-010 |
| D-T5 | Body byte/read counters distinguish accepted, discarded, over-limit, and transport-error paths | M24-007, M24-010 |
| D-T6 | Metric labels remain within the fixed vocabulary under random paths, headers, and request IDs | M24-010 |
| D-T7 | Exporter failure does not alter response or retain request resources | M24-010 |
| D-T8 | Saturated queue still serves native liveness/readiness according to the declared profile | M24-001-V, M24-010 |

This packet is a specification, not evidence that the future metric and
backpressure tests already pass. Existing `queue_limit_returns_503_when_saturated`,
`body_and_header_limits_reject_oversize`, and quarantine conformance tests are
regression coverage; D-T1–D-T8 add the M2.4 ownership/metrics assertions.

## 10. Out of scope

This specification does not implement the worker-local slab, zero-copy field
views, generated decoders, or overload metrics. Those are M24-002 through
M24-010. It does not authorize WebSockets, SSE, general Node compatibility,
M2.5 response-codec work, or M2.6 binary QPack work.
