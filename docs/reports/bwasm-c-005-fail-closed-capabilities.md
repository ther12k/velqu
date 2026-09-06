# BWASM-C-005 — Fail closed for deployment-required and unavailable capabilities

## Overview

Formalizes capability portability as a single registry with five states —
`browser`, `browser-and-native`, `simulated`, `deployment-required`,
`forbidden` — and threads ONE classification through four surfaces: the
compiler's build-time refusal, the recorded states in build artifacts,
the CLI's machine-readable deployment-requirements summary, and the
runtime's pre-side-effect policy gate with a stable problem shape.

## The portability registry (`packages/compiler/src/capability-portability.ts`)

| grant(s) | state | notes |
|---|---|---|
| timer, console, url, text, abort | browser-and-native | native graph + C-001 baseline |
| crypto | browser-and-native | pinned browser subset (SHA-2 + random, shared vectors); full native set |
| kv | browser | browser-local preview data (C-004) — never durable/multi-user |
| postgres | deployment-required | requires the native runtime's linked pool |
| payments, email, webhooks, cron, queues | forbidden | reserved names — no capability exists, never simulated |
| anything unknown | **forbidden** | unknown classifications fail closed |

## Per-surface behavior

1. **Build time (browser-wasm target)**: a route declaring a
   deployment-required or forbidden capability fails the build with a
   source-located `CompileError` BEFORE any artifact is emitted. Unknown
   grant names classify forbidden → also refused. `simulate: true` is an
   explicit simulation profile that records
   `simulatedCapabilities: [...]` in `browser-manifest.json` — it NEVER
   provides a mock implementation; forbidden names fail even under
   `simulate`.
2. **Recorded states**: `build()` writes a `portability` block (state +
   safe remediation per declared grant) into `capability-manifest.json`,
   carried into the browser set.
3. **Runtime**: `CapabilityRegistry` takes a `CapabilityPolicy`
   (`deploymentRequired` / `forbidden` id lists, supplied by the host
   from the same classification). The INSTALL gate refuses refused ids
   before they become callable; the INVOKE gate refuses again before
   kernel authorization and the handle call. Verified: on refusal,
   kernel-authorize calls = 0 and handle calls = 0.
4. **Stable problem shape**: `deploymentRequiredProblem()` → status 501,
   RFC-9457 body `{ problemId: "deployment-required", type, title,
   status, capabilityId, routeId, reason, remediation }` — no secret
   values or provider-private data (remediation text is supplied by the
   caller from the registry's safe strings).
5. **CLI**: `velqu inspect browser` reports `portabilityStates` per
   declared capability (JSON, schema-versioned) and a `portability:`
   summary line.

## Acceptance criteria

- ✅ Secrets, real Postgres, payments, email, webhooks, cron, durable
  queues are never silently mocked: postgres is deployment-required
  (refused on browser builds unless `simulate` records it explicitly),
  and every reserved integration name is forbidden — under any flag.
- ✅ Deployment-required responses contain no secret values (problem
  shape pins only ids/route/reason/safe remediation; negative-regex
  test).
- ✅ Build, inspect, runtime, and Treaty surfaces agree: one registry,
  same classification (consistency report evidence).
- ✅ The app-builder determines deployment requirements WITHOUT
  executing the route: `portabilityReport(routes, { target })` is a pure
  function over extracted declarations; surfaced via inspect JSON.
- ✅ Capability checks happen before handler or adapter side effects
  (install gate → invoke gate → kernel auth → handle call; counts
  pinned at 0/0 on refusal).
- ✅ Unknown classifications fail closed (registry + report + build).

## Test evidence

16 new tests: compiler classification goldens (incl. unknown → forbidden,
secret-free remediation), report semantics per target, build-time
refusal (postgres route → CompileError, no `browser/` dir emitted;
`simulate` records without mocking; forbidden fails even with simulate;
native build records the portability block), runtime install/invoke
gates with call counters at 0, problem-shape RFC-9457 + secret-free
regex + Response round-trip (Treaty decode path). Compiler +
browser-runtime + CLI affected suites: 235/235.

Evidence: `docs/codex-spark-browser-wasm/evidence/capabilities/c005/`
(`01-c005-tests.txt`, `02-portability-registry.json`,
`03-problem-schema-example.json`, `04-consistency-report.txt`).

## Honest notes

- The runtime policy is HOST-SUPPLIED data (derived from the compiler
  registry) — the browser-runtime package stays standalone; drift between
  the registry and a host's policy is a configuration concern surfaced by
  `inspect browser` reporting both sides.
- The simulated state changes nothing about capability availability: no
  mock implementation exists anywhere in the runtime; a simulated build
  simply records that the deployment author accepted the gap.
- Native-side behavior is unchanged: postgres through the native runtime
  works exactly as before (B-004-C wire contract, C-002 async contract).
- Real-browser lanes for the problem shape flow through Treaty's existing
  non-2xx problem decoding (tested at the Response level); Q-002 owns the
  browser-matrix CI.

Standing CI disclosure applies (zero-step verify workflows since ~#714);
local gates are the acceptance basis.
