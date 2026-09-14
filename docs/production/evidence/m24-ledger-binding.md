# M24 Ledger Binding — evidence analysis (#1343 packet 1)

Binds production-ledger rows M24-001..M24-009 to PASS with per-row
`evidence_refs`, and records why M24-GATE deliberately stays TODO. This is
evidence mapping only — no runtime change, no re-execution of the beta
program. Method follows the reviewer direction (2026-09-14): map rows
individually; preserve the distinction between implementation evidence,
measured results, and unresolved acceptance; no blanket promotion.

## Source program

The beta program executed M2.4 as `01_m24_zero_copy_ingress`: 60 task
packets (A/B/C/D implement, V verify, Z package) all `status: PASS`
under `docs/beta/program/tasks/01_m24_zero_copy_ingress/`, plus the gate
record `docs/beta/program/gates/M24-GATE.md` (PASS, candidate
`75bda51f6872524d363a0f5eadc2460cf6bc8678`) and
`docs/reports/m24-gate-review.md` (gate close commit `87adf2c5`,
2026-08-21). The production rows' acceptance clauses were checked against
each row's V-packet verification record; `evidence_refs.commit` is the
row's Z (evidence-packaging) commit; `tests` are the suites the V packets
actually cite (all exist in the current checkout).

## Row bindings

| Row | Z commit (evidence_refs.commit) | Acceptance clauses ↔ V-packet evidence |
|---|---|---|
| M24-001 | `d1aa375c` | ADR-0021 INV-1..4 + D4 T1..T12 source-backed: ownership/queue/cancellation/slot invariants (q-bridge lib, engine tests, runtime conformance, security conformance) |
| M24-002 | `01843b21` | route-before-materialization: FieldNeeds gates, `field_free_invocation_skips_request_store_slot`, `routing_precedes_body_materialization`, limits-503 conformance |
| M24-003 | `19f6493e` | worker-local generation-checked slab: slot+generation handles, stale/foreign rejection, zero-after-quiescence (engine + runtime suites) |
| M24-004 | `4907d9e9` | byte-range captures: `capture_ranges_defer_string_allocation…`, per-key materialization, reference-parity suite, HTTP conformance |
| M24-005 | `48c1ed23` | declared-header lazy access incl. `header_materialization_lowercases_names_and_keeps_values` (q-http + runtime/treaty conformance) |
| M24-006 | `8b596cca` | lazy query/cookie: frozen duplicate/percent/empty/invalid semantics (q-http fuzz_parsers + regression corpus, engine suite) |
| M24-007 | `975b5682` | bounded read-once body: oversize/limits rejection, disconnect/cancellation stop reads (runtime conformance + treaty conformance; V-packet records the targeted `cargo test` runs PASS) |
| M24-008 | `5b4a76ca` | native-backed prototypes: cached shapes, opaque handles, RequestExpiredError stability (engine + runtime suites) |
| M24-009 | `229cd6aa` | ingress/bridge observability: bounded/disableable metrics, FieldNeeds visibility (runtime conformance; fuzz/conformance closure in M24-010-Z) |

## M24-GATE — why it stays TODO

Clause-by-clause against the production row's acceptance:

1. **"M24-001 through M24-009 are PASS"** — now true in the ledger (this
   binding).
2. **"No global request-store mutex remains"** — established by
   M24-003's worker-local slab implementation and its suites.
3. **"Unread fields have zero materialization counters"** — established
   by M24-009's counters + the M24-002 field-free invocation test.
4. **"C0 >= 90% of matched raw Rust; C1/C3 p95 do not regress; bridge
   safety suites pass"** — split:
   - **C0 ≥ 90% of matched raw Rust: satisfied by committed measured
     evidence.** Gate-time five-repetition protocol run
     (`benchmarks/raw/warm/g0-warm-1787214167.jsonl`, 5 reps, c=1/10/50,
     zero errors): velqu C0 median p50 28.4/85.8/326.5 μs vs matched
     raw-rust 27.9/77.5/343.9 μs → **98.2% / 90.3% / 105.3%**. Current
     five-repetition run on today's runtime (2026-09-10,
     `warm-1789054920075.jsonl`): 48.3/129.5/490.9 vs 47.5/158.0/490.8 →
     **98.4% / 122.1% / 100.0%**. Both artifacts are committed.
     *Lineage fact discovered during this binding pass, recorded for
     honesty:* the gate-time warm run was generated 2026-08-20T08:27Z —
     about two hours BEFORE the first of the 68 `m24-*` implementation
     commits (~10:04Z) — so it measured the pre-M2.4 runtime; the
     post-M2.4 contemporaneous C0-vs-raw number was never committed at
     gate time. The clause is nonetheless established on the post-M2.4
     runtime by the 2026-09-10 five-repetition run above.
   - **C1/C3 p95 do not regress: NOT bindable from existing evidence.**
     The natural reading (post-M2.4 vs pre-M2.4, same protocol) has no
     committed measurement pair: the gate accepted the pre-M2.4 run, and
     the only later five-repetition run (2026-09-10) spans the M2.5/M2.6/
     M3 milestones, so any delta is not attributable to M2.4. The raw
     numbers (velqu C1 p95 c=1: 290.3 μs pre-M2.4 → 339.3 μs today; C3:
     276.2 → 553.8) are recorded here as an observation across the whole
     later stack, NOT as an M2.4 regression finding. Two resolution
     paths: (a) a same-protocol A/B against the pre-M24 commit
     (`e5acd462^`), scheduled after the in-flight 72 h soak frees the
     benchmark host; (b) an explicit owner disposition that the clause is
     superseded by the current evidence regime. Until one lands, the gate
     row stays TODO.
   - **Bridge safety suites pass** — gate review: clean `./scripts/verify`
     at candidate `75bda51f` (workspace tests, Clippy, Bun 36/36,
     benchmark artifact parity); M24-010-Z closes fuzz/conformance.
5. The gate's original blocker (recorded at `ce751fe8`) was benchmark
   artifact hash parity, resolved before close — not a perf clause.

Consequence: the M25 block (M25-001 depends on M24-GATE) cannot be bound
until the gate resolves. This is the intended honesty cost of not
bulk-promoting.

## Provenance correction carried in this packet

`benchmarks/raw/ga-m6-fuzz/PROVENANCE.md` is corrected per the 2026-09-14
review: the soak was never cut short (it survived the environment
rebuild; the "1.5 h of 72 h" sentence described a wrong belief), and
`miri-ledger startedAt` is the Miri stage START (00:29:41Z), not the
finalization of stages 1+2 (stage 1 finalized 00:29:41Z; Miri completed
~00:59:18Z; soak launched 00:59:41Z). Raw logs and run identities are
untouched.
