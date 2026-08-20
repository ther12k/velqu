---
task_id: BETA-017-E
parent_task: BETA-017
milestone: BETA
priority: P0
mode: IMPLEMENT
status: TODO
context_card: context/milestones/BETA.md
commit_required: true
---

# BETA-017-E — Supported beta platforms

## Atomic goal

Supported beta platforms.

## Parent intent

Close only the decisions necessary to publish a public beta.

## Dependencies

- `BETA-017-D` — `tasks/08_public_beta/BETA-017-D-security-contact.md`

## Read only

### Package context

- `LOW_CONTEXT_AGENT_PROMPT.md`
- `GLOBAL_INVARIANTS.md`
- `context/milestones/BETA.md`
- `context/components/devex-beta.md`

### Source files

- `AGENTS.md`
- `docs/beta/`
- `scripts/verify`
- `package.json`
- `.github/workflows/verify.yml`
- `scripts/package`
- `scripts/release-packet`
- `packages/cli/package.json`

Do not scan unrelated directories unless a listed file imports a directly required implementation.

## Steps

1. Confirm every dependency is complete and the working tree is clean.
2. Read only the listed source files and the named milestone context card.
3. Find the existing behavior and its nearest tests; do not redesign adjacent subsystems.
4. Add or adjust the smallest test that proves this microtask when behavior or security changes.
5. Implement exactly this deliverable: Supported beta platforms.
6. Run the targeted commands below and inspect failures rather than weakening assertions.
7. Review the diff for unrelated edits, formatting noise, generated artifacts, and hidden fallback behavior.
8. Commit one atomic change and return the required handoff.

## Parent acceptance guardrails

- Decisions are recorded in ADR/open-decision log.
- No agent invents owner authority.
- Security reporting channel exists.
- Platform/support scope is published.

## Targeted commands

```bash
bun test
```
```bash
bun run typecheck
```

## Required evidence for this microtask

- Accepted decision records.
- Release authorization.
- Contact/support document.
- [ ] No open beta P0/P1 findings or unapproved waivers.
- [ ] A clean external user can install, scaffold, develop, test, build, and deploy a real Velqu app.
- [ ] Fetch, multi-worker service mode, Treaty, optional Postgres, and auth reference work on the actual runtime.
- [ ] Real-world, cold, warm, and CPU/JIT evidence is reproducible and honestly reported.
- [ ] Security baseline, two-hour/one-million-request soak, observability, config, proxy/drain, and clean packaging pass.
- [ ] Release packet is self-verifying and owner decisions are closed.
- [ ] Release is labeled beta, non-SLA, trusted-code-only, and not production-ready GA.
- Canonical microbenchmarks with repetitions.
- Real PostgreSQL W1/W2/W3.
- Controlled I/O and fan-out.
- CPU/JIT crossover.
- 1/2/4 worker scaling.
- Two-hour/one-million-request soak.
- No GA/SLA claim.
- No full Node/Bun compatibility.
- No hostile tenant sandbox.
- No WebSocket/SSE.
- No ORM in core.
- No Windows/macOS support promise unless separately accepted.

At minimum, the handoff must identify the exact changed files, test names, command results, and commit hash.

## Out of scope

- Other implementation bullets from the same parent task.
- Later milestones or optional features.
- Broad refactors not required by the acceptance criteria.
- Changing benchmark/report claims without raw evidence.

## Commit guidance

Suggested subject:

```text
beta-017-e: supported beta platforms
```

## Stop condition

Stop after this task is committed and handed off. Do not automatically begin the next task.

## Handoff format

Use `templates/TASK_RESULT_TEMPLATE.md`. If blocked, use `templates/BLOCKER_TEMPLATE.md`.
