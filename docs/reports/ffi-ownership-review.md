---
type: Evidence Report
title: FFI / Ownership Review (Rust ↔ QuickJS boundary)
status: complete
milestone: M1
---

# FFI and ownership review

Scope: every place Rust and QuickJS exchange values or memory in M1.

## Inventory

| Boundary | Mechanism | Ownership rule |
|---|---|---|
| handler/policy refs | `Persistent<Function<'static>>` saved inside `ctx.with`, cached in `BTreeMap` | refcounted by QuickJS; cache dropped on the worker thread before Runtime drop (field order in `WorkerInner`: handler_cache → ctx → rt) |
| request handles | opaque `(slot: usize, generation: u64)` numbers crossing FFI | no pointers cross; the Rust-side slab owns `RequestMeta`; settle bumps generation → expired access throws a JS Error |
| request field reads | native fns return JSON strings / body text / length | engine-side `JSON.parse`; no shared buffers |
| body bytes into JS | `__velquFillBytes(slot, gen, target: Uint8Array)` | **the one `unsafe` block** (below) |
| timer capability | JS-side op table stores resolve/reject closures; native registry stores only `op_id → invocation_id` (u64s) | completions arrive on the worker loop as messages; no JS values cross threads |
| promise settlement | prelude watch table + `__velquOpResolve/Reject` called from the worker | single-threaded; no cross-thread JS refs |
| exception details | `Exception::message()/stack()` read inside `ctx.with` | copied into `String` (owned) before leaving the scope |

## The single `unsafe` block

`crates/q-engine-quickjs/src/worker.rs`, `__velquFillBytes`:
`std::ptr::copy_nonoverlapping` from an owned `Vec<u8>` (body snapshot) into
the backing store of a caller-provided `Uint8Array`. Invariants documented
inline: (1) the TypedArray is owned by the current native call on the worker
thread; (2) its length was pre-allocated from the same body snapshot via
`__velquReqBodyLen`; (3) single-threaded execution means no aliasing while
the copy runs. Review status: reviewed, bounded, and the only unsafe in the
workspace (grep-verified).

## Lifetime rules enforced by construction

1. All JS values live only inside `ctx.with` closures; every crossing out
   converts to owned `'static` data (`Outcome`, `LoadStats`, `String`).
2. `WorkerInner` field order guarantees the Persistent cache and Context drop
   BEFORE the Runtime — QuickJS asserts on live objects at `JS_FreeRuntime`
   otherwise (observed during development; fixed; regression-tested by every
   engine test teardown).
3. Late native completions carry only op ids; if the op was settled/cancelled
   the registry entry is gone and the completion is dropped+counted — stale
   completions can never touch reused invocation state.
4. Cancellation settles the request slot immediately; retained JS wrappers
   fail deterministically with "request handle expired" (tested).

## Engine-adapter seam

`q-engine::Engine` is the only trait the runtime sees; `q-engine-quickjs`
owns all rquickjs usage. Upstream-QuickJS comparison (ADR-0006) would add a
second impl without touching the runtime. Honest note: with exactly one
implementation, the trait's generality is currently untested against a second
engine — the seam exists but is not yet exercised by an alternative engine.

## Panic safety

Worker code avoids `unwrap()` on engine results at request time (errors map
to outcomes); the interrupt handler converts runaway loops into deadline
kills. A panic inside the worker thread would poison the mutex → runtime
exits non-zero on next access (fail-stop, not silent corruption). hyper
panics observed during development (missing `TokioTimer`) were fixed with
`.timer(...)`; connection tasks are isolated per-spawn.
