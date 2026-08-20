---
type: Workstream
title: Beta Packaging, Supply Chain, and Release
status: draft
tags:
- packaging
- release
- supply-chain

---

# Beta Packaging, Supply Chain, and Release

## Beta artifacts

- `@velqu/core`, `@velqu/schema`, `@velqu/contract`, `@velqu/treaty`, `@velqu/compiler`, `@velqu/cli`, `@velqu/testing` prerelease packages.
- Optional Postgres/auth packages where accepted.
- `velqu-runtime` Linux binary.
- QPack compiler/inspect/migration tools.
- Source ZIP and Git bundle.
- Documentation/examples.
- SBOM, checksums, review index, evidence index, known limitations.

## Release directory rule

```text
release/<version>/
  SOURCE-COMMIT.md
  source-<commit>.zip
  velqu-<commit>.bundle
  binaries/
  packages/
  SBOM.md or referenced machine artifact
  REVIEW_INDEX.md
  EVIDENCE_INDEX.md
  SHA256SUMS.md
```

The plan itself stays Markdown-only; implementation evidence may use machine-readable formats as required.

## Beta versioning

- `0.1.0-beta.N`.
- npm dist-tag `next` or owner-approved beta tag.
- Breaking changes require migration notes.
- QPack/runtime mismatch fails clearly.
- Rollback/yank procedure is rehearsed.

## Supply-chain baseline

Checksums and SBOM are mandatory. Publisher signatures and complete reproducible provenance are included when owner keys/infrastructure exist; they become strict later-GA gates.
