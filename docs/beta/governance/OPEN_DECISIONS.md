---
type: Decision Log
title: Owner Decisions Required for Beta
status: draft
tags:
- decisions
- owner
- beta

---

# Owner Decisions Required for Beta

The agent may prepare options but may not invent authority.

| ID | Decision | Blocks | Status |
|---|---|---|---|
| OD-BETA-001 | Public repository and organization | Publishing | Accepted (2026-08-20) |
| OD-BETA-002 | License and contribution model | Public beta | Accepted (2026-08-20) |
| OD-BETA-003 | Beta release authority and version | Public beta | Accepted (2026-08-20) |
| OD-BETA-004 | Security contact/disclosure channel | Public beta | Accepted (2026-08-20) |
| OD-BETA-005 | Supported beta platforms; working default Linux x86_64 glibc | Packaging/docs | Accepted (2026-08-21) |
| OD-BETA-006 | Reverse-proxy-first versus direct TLS promise | Deployment docs | Accepted (2026-08-21) |
| OD-BETA-007 | Official first-party Postgres package versus reference capability | Package list | Open |
| OD-BETA-008 | Public benchmark wording and comparison table | Beta announcement | Accepted (2026-08-21) |
| OD-BETA-009 | Support channel and response expectations | Beta docs | Open |

Every accepted decision records date, decider, alternatives, consequences, and superseded text.


## OD-BETA-001 Decision Record

```text
Decision ID: OD-BETA-001
Title: Public repository and organization
Status: accepted
Date: 2026-08-20
Decider: Owner (ther12k)
Required by gate: Publishing
```

### Context

- A public beta requires a canonical, visible repository for source access, issues, and contributions.
- All packet work is already delivered through `https://github.com/ther12k/velqu` (one PR per packet, one issue per packet); this record makes that existing arrangement the accepted decision instead of an implicit default.

### Options considered

- Keep the repository private and publish packages only: rejected — the beta gate requires a public issue workflow and source visibility.
- Delay the decision until GA: rejected — this decision blocks Publishing, which is a beta gate.
- Adopt `ther12k/velqu` as the canonical public repository: accepted.

### Decision

- The canonical public repository is `https://github.com/ther12k/velqu` under the `ther12k` owner namespace; issues are tracked in-repo at `https://github.com/ther12k/velqu/issues`.

### Consequences

- Package metadata, documentation, and the release packet can reference one canonical repository, issue tracker, and homepage.
- Owner decision OD-003 in `docs/open-decisions.md` is closed by this record.
- Moving to a dedicated organization would be a new owner decision superseding this one.

### Security/operations implications

- Public issues are a public channel; vulnerability disclosure must not depend on them — the disclosure channel remains decision OD-BETA-004.
- This record changes no repository contents, credentials, or automation; it only names the already-existing remote.

### Documentation and task updates

- `docs/beta/governance/OPEN_DECISIONS.md` (this record)
- `docs/open-decisions.md` (OD-003 marked decided)
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-017-A-repository-organization.md` (completion record)

## OD-BETA-002 Decision Record

```text
Decision ID: OD-BETA-002
Title: License and contribution model
Status: accepted
Date: 2026-08-20
Decider: Owner (ther12k)
Required by gate: Public beta
```

### Context

- A public beta needs an explicit license and contribution terms before external distribution.
- The repository previously used `UNLICENSED-BEFORE-OWNER-DECISION` as a deliberate publication blocker.

### Options considered

- MIT License: accepted for its permissive reuse terms and simple contribution model.
- Apache-2.0: not selected for this beta decision; it adds patent and NOTICE obligations not required by the chosen scope.
- Keep the repository unlicensed: rejected because public beta distribution and contributions require explicit terms.

### Decision

- Velqu source is released under the MIT License in the repository root `LICENSE` file.
- Contributions are accepted through reviewed GitHub pull requests and are distributed under the repository MIT License, subject to contributor authority and compatible licensing.
- Contribution workflow and scope rules are documented in `CONTRIBUTING.md`.
- Public issues are not a vulnerability disclosure channel; security reporting remains governed by OD-BETA-004.

### Consequences

- Rust workspace crates and repository source have a canonical public license.
- External users may use, modify, and redistribute the software under MIT terms with the required notice and disclaimer.
- Contributions require provenance review and compatible licensing before merge.
- Replacing MIT or changing contribution terms requires a new owner decision superseding OD-BETA-002.

### Security/operations implications

- MIT does not grant trademark rights or imply support or production readiness.
- Maintainers must reject submissions containing secrets, private data, incompatible licensed code, or unreviewed generated artifacts.
- Vulnerability reports must use the private channel defined when OD-BETA-004 is accepted.

### Documentation and task updates

- `LICENSE`
- `CONTRIBUTING.md`
- `docs/beta/governance/OPEN_DECISIONS.md` (this record)
- `docs/open-decisions.md` (OD-004 marked decided)
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-017-B-license-contribution-model.md` (completion record)

## OD-BETA-003 Decision Record

```text
Decision ID: OD-BETA-003
Title: Beta release authority and version
Status: accepted
Date: 2026-08-20
Decider: Owner (ther12k)
Required by gate: Public beta
```

### Context

- Public beta publication requires one accountable release authority and an explicit version label.
- The project must distinguish beta authorization from GA, production, and later release lines.

### Options considered

- Owner as sole authority for `0.1.0-beta.1`: accepted; preserves explicit owner control and beta labeling.
- Maintainers or automation as independent release authority: rejected; implementation agents and automation may prepare artifacts but cannot invent owner authority.
- Authorize `0.1.0`: rejected; stable-looking version would overstate beta readiness.

### Decision

- Owner is the sole authority to approve, publish, withdraw, or yank Velqu public beta artifacts.
- First public beta version authorized by this decision is `0.1.0-beta.1`.
- Release requirements, evidence, rollback authority, and beta limits are documented in `docs/beta/governance/RELEASE_AUTHORITY.md`.

### Consequences

- Maintainers can produce self-verifying release packets but cannot publish or label releases without Owner approval.
- `0.1.0-beta.1` remains explicitly beta, non-SLA, trusted-code-only, and not production-ready GA.
- Later versions, GA, production commitments, and changed authority require new owner decisions.

### Security/operations implications

- Owner may withdraw or yank a release when evidence is incomplete, checksums fail, or security issues require withdrawal.
- Release packets retain source-commit and checksum evidence; withdrawal does not rewrite historical records.
- Beta authority does not imply hostile-code isolation, support commitments, or platform promises.

### Documentation and task updates

- `docs/beta/governance/RELEASE_AUTHORITY.md`
- `docs/beta/governance/OPEN_DECISIONS.md` (this record)
- `docs/open-decisions.md` (OD-006 marked decided)
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-017-C-release-authority.md` (completion record)

## OD-BETA-004 Decision Record

```text
Decision ID: OD-BETA-004
Title: Security contact/disclosure channel
Status: accepted
Date: 2026-08-20
Decider: Owner (ther12k)
Required by gate: Public beta
```

### Context

- A public beta needs a private vulnerability reporting channel that keeps sensitive details out of public issues.
- No security email or advisory workflow was previously documented; inventing a personal address or response SLA would create unsupported operational commitments.

### Options considered

- GitHub Security Advisories for `ther12k/velqu`: accepted; private, repository-scoped, and already supported by the public repository.
- Publish a dedicated security email: rejected for this beta; no owner-provided address exists and publishing one would invent contact authority.
- Accept reports in public issues: rejected; public disclosure can expose users before remediation.

### Decision

- Vulnerability reports must use the private GitHub Security Advisory flow at `https://github.com/ther12k/velqu/security/advisories/new`.
- Public issues, pull requests, and discussions are not vulnerability disclosure channels.
- Reporters should include affected version/commit, impact, prerequisites, safe reproduction details, logs/configuration where safe, and known workarounds.
- Triage and coordinated disclosure expectations are documented in `SECURITY.md`; no response-time or remediation SLA is promised for beta.

### Consequences

- Sensitive vulnerability details have a documented private intake path.
- Maintainers may request additional details, coordinate fixes, publish an advisory, or recommend withdrawal/yank through the release authority process.
- The policy does not claim a 24/7 security team, guaranteed response time, or production support commitment.

### Security/operations implications

- GitHub account access is required to submit an advisory; reporters must avoid putting secrets or private data in public channels.
- The beta remains trusted-code-only, non-SLA, and not production-ready GA.
- Changes to the disclosure channel require a new owner decision superseding OD-BETA-004.

### Documentation and task updates

- `SECURITY.md`
- `CONTRIBUTING.md`
- `docs/beta/governance/OPEN_DECISIONS.md` (this record)
- `docs/open-decisions.md` (security contact marked decided)
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-017-D-security-contact.md` (completion record)

## OD-BETA-005 Decision Record

```text
Decision ID: OD-BETA-005
Title: Supported beta platforms
Status: accepted
Date: 2026-08-21
Decider: Owner (ther12k)
Required by gate: Packaging/docs
```

### Context

- Public beta users need an exact platform promise that separates measured support from CI portability signals and development-only targets.
- Existing beta guidance requires Linux x86_64 glibc and leaves ARM64 conditional; this decision makes that boundary explicit.

### Options considered

- Linux x86_64 glibc only: accepted; matches current measured support and release evidence basis.
- Linux x86_64 plus ARM64 glibc: rejected for this beta; CI coverage alone does not establish packaged artifact and install support.
- Advertise macOS, Windows, or musl/static targets: rejected; those targets lack accepted beta artifact and support evidence.

### Decision

- The only supported public-beta platform is Linux x86_64 with glibc.
- Linux ARM64 glibc is conditional and unpromised; macOS is development-only best effort; Windows, musl/static-libc, and other platforms are unsupported.
- Canonical scope and evidence boundaries are documented in `docs/beta/governance/PLATFORM_SUPPORT.md` and linked from `docs/beta/workstreams/PLATFORM_SUPPORT.md`.

### Consequences

- Release and packaging claims can advertise one tested platform without implying universal portability.
- CI may continue exercising additional runners for portability signals without expanding public support.
- Platform expansion requires owner acceptance, reproducible artifact/install evidence, and policy/release-packet updates.

### Security/operations implications

- Unsupported platform deployments may lack validated runtime, dependency, TLS, or artifact behavior and receive no beta support promise.
- The beta remains non-SLA, trusted-code-only, and not production-ready GA.
- Platform-specific failures must not be represented as universal framework behavior without matched evidence.

### Documentation and task updates

- `docs/beta/governance/PLATFORM_SUPPORT.md`
- `docs/beta/workstreams/PLATFORM_SUPPORT.md`
- `docs/beta/governance/OPEN_DECISIONS.md` (this record)
- `docs/open-decisions.md` (OD-005 marked decided)
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-017-E-supported-beta-platforms.md` (completion record)

## OD-BETA-006 Decision Record

```text
Decision ID: OD-BETA-006
Title: Reverse-proxy-first versus direct TLS promise
Status: accepted
Date: 2026-08-21
Decider: Owner (ther12k)
Required by gate: Deployment docs
```

### Context

- The beta runtime currently serves plain HTTP/1.1 and binds to loopback by default; it does not load certificates or private keys.
- Public deployment needs an explicit boundary for TLS termination and trusted proxy metadata.

### Options considered

- Reverse proxy terminates public TLS and forwards HTTP to loopback runtime: accepted; matches current runtime behavior and limits certificate handling to the edge.
- Add native runtime TLS/HTTPS support for beta: rejected; not implemented or tested by this task.
- Recommend a proxy while leaving direct TLS ambiguous: rejected; ambiguity would allow unsupported public deployment claims.

### Decision

- Public beta uses a reverse-proxy-first posture. Trusted reverse proxy terminates public TLS and forwards HTTP to the Velqu runtime on a private listener.
- Direct runtime TLS/HTTPS, HTTP/2 termination, certificate loading/rotation inside runtime, and public exposure of plain HTTP are not supported beta promises.
- Canonical deployment boundary is documented in `docs/beta/governance/REVERSE_PROXY_POLICY.md`.

### Consequences

- Operators must manage public certificates, edge limits, forwarding policy, health checks, and graceful drain at the proxy boundary.
- Runtime remains plain HTTP/1.1 on loopback by default; this decision adds no runtime TLS or forwarded-header implementation.
- Native TLS or direct-TLS support requires a new owner decision and separate implementation/evidence.

### Security/operations implications

- Forwarded host, scheme, and client metadata must be trusted only from the configured proxy; direct client spoofing must not reach a trusted runtime boundary.
- Public direct access to plain HTTP is insecure and unsupported.
- Beta remains non-SLA, trusted-code-only, and not production-ready GA.

### Documentation and task updates

- `docs/beta/governance/REVERSE_PROXY_POLICY.md`
- `docs/beta/workstreams/PLATFORM_SUPPORT.md`
- `docs/beta/governance/OPEN_DECISIONS.md` (this record)
- `docs/open-decisions.md` (OD-008 marked decided)
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-017-F-reverse-proxy-first-statement.md` (completion record)

## OD-BETA-008 Decision Record

```text
Decision ID: OD-BETA-008
Title: Public benchmark wording and comparison table
Status: accepted
Date: 2026-08-21
Decider: Owner (ther12k)
Required by gate: Beta announcement
```

### Context

- Beta publication needs benchmark wording that stays within measured evidence and the beta checklist's prohibition on universal or cloud-cold-start claims.
- Current gate evidence is Velqu-only (cold, warm, route-count, startup profile); cross-framework comparisons exist only as historical M0–M2 runs.

### Options considered

- Publish only current scoped Velqu evidence; label all comparator tables historical: accepted; matches retained evidence and avoids overclaiming.
- Publish matched cross-framework comparisons as current: rejected; no fresh repeated comparator run exists under the beta evidence standard.
- Avoid publishing any numbers: rejected; scoped, traceable numbers are useful and permitted by the evidence standard.

### Decision

- Public benchmark wording follows `docs/beta/governance/BENCHMARK_WORDING.md`: claims must cite the run, raw-evidence path, host class, release builds, loopback HTTP/1.1, frozen fixtures, repetitions, and percentiles.
- Historical comparator figures may be shown only when explicitly labeled historical and not current gate evidence.
- Universal, cloud/scale-to-zero, production, SLA, and unscoped multiplier claims are prohibited; losses and failed budgets remain honestly reported.

### Consequences

- Announcements can cite current Velqu-only percentiles and protocol facts without implying cross-framework superiority.
- Any future public comparison table requires a fresh matched repeated run under the benchmark standard before being labeled current.
- Existing narrative reports now scope or label historical claims instead of presenting unscoped multipliers.

### Security/operations implications

- Benchmark wording makes no reliability, support, or production-readiness commitment.
- Claims remain bound to retained raw evidence; selective reporting is prohibited.
- Beta remains non-SLA, trusted-code-only, and not production-ready GA.

### Documentation and task updates

- `docs/beta/governance/BENCHMARK_WORDING.md`
- `docs/reports/final-report.md` (claims scoped/labeled historical)
- `docs/reports/release-gate-report.md` (claims scoped/labeled historical)
- `docs/beta/governance/OPEN_DECISIONS.md` (this record)
- `docs/open-decisions.md` (OD-009 marked decided)
- `docs/codex-spark-beta/tasks/08_public_beta/BETA-017-G-public-benchmark-wording.md` (completion record)
