# RC1 Nomination Record — GA Track

**Decision:** owner, 2026-10-03 (session reply selecting recommendation 1
of the #1321 decision menu, posted 2026-10-02): nominate **master
`8b3dce23db884b11e3a17543825d839453c6e0a4`** as the first Release
Candidate of the GA track.

| Field | Value |
|---|---|
| Candidate SHA (full) | `8b3dce23db884b11e3a17543825d839453c6e0a4` |
| Content | the live `0.1.0-beta.2` npm set (adoption fixes #1399/#1402/#1407) + docs consistency (#1413/#1414); no code delta beyond the beta.2 line |
| CI at nomination | verify PASS on x86_64 and aarch64 (PR #1414 run); all lanes green |
| Comparison attached | `rc1-comparison-b8fee349-to-8b3dce23.md` (b8fee349 = the r4-soak candidate; supersedes the 2026-09-21 chat comparison b8fee349..153ef9a8) |
| Freeze | the RC revision is fixed; changes flow only as recorded release corrections (development on master continues independently) |

## Qualification path (evidence-transfer doctrine: every campaign rerun
fresh on the exact SHA; no inheritance from b8fee349 evidence)

Launched by the session recording this decision:

- **M6-002/M6-003 fuzz + sanitizer campaign** — `scripts/fuzz-campaign.sh`
  at the RC SHA (fail-closed ledger; ledger lands under
  `benchmarks/raw/ga-m6-fuzz/` per campaign convention). Started
  2026-10-02T18:2xZ (this host, nightly + cargo-fuzz 0.13.2).
- **M6-003 Miri campaign** — `scripts/miri-campaign.sh`, queued after the
  fuzz/sanitizer stages complete (same host contention discipline).
- **M6-009 soak r5 (72 h)** — launched **2026-10-02T18:19:36Z** on the
  assigned stable host (Halotec, 53 days uptime at launch), container
  `velqu-soak-rc1-r5` (`b4223c19f79c`), image `velqu-bench:multihost`
  (**e1de7b18613f — bit-identical to r4's image**), containment identical
  to r4 (`--network none --cpus 3 --memory 4g --restart no`).
  q-soak built at the RC SHA via host build with the canonical remap
  flags; **sha256 `24bfc207008ac6cc79a9a5c5c5978368ad5c35861039bb1c371fa70ec4bc78b5`,
  verified bit-identical on the launch host**. Args identical to r4
  (`--workers 2 --duration-secs 259200 --window-secs 120`, chaos
  disabled). Expected completion ≈ 2026-10-05T18:20Z.
- **M6-008 chaos rerun** — to be scheduled after the campaign stages
  (freshness rerun at RC per the reconciliation delta).

## Not yet decided (remain on the #1321 menu)

Canary assignments (item 2), greenfield rollback disposition (item 3),
publisher identity (item 4). M7-006 execution and the M8-003 signing
event wait on those.
