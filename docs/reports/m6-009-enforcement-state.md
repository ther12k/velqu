# M6-009 enforcement state — perf gates implemented, merge blocking pending admin ruleset

Status record for #1319, split from the soak evidence per owner review
(2026-09-16). M6-009's performance acceptance has three distinguishable
parts; conflating them hides the one remaining action.

| part | state | evidence |
|---|---|---|
| Performance-regression gates implemented | **DONE** | `benchmarks/gate-thresholds.json` (v2, committed floors); `scripts/cold-start-gate.ts`, `scripts/throughput-latency-gate.ts` + fail-closed test suite; both run on every runtime-affecting PR and weekly (`.github/workflows/perf-gate.yml`) and have been green on all merged PRs since introduction |
| Automatic merge enforcement (repository-level) | **PENDING one owner/admin action** | `master` has NO GitHub branch protection/ruleset, so a red gate can still be merged manually. The workflow now exposes exactly one always-reporting aggregate check, `perf-gate` (fails if either real gate failed/cancelled; explicit "not applicable" success when a PR touches no runtime path), making required-check semantics safe — the workflow itself triggers on every PR and decides path relevance inside via `dorny/paths-filter`, so the aggregate never silently disappears. **Owner action:** create a branch ruleset on `master` requiring the `perf-gate` check (and, if desired, `verify`). |
| 24h/72h soak + analyzer verdict | **IN PROGRESS** | r3 running (candidate b8fee349); acceptance unchanged: 72h + `--min-hours 72` analyzer + evidence packet; no r4 on this host if interrupted |

Deliberately NOT done to work around the missing ruleset: no waiver, no
rewrite of the M6-009 wording, and no weakening of thresholds. Merge
discipline to date has kept every gated merge green, but discipline is
not enforcement; the ruleset closes that gap.

Tightening the gate floors to catch moderate (not just catastrophic)
regressions remains an owner benchmark decision per AGENTS constraint 12
and is recorded in `gate-thresholds.json`'s own note.
