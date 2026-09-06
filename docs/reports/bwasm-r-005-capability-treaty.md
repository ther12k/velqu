# BWASM-R-005 — Capability Registry and Treaty Integration

## Result

**PASS** — capability injection and typed Treaty calls work through the
same browser runtime boundary, with authorization strictly before side
effects and a single shared seam for both Treaty modes.

## Capability registry (`src/capability-treaty.ts`)

- `CapabilityRegistry` + `capabilityViewForRoute` — a route's capability
  view contains **exactly** the capabilities its compiled declaration
  names; anything else is structurally absent.
- **Authorization order is the contract** (regression-tested):
  1. route declaration check (not-declared → refuse, zero side
     effects, zero kernel calls);
  2. kernel inventory authorization (`authorizeCapability` — the
     deployment-required class);
  3. host-bridge handle lookup (exact version semantics per
     q-capabilities at inventory load).
- Failures are machine-readable: `CapabilityError.rejection` is
  `not-declared` / `not-in-inventory` / `version-mismatch` with the
  capability id and detail.

## Treaty integration — one seam, two modes

- **Request/Response mode**: `treatyFetchFromRuntime(runtime)` — a
  Treaty `fetchImpl` delegating to `BrowserRuntime.fetch` (the standard
  boundary; no listening server).
- **Direct mode**: `treatyDispatchFromRuntime(runtime, routes)` — a
  Treaty `DispatchImpl` that builds a standard `Request` and drives the
  SAME `runtime.fetch` (kernel routing/validation — no semantic
  bypass), returning the Treaty `DispatchOutcome` shape with abort and
  network classes.
- Route IDs, declared-status narrowing, and canonical problem decoding
  are unchanged: both modes traverse the identical kernel path, so
  Treaty's compile-time status typing behaves exactly as native builds.

## Tests (10 new; package 59/59)

- Allow path: authorized + declared → handle executes.
- **Side-effect-before-authorization regression**: a spy handle with a
  not-declared route records ZERO calls (the regression the task
  names).
- Deployment-required: inventory miss → machine-readable
  `not-in-inventory` naming the capability.
- View shape: empty declaration ⇒ empty view; declaration ⇒ exact keys.
- Fetch mode: typed response via `fetchImpl`.
- Direct mode: same route id + status + body via `DispatchImpl`.
- **Direct-vs-fetch differential**: same invocation → identical status
  and body across two independent runtimes.
- Unknown route id → network-class failure naming the id; abort
  propagation covered.

## Evidence

- Native/browser behavior diff: the dispatcher corpus diff (R-002,
  `evidence/dispatcher/native-browser-diff.txt` — NATIVE-BROWSER-DIFF-OK)
  already establishes native/browser parity for routing and problems;
  this packet's Treaty adapters traverse that same kernel path, so the
  parity carries to Treaty calls by construction. A capability
  registry manifest is the pack's `capability_inventory` (verified at
  init, K-002/K-005) plus the installed handle set — no new manifest
  format introduced.
- Treaty consumer fixture: `test/capability-treaty.test.ts` (fetch +
  direct modes against contract-shaped route tables).
- Typecheck green; `./scripts/verify` ALL PASS (see gates below).

## Boundaries

- Capability `call` implementations (timer/crypto/log/restricted-fetch
  host bridges) are BWASM-C-001 scope; this packet freezes the
  authorization path and the shape of the bridge.
- Real-browser lanes remain Q-002.

Standing CI disclosure applies (zero-step verify workflows since
~#714); local gates are the acceptance basis.
