---
type: Engineering Standard
title: Compatibility and Versioning
description: Versioned framework, Treaty, contract, pack, schema, runtime, capability,
  engine, platform, deprecation, and provenance policies.
tags:
- compatibility
- versioning
- semver
- abi
- contracts
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# Compatibility dimensions

Project Q versions several independent contracts:

```text
framework authoring API
Treaty client API
contract format
application pack format
schema IR
runtime ABI
native capability ABI
engine version/bytecode ABI
CLI/compiler
```

One package version string cannot replace explicit metadata.

# Semantic package versioning

Public npm/Rust packages follow semantic versioning after the first public contract is declared. Before 1.0, breaking changes remain possible but must be documented and tested.

Internal crates/packages can change together during M0–M2.

# Application pack compatibility

Runtime loader checks:

```text
qpack format version
runtime ABI range
schema IR version
capability ABI versions
engine/bytecode exact version when bytecode
contract/application digest
target architecture
```

Unknown mandatory fields or incompatible versions fail closed.

# Engine compatibility

Source mode may tolerate a documented engine range if conformance passes.

Bytecode mode requires exact compatible engine/ABI identity. No portability promise.

# Framework API compatibility

Public route/schema/policy/Treaty APIs require:

- declaration/type tests;
- migration notes for breaking changes;
- deprecation before removal where practical;
- no accidental public internals.

The compiler may normalize old syntax for a bounded period but build reports show deprecation.

# API contract compatibility

Semantic diff uses route and schema semantics rather than raw OpenAPI text.

Default classifications are configurable, but CI policy must be explicit.

Possible statuses:

```text
compatible
warning
breaking
unknown
```

`unknown` fails strict release until reviewed.

# Treaty compatibility

Published contract package declares:

- contract format version;
- API contract hash/version;
- minimum Treaty runtime version;
- generated client runtime version.

Source mode requires compatible framework type packages.

# Capability compatibility

A capability declares:

```text
id
semantic version
native ABI
configuration schema version
permissions model version
```

A runtime can support multiple compatible capability versions only when tested. It does not guess.

# Runtime API compatibility

Project Q does not claim Node/Bun API compatibility. The compatibility manifest lists supported globals/modules/features and conformance status.

Statuses:

```text
supported
partial
experimental
unsupported
deprecated
```

“Partial” links limitations.

# Platform support

A platform is supported only when:

- release build exists;
- runtime conformance passes;
- packaging/integrity passes;
- known limitations are documented;
- CI or release evidence is reproducible.

Development success alone is not a support promise.

# Deprecation

A deprecated framework API:

- emits compiler warning with replacement;
- remains covered by tests during support window;
- appears in changelog;
- is removed only in a permitted breaking release.

API route deprecation appears in contract/OpenAPI/Treaty metadata but does not automatically change runtime behavior.

# Contract lock workflow

```bash
q contract diff --against contract.lock.json
q contract accept
```

`accept` requires explicit developer action and records the new hash. CI should not overwrite the baseline automatically.

# Reproducibility and provenance

Every release records:

- source commit;
- tool versions;
- lock hashes;
- runtime/compiler/engine versions;
- qpack/contract hashes;
- test and benchmark evidence;
- SBOM/license report.

# M0–M2 policy

All public names/versions are provisional. Do not publish a stability promise. Internal artifact versions begin at `1` so incompatible parsing fails clearly rather than relying on package pre-1.0 semantics.
