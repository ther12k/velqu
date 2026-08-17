---
type: Roadmap
title: Evidence-Driven Project Q Roadmap
description: Conditional phases from feasibility through alpha, scalability, ecosystem,
  and future isolation.
tags:
- roadmap
- delivery
- milestones
- alpha
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# Roadmap

The roadmap is evidence-driven. Dates are intentionally omitted.

# Phase M0 — Contracts and baseline

Outcome:

- product and architecture bundle;
- fair raw Rust/Bun/Elysia fixtures;
- cold-start harness;
- TypeScript contract/Treaty spike;
- risk and traceability baseline.

Gate: benchmark and type-system methodology is executable.

# Phase M1 — Runtime bridge proof

Outcome:

- Rust HTTP/1.1 host;
- one QuickJS worker;
- cached handler table;
- lazy request handle;
- sync/async/cancel/error paths;
- bridge and cold-start measurements.

Gate: architecture pass, conditional pass, or documented fail.

# Phase M2 — Compiler + Treaty vertical slice

Outcome:

- static route compiler;
- minimal schema IR;
- status-aware policy;
- application pack;
- remote/source/published Treaty;
- OpenAPI and contract lock;
- proof app and matched comparisons.

Gate: independently reviewable first version. Initial implementation agent stops here.

# Phase M3 — Developer alpha foundation

Conditional on M2 approval:

- robust `q dev`;
- incremental compiler;
- source maps;
- form/cookie support;
- outbound fetch and crypto;
- module/plugin SDK draft;
- improved OpenAPI;
- Linux x86_64/aarch64 packaging;
- documentation/tutorial;
- security fuzzing expansion.

Gate: private alpha suitability.

# Phase M4 — Service-mode scalability

- multi-worker scheduler;
- adaptive worker experiment;
- native shared-service model;
- load shedding and fairness;
- OpenTelemetry adapter;
- steady-state benchmark suite;
- worker crash/restart policy.

Gate: service-mode concurrency and memory budgets.

# Phase M5 — Ecosystem and interoperability

Evidence-selected optional packages:

- CORS;
- signed cookies;
- multipart;
- static files;
- rate limiting;
- database service adapters;
- WebSocket/SSE;
- Standard Schema adapters;
- framework migration helpers.

Gate: each package has size/startup/security/conformance evidence and does not enlarge core by default.

# Phase M6 — Strong isolation research

Separate product/research stream:

- process supervisor;
- operating-system sandbox;
- tenant package policy;
- IPC;
- resource accounting;
- durable invocation model.

This is not implied by same-process QuickJS.

# Cross-phase rules

- no phase begins by declaring previous hypotheses proven;
- any material design change receives an ADR;
- benchmark baselines are re-pinned and archived, not overwritten;
- negative evidence remains;
- public compatibility claims include exact tested versions;
- final name/license/repository/governance require owner approval;
- feature count never substitutes for closing performance, safety, and type-system gates.
