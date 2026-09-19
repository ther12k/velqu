# ADR-0045: Compiler-answered context plans (specialized context construction)

- Status: Accepted
- Date: 2026-09-19
- Deciders: Owner (direction), agent (implementation)
- References: #1392 (slotless cross-host), #1393 (three-tier decomposition),
  M24-008 (shared context prototypes), ADR-0044 (build-time constant folding
  philosophy)

## Context

The three-tier C3 decomposition (#1393) showed the remaining engine-side
dynamic-route cost is overwhelmingly Velqu integration, split by host:
in-worker context glue is 58.6% (local) / 68.8% (Oracle) of the engine-only
invocation, and the host↔worker handoff is 81.7% on the 4.15-kernel Halotec
host. Actual QuickJS execution is 0.19–0.39 µs everywhere. Inside the
worker, the per-invocation glue is: `Persistent` function restores, `pre`
object construction with `serde_json::Value`→JS conversion, a fresh
`routePlan` JS object, and the generic `__velquMakeCtx` which re-derives —
per call — questions the compiler already answered (which fields exist, is
there a slot, are lazy facilities needed).

## Decision

1. Add a `ContextPlan` to the portable model:
   `Requestless | ValidatedParamsOnly | ValidatedFields | RequestBacked`.
2. Derive the plan **once per route at load** from already-compiled
   metadata (RoutePlan `FieldNeeds`, validation strategies, policy
   presence, `full-request`/`raw-response` capabilities, body binding) —
   mirroring exactly the runtime `needs_request_store` predicate, which is
   fully route-static. Zero per-request computation, zero pack-format
   change (the RoutePlan IS compiler output).
3. `InvocationSpec` carries the plan (explicit field; every construction
   site states its plan).
4. The worker dispatches on the plan. `ValidatedParamsOnly` (the canonical
   C3 shape: one natively validated params field, nothing else) uses a
   specialized prelude constructor:
   `Object.create(__velquContextPrototype)` + shared frozen `routePlan` +
   `ctx.params = prevalidated` + cached handler call. No `pre` object, no
   per-field `hasOwnProperty` branching, no fresh `routePlan`.
5. `routePlan` becomes a **shared, frozen, per-route object** (hoisted
   immutable metadata). Observable refinement: a handler that mutates
   `ctx.routePlan` now fails fast (strict mode) instead of mutating a
   per-call copy; reading behavior is unchanged and all existing tests
   read-only.
6. Fail-closed guard: the specialized constructor runs only when
   `plan == ValidatedParamsOnly && slot == NO_REQUEST_SLOT`. Any slot, any
   other plan, policy, full-request, or JS-body fallback keeps the generic
   `__velquMakeCtx` path with identical semantics (own-property presence,
   nullable values, `Object.keys()` shape, lazy facilities).
7. `ValidatedFields` and `Requestless` are modeled but keep the existing
   generic slotless construction this packet; specializing them requires
   their own measured evidence first.

## Success metric (owner-set) — MEASURED OUTCOME (2026-09-19, honest negative)

The metric was: `q-c3-decompose` B−A must drop materially on all three
hosts with A and C−B approximately unchanged versus #1393. Measured:

- local: B−A 6.99 → 6.69 µs (B3 6.30); end-to-end paired ratio vs
  `b51927ed` 1.01/0.99/1.05 (c=1/10/50, 8 pairs, 0 errors) — neutral.
- Halotec: B−A 13.16 → 13.51 µs — unchanged.
- Oracle: co-tenant interference during the run (B3 11.69 µs matches
  #1393's 11.68; the B tier read 17.7 under noise) — unchanged at best.

**Conclusion: the metric is NOT met.** The specialized path activates
(proven by the frozen-routePlan semantics tests, which only the
specialized constructor can satisfy) and removes the designed work (pre
object, per-field branching, per-call routePlan), but context
construction's true share of in-worker glue was ~0.3–0.6 µs — the
#1392/#1393-era stage attribution over-counted it (nested stage timers
double-count overlapping scopes). The remaining in-worker glue is the
async-handler promise machinery, result conversion, and settlement
bookkeeping. The ContextPlan plumbing is RETAINED (correct, tested,
near-zero risk, and the carriage the sync-completion specialization
needs); the NEXT packet is redirected to the measured remainder:
in-worker H-style decomposition + SYNC-handler completion.

## Consequences

- Context construction decisions move from request time to load time
  (ADR-0044 philosophy applied to contexts).
- The prelude gains one specialized constructor; the generic path is the
  documented fallback and the semantics reference.
- `ctx.routePlan` identity becomes per-route stable (was per-call); frozen.
- No pack format, OpenAPI, contract, or public API change.
- Handoff decomposition (H1/H2/H3) remains the NEXT measurement packet; no
  transport change in this ADR.
