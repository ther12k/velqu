# Private Alpha Feedback Classification (P0 / P1 / P2)

Formal issue disposition and triage of developer feedback items (FB-001 through FB-005)
collected during the Private Alpha Developer Preview (M4A-010-B).

## Priority Triage Criteria

- **P0 (Alpha/Beta Blocker)**: Blocks core workflows (init, dev loop, build, run), causes
  data loss, leaks secrets, or violates runtime memory/deadline safety invariants.
  Must be resolved before closing M4A milestone.
- **P1 (Public Beta Requirement)**: Significant developer friction or packaging limitation
  acceptable for invited private alpha with documentation, but required for open public beta.
  Tracked into designated Public Beta tasks.
- **P2 (Advisory / Post-Beta Backlog)**: Minor developer ergonomics, documentation enhancement,
  or post-beta feature request. Non-blocking.

---

## Classified Disposition Ledger

| Item ID | Summary | Priority | Disposition & Roadmap Tracking |
|---|---|---|---|
| **FB-001** | `workspace:*` dependency resolution requires monorepo context or symlinks | **P1** | **Tracked to Public Beta packaging**: Tracked in `BETA-010` (platform packaging matrix) and `BETA-016` (clean external install verification with published npm tarballs). Private alpha disclosure in README and scaffold templates provides explicit working workaround. Zero blocking P0 for alpha. |
| **FB-002** | Undeclared HTTP status conversion to 500 contract violation | **P2** | **Working as intended by design (Invariant #9 & #10)**: Handlers returning undeclared statuses violate the single-schema contract driving client typing. Documented prominently in `ROUTES-SCHEMAS.md` and `LIMITS-AND-NON-GOALS.md`. |
| **FB-003** | Bounded `defer` durable queue confusion (process survival) | **P2** | **Documentation reinforced**: `docs/specs/defer-api.md` and `LIMITS-AND-NON-GOALS.md` explicitly state that `defer` is bounded in-memory best-effort work, never durable jobs. Added prominent non-goal warning. |
| **FB-004** | Service profile grammar fail-closed error on bare `--profile service` | **P2** | **Working as intended**: Runtime grammar `serverless` \| `service:N` enforces explicit worker bounds. Clear actionable diagnostic message confirmed effective. |
| **FB-005** | Outbound fetch default-deny on loopback/private addresses | **P2** | **Working as intended (ADR-0033)**: Default-deny SSRF posture protects production deployments. Opt-in loopback trust documented in `FETCH-CAPABILITIES.md` for local testing. |

---

## Triage Summary

- **Total items**: 5
- **P0 (Blocking Alpha Exit)**: **0** (All core workflows functional without workarounds)
- **P1 (Deferred to Public Beta Tasks)**: **1** (FB-001 -> BETA-010/BETA-016)
- **P2 (Clarifications / Backlog / As-Designed)**: **4** (FB-002, FB-003, FB-004, FB-005)

**Conclusion**: Zero open P0 blockers. The private alpha developer preview fulfills all
functional and stability requirements for M4A exit.
