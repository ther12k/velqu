soak-run: replacement run r2 — ENVIRONMENT-INTERRUPTED (second reboot of 2026-09-15)
classification: environment-interrupted (host shutdown/reboot; the soak process did not fail)

Timeline (UTC):
- 2026-09-15T07:32Z  r2 launched (q-soak sha256 6497fb16bc9e42ff85fc7726e602638a70461f4e3d74094e5851f6c9fdc6794d,
                     built from exact b8fee349 worktree; --workers 2 --duration-secs 259200 --window-secs 120,
                     chaos disabled)
- 2026-09-15T07:44Z  last window written (seq 5, elapsedSecs 720.5); six windows total, all healthy
- 2026-09-15T07:46Z  host shutdown begins (last -x: "shutdown system down ... 14:46" local = 07:46Z);
                     soak process died with the host — empty stderr, no summary file
- 2026-09-15T08:46Z  host boots again (kernel 7.0.0-31; prior boots ran 7.0.0-28 — the shutdown
                     installed a kernel update; 1-hour outage)

Context: this is the SECOND host power event today (the first, ~02:15Z shutdown / 03:10Z boot, killed
run r1 at 25.22h — see ../ga-m6-soak-72h/REBOOT-INCIDENT-2026-09-15.md and CONTENTION-INCIDENT.md).
r2 was launched 12 minutes of wall-clock before the second shutdown; it recorded 0.2 hours of data.

Disposition (per the owner's standing option-2 decision of 2026-09-15 — "launch a fresh continuous
72-hour soak. Keep the existing acceptance requirement; do not waive it because of this interruption"):
- r2 is preserved here as-is; its 6 windows are valid data for the interval they cover and are NOT
  combined with any other run's hours.
- No completion summary exists or may be synthesized for r2.
- A replacement run r3 (fresh identity, benchmarks/raw/ga-m6-soak-72h-r3/) was launched after this
  incident was recorded, per the same standing decision.
- If host power events continue to recur before r3 completes, the choice between further relaunches,
  a checkpointed soak design (changes acceptance semantics), or a different host is an OWNER decision
  and is surfaced, not made unilaterally.
