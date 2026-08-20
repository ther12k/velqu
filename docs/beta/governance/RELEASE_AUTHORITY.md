---
type: Release Authority
title: Velqu Public Beta Release Authority
status: accepted
version: 0.1.0-beta.1
date: 2026-08-20
tags:
- release
- authority
- beta
---

# Velqu Public Beta Release Authority

## Authority

The Owner is the sole release authority for Velqu public beta artifacts. The
Owner may approve, publish, withdraw, or yank a beta release after reviewing
its release packet and required evidence.

## Authorized beta version

This decision authorizes the first public beta version:

```text
0.1.0-beta.1
```

This authorization does not authorize `0.1.0`, a GA release, an SLA promise, or
any later version. Each material release line or change in release authority
requires a new owner decision.

## Required release evidence

Before publication, the release packet must identify one source commit and
include, at minimum:

- source ZIP and Git bundle;
- source commit record;
- SHA-256 manifest and checksum verification;
- review and evidence indexes bound to that commit;
- captured test, typecheck, formatting, lint, and package results applicable to
  changed scope;
- benchmark, fuzz, soak, security, SBOM, and known-limitation evidence required
  by the beta gate.

The release packet must be self-verifying. Normative requirements and measured
results must remain separate.

## Beta limits

- Release label is **public beta**.
- Release is **non-SLA** and **not production-ready GA**.
- Runtime executes trusted application code only; beta release authority does
  not claim hostile-code isolation.
- No full Node/Bun compatibility, WebSocket/SSE, ORM-in-core, or unsupported
  platform promise is implied.

## Rollback and withdrawal

The Owner may stop publication, withdraw a release, or request package yank
when release evidence is incomplete, checksums do not match, a security issue
requires withdrawal, or a release violates stated beta limits. Withdrawal does
not rewrite historical evidence; it records the affected version and reason.

## Governance boundary

Maintainers may prepare artifacts and evidence but may not publish or label a
release without Owner approval. This record grants no authority for GA,
production, commercial support, trademark, or platform commitments.
