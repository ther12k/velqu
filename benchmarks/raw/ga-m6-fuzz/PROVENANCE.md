# Provenance — ga-m6-fuzz campaign on candidate b8fee349

Campaign: ga-m6-sustained-fuzz, started 2026-09-13T22:33:27Z, seed 1789338807,
libFuzzer via cargo-fuzz 0.13.2, nightly 1.100.0-nightly. Candidate:
b8fee34909c0d6a4b3b9f76aebe9665cf3192c99 (verified by the orchestrator
before stage 1).

Outcome: campaign-ledger.json verdict campaignPassed=true,
totalNewFindings=0 across all 7 fuzz targets, the TS Treaty encoder
campaign, S1 (ASan workspace) and S2 (UBSan QuickJS C FFI).
miri-ledger.json: coverageComplete=true, anyFindings=false, verdict
"passed" (5/5 required crates).

Incident note (honest record, corrected 2026-09-14): mid-run, the
execution environment was rebuilt around this workspace. The running
chain was NOT visible from the new environment's process table, and two
wrong annotations were made from post-reset snapshots before the facts
were established. This note is the corrected account; raw logs and run
identities are untouched.

Corrected timeline (from the artifacts' own timestamps):

- 2026-09-13T22:33:27Z — stage 1 (fuzz + TS + S1 ASan + S2 UBSan) began.
- 2026-09-14T00:29:41Z — stage 1 finalized (campaign-ledger.json written);
  the Miri stage STARTED at this instant (miri-ledger.json startedAt is
  the stage-2 start, not a completion time — an earlier revision of this
  note misread it as "stage 1+2 finalized").
- ~2026-09-14T00:59:18Z — Miri finalized (miri-ledger.json last write,
  after the final crate q-pack).
- 2026-09-14T00:59:41Z — the orchestrator, gating on stage 1+2 exit
  status, launched the 72 h soak bound to this candidate
  (candidate-commit.txt).

Correction of the earlier wrong claim: the soak was NOT cut short. The
detached soak process survived the environment rebuild (the rebuild
replaced toolchains, /tmp, and shell state but not the running process or
the workspace mount), and its window stream is continuous through the
incident — including across a directory rename made under the mistaken
"interrupted" assumption. The soak runs to its full 72 h; its own
contention/incident record lives at ../ga-m6-soak-72h/CONTENTION-INCIDENT.md
(including a ~4-minute toolchain-rebuild contention interval and a
25-second duplicate soak launch that was killed before flushing any
window). The "1.5 h of 72 h, archived separately" sentence in the
original note described what was then believed, not what happened; the
directory it pointed to has been restored to the live soak directory.

This campaign evidence is complete and was verified file-by-file
post-hoc (all 14 logs scan zero-error; both ledgers parse and carry
final verdicts).

The #1318 rule "campaignPassed=true AND every finding dispositioned" is
satisfied: this run produced zero findings. The single finding of the GA
program so far (problem-envelope shadowing on the previous candidate
47ddc3fc) was dispositioned by fix + regression test + clean sustained
rerun in PR #1339; see the archived prior campaign directory.
