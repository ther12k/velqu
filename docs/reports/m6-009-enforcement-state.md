# M6-009 enforcement state — gates implemented, ruleset active, soak pending

Status record for #1319, split from the soak evidence per owner review
(2026-09-16). M6-009's performance acceptance has three distinguishable
parts; conflating them hides the one remaining action.

| part | state | evidence |
|---|---|---|
| Performance-regression gates implemented | **DONE** | `benchmarks/gate-thresholds.json` (v2, committed floors); `scripts/cold-start-gate.ts`, `scripts/throughput-latency-gate.ts` + fail-closed test suite; both run on every runtime-affecting PR and weekly (`.github/workflows/perf-gate.yml`) and have been green on all merged PRs since introduction |
| Automatic merge enforcement (repository-level) | **DONE (2026-09-17)** | Branch ruleset **`perf-gate required on master (M6-009)`** (id `23573680`), enforcement `active`, scoped to `refs/heads/master`, requiring status check `perf-gate` from the `github-actions` app (integration id `15368`). See below for scope notes and verification. |
| 24h/72h soak + analyzer verdict | **BLOCKED (host instability — owner action required)** | r3 was environment-interrupted at 26.254 h (787 healthy windows, flat RSS) by a third host power event — this one unclean (no shutdown record in wtmp; no persistent journal, so cause undetermined; not a kernel-update reboot). Full record: `benchmarks/raw/ga-m6-soak-72h-r3/REBOOT-INCIDENT-2026-09-16-R3.md`. Three consecutive interruptions (r1 25.22 h, r2 0.2 h, r3 26.25 h) disqualify this host: per the standing boundary there is **no r4 here**; the next 72 h soak requires an owner-provided demonstrably stable host, same candidate `b8fee349`, identical q-soak binary (sha256 `6497fb16…6794d`), acceptance unchanged, checkpoint/resume not acceptable. r1–r3 windows are preserved as progress-only evidence, never combined |

## Ruleset details (created 2026-09-17)

Created via the GitHub REST API by the implementation agent under
explicit owner authorization (owner review 2026-09-17: after `perf-gate`
was proven as the single required check — state-machine aggregate,
correct on runtime-relevant and docs-only PRs alike — the owner surfaced
repository admin access for exactly this action, naming the desired
configuration as "require perf-gate on master").

Stored configuration (API readback: `docs/production/evidence/raw/m6-009-ruleset-readback.json`):

- `enforcement: active`, `target: branch`, includes `refs/heads/master`.
- `required_status_checks`: context `perf-gate`, `integration_id 15368`.
- `strict_required_status_checks_policy: false` — the check itself is
  required; branches are not additionally required to be up to date.
- `bypass_actors: []` — no one can merge past a red `perf-gate`. (Admins
  can still edit or delete the ruleset itself; GitHub offers no
  unbreakable rule, so this is enforcement with an auditable, owner-only
  escape hatch, not an absolute.)

Why `perf-gate` works as a required check at all: the aggregate job is
an every-PR always-reporting check (owner review, #1383 state machine) —
it never silently disappears, and a skipped gate job can never launder
into aggregate success. GitHub treats any required-check conclusion other
than `success` as blocking, so the skip-irrelevant path still reports
`perf-gate: success` explicitly rather than vanishing.

Known scope boundaries, recorded honestly:

- Required status checks gate the **pull-request merge path** (the
  mandated delivery path per `docs/beta/program/WORKFLOW.md`). Direct
  pushes to `master` are not gated by status-check rules — an inherent
  GitHub limitation, not a repo-local gap. Optional hardening (owner
  call, deliberately not done unilaterally): add a `push_restrictions`
  rule to the same ruleset to block direct pushes entirely.
- Only `perf-gate` is required. The earlier suggestion of also requiring
  `verify` was dropped in the owner's final disposition ("the single
  required check").

## Behavioral verification (PR #1387, the branch carrying this record)

Observed via the GraphQL `pullRequest { mergeable, mergeStateStatus }`
on 2026-09-17, immediately after opening the PR:

1. With `perf-gate` `IN_PROGRESS` (conclusion `null`) and no merge
   conflicts (`mergeable: MERGEABLE`), GitHub reported
   `mergeStateStatus: BLOCKED` — the pending required check alone blocks
   the merge.
2. The moment `perf-gate` completed (`SUCCESS`), the state flipped to
   `UNSTABLE` — no required check is pending or failing anymore; the
   remaining `UNSTABLE` signal comes only from non-required checks
   (`verify` lanes, browser lanes) still running. The block is therefore
   bound exactly to `perf-gate`'s conclusion, nothing else.

This is the end-to-end proof of "performance regressions block merge
automatically" for the PR path: a red or pending `perf-gate` cannot be
merged around by anyone (`bypass_actors: []`).

Historical note preserved: before the ruleset existed, merge discipline
(green gates on every packet) was convention, not enforcement — no
waiver, no M6-009 rewrite, and no threshold weakening was ever used to
work around that gap. Tightening the gate floors to catch moderate (not
just catastrophic) regressions remains an owner benchmark decision per
AGENTS constraint 12 and is recorded in `gate-thresholds.json`'s own
note.
