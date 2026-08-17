---
type: Architecture Decision
title: 'ADR-0010: Capability-Based Plugins and Services'
description: Defines explicit compiler, native capability, and TypeScript module extension
  boundaries.
tags:
- adr
- plugins
- capabilities
- services
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
---

# ADR-0010: Capability-Based Plugin and Service Model

## Decision state

Proposed baseline.

## Context

Mutable runtime plugin registration creates order-dependent behavior and can link broad authority. Project Q also needs optional native APIs without recreating Node.js.

## Decision

Use three explicit extension types:

- compiler plugins;
- native runtime capabilities;
- ordinary TypeScript framework modules.

Capabilities declare identity, version, ABI, permissions, configuration, and provided operations. Services are application dependencies with lazy/eager lifecycle, not ambient runtime globals.

## Consequences

- route capability/security inventory is possible;
- unused modules can be excluded;
- native plugin development requires Rust and ABI discipline;
- runtime “install anything dynamically” is not supported.

## Rejected alternatives

- mutable `app.use(plugin)` with unrestricted access;
- runtime-loaded arbitrary dynamic libraries;
- a full dependency-injection container;
- database/auth/ORM built into core.

## Validation

Dependency cycles/conflicts fail, lazy services do not affect unrelated cold routes, shutdown is deterministic, and capability imports are visible in manifests.
