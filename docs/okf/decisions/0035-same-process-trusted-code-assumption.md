---
type: Architecture Decision Record
title: ADR-0035 Same-Process Trusted Code Assumption
status: accepted
date: 2026-08-28
implements: AGENTS.md constraint 14, ADR-0003 (no Node compatibility), ADR-0014 (version-pinned bytecode), ADR-0033/0034 (fetch security policy and trust)
---

# ADR-0035: Same-Process Trusted Code Assumption

## Context

Velqu embeds one QuickJS-family engine inside the Rust host process.
Embedding a JS engine is sometimes mistaken for sandboxing untrusted
code. It is not, and M2.8 makes the question load-bearing: a native
`fetch` capability gives application code real network reach, and
someone will eventually ask "can I use this to run other people's
code?". The answer must be frozen in writing before any M28 client
ships: **no**.

This ADR restates and elevates the long-standing working rule
(AGENTS.md constraint 14) into a first-class architectural decision
with a threat model, so no doc, README, or marketing sentence can
drift into calling the engine a sandbox.

## Decision

### 1. The engine executes trusted application code only

The single QuickJS worker runs **pack-compiled, owner-built
application code** and nothing else. Trust derives from the build
pipeline: the compiler produces the pack from owner-reviewed sources
(ADR-0004: no app dry-run, static discovery), bytecode is
version-pinned with no arbitrary-bytecode path (ADR-0014), and pack
integrity hooks exist at load (ADR-0026: integrity, with authenticity
as out-of-band deployment policy). Whoever can deploy a pack already
owns the process.

### 2. The process interior is inside the trust boundary

The security model protects:

- the **host network** from the application — outbound fetch is
  policy-governed (ADR-0033: schemes, SSRF classes, TLS, redirects,
  timeouts, bodies);
- the **client** from information leaks — unexpected errors are
  redacted, problems are RFC 9457 (ADR-0011).

It explicitly does **not** protect the process from the application:
memory, file descriptors, CPU, and any secrets already readable by
the process are inside the boundary. Bounded queues, bodies, and
deadlines (AGENTS.md constraint 11) keep the host healthy against
*accidents* and *load*, not against a malicious bundle — a trusted
author does not need to be defended against, and an untrusted author
must never be given the process.

### 3. Fetch changes nothing about process trust

The M28 fetch capability is a *network* control, not an isolation
boundary. It limits where trusted code may connect; it does not
constrain what trusted code could otherwise reach in-process. Docs
must never present fetch policy as evidence that untrusted bundles
are safe to run.

### 4. Unsupported deployment modes (explicit)

- Running user-uploaded or third-party-registry packs in-process:
  **unsupported** in beta and GA-track until a separate, explicit
  isolation ADR exists (process-per-tenant, WASM, or similar).
- Multi-tenant execution inside one worker: **unsupported** (also
  ADR-0008: one runtime per worker, M1/M2 scope).
- Treating the QuickJS engine, capability system, or fetch policy as
  a hostile-code sandbox: **never** — this is the naming rule below.

### 5. Naming rule

No repository documentation, comment, README, or release note may
describe the engine, worker, capability system, or fetch policy as a
"sandbox" for untrusted code. The canonical statement is pinned as
`q_capabilities::TRUSTED_CODE_ASSUMPTION` and tested so the wording
keeps its three load-bearing properties (trusted code, not a sandbox,
network is the adversary).

## Threat model (what is and is not mitigated)

| Scenario | Status |
| --- | --- |
| Application dials internal services | Mitigated — ADR-0033 SSRF policy |
| Application exfiltrates secrets already in the process | **Not mitigated** — inside trust boundary; secret management is a deployment concern |
| Application exhausts memory/CPU | Partially — bounded queues/bodies/deadlines cover accident-class load; adversarial resource use is an operator concern |
| Hostile bundle uploaded by attacker and served by operator | **Out of scope** — pack deployment is owner-only; authenticity is out-of-band policy (ADR-0026) |
| Arbitrary/hostile bytecode injected into a pack | Mitigated at load — version-pinned bytecode, no arbitrary path (ADR-0014), fail-closed verify |
| Multi-tenant code isolation | Out of scope — would require a future isolation ADR |

## Consequences

- `docs/`, READMEs, and future marketing copy must follow the naming
  rule; reviewers reject "sandbox" descriptions of the worker.
- Any future untrusted-code product surface requires a new ADR that
  introduces real isolation (not wording).
- The M28-001-V verify packet maps this assumption to the pinned
  constant test; the M28-GATE review re-checks it.

## Status

Accepted (M28-001-D). Pinned wording and tests in
`crates/q-capabilities/src/fetch_policy.rs`
(`TRUSTED_CODE_ASSUMPTION`).
