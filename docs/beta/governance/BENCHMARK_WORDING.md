---
type: Benchmark Wording Policy
title: Velqu Public Beta Benchmark Wording
status: accepted
date: 2026-08-21
version: 0.1.0-beta.1
tags:
- beta
- benchmark
- wording
- evidence
---

# Velqu Public Beta Benchmark Wording

## Accepted public wording basis

Public beta benchmark claims must come from the current repeated gate evidence
and must state the measurement boundary with the claim. The canonical current
runs are:

- Cold: `benchmarks/raw/cold-start/summary.json` (Velqu-only gate run, 5
  fresh-process samples per class, zero failures).
- Warm: `benchmarks/raw/warm/summary.json` (5 randomized repetitions,
  concurrency 1/10/50, 0 errors).
- Route-count: `benchmarks/raw/route-count/summary.json` (5 fresh processes per
  cell, randomized order).
- Startup profile: `benchmarks/raw/profiles/startup-10000.json`.

Every public claim must identify: the host class (Linux x86_64 glibc), release
builds, loopback HTTP/1.1, frozen proof fixtures, pinned candidate versions,
the repetition count, and the raw-evidence path.

## Allowed claim forms

- Percentile statements traceable to a current summary (p50/p95/p99 with the
  run id and raw path).
- Protocol descriptions (repetitions, randomized order, classes, zero errors).
- Scoped observations explicitly labeled "for this host and fixture".
- Historical comparator numbers only when explicitly labeled
  "historical context, not current gate evidence" and tied to their original
  run.

## Prohibited claim forms

- Universal or unqualified speed claims ("fastest", "always faster").
- Cloud, container-platform, or scale-to-zero cold-start inference from local
  process data.
- Production or SLA performance guarantees.
- Unscoped multiplier claims (for example "Nx faster") without the comparison
  set, host, protocol, run id, and status (current vs historical) attached.
- Any claim not backed by retained raw evidence in the repository.
- Selective reporting: losses, failures, and failed budgets must appear
  alongside wins.

## Report hygiene

- Reports generated from raw data (`scripts/generate-benchmark-reports.py`)
  remain the authoritative formatted layer; hand-edited numbers are forbidden.
- Narrative documents (final report, release-gate report) may summarize but
  must reference the current evidence paths and keep historical comparisons
  labeled historical.
- Failed budgets (for example the 1,000-route scaling budget miss) stay
  recorded honestly and are not re-worded as passes.

## Beta limits

- Benchmarks are local-process, single-host, trusted-code-only, non-SLA, and
  not a production-readiness claim.
- Public comparison tables against other frameworks require a fresh matched
  repeated run under this policy before being presented as current.
