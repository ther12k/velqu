---
type: Architecture Decision Record
title: ADR-0032 Capability Semver/ABI Compatibility Policy
status: accepted
date: 2026-08-27
implements: ADR-0029 (capability identity, versioning, and requirements)
---

# ADR-0032: Capability Semver/ABI Compatibility Policy

## Context

ADR-0029 rule 2 requires exact version matching for capability
requirements because "no implicit compatibility" was the only honest
policy before one was defined. ADR-0029 explicitly reserved the right
for M27-009-D to relax exact matching "only with an explicit, tested
semver policy that supersedes this ADR's rule 2". The SDK surface
(`CapabilitySdk`, `CancellableCapability`, lifecycle model) now exists
and is stable enough that a compatibility policy can be written down,
pinned, and tested. Without it, every patch of a first-party capability
would force a rebuild of every dependent pack even for pure fixes.

## Decision

### 1. Versions are packed semver triples with hard ceilings

The pack-format capability version stays a single integer
(`CapabilityVersion(u32)`); the interpretive layer decomposes it as
`major*1_000_000 + minor*1_000 + patch`. Each component is bounded to
0–999 (`MAX_SEMVER_COMPONENT`). Integers that do not decompose under
the ceiling are typed failures (`CompatError::UnpackableVersion`) —
never silently truncated or guessed.

### 2. The major version is the ABI revision

A major bump is ABI breaking by definition: breaking `CapabilitySdk`
or `CancellableCapability` trait shapes, the lifecycle state machine,
or any settled native-operation contract requires a new major. Within
a major, minor bumps owe backward-compatible additive growth and patch
bumps owe compatible fixes; the policy trusts provider discipline and
gives consumers a selector — not automatic acceptance.

### 3. Exact match remains the default

`q-capabilities::identity` resolution is unchanged: exact integer
matching stays the default requirement form in pack manifests.
Relaxation is opt-in through `VersionSelector::CompatibleWith`, which
satisfies a requirement with any provider whose ABI revision equals the
required major and whose (minor, patch) is at least the required pair.
Providers that fail to unpack never satisfy a policy selector.
`VersionSelector::Exact` mirrors identity resolution for uniformity.

### 4. The SDK ABI revision is explicit

`SDK_ABI_REVISION` pins the current host SDK ABI at 1; a test fails on
any unnoticed change so bumps are deliberate, visible events.

## Alternatives considered

- **Keep exact-match-only**: honest but forces pack rebuilds across the
  whole dependency graph for compatible fixes; pushed costs onto every
  consumer with no escape hatch.
- **Loose range requirements everywhere (`^1.x` defaults)**: re-introduces
  implicit compatibility, the exact failure mode ADR-0029 rejected.
- **Unbounded semver components**: whole-integer packing without
  ceilings makes "which major?" ambiguous for large ints; ambiguity in
  ABI identity is unacceptable.

## Consequences

- `q-capabilities::compat` is the single source of truth for the policy;
  identity resolution semantics are untouched.
- Pack manifests continue shipping exact integers today; selector-based
  requirements ride future format packets when adopted end-to-end.
- Tests pin: round-tripping, ceiling failures, ABI classification,
  selector satisfaction direction (newer-in-major yes, older or other
  major no), unpackable-provider rejection, and the pinned ABI revision.

## Status

Accepted (M27-009-D). Tests in `crates/q-capabilities/src/compat.rs`.
