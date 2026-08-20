---
type: Workstream
title: Beta Platform Support
status: draft
tags:
- platform
- linux
- support

---

# Beta Platform Support

## Working beta promise

- Linux x86_64 glibc is mandatory.
- Linux ARM64 glibc is included when CI/build infrastructure is available and the owner accepts it.
- Other platforms are best-effort development only unless explicitly added.

## Required platform tests

- Clean install of CLI/packages/runtime.
- Build source and bytecode packs.
- Run unit/runtime/remote Treaty tests.
- Run proof service.
- Readiness, drain, cancellation, fetch TLS, Postgres, and multi-worker smoke.
- Verify checksums and artifact fingerprint rejection.

## Container profile

Reverse-proxy-first, unprivileged user, read-only application pack where practical, explicit writable temp directory, signal-aware shutdown, and documented memory/CPU limits.
