# 24 h soak (ga-m6-soak) — host-contention annotations

Owner direction (2026-09-12 review): this 24 h run is NOT "uncontended".
Record the known heavy-work intervals; exclude those windows from any
throughput/latency baseline claims. RSS/leak analysis is unaffected (the
leak criterion is monotonic growth, robust to transient load).

Soak started 2026-09-11 23:33 local (window N ends at ~23:33+N minutes).
Known contention intervals (approximate, from session history):

| interval (local) | windows (approx) | activity |
|---|---|---|
| 23:39–23:47 | ~6–14 | cargo release build (fuzz toolchain bring-up) |
| 23:48–00:04 | ~15–31 | libFuzzer smoke campaigns (5 targets, 60 s each) |
| 00:05–00:10 | ~32–37 | cold-start gate run (20 samples) |
| 00:16–00:47 | ~43–74 | `./scripts/verify` (full canonical gate incl. cargo test + release builds + bun test; load avg ~30, CLI-test timeouts observed) |

Rule for the final #1319 report: windows in these ranges are valid for
RSS/leak/no-error evidence, INVALID for throughput/latency baselines.
The clean 72 h run (ga-m6-soak-72h) is the uncontended representative
soak: no builds, tests, or fuzz campaigns may run on this host while it
is in progress (chained to start only after the Miri campaign drains).
