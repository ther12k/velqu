# BWASM-R-003 — Browser Handler-Bundle Contract

## Result

**PASS** — the deterministic contract connecting compiled application
handlers to the browser runtime is frozen, implemented, and evidenced:
spec (`docs/specs/browser-handler-abi.md`), narrow registration API
with fail-closed validation (`src/handler-bundle.ts`), byte-stable
metadata emission with golden fixture + reproducibility hash, and
diagnostic snapshots for every negative class.

## Contract summary

- **`HANDLER_ABI_VERSION = 1`**, independent of package versions;
  unknown versions fail closed with both versions named and the
  remedy stated.
- **`defineBrowserHandlers(registrations, packExpectation)`** — the
  ONLY registration path (no ambient globals). Validation before any
  execution: ABI match → well-formedness → duplicates → missing
  pack-declared handlers → extra/undeclared handlers → undeclared
  statuses. An undeclared route or status cannot register silently.
- **Invocation**: `HandlerContext` carries kernel-plan data only
  (validated params/query/headers/body/bodyText, route identity,
  deadline); `HandlerResult` is a declared-status response or a typed
  problem. Kernel completion re-validation still applies (R-002) — the
  bundle cannot widen the contract.
- **Emission**: deterministic metadata (sorted handlers/statuses, fixed
  key order, 2-space LF, trailing newline) — byte-stable; source
  locations sanitized to project-relative paths (host prefixes
  stripped).

## Evidence (`docs/codex-spark-browser-wasm/evidence/handler-bundle/`)

- `golden-handlers.meta.json` — the golden bundle fixture.
- `reproducibility.txt` — shuffled-input emissions identical,
  sha256 `249f36b0d2786d9fd4757bc85faf88d7d3623c5bdad1825a2b59aaba16392143`.
- `diagnostic-snapshots.txt` — all five negative classes with their
  actionable messages (unknown ABI 7, duplicate key, missing handler
  naming the gap, undeclared route `rogue.x`, undeclared status 418
  naming the declared set).

## Acceptance disposition

- ✅ Same source → byte-stable metadata (shuffled-input reproducibility
  + golden fixture; 38/38 package tests).
- ✅ Unknown ABI versions fail closed with actionable diagnostics
  (snapshot pinned).
- ✅ Duplicate/missing handler IDs rejected before execution
  (snapshots + tests).
- ✅ Source locations survive sanitized (host prefixes stripped; tests
  cover POSIX + Windows forms).
- ✅ Handlers cannot silently register undeclared routes or statuses
  (registration validates against the pack manifest; kernel completion
  re-validates declared statuses).

## Boundary notes

Compiler-side bundle generation from application source is BWASM-B-001;
Worker execution with `deadlineMs` enforcement is BWASM-R-004. Real
browser lanes remain Q-002. During test-fixture work one emitter
"discrepancy" was investigated and shown to be a test-data bug
(duplicated status input faithfully emitted) — recorded here for
honesty; no emitter change was needed.

Standing CI disclosure applies (zero-step verify workflows since
~#714); local gates are the acceptance basis.
