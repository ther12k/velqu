# BETA-012-F — Deployment

## Overview

Audited and strengthened `docs/beta/DEPLOYMENT-REVERSE-PROXY.md`, the canonical beta deployment guide (reverse-proxy-first posture, forwarded-header policy, health/readiness/drain, rollout sequence, boundaries). The guide was already beta-accurate; the gap was that its Nginx sample had never been exercised. This packet rehearses the sample's proxy semantics end-to-end and records the rehearsal honestly in the doc.

## Rehearsal (2026-09-04, this worktree)

Built the proof pack and release runtime, then rehearsed a non-TLS derivation of the doc's Nginx block (`listen 8080`, backend on a private port) with real processes:

- Runtime: `velqu-runtime --pack examples/proof/dist/app.qpack --host 127.0.0.1 --port 3100 --proxy-mode reverse-proxy` (in a minimal glibc container, host network).
- Edge: nginx:alpine with the doc's config shape (loopback `proxy_pass`, forwarded headers set as data, health locations proxied).
- Results through the edge: `/health/live` = `{"status":"ok"}`, `/health/ready` = `{"ready":true}`, `/hello/nginx` = `{"message":"Hello nginx"}`.

The doc now states exactly this: the proxy semantics (loopback `proxy_pass`, forwarded headers as data, health endpoints through the boundary) were rehearsed with a non-TLS derivation; the TLS directives are standard nginx configuration requiring a real certificate environment to exercise.

## Guardrail compliance

- **Every command/sample is tested** — build commands and the proxy derivation above; the doc's "Verify locally" commands are the standard battery (below). Honest note added for the untestable-here TLS directives.
- **No universal performance claim** — none made; the doc claims posture, not performance.
- **No production-ready/SLA wording** — explicit "not a production readiness or availability guarantee" framing retained.
- **Bytecode vs JIT accurate** — not re-discussed here (covered in INSTALL/ARCHITECTURE); no inaccurate statement present.

## Link check

Doc references ADR-0034, `docs/beta/INSTALL.md` sibling docs, and repo paths — all exist; doc indexed in `docs/beta/INDEX.md` and `docs/beta/README.md`.

## Gates

- `cargo test -p velqu-runtime` — pass (8 suites ok)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` — pass
- `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Documentation change only; no runtime behavior modified.
- The rehearsal used ephemeral local containers (debian-slim runtime, nginx:alpine edge); no external systems touched.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
