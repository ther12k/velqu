# M2.4 Ingress Ownership, Queue Admission, and Request-Slot Lifecycle

- **Status:** Accepted specification for M24-001-B (2026-08-20)
- **Parent ADR:** [ADR-0021 — M2.4 Zero-Copy Ingress and Worker-Local Request Ownership](../okf/decisions/0021-m24-zero-copy-ingress-ownership.md)
- **Scope:** body ownership, queue admission, disconnect cancellation, and request-slot lifecycle
- **Implementation packets:** M24-001-C/D, M24-002, M24-003, M24-007, M24-010

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

## 8. Out of scope

This specification does not implement the worker-local slab, zero-copy field
views, generated decoders, or overload metrics. Those are M24-001-C/D and
M24-002 through M24-010. It does not authorize WebSockets, SSE, general Node
compatibility, M2.5 response-codec work, or M2.6 binary QPack work.
