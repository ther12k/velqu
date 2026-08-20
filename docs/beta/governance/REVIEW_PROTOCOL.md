---
type: Governance
title: Finite Beta Review Protocol
status: draft
tags:
- review
- gates
- severity

---

# Finite Beta Review Protocol

## Severity

- **P0:** security bypass, memory/data corruption, unbounded externally triggerable work, deadlock/hang, false readiness, wrong handler/policy/schema execution, invalid artifact accepted, or unsupported beta claim.
- **P1:** deterministic correctness, cancellation/resource retention, contract incompatibility, material operational failure, or required evidence gap.
- **P2:** maintainability, optional performance, polish, or post-beta ecosystem work.

## Blocking rule

A finding blocks the current milestone only when it violates a frozen P0/P1 invariant, lacks required evidence, contradicts source, or changes trust boundaries without authority. New optional features go to backlog rather than endless closure revisions.

## Review request

State baseline/ending commit, changed files, tasks claimed PASS, commands/results, raw evidence paths, known failures/waivers, security/performance-sensitive changes, clean tree, artifact names, and checksums.

## Final beta review

The reviewer checks every `BETA-GATE` item through the final review packet. “Beta ready” is not used until the reviewer and owner release authority accept the packet.
