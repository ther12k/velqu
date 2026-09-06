# BWASM-R-004 — Isolated Worker Execution with Cancellation and Hard Recovery

## Result

**PASS** — `WorkerHost` provides deterministic, bounded handler
execution outside the parent realm: host-enforced deadlines with
kill-and-replace recovery, session-scoped result routing (cross-project
theft structurally impossible), bounded log forwarding, structured
payload errors, and an isolation-honesty statement carried in the
Worker bootstrap itself. 49/49 package tests (13 new), typecheck clean,
browser bundle builds.

## Design (ADR-0037 §3, ADR-0038)

- **Dedicated Worker + validated message protocol**
  (`WORKER_PROTOCOL_VERSION = 1`): `invoke`/`cancel` host→worker;
  `ready`/`result`/`log`/`fatal` worker→host. Every inbound message is
  field-validated (protocol version, session id, invocation id, payload
  shape) — unknown or malformed messages are dropped with a
  stale-message counter increment, never trusted.
- **Lifecycle: pooled single worker, kill-and-replace.** One Worker
  executes one invoke at a time; on deadline overrun, log-volume
  overrun, or a `fatal` report the supervisor terminates it
  (`hardTerminate`), deterministically rejects every pending
  invocation, and the NEXT `execute()` transparently spawns a fresh
  Worker — the host remains usable after any crash (tested).
- **Deadline propagation**: the plan's `deadlineMs` is enforced
  host-side (the Worker cannot extend it); `AbortSignal` before dispatch
  (immediate, no invoke posted) and during execution (cancel message +
  AbortError) are both covered, including the abort/deadline race.
- **Budgets**: ≤64 forwarded log lines per invocation, ≤512 B/line,
  1 MiB result cap (oversized/uncloneable results → structured
  `OVERSIZED_PAYLOAD`/`UNCLONEABLE_PAYLOAD` errors, never silent
  truncation).
- **Redaction**: the Worker bootstrap redacts host-absolute path
  fragments from error stacks before posting.
- **No cross-realm leakage**: the protocol carries plain data only — no
  parent DOM handles, editor tokens, or provider credentials exist on
  any message type; the bootstrap source contains no `document`/`window`
  references (pinned by test).
- **Session scoping**: results carry the runtime's session id; a
  foreign-session message is dropped (cross-project result theft test).
- **Isolation honesty** (acceptance criterion, in the bootstrap source
  AND this report): Worker isolation is **not by itself a proven
  hostile-code sandbox** — same-origin Workers share origin authority;
  trusted-handler conventions plus deployment posture (separate preview
  origin, sandboxed iframe; ADR-0038 §3) are the real boundaries.

## Test evidence (13 tests, `test/worker-host.test.ts`)

| Adversarial case | Result |
|---|---|
| Infinite loop → deadline termination → hard recovery → **next call succeeds on a fresh Worker** | pass |
| Late result after dispose/terminate → dropped (`staleMessagesDropped ≥ 1`) | pass |
| Foreign-session result → dropped (cross-project theft blocked) | pass |
| Pre-aborted signal → immediate AbortError, **zero** invoke messages posted | pass |
| Abort mid-flight → AbortError | pass |
| Log flood (100 lines > 64 cap) → hard terminate; replacement Worker serves next call | pass |
| Oversized result (1 MiB+) → structured `OVERSIZED_PAYLOAD` | pass |
| `fatal` report → hard recovery; replacement Worker serves next call | pass |
| Bootstrap source: no `document`/`window`; carries redaction + honesty statement | pass |

Plus the full R-001/R-002/R-003 suites (49 total, 0 fail), typecheck
green, browser-target bundle builds (15,344 B, R-001 smoke unchanged).

## Notes and boundaries

- **Test-double honesty**: the supervisor is exercised against an
  in-process Worker double implementing the exact wire protocol; real
  browser Worker construction uses the same injected-factory seam.
  Real-browser CSP/isolation smoke belongs to BWASM-Q-002 (registered
  evidence lane); this packet's isolation claims are limited to the
  protocol/supervisor properties tested above.
- Two test-side defects found and fixed during development (a
  shared-worker double leaking listeners across kill-and-replace; a
  test input with a duplicated status) — recorded for honesty; no
  production-code change resulted from either.
- One production narrowing fix made during typecheck: `fatal` messages
  correctly carry no `invocationId` (the initial validation rejected
  them — caught by the crash/restart test before any merge).

Standing CI disclosure applies (zero-step verify workflows since
~#714); local gates are the acceptance basis.
