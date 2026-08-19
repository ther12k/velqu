# Velqu Master Production Agent Prompt

## Mission

Continue the Velqu repository from the exact reviewed baseline below through the ordered production-readiness program. Implement every technically executable task in `TASKS.production.json`, preserve the architectural thesis, create milestone evidence, and deliver one final review packet suitable for independent source and artifact review.

```text
Baseline archive: velqu-m0-m2-20260819T093558Z.zip
Baseline SHA-256: 03a06bbdcc7b4f7626dd5b287983c4f3b6d26ff82e4895923284d76af92debb5
Starting state: M2.3-r1; full M2.3 is not yet accepted
```

## Authority

This prompt authorizes the ordered milestones:

```text
BASE
M2.3-r2
M2.4
M2.5
M2.6
M2.7
M2.8
M3
M4
M5
M6
M7
M8
```

Proceed without repeatedly asking for confirmation. Stop or mark `BLOCKED` only when:

- a required owner decision is genuinely needed at its stated gate;
- a safety policy prohibits the task;
- a hard architecture kill criterion is met;
- required credentials/external infrastructure are unavailable and no local deterministic substitute exists.

When blocked, record the reason and continue independent tasks. Never invent an owner decision, test result, benchmark result, security approval, or external deployment result.

## Sources of truth

Read in this order:

1. repository `AGENTS.md`;
2. accepted ADRs, especially ADR-0018 and the new production-roadmap ADR;
3. this prompt;
4. `TASKS.production.json` and `docs/okf/engineering/production-readiness-gates.md`;
5. current source and tests;
6. raw evidence and generated reports.

When prose conflicts with source or raw evidence, correct the prose and verification tooling. Never weaken source/tests to preserve a summary claim.

## Non-negotiable architecture

1. Production is Rust + pinned QuickJS-NG through `rquickjs`; Bun is development/build/test tooling.
2. Rust performs routing before JavaScript.
3. The compiler never executes the application or service factories to discover routes.
4. Production performs no TypeScript transpilation, route/schema/plugin discovery, OpenAPI generation, or runtime plan compilation.
5. Current QPack execution is numeric and fail-before-ready; legacy compatibility is versioned and isolated.
6. Each QuickJS runtime has exactly one owner thread; no `JSValue` crosses workers.
7. Request data is lazy, bounded, generation-checked, and invalidated at settlement.
8. Every native operation, microtask drain, queue, body, pool, stream, worker, and shutdown path is bounded.
9. Expected HTTP failures are typed declared values; unexpected errors are redacted.
10. One canonical route/schema graph drives runtime, Treaty, OpenAPI, contract lock, and semantic diff.
11. Optional capabilities do not enlarge unrelated applications.
12. Same-process application code is trusted; do not claim hostile-code sandboxing.
13. Do not implement general Node compatibility, CommonJS, Express/Elysia compatibility, ORM-in-core, automatic cloud provisioning, WebSocket, or SSE unless a later owner-approved ADR authorizes it.
14. Do not switch engines, add Zig, or add a Bun/JSC production target merely to improve one benchmark. Use the explicit product decision gates.

## Execution protocol

For each task:

1. Mark `IN_PROGRESS` in `TASKS.production.json`.
2. Read its dependencies and frozen acceptance criteria.
3. Make the smallest coherent implementation.
4. Add positive, negative, boundary, and regression tests.
5. Run scoped checks during development.
6. Update source-backed documentation and raw evidence.
7. Run the milestone verification command.
8. Mark `PASS`, `FAIL`, `BLOCKED`, or approved `WAIVED` with links.
9. Commit atomically with the task ID.

At each milestone gate:

- run the complete repository verification;
- run required benchmarks/conformance/security checks;
- create a report and machine-readable evidence manifest;
- ensure source/report/raw evidence agree;
- produce a clean checkpoint commit and package;
- continue only when the local gate passes.

## Git and artifact discipline

- Work from a real Git repository.
- Keep commits atomic and named with task IDs.
- Never rewrite or delete negative historical evidence.
- Keep the tree clean at every milestone gate.
- Final delivery must include a git bundle or complete patch series in addition to a source ZIP.
- Source ZIP directories are commit-named and contain `SOURCE-COMMIT.txt`.
- Produce SHA-256 checksums for every archive/binary/report bundle.

## Testing discipline

Required layers as applicable:

```text
Rust unit/integration/property tests
TypeScript unit and negative type tests
runtime conformance
Treaty mode parity
pack/router/schema/bridge fuzzing
scheduler/cancellation races
source-to-evidence parity tests
platform smoke/conformance
load/chaos/soak tests
clean install/upgrade/rollback tests
```

Do not count test attributes manually. Capture actual runner summaries and bind them to the checkpoint commit.

## Performance discipline

- Use release builds.
- Pin candidate versions and artifact hashes.
- Randomize candidate order.
- Keep raw samples and failures.
- Report p50/p95/p99, CPU, RSS, errors, queue/pool wait, and environment.
- Separate engine load, local process cold start, container cold start, and platform scale-from-zero.
- Do not call a spot check canonical evidence.
- Never change a fixture after seeing results without retaining both versions and explaining the change.
- If Velqu loses a workload, record it honestly.

## Security discipline

- Fail closed on policy, artifact, capability, TLS, readiness, and version errors.
- Use bounded cleanup and physical native-task cancellation.
- Preserve queue-empty-or-quarantined and resource-quiescence invariants.
- Redact secrets before logs/errors/source maps leave the host.
- Run threat/fuzz/sanitizer/dependency/supply-chain gates at their milestones.
- A digest is integrity, not authenticity; signatures are separate.

## Scope and review discipline

The plan is finite. A finding blocks a milestone only when it violates a frozen P0/P1 invariant, lacks required evidence, contradicts source, or changes trust boundaries. New optional features become later backlog items. Do not enter endless closure loops for P2 polish.

## Owner decisions

Prepare options but do not decide:

- public repository/organization;
- license/contribution model;
- supported-platform promise;
- governance/release authority/date;
- security disclosure authority;
- final public benchmark wording;
- direct TLS/HTTP2 requirement versus reverse-proxy first profile;
- whether Postgres is official GA package or reference capability.

Unresolved decisions block only their stated gates.

## Final delivery

Do not stop after code compiles. Deliver:

```text
clean final source archive
git bundle or patch history
release binaries/packages/QPack tools where authorized
SHA256SUMS
SBOM and provenance
signatures when owner keys are available
TASKS.production.json with final states
REVIEW_INDEX.json
all milestone reports and raw evidence
platform/security/soak/benchmark reports
final PRR packet using the provided template
known limitations and P2 backlog
```

The final assertion must be evidence-based. If M8 owner/reviewer approval is absent, use `PRODUCTION-READY GA — pending reviewer and owner approval`, not `approved`.
