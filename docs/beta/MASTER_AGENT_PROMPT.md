---
type: Agent Prompt
title: Velqu Master Beta Agent Prompt
status: draft
tags:
- agent
- beta
- implementation
- velqu

---

# Velqu Master Beta Agent Prompt

## Mission

Continue Velqu from the exact baseline below through the ordered beta-readiness program. Implement every technically executable task in [the task ledger](04_TASK_LEDGER.md), preserve the architecture, produce milestone evidence, and deliver one final beta review packet.

```text
Baseline commit:  e2b379d775a79e619753aaf39eb9ea5f8a763f15
Baseline ZIP:     velqu-m0-m2-20260819T141529Z.zip
Baseline SHA-256: e66bd2da0d7e74ae277a819df6d38c453a119413eaf939755ccabc97efbcce41
Baseline bundle:  velqu-m2.3-r3-e2b379d.bundle
Bundle SHA-256:   a5ba061b422e857e1f8f1411ed5ced90c3148e492a6ed950aa418600e91d3554
Starting status:  M2.3-r3 implementation checkpoint; M23 production gate remains open
Target release:   0.1.0-beta.1
```

## Authority

You are authorized to execute the milestones in this exact order:

```text
G0 → M24 → M25 → M26 → M27 → M28 → M3 → M4A → BETA
```

Parallel work is allowed only where the dependency map permits it. Do not repeatedly ask for confirmation. Mark a task `BLOCKED` only when an owner decision, external credential/infrastructure, safety restriction, or explicit kill criterion genuinely prevents it. Continue independent tasks.

## Sources of truth

Read in this order:

1. repository `AGENTS.md` and accepted ADRs;
2. this prompt and [beta definition](01_BETA_DEFINITION.md);
3. milestone plans and [task ledger](04_TASK_LEDGER.md);
4. current source and dependency locks;
5. executable tests and captured output;
6. raw evidence;
7. generated reports;
8. summary prose.

When prose conflicts with source or raw data, correct the prose and verification tooling. Never weaken source or tests to protect a claim.

## Non-negotiable architecture

1. Production execution remains Rust + pinned QuickJS-NG through `rquickjs`; Bun is development/build/test tooling.
2. Rust routes before JavaScript and current packs are verified numeric artifacts.
3. The compiler never executes the application or service factories to discover structure.
4. Production performs no TypeScript transpilation, route/schema/plugin discovery, OpenAPI generation, or runtime plan compilation.
5. Each QuickJS runtime has exactly one owner thread; no `JSValue` crosses workers.
6. Request data is lazy, bounded, generation-checked, and invalidated at settlement.
7. Every queue, body, decoder, microtask drain, native operation, pool, stream, worker, defer queue, and shutdown path is bounded.
8. Native cancellation physically releases work; terminal metrics classify exactly once.
9. One canonical route/schema graph drives runtime, Treaty, OpenAPI, public contract identity, and semantic diff.
10. Optional capabilities do not enlarge unrelated applications.
11. Same-process application code is trusted. Never claim hostile-code sandboxing.
12. Do not add general Node compatibility, CommonJS, Express/Elysia compatibility, ORM-in-core, automatic cloud provisioning, WebSocket, or SSE.
13. Do not switch engine, introduce Zig, or add a Bun/JSC production target to improve one benchmark. Use evidence and explicit ADRs.

## Task protocol

For every task:

1. mark it `IN_PROGRESS` in the repository ledger;
2. verify dependencies;
3. implement the smallest coherent change;
4. add positive, negative, boundary, race, and regression tests;
5. update source-backed docs and raw evidence;
6. run scoped verification during development;
7. run the milestone gate command;
8. mark `PASS`, `FAIL`, `BLOCKED`, or authorized `WAIVED` with evidence links;
9. commit atomically with the task ID.

Do not count tests manually. Capture actual runner summaries.

## Milestone checkpoint package

Every gate must deliver:

```text
clean source ZIP named with commit
Git bundle or complete patch series
SOURCE-COMMIT record
SHA-256 manifest
milestone report
review index
 evidence index
captured test/typecheck/clippy output
raw benchmark/fuzz/soak data where required
known limitations and P2 backlog
```

The internal release manifest must verify from the release directory. Historical artifacts stay under an explicitly labeled history directory.

## Performance discipline

- Use release builds and pinned candidate versions.
- Randomize candidate order and keep failures.
- Report p50/p95/p99, CPU, RSS, errors, queue/pool wait, versions, environment, and artifact hashes.
- Separate engine load, local process cold start, container cold start, and platform scale-from-zero.
- No spot check becomes canonical evidence.
- Record workloads where Velqu loses.

## Security discipline

- Fail closed on artifact, policy, router, schema, capability, TLS, SSRF, and readiness errors.
- Redact secrets from logs, diagnostics, reports, and crash output.
- A digest provides integrity; a signature provides authenticity.
- Public beta is blocked by known exploitable critical/high issues.

## Beta wording

Allowed after `BETA-GATE`:

> Velqu 0.1 beta is available for evaluation and non-critical services on the documented platforms.

Not allowed:

> production ready, GA, universally faster, Node-compatible, or secure for hostile same-process tenant code.

## Final delivery

Use [the final review packet template](governance/FINAL_REVIEW_PACKET_TEMPLATE.md). If any gate or owner decision remains open, say so explicitly and do not claim beta readiness.
