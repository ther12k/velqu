# Soak Incident — Environment-Interrupted Run, 2026-09-15 ~02:13Z

**Disposition (owner decision, 2026-09-15):** this run is preserved as an
**environment-interrupted run** — useful partial evidence, NOT a completed
qualification. No completion summary exists and none may be synthesized;
the analyzer keeps runs in this state progress-only by design. The
selected 72-hour soak will be rerun fresh (new run identity, separate
output location); the 72h acceptance clause is NOT waived.

## Classification: environment-interrupted run

- Candidate: `b8fee349` (tested runtime identity, unchanged).
- Observed duration: 25.22 h (756 retained windows, 120 s each).
- No runtime failure in the retained output (stderr empty, no panic).
- No completion summary, no qualification verdict — correctly so.
- Crossing 24 h establishes the observed duration only; it is not a
  completed 24 h qualification.

## What happened

The selected 72-hour soak (candidate `b8fee349`, launched 2026-09-14
~00:59Z) stopped writing windows at **02:13:10Z on 2026-09-15** and never
resumed. Diagnosis at ~04:01Z:

- Last window: seq 755, elapsed 25.22 h (756 windows, 120 s each).
- No `soak-summary.json` was written — the harness only writes its
  completion summary at natural run end, so this run has **no completion
  record** and cannot qualify under `scripts/analyze-soak.py` rules
  (progress-only by design).
- `soak-stderr.log` is empty (0 bytes since launch) — no panic message.
- The host went down and came back: `last -x` records a **shutdown at
  2026-09-15 09:15 WIB (02:15Z)** and the current boot at 10:10 WIB
  (03:10:14Z) — a 55-minute outage. The last soak write (02:13:10Z)
  precedes the recorded shutdown by ~2 minutes.
- The persistent journal covers only the current boot (`journalctl
  --list-boots` shows a single boot), so the prior boot's final logs are
  not retained and **the initiating party is not identifiable from the
  available records**. Classification: environment/provider-level restart,
  initiator unknown — recorded as unknown rather than diagnosed further.
- A recorded shutdown entry means this was not a hard power cut inside
  the guest; the 55-minute outage pattern is consistent with the
  sandbox/VM host being restarted by its provider.
- Host reboot history (`last -x`): boots on Sep 9, Sep 10, Sep 11, Sep 12
  (preceding uptime 2d 17.9h), Sep 15. Multi-day continuous windows are
  possible but reboots every 1–3 days are part of this environment's
  history — a material risk for any future 72 h run on this host.
- This is the fourth environment-level disruption of the GA campaign
  (three earlier sandbox rollbacks — the first confirmed not to touch the
  soak — and this restart, which did).

## What the run achieved before dying (progress-only, no verdict)

From `scripts/analyze-soak.py` over the retained raw JSONL (218 KB, all
756 windows retained):

- 25.22 h continuous (> the 24 h M6-009 minimum), 165,503,340 completed
  and verified requests (window sum) — 16.5x the 10M clause.
- RSS: initial 6040 → final 3436 KiB (peak 6296): **net −2604 KiB**;
  terminal-quarter slope −96.3 KiB/h; max window step +752 KiB.
- Queue slots peaked at the configured 2048 bound; ownership pending
  never exceeded 2 (worker count).

These are progress statistics over an interrupted run — they are NOT a
qualification verdict and do not substitute for a completed 72 h run or
an owner disposition.

## Consequences for M6-009 / #1319

- The "24-hour minimum and selected 72-hour soak pass" acceptance is NOT
  satisfied: the selected 72 h run did not complete and produced no
  completion record. The clause is retained, not waived.

## Owner decision (2026-09-15)

Option 2, as directed by the owner:

1. Preserve this interrupted run as labeled partial evidence (this file
   plus the raw JSONL; last-write time 02:13:10Z and boot time 03:10:14Z
   recorded separately above).
2. Use the freed host for the two existing evidence gaps first:
   the M24 matched A/B (pre-M24 `3bcb6302` vs one explicitly selected
   M24-complete endpoint, identical protocol) and the M6-006
   native-reproducibility evidence (two clean supported build
   environments with pinned inputs, hashes + difference report retained).
3. Requalify the host's measurement conditions, then launch a fresh
   continuous 72 h soak under a new run identity in a separate output
   location. The tested runtime identity stays `b8fee349` unless an
   actual runtime change requires otherwise; the replacement run must
   record the executable digest and build inputs actually used.
4. The retained 25.22 h is never combined with a later run's hours to
   claim one continuous 72 h run. The completed fuzz/sanitizer/Miri
   campaigns are not repeated — their evidence is intact.
5. Scheduling trade-off recorded honestly: the shorter packets become
   available for review sooner; the replacement soak's completion is
   delayed by the time spent on them. No calendar guarantee in either
   direction. The prior Sep 17 01:00Z target no longer applies; the
   replacement target derives from its actual launch time.

## Raw evidence retained

- `soak.jsonl` — 756 windows, complete and contiguous through seq 755
  (analyzer's coverage checks pass over the retained span).
- `soak-stderr.log` — empty.
- `candidate-commit.txt` — b8fee34909c0d6a4b3b9f76aebe9665cf3192c99.
- `CONTENTION-INCIDENT.md` — prior incident (2026-09-14), separate event.
