---
type: Architecture Decision Record
title: ADR-0029 Capability Identity, Versioning, and Requirements
status: accepted
date: 2026-08-26
implements: ADR-0028 (capability ABI and lifecycle), ADR-0010 (capability principle)
---

# ADR-0029: Capability Identity, Versioning, and Requirements

## Context

ADR-0028 froze the capability lifecycle and stated that identity
exists at link time and that version conflicts fail before ready.
Before the compiler resolver (M27-002) can prune modules or hash an
inventory, and before the SDK (M27-009) can promise ABI stability,
the runtime needs the exact grammar of a capability id, the meaning
of a version, and the shape of a requirement. This ADR freezes those
three definitions and their fail-closed resolution.

## Decision

### 1. Capability ids are validated `namespace:name` strings

- Grammar: `namespace ':' name` where the namespace vocabulary is
  **closed** — `runtime` is the only member today. Adding a namespace
  is an ADR-level decision, never a string tweak.
- Name charset: `[a-z0-9-]`, non-empty, ≤ 48 bytes; total id ≤ 64
  bytes. No uppercase, no underscores, no extra separators.
- Parsing is fail-closed with typed errors (empty, missing
  separator, empty namespace/name, unknown namespace, invalid
  character with offset, over-length). Nothing is repaired or
  normalized — a malformed id never reaches the linked set.

The closed namespace vocabulary is a security property: `node:fs`,
`fs`, or `Runtime:FS` are all typed rejections, so a capability id
cannot impersonate an authority that was never decided.

### 2. Versions are integers compared exactly

- A requirement is satisfied iff the linked descriptor carries the
  **same version**. Newer linked versions do not satisfy older
  requirements and vice versa.
- Rationale: implicit compatibility is how runtimes silently change
  behavior under an application. Until a semver ABI policy is
  defined and tested (M27-009-D), exact match is the only honest
  rule: an upgrade is an explicit version bump in the application's
  requirement.

### 3. Requirements and descriptors

- `CapabilityRequirement { id, version }` — what a pack (or another
  capability) needs.
- `CapabilityDescriptor { requirement, dependencies }` — what the
  runtime has linked, including the module's own requirements.
  Building the dependency graph over descriptors and rejecting
  cycles/missing edges is the compiler resolver's job (M27-002-B);
  this ADR defines only the data shapes.

### 4. Resolution integrates with the lifecycle: fail before ready

`resolve_and_install` resolves a requirement against the linked set
and only then performs the `Declared → Installed` transition
(ADR-0028). A `Missing` or `VersionConflict` result routes the
lifecycle to `Failed` — the capability can never reach `Ready` and
never serve an operation. Both error variants are typed and name the
ids and both versions involved; they are never stringly compared.

The linked-set scan is deterministic: the first descriptor carrying
the id decides. Uniqueness of the linked set itself is pinned
upstream by the compiler inventory hash (M27-002-C).

## Threat review

- **Namespace squatting / authority impersonation**: closed
  vocabulary plus fail-closed parsing means no un-decided authority
  (`node:*`, `fs`) can be named or resolved.
- **Silent upgrade/downgrade**: exact version matching makes both
  directions loud, typed failures before serving.
- **Confusing diagnostics**: every error carries the id (and both
  versions on conflict), so pack authors can act without host
  internals.
- **Parse-based attacks (length/charset)**: ids are bounded
  (≤ 64 bytes) and charset-restricted; over-length and invalid
  offsets are typed rejections, keeping ids safe to log and hash.

## Consequences

- `q-capabilities::identity` is the single source of truth for id
  grammar, version comparison, and requirement resolution; the
  manifest format, compiler resolver, and SDK all consume it.
- M27-002 builds the dependency DAG and cycle rejection on these
  shapes; M27-009-D may relax exact matching only with an explicit,
  tested semver policy that supersedes this ADR's rule 2.
- The author guide gains an "Identity and versions" section written
  against these rules.

## Status

Accepted (M27-001-B). Tests in `crates/q-capabilities/src/identity.rs`:
`ids_parse_and_round_trip`,
`malformed_ids_fail_closed_with_typed_errors`,
`exact_version_match_satisfies_requirement`,
`version_mismatch_conflicts_with_both_versions_named`,
`unlinked_capability_is_missing`,
`resolve_and_install_installs_on_success`,
`resolve_and_install_conflict_fails_lifecycle_before_ready`,
`resolve_and_install_missing_fails_lifecycle_before_ready`,
`descriptors_carry_validated_dependencies`.
