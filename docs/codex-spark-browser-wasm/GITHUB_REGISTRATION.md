# GitHub Registration Guide

The packet includes a dry-run-by-default script that uses GitHub CLI. It does not contact GitHub unless `--apply` is present.

## Prerequisites

```bash
gh --version
gh auth status
python3 --version
```

Run from the extracted packet root.

## 1. Validate local packet

```bash
python3 scripts/browser-wasm/validate_packet.py
```

The validator checks:

- unique issue IDs and titles;
- body-file existence;
- dependency references;
- phase/mode/priority values;
- required labels;
- optional issue policy;
- Markdown structure;
- dependency cycles;
- gate dependency coverage;
- manifest/issue count agreement.

## 2. Dry-run all operations

```bash
python3 scripts/browser-wasm/create_github_issues.py \
  --repo ther12k/velqu
```

This prints planned label and issue operations. It does not write.

## 3. Register program and design first

```bash
python3 scripts/browser-wasm/create_github_issues.py \
  --repo ther12k/velqu \
  --phase 00_program \
  --phase 01_design \
  --apply
```

Recommended first set:

- `BWASM-EPIC`
- `BWASM-D-001`
- `BWASM-D-002`
- `BWASM-D-003`
- `BWASM-D-004`

Resolve those decisions before assigning kernel implementation.

## 4. Register the next phase

Example:

```bash
python3 scripts/browser-wasm/create_github_issues.py \
  --repo ther12k/velqu \
  --phase 02_kernel \
  --apply
```

Repeat in dependency order. Quality/gate issues may be pre-registered for visibility, but their work must not start before dependencies are ready.

## 5. Optional issues

By default the script excludes optional issues:

- `BWASM-C-003` — PGlite local SQL
- `BWASM-X-001` — QuickJS-NG-in-WASM parity spike

Include them explicitly:

```bash
python3 scripts/browser-wasm/create_github_issues.py \
  --repo ther12k/velqu \
  --include-optional \
  --phase 05_capabilities \
  --phase 07_optional_parity \
  --apply
```

Registering an optional issue does not automatically make it a beta blocker.

## 6. Duplicate protection

The script compares exact issue titles against open and closed issues. Existing exact titles are skipped. It does not attempt fuzzy matching; inspect the dry-run if similar Browser-WASM issues already exist.

## 7. Labels

The script creates only the labels listed under `create` in:

```text
docs/codex-spark-browser-wasm/manifests/labels.json
```

It expects the repository's existing mode/priority/task labels to already exist. It aborts rather than silently creating guessed replacements.

## 8. Issue ordering and dependency IDs

Issue bodies use stable packet IDs such as `BWASM-K-003`. They do not depend on GitHub issue numbers, so they remain meaningful before registration. After creation, maintainers may add GitHub links to dependencies or the epic checklist without changing the stable IDs.

## 9. Safe test repository

To test registration mechanics without touching Velqu:

```bash
python3 scripts/browser-wasm/create_github_issues.py \
  --repo OWNER/TEST-REPO \
  --phase 00_program
```

Use dry-run first. The script never creates a repository.

## 10. Manual fallback

Every body is a standalone Markdown file. A maintainer can manually create an issue using:

- title from `manifests/issues.json`;
- body from `body_file`;
- labels from `labels`;
- no GitHub milestone object unless the owner explicitly chooses one.

## 11. After registration

- add the created issue links to the epic;
- assign decision owners;
- mark accepted owner decisions in `OWNER_DECISIONS.md` or repository ADRs;
- update the research baseline if `master` has moved materially;
- do not mass-assign all implementation issues;
- preserve implement → verify/evidence → gate separation.
