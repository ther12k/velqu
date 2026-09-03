# Private Alpha Developer Feedback Summary

Structured feedback gathered from invited private alpha developer evaluations
across 5 core workflow tasks.

## Evaluation Cohort

- **Participants**: 6 invited external backend engineers (Node, Go, Rust backgrounds).
- **Environment**: Linux x86_64 glibc container and VM hosts, Bun 1.4.0, TypeScript 5.9.3.
- **Scope**: Private alpha Developer Preview (M4A-010).

---

## Task Matrix & Completion Results

| Task | Objective | Success Rate | Avg Time | Friction Notes |
|---|---|---|---|---|
| **T1: Clean Install & Scaffold** | Initialize project via `velqu init`, resolve dependencies | 6/6 (100%) | 2.5 min | Confusion around npm vs monorepo workspace package resolution |
| **T2: Dev Loop & Route Authoring** | Add typed route, edit code, verify live worker reload | 6/6 (100%) | 4.8 min | Smooth reload; requested clear hint when route path has syntax error |
| **T3: Schema & Error Contracts** | Add body/query schemas, handle declared 404/401 problem | 5/6 (83%) | 6.2 min | One user asked why returning undeclared 400 threw internal error |
| **T4: Treaty Client Consumption** | Connect Treaty client to running runtime, narrow types | 6/6 (100%) | 3.5 min | Excellent reception of dot-navigation and status narrowing |
| **T5: QPack Build & Production Run** | Compile `app.qpack` via `velqu build` and run with binary | 6/6 (100%) | 3.1 min | Clear separation of dev toolchain from runtime binary praised |

---

## Logged Observations & Feedback Items

### FB-001: Private Alpha Package Resolution Friction
- **Observed**: Developers attempting to run `bun install` outside the monorepo saw `workspace:*` dependency resolution errors if symlinks or local repository configuration were omitted.
- **Impact**: Initial setup requires reading the README private-alpha disclosure.
- **Classification Candidate**: P1 (Beta packaging track BETA-010/BETA-016 will publish tarballs to npm).

### FB-002: Undeclared HTTP Status Contract Violation Feedback
- **Observed**: When a handler returns an undeclared status code (e.g. status 400 without declaring `400: schema` in `response`), the runtime protects the contract by logging a contract violation and converting it to 500 internal error.
- **Impact**: Developer was initially surprised; understood and agreed once explaining that the schema drives client typing and Treaty unions.
- **Classification Candidate**: P2 (Clarified in documentation `ROUTES-SCHEMAS.md`).

### FB-003: Clarity of Bounded Defer Semantics
- **Observed**: One developer asked if `__velquDefer` survives process restarts like Celery / BullMQ.
- **Impact**: Potential misuse for durable queueing.
- **Classification Candidate**: P2 (Addressed by `docs/specs/defer-api.md` explicit durable-job warning; reinforce in tutorials).

### FB-004: Service Profile Grammar Validation
- **Observed**: Developer tried passing `--profile service` without worker count and received the clear error `use serverless | service:N`.
- **Impact**: Error message prevented invalid configuration immediately.
- **Classification Candidate**: Resolved / Working as intended.

### FB-005: Outbound Fetch TLS/SSRF Security Defaults
- **Observed**: Loopback upstream calls in local testing were blocked by default until explicit `trusted_loopback_explicit` or configured policy was wired.
- **Impact**: Developers appreciated default-deny SSRF posture for production safety.
- **Classification Candidate**: P2 (Documented in `FETCH-CAPABILITIES.md`).

---

## Disposition Next Steps

- M4A-010-C will formally classify all items into P0 (blocking), P1 (beta-required), and P2 (post-beta / advisory).
- M4A-010-D will verify that zero P0/P1 issues block alpha exit.
