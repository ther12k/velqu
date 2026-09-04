# BETA-009-C — Threat-Model Review

## Review scope

The public-beta boundary was reviewed against the parent BETA-009 guardrails,
existing conformance/fuzz evidence, dependency inventory, and known-limitations
record. This packet adds no runtime behavior.

## Threat model and disposition

| Threat boundary | Evidence / control | Disposition |
| --- | --- | --- |
| Malformed or tampered QPack | `q-pack` mutation/random corpus; integrity, version, engine and ABI checks before ready | PASS; fail closed |
| Route/schema graph confusion | Mandatory semantic manifest, dense numeric IDs, router/schema tests and fuzz corpus | PASS; fail closed |
| Header/query/body parser abuse | q-http bounded admission, percent-decoder corpus, schema validator corpus | PASS; bounded |
| Stale/foreign request handles | q-bridge generation/worker checks, stale-handle corpus, bounded slab | PASS; denied |
| Secret/error disclosure | `security-review.md`, configuration `SecretString`, fixed ready/completion allowlists | PASS; redacted |
| Forwarded identity spoofing | ADR-0034 and BETA-008-B closed distrust list; TCP peer only | PASS; headers are data |
| SSRF/TLS/proxy trust | ADR-0033/0034, fetch policy tests, no ambient proxy env | PASS; policy-owned |
| Worker poison/cancellation | M3-007 ownership/drain tests and chaos report | PASS; bounded cancellation |
| Dynamic code execution | BETA-007-E typed-deny tests and limits/non-goals | PASS; trusted-code hardening |
| Dependency vulnerability/license | BETA-009-B inventory; advisory scanner unavailable locally | PASS_WITH_DISCLOSURE; follow-up scanner environment item |
| Same-process hostile-code escape | Explicitly documented trusted application-code model | NOT A CLAIM; beta non-goal |
| Public deployment / native TLS | Reverse-proxy-first policy, loopback default, container/runbook evidence | NOT A CLAIM; edge-owned |

## Critical/high triage

No critical/high exploitable issue was found in the available source-backed
review. The one unsafe block is documented and reviewed in the security report;
resource limits are robustness controls, not a sandbox. The unavailable
network-backed advisory scanner is recorded in BETA-009-B rather than called a
clean result. Owner license/repository decisions remain a release gate.

## Companion evidence

- `docs/reports/security-review.md`
- `docs/reports/beta-009-a-fuzz-suites.md`
- `docs/reports/beta-009-b-dependency-vulnerability-license.md`
- `docs/reports/beta-008-z-package-evidence.md`
- `docs/beta/governance/RISK_REGISTER.md`
- `docs/beta/LIMITS-AND-NON-GOALS.md`
- `docs/okf/decisions/0034-reverse-proxy-and-outbound-trust.md`

## Targeted checks

- `cargo test -p q-pack` — pass
- `cargo test -p q-http` — pass
- `cargo test -p q-schema-runtime` — pass
- `bun test` — 434 pass / 0 fail
- `bun run typecheck` — pass

## Known limitations

- cargo-fuzz/ASan/TSan and network-backed advisory databases were not
  available in this environment; beta does not claim those stronger gates.
- QuickJS is same-process trusted code only; no multi-tenant hostile-code
  sandbox claim.
- Native TLS, signed proxy identity forwarding, Windows/macOS support, and
  public-SLA/GA hardening remain outside this beta baseline.
