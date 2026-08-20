# AGENTS.md — Velqu (Project Q) implementation constraints

Any agent (human or AI) modifying this repository must honor the following.
Primary sources: `docs/okf/MASTER_AGENT_PROMPT.md`, `docs/okf/delivery/prd.md`,
`docs/okf/engineering/release-gates.md`.

## Non-negotiable architectural constraints

1. Production execution is Rust + a QuickJS-family engine (quickjs-ng 0.15.1 via
   rquickjs =0.12.2, pinned). Bun is dev/package/test tooling only.
2. Rust routes by method/path before JavaScript handler execution.
3. Exactly one QuickJS worker for M1/M2.
4. The compiler never dry-runs the application: no service factories, no side
   effects during route discovery.
5. Production startup performs zero route/schema/OpenAPI/plugin compilation and
   zero TypeScript transpilation.
6. Handler references resolve once after application load and stay cached.
7. Request data crossing into JS is lazy; unread fields are never materialized.
8. Native handles are opaque, generation-checked, and expire at settlement.
9. Expected HTTP failures are typed values with declared statuses; problems are
   RFC 9457-compatible; unexpected errors are redacted before leaving the host.
10. One schema contract drives types, runtime, Treaty, OpenAPI, and the lock.
11. All queues, bodies, jobs, heap, stack, and deadlines are bounded.
12. No performance claim without matched, reproducible evidence (p50/p95/p99,
    raw samples retained).
13. No public name/license/repository/marketing decisions — those are owner
    decisions tracked in `docs/open-decisions.md`. Product naming is decided: Velqu/VelquJS, `@velqu/*`, `velqu` CLI (ADR-0016).
14. Same-process QuickJS runs trusted application code only; never describe it
    as a hostile-code sandbox.
15. Post-M2 development follows the authorized sequence in ADR-0018 (M2.2.1 scheduler correctness,
    M2.3 numeric RoutePlan, M2.4 zero-copy ingress/worker slab, M2.5 schema JSON codecs,
    M2.6 binary QPack v2, M2.7 capabilities/WinterTC, M2.8 fetch, M3 multi-worker, M4 alpha).
    The forward finish line is **0.1.0-beta.1** under ADR-0020 (`docs/beta/`); the GA track
    (ADR-0019, `docs/production/`) follows post-beta. Do not build non-authorized out-of-order
    features (WebSockets, SSE, general Node compat).

## Verification

One command verifies the authorized scope:

```bash
bun run verify        # == scripts/verify
```

It must pass (or explicitly report scoped failures) before any milestone
checkpoint commit. Release Rust builds use `--release` only. Benchmark evidence
is regenerated with `bun run benchmark:all`.

## Working rules

- Keep commits atomic; clean tree at milestone checkpoints.
- Every completed P0 requirement gets code/test/evidence links in
  `docs/m0-m2-traceability.md`.
- Material design changes require an ADR under `docs/okf/decisions/`.
- Normative targets vs measured results must never be conflated in docs.
- Failures are reported honestly; never weaken a test or fixture to pass.
