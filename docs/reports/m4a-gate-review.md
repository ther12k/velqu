# M4A-GATE Review — Developer Preview and Private Alpha

Milestone exit decision for M4A (Developer Preview and Private Alpha).

## Milestone Decision: PASS

All 10 parent tasks (`M4A-001` through `M4A-010`) are complete, verified with source-backed evidence, and squash-merged to master.

### Parent Task Dependency Closure
1. **M4A-001 (Implement actual-runtime `velqu dev` loop)** — PRs #1037–#1042: `ProjectWatcher` (source/contract discovery without code execution), incremental temporary QPack generation, zero-downtime worker replacement before traffic switch, graceful old worker drain, and actionable diagnostic formatting.
2. **M4A-002 (Complete CLI command surface)** — PRs #1043–#1048: unified `velqu` CLI (`dev`, `build`, `inspect`, `contract diff`, `test`, `pack inspect`, `migrate`), deterministic `ExitCode` enum, `--json` structured receipts, actionable errors with source frames and hints.
3. **M4A-003 (Implement project scaffolding)** — PRs #1049–#1054: `velqu init` generator (no demo credentials, clean module/service/contract separation), health and greetings domain modules, unit testing setup, serverless and `service:N` runtime profile choices, optional `--with-fetch`.
4. **M4A-004 (Complete Treaty unit-local, runtime-local, and remote modes)** — PRs #1055–#1060: `unitTreatyDirect` direct in-process dispatcher, `runtimeTreaty` running the actual release `velqu-runtime` binary over HTTP with SIGTERM bounded drain, remote fetch client with network error mapping, exact method/param/body/status/problem typing.
5. **M4A-005 (Publish compact contract and SDK artifacts)** — PRs #1061–#1066: `contract.d.ts`, `openapi.json`, `contract.lock.json`, tree-shakable client with `treatyRoutes`, public contract hash calculation and binding, package verification against manifest.
6. **M4A-006 (Finalize diagnostics, source maps, and inspect output)** — PRs #1067–#1072: closed `DiagnosticCode` catalog, lazy source-map symbolization bound by pack SHA-256, sensitive credential/header redaction in console logs, detailed inspect JSON route plan output.
7. **M4A-007 (Implement bounded defer and lifecycle hooks)** — PRs #1073–#1078: worker-owned bounded defer queue, dedicated `DeferredDrain` execution phase, host-enforced capacity and deadline interrupt, closure-private queue preventing recursive bypass, `EngineStats` defer lifecycle metrics.
8. **M4A-008 (Build documentation and examples)** — PRs #1079–#1087: comprehensive documentation suite (`QUICKSTART.md`, `ROUTES-SCHEMAS.md`, `TREATY.md`, `FETCH-CAPABILITIES.md`, `RUNTIME-PROFILES.md`, `DEPLOYMENT-REVERSE-PROXY.md`, `LIMITS-AND-NON-GOALS.md`) linked into index/README with tested examples and no production-ready claims.
9. **M4A-009 (Build realistic private-alpha proof service)** — PRs #1088–#1094: 24 routes across 8 modules (health, hello, users, items, auth, upstream, ops, async), cursor-based pagination, pure-JS JWT-like bearer policy with RFC 4231 HMAC-SHA-256, native fetch bridge with SSRF/loopback controls, ops readiness/metrics routes, proof Treaty client.
10. **M4A-010 (Run invited developer alpha and close P0/P1 feedback)** — PRs #1095–#1100: clean install verification test (`packages/cli/src/clean-install.test.ts`), feedback collection report (`docs/reports/m4a-010-alpha-feedback.md`), feedback triage ledger (`docs/reports/m4a-010-feedback-classification.md`), published limitations update (`docs/beta/LIMITS-AND-NON-GOALS.md`), 0 open P0 alpha exit blockers.

### Evidence Reports & Reference Specs
- `docs/reports/m4a-010-alpha-feedback.md` — Evaluator cohort feedback across 5 core workflows
- `docs/reports/m4a-010-feedback-classification.md` — P0/P1/P2 issue classification and disposition
- `docs/specs/defer-api.md` — Bounded defer specification and operational limits
- `docs/beta/QUICKSTART.md` — Private alpha developer quickstart
- `docs/beta/ROUTES-SCHEMAS.md` — Route and schema authoring guide
- `docs/beta/TREATY.md` — Treaty client guide and mode boundaries
- `docs/beta/FETCH-CAPABILITIES.md` — Outbound fetch and capability model
- `docs/beta/RUNTIME-PROFILES.md` — Serverless and service profile guide
- `docs/beta/DEPLOYMENT-REVERSE-PROXY.md` — Reverse proxy deployment architecture
- `docs/beta/LIMITS-AND-NON-GOALS.md` — Framework boundaries, limits, and non-goals

### Standards Conformance & Open Items
- **Zero Open Alpha P0s**: The invited developer evaluation confirmed core workflows are functional without author intervention.
- **Tracked P1 Items**: FB-001 (npm package publishing) tracked cleanly into Public Beta packaging milestones (`BETA-010` platform packaging and `BETA-016` clean external install).
- **Open Decisions**:
  - `PACK_FORMAT_CURRENT` v1→v2 default flip remains owner-gated (carried from M26, tracked in REVIEW_INDEX openItems).
  - Numeric 2-worker scaling target remains an owner decision (tracked in REVIEW_INDEX openItems).
- **Standing CI Disclosure**: CI in this repository fails with zero executed steps on PRs (infrastructure-side since ~#714); local verification passes 100% from the clean candidate commit.
