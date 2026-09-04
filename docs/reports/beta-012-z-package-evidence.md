# BETA-012-Z — Package Evidence: Complete Beta Documentation and Limitations

## Overview

Evidence packaging and parent task closure for **BETA-012** ("Complete beta documentation and limitations").
With this packet, all child microtasks (BETA-012-A through BETA-012-I) and verification (BETA-012-V) are complete and passing. Parent row `BETA-012` is flipped to **PASS** in `docs/beta/04_TASK_LEDGER.md`.

## Child Packet Evidence Inventory

| Task ID | Component | Deliverable / Documentation | Evidence Report | PR |
|---|---|---|---|---|
| BETA-012-A | Installation | Rewrote `INSTALL.md` with shared, standalone, and container modes | `docs/reports/beta-012-a-installation.md` | #1174 |
| BETA-012-B | Quickstart | Rewrote `QUICKSTART.md` with tested CLI scaffold/build/run flow | `docs/reports/beta-012-b-quickstart.md` | #1175 |
| BETA-012-C | Architecture | Added `ARCHITECTURE.md` (3 artifacts, 6-step request flow, QuickJS boundary) | `docs/reports/beta-012-c-architecture.md` | #1176 |
| BETA-012-D | Contracts/Treaty | Updated `TREATY.md` and `ROUTES-SCHEMAS.md` with contract diff & lock workflow | `docs/reports/beta-012-d-contracts-treaty.md` | #1177 |
| BETA-012-E | Capabilities | Updated `FETCH-CAPABILITIES.md`, audited `POSTGRES-CAPABILITY.md`, added `AUTH.md` | `docs/reports/beta-012-e-fetch-postgres-auth.md` | #1178 |
| BETA-012-F | Deployment | Strengthened `DEPLOYMENT-REVERSE-PROXY.md` with verified Nginx rehearsal | `docs/reports/beta-012-f-deployment.md` | #1184 |
| BETA-012-G | Troubleshooting | Added `TROUBLESHOOTING.md` reproducing real fail-closed startup errors | `docs/reports/beta-012-g-troubleshooting.md` | #1186 |
| BETA-012-H | Methodology | Added `PERFORMANCE-METHODOLOGY.md` explaining QuickJS bytecode vs JIT | `docs/reports/beta-012-h-performance-methodology.md` | #1187 |
| BETA-012-I | Limitations | Updated `LIMITS-AND-NON-GOALS.md` to public beta (`0.1.0-beta.1`) | `docs/reports/beta-012-i-limitations-non-goals.md` | #1188 |
| BETA-012-V | Verification | Verified full documentation matrix, links, examples, and guardrails | `docs/reports/beta-012-v-verify-beta-documentation-limitations.md` | #1189 |

## Parent Acceptance Guardrails Verified

- **Every command/sample is tested**: Every sample across the complete documentation suite was verified against real builds.
- **No universal performance claim**: Explicitly documented across methodology and limitations.
- **No production-ready/SLA wording**: Beta framing (`0.1.0-beta.1`), non-SLA, evaluation-only terms enforced.
- **QuickJS bytecode versus JIT explained accurately**: AOT bytecode compilation into `app.qpack` eliminates cold-start parse/transpile overhead, while compute-heavy execution remains interpreted bytecode compared to native JIT machine code.
- **Docs CI / Link check**: `./scripts/validate-okf` checked all 189 internal links with 0 errors.

## Gates

- `cargo test -p velqu-runtime` — pass (8 suites ok)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Evidence packaging and status tracking only; no runtime binary behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
