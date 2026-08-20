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

Canonical public-beta support scope lives in [`../governance/PLATFORM_SUPPORT.md`](../governance/PLATFORM_SUPPORT.md).

## Working beta promise

- Linux x86_64 glibc is the only supported public-beta platform.
- Linux ARM64 glibc remains conditional and unpromised; CI coverage alone does not expand support.
- macOS is development-only best effort.
- Windows, musl/static-libc, and other platforms are unsupported unless separately accepted by the owner.

## Required platform tests

- Clean install of CLI/packages/runtime.
- Build source and bytecode packs.
- Run unit/runtime/remote Treaty tests.
- Run proof service.
- Readiness, drain, cancellation, fetch TLS, Postgres, and multi-worker smoke.
- Verify checksums and artifact fingerprint rejection.

## Container profile

Reverse-proxy-first, unprivileged user, read-only application pack where practical, explicit writable temp directory, signal-aware shutdown, and documented memory/CPU limits.
