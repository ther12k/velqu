# C3 context-plan specialization — measured outcome (2026-09-19, honest negative)

ADR-0045 packet. Implements the owner-directed `CTX_PLAN_VALIDATED_PARAMS_ONLY`
specialization and measures it against the owner-set success metric:
**B−A (in-worker glue) materially lower on all three hosts with A and C−B
approximately unchanged** (baseline: #1393). The metric is NOT met; the
measurement redirected the roadmap. Both halves are the point of this
report.

## What shipped (correct, tested, retained)

- `ContextPlan` (`Requestless | ValidatedParamsOnly | ValidatedFields |
  RequestBacked`) in the portable model; derived ONCE per route at load
  from compiled metadata (the static mirror of the serve
  `needs_request_store` predicate — zero pack-format change, zero
  request-time derivation).
- `InvocationSpec` carries the plan explicitly (every construction site
  states it; default is the generic path).
- Worker dispatch: `ValidatedParamsOnly` + `NO_REQUEST_SLOT` + no policy
  runs `__velquMakeCtxParams` — `Object.create` on the shared context
  prototype + a hoisted **frozen** per-route `routePlan` + `params` own
  property. No `pre` object, no per-field `hasOwnProperty` branching, no
  per-call `routePlan`. Any slot or other plan fails closed to the
  generic `__velquMakeCtx` (slot-override guard test).
- Semantics tests (engine suite, 125 green): ctx `Object.keys` parity
  with the generic slotless path, validated-null own property
  (`ctx.params.name === null` preserved), frozen routePlan identity
  across calls (proves the specialized path runs), shared prototype,
  slot-override guard. Conformance 39/39. fmt/clippy clean.

## Measurement (the owner's success metric)

`q-c3-decompose` (same kit as #1393; tiers B/C/B3 now construct the
production plan) — raw in
`benchmarks/raw/c3-decompose-engine/c3d-{local,halotec,oracle}-20260919T*`:

| host | #1393 B−A | this packet B−A | B3 | verdict |
|---|---|---|---|---|
| local | 6.99 µs | 6.69 µs | 6.30 µs | ~0.3–0.6 µs better — not material |
| Halotec | 13.16 µs | 13.51 µs | 15.23 µs | unchanged |
| Oracle | 11.32 µs | (17.7 µs, noisy) | 11.69 µs | unchanged at best (B3 ≈ #1393) |

A (engine floor) unchanged everywhere (0.19/0.41/0.36 µs). Oracle's B
tier and C tier ran under co-tenant interference (C doubled vs #1393);
the B3 repeat-after-C guard isolated the stable reading.

End-to-end canonical C3 A/B (`b51927ed` vs this HEAD, balanced pairing,
8 matched pairs, 0 errors, local):
**paired ratios 1.0099 / 0.9932 / 1.0495 (c=1/10/50)** — neutral.
Raw: `benchmarks/raw/c3-probe/c3x-local-20260919T070932Z*`.

## Why (root cause of the miss)

The #1392-era stage attribution (`context_construct` ≈ 2.8 µs,
`pre_object_total` ≈ 3.6 µs) came from **nested** stage timers in the
instrumented HTTP runtime — overlapping scopes double-count. The
specialized path provably removes exactly that work (activation is
proven by the frozen-routePlan tests: only the specialized constructor
produces a frozen plan), and B−A moved by only the true marginal cost:
~0.3–0.6 µs. Context construction is a SMALL share of in-worker glue.

## Where the in-worker glue actually is (next packets)

Remaining B−A (~6.3–13.5 µs) decomposition by elimination:

1. **Async-handler promise machinery** — the canonical handler is
   `async`; every invocation pays watch-attach + job-queue settlement
   even for synchronous bodies. The compiler can prove sync handler
   contracts → **SYNC-completion specialization** (owner's flagged
   direction; the ContextPlan plumbing shipped here is its carriage).
2. **Result conversion** (JS → `Outcome`) and **settlement bookkeeping**
   (ownership, sentinel slot handling, oneshot reply).
3. Then the **host↔worker H1/H2/H3** decomposition per the frozen
   roadmap (handoff remains 33–82% of C).

## Disposition

- Keep the specialization: correct, tested, near-zero risk, small real
  win, and the plan carriage the next packets need. ADR-0045 amended
  with this measured outcome.
- Do NOT claim any C3 performance improvement from this packet.
- Next measurement packet: in-worker glue decomposition (sync machinery
  share) before implementing sync-completion; handoff H1/H2/H3 after.
- Oracle run disclosed as interference-affected; no rerun was attempted
  the same day (co-tenant noise), to be repeated with the next campaign.

Diagnostic evidence only (AGENTS constraint 12).
