---
type: Governance
title: Evidence and Benchmark Standard
status: draft
tags:
- evidence
- benchmark
- verification

---

# Evidence and Benchmark Standard

## Claim hierarchy

1. Source and dependency locks.
2. Executable tests and captured command output.
3. Raw benchmark/fuzz/soak/conformance data.
4. Generated reports.
5. Handoff prose.

Verification must fail when a lower-trust layer contradicts a higher-trust layer.

## Required benchmark protocol

- Release builds.
- Pinned candidate versions and artifact hashes.
- At least five repetitions for canonical warm comparisons.
- Randomized candidate order.
- p50/p95/p99, errors, CPU, RSS, environment.
- Queue/pool/bridge stage timings where relevant.
- Raw samples retained.
- Cold categories reported separately.

## Gate evidence

Every gate includes:

```text
source commit
source ZIP and Git bundle
checksums
test/typecheck/clippy output
raw evidence
report
review index
evidence index
known limitations
```

## No manual test counts

Use actual runner output. A static test-attribute count may be supplementary but never the authoritative pass count.
