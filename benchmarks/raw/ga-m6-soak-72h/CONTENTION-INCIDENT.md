# Soak survival + contention annotation — ga-m6-soak-72h (b8fee349)

Launched 2026-09-14T00:59:41Z by the chain orchestrator AFTER the fuzz/TS/
ASan/UBSan campaign and the Miri stage passed on this candidate
(see ../ga-m6-fuzz/PROVENANCE.md and its ledgers).

Survival note: mid-run, the execution environment was rebuilt around the
workspace (toolchains, /tmp, shell state replaced; detached processes
survived). An incorrect "interrupted" annotation was briefly written and
this directory briefly renamed; the soak process never stopped — its
window stream is continuous (the rename did not change the output inode;
windows 46-47 were written during the rename interval). The wrong
annotation has been replaced by this note.

Contention intervals (precedent: ../ga-m6-soak/CONTENTION.md — exclude
these windows from throughput/latency claims; RSS/leak analysis is
unaffected since the criterion is monotonic growth):

- ~2026-09-14 09:30–09:34 local — release rebuild of q-bench-support on
  the recovered toolchain plus a 25-second duplicate soak launch
  (killed before its first window flush). Approximately windows 45-47
  may show reduced throughput.

Soak runs to 2026-09-17T00:59Z (72 h). Evidence closes #1319 when the
full 72 h record with flat-RSS analysis is committed.
