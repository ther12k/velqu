# BETA-009-Z — Package Evidence for Beta Security and Reliability Baseline

## Parent closure

BETA-009 is **PASS for the reviewed beta scope**. Implementation packets A–E
and verification packet V are complete through PR #1158. This packet packages
source paths, raw evidence, command results, and the remaining explicit
limitations. It adds no runtime behavior.

## Evidence index

| Area | Canonical evidence |
| --- | --- |
| Pack/router/schema/bridge/HTTP fuzz | `docs/reports/beta-009-a-fuzz-suites.md`, `docs/reports/security-review.md`, minimized corpus tests |
| Dependency/license inventory | `docs/reports/beta-009-b-dependency-vulnerability-license.md`, `docs/reports/beta-009-b-dependency-scan.json`, `scripts/dependency-scan.sh` |
| Threat model | `docs/reports/beta-009-c-threat-model-review.md`, `docs/beta/governance/RISK_REGISTER.md`, ADR-0034/0035 |
| Worker/upstream/DB chaos | `docs/reports/beta-009-d-chaos-tests.md`, `docs/reports/m3-010-b-chaos.md`, `benchmarks/raw/worker-scaling/soak-summary.json` |
| Critical/high blocker review | `docs/reports/beta-009-e-no-known-critical-high-exploitable-issue.md` |
| Verification mapping | `docs/reports/beta-009-v-verify-security-reliability-baseline.md` |
| Beta boundaries/known limitations | `docs/beta/LIMITS-AND-NON-GOALS.md`, `docs/beta/governance/RELEASE_AUTHORITY.md` |

## Guardrail result

- All beta trust boundaries are documented: artifact integrity, numeric
  dispatch, parsers/limits, bridge lifetime, capabilities/SSRF/TLS, secrets,
  forwarded ingress, drain/shutdown, dependency posture, and trusted-code
  boundary.
- No known exploitable critical/high issue exists in the reviewed evidence set;
  the release remains blocked by any future known critical/high finding.
- Fuzz/chaos findings are triaged; injected failures are classified and
  bounded, with no unexplained crash, panic, orphan invocation, or stale
  handle access.
- Same-process QuickJS is consistently documented as trusted application code,
  never a hostile-code or multi-tenant sandbox.

## Final gate snapshot

- q-pack/q-http/q-schema-runtime fuzz and corpus suites — pass
- q-engine-quickjs/q-capabilities/q-bridge/q-runtime companion suites — pass
- `bun test` — 434 pass / 0 fail; `bun run typecheck` — pass
- `cargo fmt --all --check` and clippy `-D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE), isolated netns
- BETA-008 proxy/container contract smokes — pass
- Benchmark manifest refreshed from the actual release artifact after a stale
  hash was detected; rerun passed without weakening assertions.

## Disclosures

- cargo-audit/cargo-deny/OSV/SBOM network scanners and cargo-fuzz/ASan/TSan
  were unavailable in this environment; BETA-009-B/E record this explicitly.
- Existing soak is deterministic fixture evidence, not a production-scale
  availability/SLA claim.
- Owner license/repository decision remains tracked separately; workspace
  `UNLICENSED-BEFORE-OWNER-DECISION` is intentional and public publication is
  not implied.
- Native runtime TLS, signed forwarded identity, Windows/macOS support, and
  hostile-code isolation remain outside the beta promise.
