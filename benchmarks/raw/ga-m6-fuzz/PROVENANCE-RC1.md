# RC1 campaign provenance (2026-10-02/03)

Candidate: **RC1 `8b3dce23db884b11e3a17543825d839453c6e0a4`** (OD-011).
Campaign commit: **`b817788b`** (branch `rc1-fuzzkit-fix`) = the RC SHA
plus ONE harness-only change — the fuzz-target compile fix (#1416/#1417,
`property_order: None` corpus initializers). **The crates under test are
byte-identical to the RC SHA**; the delta exists because the fuzz kit
itself had drifted (see attempt history below).

This file records the run conditions for the ledgers in this directory
(`campaign-ledger.json`, `miri-ledger.json`, `ts-treaty-ledger.json`,
per-target and sanitizer logs). It supplements — does not replace — the
b8fee349-era `PROVENANCE.md`, which remains the record of the original
campaign's environment incident.

## Attempt history (all recorded honestly; the final ledger is attempt 3)

1. **Attempt 1 — environment failure, no verdict implication.** The
   toolchain-sysroot `cc` cannot resolve `-lstdc++` for libFuzzer links;
   all seven targets failed at link (`fuzzer-error`, zero executions,
   zero findings). S1/S2/TS stages passed in the same ledger. Fix:
   `LIBRARY_PATH=/usr/lib/gcc/x86_64-linux-gnu/13` (verified by a link
   test before/without).
2. **Attempt 2 — fuzz-kit drift exposed (#1416).** With the env fix:
   five targets ran to zero findings; `schema_validate` and
   `codec_encoders` failed at COMPILE (`missing field property_order`)
   — the field landed after the last campaign (b8fee349) without
   updating the targets. Fixed harness-only (#1417, merged to master).
3. **Attempt 3 — the recorded verdict.** Full rerun on `b817788b`:
   `campaignPassed: true`, `totalNewFindings: 0`, seven targets
   zero-findings (corpus growth per the ledger), TS campaign pass,
   S1 ASan pass, S2 UBSan pass. Miri (separate driver, same commit):
   `verdict: passed`, `coverageComplete: true`, zero findings over the
   five in-scope crates; exclusions unchanged from the recorded scope.

## Run identity

- Started 2026-10-02T20:03:25Z (fuzz/sanitizer), 2026-10-02T21:59:52Z
  (Miri); host = the qualification workstation (toolchain env),
  co-tenant: soak r5 runs REMOTELY on Halotec (not on this host);
  the 15-minute chaos rerun ran locally mid-campaign (noted for load
  context only — verdict criteria are correctness classifications).
- Toolchains: cargo-fuzz 0.13.2 on nightly; Miri nightly
  1.100.0-nightly (809936eac 2026-09-12); ASan/UBSan stages per the
  campaign script's recorded forms.
- Corpus: the worktree fuzzed from an empty corpus (gitignored
  lineage lives in the main checkout); after the campaign the enriched
  corpus was union-merged back per the M6-002 continuity discipline.
