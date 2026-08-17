---
type: Architecture Decision
title: 'ADR-0013: Rust-Only Initial Host'
description: Defers Zig until a bounded hotspot and evidence justify a second native
  toolchain.
tags:
- adr
- rust
- zig
- toolchain
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# ADR-0013: Rust-Only Host for Initial Milestones

## Decision state

Proposed baseline.

## Context

The original idea considered Rust or Zig. Using both from the beginning would introduce another FFI, build chain, ownership model, and contributor requirement before any measured hotspot exists.

## Decision

Implement the host, bridge, capabilities, and tooling in Rust for M0–M3. Zig may be considered only for a bounded measured component through a later ADR.

## Consequences

- one native language/toolchain;
- easier debugging and contributor onboarding;
- potentially misses a future Zig advantage;
- optimization decisions remain profile-driven.

## Rejected alternatives

- split HTTP and bridge between Rust and Zig immediately;
- rewrite mature HTTP/TLS/crypto primitives from scratch;
- choose language per module without governance.

## Validation

Rust baseline must meet correctness and performance gates. A Zig proposal must include a benchmark, maintenance case, FFI contract, and fallback.
