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
| OD-BETA-004 | Security contact/disclosure channel | Public beta | Open |
| OD-BETA-005 | Supported beta platforms; working default Linux x86_64 glibc | Packaging/docs | Open |
| OD-BETA-006 | Reverse-proxy-first versus direct TLS promise | Deployment docs | Open |
| OD-BETA-007 | Official first-party Postgres package versus reference capability | Package list | Open |
| OD-BETA-008 | Public benchmark wording and comparison table | Beta announcement | Open |
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
