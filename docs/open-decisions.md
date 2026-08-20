---
type: Decision Register
title: Open Owner Decisions and Implementation Decisions
status: active
---

# Open decisions

## Owner decisions (cannot be made by the implementation agent)

| ID | Decision | Status | Notes |
|---|---|---|---|
| OD-001 | Final public product name | **DECIDED (2026-08-18)** | Velqu (brand) / VelquJS (descriptive) — ADR-0016 |
| OD-002 | Public package scope/import path | **DECIDED (2026-08-18)** | `@velqu/*`, CLI `velqu`, runtime binary `velqu-runtime` — ADR-0016 |
| OD-003 | Public repository/organization | **DECIDED (2026-08-20)** | `ther12k/velqu` — OD-BETA-001 record in `docs/beta/governance/OPEN_DECISIONS.md` |
| OD-004 | License | **DECIDED (2026-08-20)** | MIT License in `LICENSE`; contribution terms in `CONTRIBUTING.md`; OD-BETA-002 record in `docs/beta/governance/OPEN_DECISIONS.md` |
| OD-005 | First platform support promise | OPEN | only Linux x86_64 tested here |
| OD-006 | Public release date/governance | **DECIDED (2026-08-20)** | Owner controls public beta release; `0.1.0-beta.1`; details in `docs/beta/governance/RELEASE_AUTHORITY.md` and OD-BETA-003 |
| OD-007 | Security contact/disclosure channel | **DECIDED (2026-08-20)** | GitHub Security Advisories at `https://github.com/ther12k/velqu/security/advisories/new`; OD-BETA-004 record in `docs/beta/governance/OPEN_DECISIONS.md` |

## Implementation decisions (narrow safe defaults, ADR where material)

| ID | Decision | Default chosen | Rationale / ADR |
|---|---|---|---|
| ID-001 | HTTP stack | hyper 1.x + hyper-util + tokio | master prompt §9: mature libraries, custom transport out of thesis |
| ID-002 | Engine binding | rquickjs =0.12.2 vendoring quickjs-ng 0.15.1, single worker OS thread | ADR-0006/0008; pinned exactly; bytecode optional and version-matched if enabled |
| ID-003 | Worker model | one dedicated OS thread owning the JS runtime; commands via channels; native ops complete back onto the worker loop | deterministic, no cross-thread JS value leakage, easy generation checks |
| ID-004 | Pack format | single-file JSON `app.qpack` v1 (`docs/specs/pack-format-v1.md`) with embedded bundle + sourcemap + sha256 integrity | simple, inspectable, deterministic hashing |
| ID-005 | Compiler strategy | TypeScript compiler API static analysis + Bun.build bundling; bundle registers handlers at runtime load | no app dry-run; handler table verified at load |
| ID-006 | JSON strategy policy | measured per shape in M1 (A: engine JSON vs B: Rust serde + object conversion); route manifests record the chosen strategy | ADR-0009 explicit fallback visibility |
| ID-007 | Validation strategy | native Rust validator for IR v1 subset; JS fallback must appear in build report | SCHEMA-005 |
| ID-008 | Source maps | Bun.build sourcemap at build; Rust maps QuickJS exception line/col through the map before logging | M1 requirement |
| ID-009 | Liveness (C0) | static JSON responses configured in the pack, served by Rust before JS | RUN-009 |
| ID-010 | Problem URNs | `https://velqu.dev/problems/*` registry frozen in pack-format spec | RFC 9457-compatible; placeholder domain until OD-002 |
| ID-011 | Treaty navigation ergonomics | Route-id segments (`api.hello.get({name}).get()`) instead of Eden-exact single-segment (`api.hello({name})`) | single-segment form is ambiguous when one prefix hosts multiple routes (`/users` POST vs `/users/:id` GET); route-id form is collision-free, typed, compiler-friendly; owner may revisit final ergonomics |

Each material change to these defaults during implementation will add a new
ADR under `docs/okf/decisions/` and update this register.
