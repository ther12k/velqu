soak-run: replacement run r3 — ENVIRONMENT-INTERRUPTED (third host power event; unclean, cause undetermined)
classification: environment-interrupted (host down event WITHOUT a clean shutdown record; the soak process did not fail)

Timeline (UTC):
- 2026-09-15T12:09Z  r3 launched (q-soak sha256 6497fb16bc9e42ff85fc7726e602638a70461f4e3d74094e5851f6c9fdc6794d,
                     built from exact b8fee349 worktree; --workers 2 --duration-secs 259200 --window-secs 120,
                     chaos disabled) — see candidate-commit.txt
- 2026-09-16T14:23:37Z  last window written (seq 786, elapsedSecs 94515.5 = 26.254 h); 787 windows total
                     (seq 0–786), all healthy; stderr empty (0 bytes), no summary file
- ~2026-09-16T14:25:37Z  next 120 s window would have been due; it never landed — the host ceased
                     normal operation within this ~2-minute bound
- 2026-09-16T15:13:21Z  host boots again (uptime -s = 2026-09-16 22:13:21 +0700; wtmp: "reboot
                     system boot ... Wed Sep 16 22:13"); ≈50 min outage

Unclean-shutdown evidence: `last -x shutdown reboot` shows NO `shutdown` record between the
2026-09-15T15:46Z-boot-era entries and the 2026-09-16T22:13(+0700) reboot — unlike the two
2026-09-15 events, which each had explicit `shutdown system down` rows. The kernel did NOT change
across this event (running 7.0.0-31-generic since the 2026-09-15 boot; `uname -r` after this event is
still 7.0.0-31-generic), so this was not an unattended-upgrades kernel reboot; the post-reboot
unattended-upgrades.log entries (2026-09-17 06:39Z, polkit/perl packages) are routine and unrelated.
Cause of the down event (crash / hang / power loss / hard reset) is UNDETERMINED: the host keeps no
persistent journal (`journalctl -b -1`: "no persistent journal was found"), so no prior-boot kernel
log exists.

Progress-only data in this directory (NOT acceptance evidence — the run did not reach 72 h and is
disqualified as the M6-009 soak of record):
- 787 healthy 120 s windows over 26.254 h; ~217.6 M requests delivered (steady ≈2.2 k ops/s)
- process RSS flat: min 4584 / max 6212 KiB across all windows (first 5952, last 4752); no
  monotonic growth observed in the covered interval
- the process died WITH the host: stderr is empty and no in-process failure artifact exists

Disposition (per the owner's standing boundary, restated 2026-09-17 after M6-009 enforcement landed):
- r3 is preserved here as-is; its windows are valid data for the interval they cover and are NOT
  combined with r1/r2 hours. No completion summary exists or may be synthesized.
- NO replacement run (r4) may be launched on this host: three consecutive environment interruptions
  (r1 25.22 h, r2 0.2 h, r3 26.25 h) demonstrate the host is not demonstrably stable for a 72 h
  uninterrupted acceptance run.
- The next soak must run on a demonstrably stable host, same candidate (b8fee349) and identical
  binary (q-soak sha256 6497fb16bc9e42ff85fc7726e602638a70461f4e3d74094e5851f6c9fdc6794d),
  fresh 72 h from launch, acceptance unchanged, checkpoint/resume NOT acceptable. Provisioning that
  host is an OWNER action and has been surfaced.
