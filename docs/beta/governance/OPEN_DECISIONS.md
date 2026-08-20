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
| OD-BETA-002 | License and contribution model | Public beta | Open |
| OD-BETA-003 | Beta release authority and version | Public beta | Open |
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
