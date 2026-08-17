---
type: Reference
title: Open Knowledge Format v0.2 Bundle Conventions
description: OKF v0.2 directory, frontmatter, reserved file, link, provenance, trust,
  validation, and update conventions used here.
tags:
- okf
- format
- google-cloud
- markdown
- provenance
status: stable
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: okf-spec
  resource: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md
  title: Open Knowledge Format v0.2 Specification
---

# Format target

This bundle targets Open Knowledge Format v0.2 as described by the Google Cloud Platform knowledge-catalog specification.

OKF represents knowledge as a directory hierarchy of Markdown files. Concept documents use YAML frontmatter. Reserved files such as directory `index.md` and `log.md` have special handling.

# Bundle conventions used here

## Root `index.md`

The root index contains only:

```yaml
---
okf_version: "0.2"
---
```

followed by navigation content.

## Concept documents

Every non-reserved Markdown concept includes at least:

```yaml
---
type: ...
title: ...
description: ...
tags: []
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: ...
sources: []
---
```

Only `type` is treated as structurally essential by this bundle's validator; the additional fields improve navigation and provenance.

## Reserved files

Subdirectory `index.md` and all `log.md` files do not use concept frontmatter. They remain navigation/history resources.

## Links

Internal links are relative Markdown links and must resolve within the bundle.

## Sources

Source records include:

```yaml
- id: stable-id
  resource: URL-or-bundle-path
  title: Human title
```

External sources do not imply verification of all proposed architecture statements.

# Lifecycle and trust

Project documents are `draft` unless explicitly changed through review and evidence.

Structural validation means:

- frontmatter parses;
- required fields exist;
- reserved-file rules are followed;
- internal links resolve;
- hashes/inventory can be generated.

It does not certify the architecture, approve product decisions, or prove performance.

# Extension fields

The bundle uses practical fields such as:

```text
title
description
tags
status
generated
sources
```

Consumers should preserve unknown fields when modifying documents.

# Local validator

The package validator checks:

1. root index version frontmatter;
2. non-reserved concept frontmatter and `type`;
3. reserved files have no frontmatter;
4. internal Markdown links resolve;
5. source resources that are bundle-relative resolve;
6. duplicate source IDs within a document;
7. bundle inventory and hashes.

External URL availability is not part of offline validation.

# Update rules

- preserve the original design-session reference;
- append material events to `log.md`;
- add ADRs for material decisions;
- change `draft` only with authority/evidence;
- link measurements to exact reports and artifacts;
- distinguish target values from observed results;
- regenerate bundle report and manifest after changes.
