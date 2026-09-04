# BETA-007-E — No Dynamic Code Execution

## What was built

Every production runtime context now enforces **no dynamic code
execution** before any application code runs:

- **Typed denial of all runtime routes to dynamic code.** A lockdown
  script (`prelude::NO_DYNAMIC_CODE_LOCKDOWN`) is installed host-side
  in `create_context` — the single context-creation hook used by every
  profile and both deployment modes, before the prelude, the bundle,
  or embedded bytecode evaluates. It replaces:
  - the `eval` global (direct `eval('...')`, indirect `(0, eval)(...)`,
    and `eval.call(...)` all resolve the replaced binding — pinned by
    tests, not assumed),
  - the `Function` global (`new Function(...)`, `Function(...)`),
  - the function/async/generator prototype constructor routes
    (`(function(){}).constructor(...)`,
    `(async function(){}).constructor(...)`,
    `(function*(){}).constructor(...)`),
  with a typed `TypeError`: `velqu: dynamic code execution is
  disabled (...)`. Locked properties are non-writable and
  non-configurable, so later redefinition fails (test-pinned).
- **Fail-closed installation**: a lockdown failure is a
  context-creation error — startup rejects; the runtime never serves
  with a dynamic route live.
- **Static code is unaffected** (test-pinned per profile): plain
  functions, classes, closures, generators, and JSON all behave
  identically; object instances keep their `constructor` identity —
  only FUNCTION objects route to the deny constructor.

## Honest engine note (investigated, not assumed)

quickjs-ng gates ALL script evaluation — including the host's own
`ctx.eval` used for the prelude and the source-bundle recovery path —
behind the `Eval` intrinsic, so the intrinsic cannot be excluded.
Excluding it was attempted and reverted: with it absent, even the
host's own evaluation fails. With it present, the global-binding
replacement covers the direct eval form as well (empirically pinned by
`dynamic_code_routes_fail_typed_in_every_profile`). The guarantee is
hardening for trusted application code — never a hostile-code sandbox
(AGENTS.md constraint 14).

## Tests (4 new + honest probe helper, in `worker::tests`; 24 lib total)

- `dynamic_code_routes_fail_typed_in_every_profile` — 7 routes × 3
  profiles, each asserting the typed message.
- `lockdown_is_tamper_resistant_by_construction` — locked property
  cannot be redefined.
- `static_code_still_runs_after_lockdown` — functions/classes/
  generators per profile.
- `lockdown_marker_present_and_instances_keep_identity` — marker set;
  `({}).constructor === Object` unaffected.

## Examples / docs

- `docs/beta/LIMITS-AND-NON-GOALS.md`: "No dynamic code execution"
  bullet in Runtime and platform limits — the guarantee, the covered
  routes, the typed error, and the hardening-not-sandbox framing.

## Gates (fresh on this branch)

- `cargo test -p q-engine-quickjs` -> 24 lib (4 new) + 117 worker +
  1 doc-support, 0 failures; `-p velqu-runtime` -> 96 lib + 35
  runtime_conformance + 16 fetch/source-map, 0 failures; `-p q-http`
  14; `-p q-bridge` 11
- fmt / clippy (`-D warnings`) / typecheck -> clean
- `bun test` -> 434 pass / 0 fail (67 files)
- `./scripts/validate-okf` -> PASS
- `./scripts/verify` -> ALL PASS (M0–M2 + M2.2.1 + M2.3 +
  M23R2-GATE-CLOSE verified) — isolated netns; standing port-3000
  note (BETA-002-C). One manifest-refresh iteration after verify's
  release rebuild. No test weakened.

## Disclosures

- Behavior change: applications that legitimately need dynamic code
  execution cannot run on the production runtime. The compiler emits
  statically compiled bundles, so the supported workflow is unaffected
  (all conformance and proof suites pass unchanged).
- Standing: CI `verify` workflows stall with zero executed steps on PR
  creation (infrastructure-side, tracked since ~#714); local verify is
  the real gate evidence.
