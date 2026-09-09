# Evidence and Verification

Velqu separates **normative targets** from **measured results**; a
performance claim without matched, reproducible evidence is a defect.

## One command

```bash
bun run verify        # tests + typecheck + OKF validation, authorized scope only
```

Release Rust builds use `--release` only. Benchmarks regenerate via
`bun run benchmark:all`. Clean-network environments: run inside
`unshare -rn bash -c 'ip link set lo up; ./scripts/verify'`.

## Evidence rules

- Raw samples are retained; summaries report p50/p95/p99
  (`benchmarks/raw/`, index in `benchmarks/manifest.json`).
- Every completed P0 requirement gets code/test/evidence links in
  [docs/m0-m2-traceability.md](https://github.com/ther12k/velqu/blob/master/docs/m0-m2-traceability.md).
- Evidence must include the exact source commit and exact artifact
  hashes.
- Failures are reported honestly; tests and fixtures are never weakened
  to pass.
- Gate/evidence issues are not closed on self-authored implementation
  claims alone.

## Delivery workflow

One branch per packet, PR with `Closes #<issue>`, squash-merge keeps one
atomic commit per packet —
[docs/beta/program/WORKFLOW.md](https://github.com/ther12k/velqu/blob/master/docs/beta/program/WORKFLOW.md).

## Known CI state (honest disclosure)

The GitHub Actions verify workflows have been stalled with zero executed
steps; local gates in clean worktrees are the current acceptance basis
for every packet. See the repository commit history for per-packet
verification transcripts.

## Independent-build reproducibility

The verify battery includes an independent-build reproducibility check:
two builders produce byte-identical application artifacts (QPack and
companions) from the same source commit.
