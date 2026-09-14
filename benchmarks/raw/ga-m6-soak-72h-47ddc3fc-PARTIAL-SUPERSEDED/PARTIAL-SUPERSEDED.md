# PARTIAL RECORD — SUPERSEDED CANDIDATE

Candidate: `47ddc3fc8aca2ee92ddd284661be4b943d7a7b8a` (see `candidate-commit.txt`).

Status at stop: **349 windows / 11.64h of the 72h bound** (stopped deliberately, not failed).

Why stopped: while the GA chain was running against this candidate, the
`codec_encoders` fuzz target found a real defect — RFC 9457 problem
extensions could shadow frozen envelope members (input
`{"\xff":0,"status":{}}` parsed back with `status` as `{}` instead of the
declared 422). Fixed at three layers and merged as
`b8fee349` (PR #1339). Per the candidate-binding policy, a moved candidate
invalidates in-flight chain evidence, so this soak was stopped and the full
chain is rerunning against the new merge commit.

Partial-record health at stop (honest, not a claim):

- throughput per 120s window: ~1.9–2.4k ops/s, no monotonic decay observed
- process RSS: first 4808 KiB → last 2980 KiB (min 2944 / max 5044); no
  growth trend across the 349 windows
- 0 windows with error fields; queue rejected counters are load-shed
  counters from the saturated-loop soak design, not failures

This directory is retained as the audit trail of the superseded attempt.
It is NOT evidence of a completed soak and must not be cited as one.
