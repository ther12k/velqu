---
type: Platform Support Policy
title: Velqu Public Beta Platform Support
status: accepted
date: 2026-08-21
version: 0.1.0-beta.1
tags:
- beta
- platform
- support
---

# Velqu Public Beta Platform Support

## Supported platform

Velqu public beta `0.1.0-beta.1` supports:

- Linux x86_64;
- glibc-based userland;
- the release artifact and runtime combinations validated by the beta release
  packet.

This is the only public platform promise for beta. Support means the platform
is eligible for documented install, build, test, run, and release-artifact
verification under the beta evidence process.

## Not supported by this promise

- Linux ARM64 remains conditional and unpromised for public beta. CI coverage
  alone does not expand the support promise.
- macOS is development-only best effort, not a supported release target.
- Windows is unsupported.
- musl, static-libc, and other libc/OS combinations are unsupported.
- Other architectures and operating systems require a new owner decision before
  they can be advertised as supported.

## Evidence boundary

Local Linux x86_64 glibc verification is the measured support basis. The CI
matrix may exercise additional runners, including ARM64, for portability
signals; those jobs do not establish a public support commitment without
separate owner acceptance and release-artifact evidence.

Platform evidence must distinguish:

- tested source/build/runtime behavior;
- CI coverage;
- packaged artifact availability; and
- supported public release scope.

No claim of universal portability, cloud-cold-start behavior, or production
SLA follows from this policy.

## Operational limits

The beta is non-SLA, trusted-code-only, and not production-ready GA. Deployment
should follow documented reverse-proxy and runtime constraints. Platform
expansion requires an owner decision, reproducible artifact/install evidence,
and updates to this policy and the release packet.
