---
type: Bundle Report
title: Project Q OKF Bundle Report
description: Generated inventory, trust boundary, local structural validation, and
  document manifest.
tags:
- okf
- report
- validation
- manifest
status: draft
generated:
  by: openai/gpt-5.6-pro
  at: '2026-08-17T19:27:58+07:00'
sources:
- id: okf-spec
  resource: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md
  title: Open Knowledge Format v0.2 Specification
---

# Summary

This design and implementation-handoff package targets **Open Knowledge Format v0.2**.

| Metric | Value |
|---|---:|
| Markdown files | 68 |
| Concept documents | 60 |
| Reserved/root index and log files | 8 |
| Directories represented | 7 |
| Internal Markdown links checked | 168 |
| Approximate body words | 35,985 |
| Uncompressed Markdown bytes | 324,871 |

# Trust and lifecycle

- Product, architecture, ADR, PRD, delivery, and engineering concepts remain `draft`.
- Reference-format/source-note documents may be marked `stable` only as preserved references.
- No document claims implementation verification.
- Performance values are targets, comparative gates, or hypotheses—not observed framework results.
- The working name **Project Q** is not a public naming decision.

# Local structural validation

**PASS**

Checks:

1. root `index.md` contains only `okf_version: "0.2"` in frontmatter;
2. every non-reserved Markdown concept has parseable frontmatter and a non-empty `type`;
3. subdirectory `index.md` and `log.md` resources have no frontmatter;
4. internal Markdown links resolve;
5. bundle-relative source resources resolve;
6. source IDs are not duplicated within a concept.

This is local structural validation, not certification or approval by Google Cloud or another party.

## Errors

None.

## Warnings

None.

# Documents by directory

| Directory | Markdown files |
|---|---:|
| `architecture` | 17 |
| `decisions` | 15 |
| `delivery` | 7 |
| `engineering` | 10 |
| `project` | 8 |
| `references` | 6 |
| `root` | 5 |

# Concept types

| Type | Count |
|---|---:|
| AI Agent Implementation Prompt | 1 |
| Architecture Decision | 14 |
| Architecture Review | 1 |
| Architecture Specification | 14 |
| Bundle Report | 1 |
| Delivery Plan | 1 |
| Design Principles | 1 |
| Design Session Record | 1 |
| Engineering Specification | 2 |
| Engineering Standard | 6 |
| Knowledge Bundle Guide | 1 |
| Product Backlog | 1 |
| Product Requirements Document | 1 |
| Product Strategy | 2 |
| Project Charter | 1 |
| Protocol Specification | 1 |
| Reference | 3 |
| Reference Register | 1 |
| Requirements Specification | 1 |
| Reserved | 7 |
| Reserved Root Index | 1 |
| Risk Register | 1 |
| Roadmap | 1 |
| Scope Definition | 1 |
| Security Standard | 1 |
| Traceability Matrix | 1 |
| User Experience Specification | 1 |

# Inventory

| File | Type | Status | Title | Bytes | Body words |
|---|---|---|---|---:|---:|
| [`MASTER_AGENT_PROMPT.md`](MASTER_AGENT_PROMPT.md) | AI Agent Implementation Prompt | draft | Master Implementation Prompt — Project Q M0–M2 | 26,119 | 3,328 |
| [`README.md`](README.md) | Knowledge Bundle Guide | draft | Project Q Framework Design and Product Handoff | 6,267 | 660 |
| [`architecture/capabilities-and-services.md`](architecture/capabilities-and-services.md) | Architecture Specification | draft | Native Capabilities and Application Services | 5,992 | 653 |
| [`architecture/cold-start-model.md`](architecture/cold-start-model.md) | Architecture Specification | draft | Cold-Start Definition and Measurement Model | 5,515 | 647 |
| [`architecture/compiler-and-build.md`](architecture/compiler-and-build.md) | Architecture Specification | draft | Compiler and Build Architecture | 6,764 | 767 |
| [`architecture/concurrency-and-isolation.md`](architecture/concurrency-and-isolation.md) | Architecture Specification | draft | Concurrency, Workers, and Isolation | 5,435 | 636 |
| [`architecture/contract-type-system.md`](architecture/contract-type-system.md) | Architecture Specification | draft | Contract Type System | 5,136 | 533 |
| [`architecture/index.md`](architecture/index.md) | Reserved | n/a | Architecture | 1,058 | 119 |
| [`architecture/lifecycle-policies-and-plugins.md`](architecture/lifecycle-policies-and-plugins.md) | Architecture Specification | draft | Lifecycle, Policies, Modules, and Plugins | 6,859 | 725 |
| [`architecture/observability-and-diagnostics.md`](architecture/observability-and-diagnostics.md) | Architecture Specification | draft | Observability and Diagnostics | 4,872 | 521 |
| [`architecture/overview.md`](architecture/overview.md) | Architecture Specification | draft | Project Q Architecture Overview | 6,805 | 650 |
| [`architecture/packaging-and-deployment.md`](architecture/packaging-and-deployment.md) | Architecture Specification | draft | Packaging and Deployment | 5,080 | 545 |
| [`architecture/quickjs-engine.md`](architecture/quickjs-engine.md) | Architecture Specification | draft | QuickJS Engine Integration | 5,876 | 620 |
| [`architecture/request-response-bridge.md`](architecture/request-response-bridge.md) | Architecture Specification | draft | Request and Response Bridge | 7,297 | 870 |
| [`architecture/review-and-corrections.md`](architecture/review-and-corrections.md) | Architecture Review | draft | Reviewed Architecture: Verdict, Corrections, and Open Tensions | 9,512 | 1,117 |
| [`architecture/routing-and-http.md`](architecture/routing-and-http.md) | Protocol Specification | draft | Routing and HTTP Semantics | 5,989 | 716 |
| [`architecture/rust-host-runtime.md`](architecture/rust-host-runtime.md) | Architecture Specification | draft | Rust Host Runtime | 6,168 | 686 |
| [`architecture/schema-and-validation.md`](architecture/schema-and-validation.md) | Architecture Specification | draft | Schema and Validation Architecture | 6,181 | 651 |
| [`architecture/treaty-client.md`](architecture/treaty-client.md) | Architecture Specification | draft | Treaty-Style Typed Client | 5,523 | 603 |
| [`bundle-report.md`](bundle-report.md) | Bundle Report | draft | Project Q OKF Bundle Report | 13,798 | 1,465 |
| [`decisions/0001-rust-quickjs-bun-toolchain.md`](decisions/0001-rust-quickjs-bun-toolchain.md) | Architecture Decision | draft | ADR-0001: Rust Host, QuickJS Engine, Bun Toolchain | 2,491 | 228 |
| [`decisions/0002-cold-start-first.md`](decisions/0002-cold-start-first.md) | Architecture Decision | draft | ADR-0002: Cold-Start-First Product Priority | 2,190 | 204 |
| [`decisions/0003-no-node-compatibility.md`](decisions/0003-no-node-compatibility.md) | Architecture Decision | draft | ADR-0003: No General Node/Bun Compatibility | 2,009 | 187 |
| [`decisions/0004-static-contract-no-app-dry-run.md`](decisions/0004-static-contract-no-app-dry-run.md) | Architecture Decision | draft | ADR-0004: Static Contract Compilation Without App Dry-Run | 1,966 | 185 |
| [`decisions/0005-native-routing-lazy-bridge.md`](decisions/0005-native-routing-lazy-bridge.md) | Architecture Decision | draft | ADR-0005: Native Routing and Lazy Bridge | 1,756 | 181 |
| [`decisions/0006-engine-adapter-quickjs-ng.md`](decisions/0006-engine-adapter-quickjs-ng.md) | Architecture Decision | draft | ADR-0006: Engine Adapter and QuickJS-NG Initial Target | 2,071 | 176 |
| [`decisions/0007-treaty-dual-contract-mode.md`](decisions/0007-treaty-dual-contract-mode.md) | Architecture Decision | draft | ADR-0007: Dual Treaty Contract Modes | 1,690 | 150 |
| [`decisions/0008-one-runtime-per-worker.md`](decisions/0008-one-runtime-per-worker.md) | Architecture Decision | draft | ADR-0008: One QuickJS Runtime per Worker | 1,677 | 137 |
| [`decisions/0009-schema-ir-explicit-fallback.md`](decisions/0009-schema-ir-explicit-fallback.md) | Architecture Decision | draft | ADR-0009: Native Schema IR and Explicit Fallback | 1,434 | 138 |
| [`decisions/0010-capability-plugin-model.md`](decisions/0010-capability-plugin-model.md) | Architecture Decision | draft | ADR-0010: Capability-Based Plugins and Services | 1,552 | 144 |
| [`decisions/0011-rfc9457-typed-results.md`](decisions/0011-rfc9457-typed-results.md) | Architecture Decision | draft | ADR-0011: Typed Status Results and Problem Details | 1,589 | 122 |
| [`decisions/0012-evidence-before-performance-claims.md`](decisions/0012-evidence-before-performance-claims.md) | Architecture Decision | draft | ADR-0012: Evidence Before Performance Claims | 1,796 | 169 |
| [`decisions/0013-rust-only-initially.md`](decisions/0013-rust-only-initially.md) | Architecture Decision | draft | ADR-0013: Rust-Only Initial Host | 1,310 | 133 |
| [`decisions/0014-version-pinned-bytecode.md`](decisions/0014-version-pinned-bytecode.md) | Architecture Decision | draft | ADR-0014: Version-Pinned Trusted Bytecode | 1,624 | 142 |
| [`decisions/index.md`](decisions/index.md) | Reserved | n/a | Architecture Decisions | 1,453 | 150 |
| [`delivery/backlog.md`](delivery/backlog.md) | Product Backlog | draft | Prioritized Project Q Backlog | 7,815 | 926 |
| [`delivery/index.md`](delivery/index.md) | Reserved | n/a | Delivery | 485 | 62 |
| [`delivery/mvp.md`](delivery/mvp.md) | Delivery Plan | draft | MVP and Feasibility Milestones | 6,469 | 772 |
| [`delivery/prd.md`](delivery/prd.md) | Product Requirements Document | draft | Project Q Product Requirements Document | 20,106 | 2,447 |
| [`delivery/risks-and-open-questions.md`](delivery/risks-and-open-questions.md) | Risk Register | draft | Risks, Open Questions, and Stop Conditions | 6,772 | 799 |
| [`delivery/roadmap.md`](delivery/roadmap.md) | Roadmap | draft | Evidence-Driven Project Q Roadmap | 2,987 | 318 |
| [`delivery/traceability.md`](delivery/traceability.md) | Traceability Matrix | draft | M0–M2 Requirements Traceability | 5,230 | 588 |
| [`engineering/benchmark-methodology.md`](engineering/benchmark-methodology.md) | Engineering Standard | draft | Reproducible Benchmark Methodology | 5,987 | 710 |
| [`engineering/coding-standards.md`](engineering/coding-standards.md) | Engineering Standard | draft | Coding and Review Standards | 4,549 | 510 |
| [`engineering/compatibility-and-versioning.md`](engineering/compatibility-and-versioning.md) | Engineering Standard | draft | Compatibility and Versioning | 4,132 | 485 |
| [`engineering/index.md`](engineering/index.md) | Reserved | n/a | Engineering | 599 | 64 |
| [`engineering/performance-budgets.md`](engineering/performance-budgets.md) | Engineering Standard | draft | Performance Budgets and Decision Gates | 5,620 | 672 |
| [`engineering/release-gates.md`](engineering/release-gates.md) | Engineering Standard | draft | Milestone and Release Gates | 4,284 | 507 |
| [`engineering/repository-layout.md`](engineering/repository-layout.md) | Engineering Specification | draft | Repository Layout and Boundaries | 4,751 | 434 |
| [`engineering/security-model.md`](engineering/security-model.md) | Security Standard | draft | Security Model and Secure Defaults | 5,599 | 603 |
| [`engineering/testing-and-conformance.md`](engineering/testing-and-conformance.md) | Engineering Standard | draft | Testing and Conformance Strategy | 5,129 | 561 |
| [`engineering/tooling-and-build.md`](engineering/tooling-and-build.md) | Engineering Specification | draft | Tooling and Build System | 4,657 | 540 |
| [`index.md`](index.md) | Reserved Root Index | n/a | Project Q Framework Knowledge Bundle | 1,260 | 130 |
| [`log.md`](log.md) | Reserved | n/a | Project Q Design Update Log | 1,367 | 169 |
| [`project/charter.md`](project/charter.md) | Project Charter | draft | Project Q Charter | 4,861 | 554 |
| [`project/competitive-strategy.md`](project/competitive-strategy.md) | Product Strategy | draft | Competitive Strategy | 5,193 | 576 |
| [`project/index.md`](project/index.md) | Reserved | n/a | Project | 701 | 74 |
| [`project/personas-and-journeys.md`](project/personas-and-journeys.md) | User Experience Specification | draft | Personas and User Journeys | 4,166 | 472 |
| [`project/principles.md`](project/principles.md) | Design Principles | draft | Project Q Design Principles | 4,370 | 488 |
| [`project/requirements.md`](project/requirements.md) | Requirements Specification | draft | Project Q Requirements | 8,202 | 921 |
| [`project/scope-and-non-goals.md`](project/scope-and-non-goals.md) | Scope Definition | draft | Scope and Non-Goals | 3,719 | 420 |
| [`project/vision-and-positioning.md`](project/vision-and-positioning.md) | Product Strategy | draft | Vision and Positioning | 4,648 | 490 |
| [`references/design-session.md`](references/design-session.md) | Design Session Record | draft | Project Q Design Session Decisions | 3,490 | 392 |
| [`references/elysia-2-and-eden.md`](references/elysia-2-and-eden.md) | Reference | stable | Elysia 2 and Eden Treaty Design Notes | 3,735 | 331 |
| [`references/index.md`](references/index.md) | Reserved | n/a | References | 515 | 73 |
| [`references/llrt-quickjs-rquickjs.md`](references/llrt-quickjs-rquickjs.md) | Reference | stable | LLRT, QuickJS, QuickJS-NG, and rquickjs Notes | 3,100 | 320 |
| [`references/okf-format.md`](references/okf-format.md) | Reference | stable | Open Knowledge Format v0.2 Bundle Conventions | 3,105 | 334 |
| [`references/source-register.md`](references/source-register.md) | Reference Register | stable | External Source Register and Precedence | 4,414 | 285 |

# Handoff boundary

The implementation agent is authorized only for M0–M2 under [the master prompt](MASTER_AGENT_PROMPT.md). The bundle deliberately prevents broad alpha scope until the Rust–QuickJS bridge and complete cold-start thesis have evidence.

# Regeneration

After editing:

1. validate the bundle;
2. regenerate this inventory;
3. generate `MANIFEST.json` with content hashes;
4. package the directory;
5. calculate the ZIP SHA-256;
6. record the update in `log.md`.
