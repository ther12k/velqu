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

Incident note (honest record): mid-run, the execution environment was
rebuilt around this workspace. The running chain was NOT visible from the
new environment's process table, and an initial post-reset directory
listing (taken while stage 1 was still running) briefly led to a wrong
"interrupted mid-stage-1" annotation. The chain in fact ran to completion
inside the old container: stage 1+2 finalized at 2026-09-14T00:29Z
(miri-ledger startedAt) and the orchestrator — which gates stage 3 on
stage 1+2 exit status — launched the soak at 00:59Z bound to this
candidate. Only the soak was cut short (1.5 h of 72 h, archived
separately); this campaign evidence is complete and was verified file-by-
file post-hoc (all 14 logs scan zero-error; both ledgers parse and carry
final verdicts).

The #1318 rule "campaignPassed=true AND every finding dispositioned" is
satisfied: this run produced zero findings. The single finding of the GA
program so far (problem-envelope shadowing on the previous candidate
47ddc3fc) was dispositioned by fix + regression test + clean sustained
rerun in PR #1339; see the archived prior campaign directory.
