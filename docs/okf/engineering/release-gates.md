---
type: Engineering Standard
title: Milestone and Release Gates
description: PASS/FAIL rules for M0, M1, M2, future alpha, performance claims, security,
  waivers, and final reporting.
tags:
- release-gates
- quality
- performance
- security
- handoff
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# Gate philosophy

A milestone is complete only when observable acceptance and evidence pass. “Code exists” is not completion.

Status values:

```text
PASS
FAIL
PARTIAL
UNEXECUTED
WAIVED
```

Only PASS and explicitly authorized WAIVED satisfy a required gate.

# M0 gate

Required:

- OKF structure and links pass;
- implementation audit and open decisions exist;
- proof route semantics are frozen;
- raw Rust/Bun/Elysia baseline versions and commands recorded;
- cold-start harness produces raw machine-readable samples;
- baseline correctness fixtures pass;
- route/schema/policy/Treaty type spike compiles;
- 100/500/1,000 route type benchmark exists;
- no performance claim is made.

# M1 gate

Required:

- actual Rust binary serves one QuickJS worker;
- handler references cached;
- native route before JS demonstrated;
- text, small JSON, params, JSON input, async, cancel, throw paths pass;
- opaque request handle expiry/ownership tests pass;
- body/header/queue/heap/stack/time limits pass or are explicitly scoped;
- application pack tamper/version mismatch fails before ready;
- bridge A/B/C strategy report exists;
- process cold start and idle RSS raw data exists;
- source maps are usable;
- go/conditional-go/stop decision recorded.

No compiler/Treaty breadth compensates for failing bridge safety.

# M2 gate

Required:

- compiler discovers static contract without app dry-run;
- duplicate/shadow/dynamic/unsupported import fixtures pass;
- application pack and manifests deterministic;
- minimal schema IR conformance passes;
- policy context and 401 propagation pass;
- status-aware responses/problems pass;
- source and published Treaty parity passes;
- unit-local and runtime-local modes pass and are labeled;
- proof OpenAPI/contract lock/build report generated;
- matched candidate correctness passes;
- cold-start report covers C1–C4 and 25/1,000 routes;
- traceability closed or authorized waivers recorded;
- security/redaction/FFI review report exists;
- one clean verification command passes;
- repository clean;
- archive, SHA-256, and exact stop point delivered.

# Future private alpha gate

Not authorized in initial prompt. Candidate requirements:

- Linux x86_64/aarch64;
- outbound fetch/crypto;
- robust dev mode;
- service profile/concurrency limits;
- fuzz/security completion;
- compatibility docs;
- examples/tutorial;
- public naming/license/repo decisions;
- evidence-reviewed performance statement.

# Performance claim gate

Before any comparative public claim:

- matched feature audit approved;
- release builds and versions pinned;
- raw data retained;
- sufficient samples and failures included;
- p50/p95/p99 reported;
- cold metric includes full process path;
- C3/C4 primary result;
- idle memory and bridge tradeoffs disclosed;
- exact environment and artifacts published;
- wording limited to tested scope;
- owner approves messaging.

# Security gate

- no known memory-safety P0;
- no bytecode/pack trust bypass;
- no secret redaction failure;
- no unbounded externally controlled queue/body;
- cancellation/late completion safe;
- same-process trusted-code limitation prominent;
- dependency vulnerabilities reviewed;
- unexecuted sanitizer/fuzz platforms disclosed.

# Waiver process

A required waiver includes:

```text
gate/requirement
failure
reason
authority
date
risk
compensating control
expiry/review milestone
```

The AI implementation agent may recommend but cannot invent authority.

# Final report format

```text
Status
Authorized scope
Architecture outcome
Requirements completed
Files/modules added
Commands and exact results
Cold-start and bridge evidence
Memory/artifact/type-system evidence
Security findings
Known limitations
Failed/unexecuted/waived checks
Open owner decisions
Exact stop point
Commit
Archive
SHA-256
```

No phrase such as “production ready,” “complete,” or “faster than Elysia” is used unless its exact gate is satisfied.
