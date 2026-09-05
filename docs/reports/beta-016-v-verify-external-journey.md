# BETA-016-V — Verify External Clean-install and Tutorial Verification

## Overview

Verification closure for parent task **BETA-016** ("Run external
clean-install and tutorial verification"). Every parent acceptance
criterion is mapped to the external transcript stage that proves it,
with the container re-inspected at closure time (no leftover
listeners; all stage transcripts present and ending OK).

## External journey stages (container `velqu-beta-external:0.1.0-beta.1`, digest `sha256:9076de16f6ec…a2f5570`, user `beta`)

| Stage | Packet | Transcript outcome |
|---|---|---|
| Fresh environment | BETA-016-A | image built from digest-pinned base; manifest probe `MANIFEST-OK` (pre-install) |
| Install CLI/runtime from source archive | BETA-016-B | `INSTALL-OK` (6 steps, 214 crates cold; runtime + CLI respond) |
| Scaffold app | BETA-016-C | `SCAFFOLD-OK` (`velqu check: 3 routes — clean`) |
| Tests/dev/build | BETA-016-D | `DEVBUILD-OK` (tests pass; build ×2 byte-identical; dev + production runtimes probed) |
| Deploy proof service | BETA-016-E | `APP-OK → EDGE-OK → VERIFY-OK → ROLLBACK-OK` (corrected lifecycle, re-run in F) |
| Use Treaty client | BETA-016-F | `TREATY-OK` (typed calls live; contract tests 5/0 **without skipping**; teardown proven; second run repeats) |
| Closure hygiene (this packet) | — | no listener on :3000 or :8080; no orphan processes; all five transcripts present |

## Parent acceptance guardrails — evidence mapping

| Guardrail | Evidence | Result |
|---|---|---|
| **No local unpublished dependency** | B installed everything from a `git archive` source tarball (sha256 recorded); app deps resolve through the documented `node_modules/@velqu/*` links into that installed tree; no npm publication assumed | PASS |
| **Tutorial succeeds verbatim** | QUICKSTART commands run word-for-word (`create`, documented link step, `build`, `check`, `dev` probes, `client`); INSTALL.md bodies verbatim through the edge (`{"status":"ok"}`, `{"message":"Hello beta"}`); the tutorial was made verbatim-true by the D fixes (linked-path CLI invocation, `--project .`, install-tree runtime resolution, `cli` in the link step) | PASS |
| **Failures produce actionable diagnostics** | Demonstrated in practice, not asserted: D's three defect diagnostics (missing `velqu` binary, `examples/proof` default leak, runtime-binary not-found + named fix path); E's privilege refusal for `edge`; F's pre-flight guard naming the leftover-service cause; corrected rollback refusing to proceed without a pidfile | PASS |
| **Artifacts can be rolled back/uninstalled** | install: `rm -rf ~/velqu` (single tree); scaffold/app: `rm -rf ~/hello-velqu`; deploy: `ROLLBACK-OK` with edge closed + upstream released + artifacts removed (kernel-state stop proof, post-rollback verify fails closed); treaty: teardown releases the port and an immediate re-run is clean | PASS |

## Issues found across the journey and their resolutions (summary)

1. B: archive lacked a root directory → `--prefix=velqu/` + extraction guard.
2. B: root-owned tooling homes broke unprivileged builds → chown + probe writability check.
3. D: scaffold scripts assumed a global `velqu` binary → linked-path invocation with `--project .`.
4. D: dev server could not find the runtime externally → install-tree candidates + actionable error.
5. D: QUICKSTART link step omitted `cli` → documentation fixed.
6. E: rollback resolved root's `$HOME` — service stop silently skipped (claim corrected in F; lifecycle re-run with owner-aware paths, pidfile fail-closed, port-release assertions).
7. F: `kill -0` liveness defeated by zombies (container init never reaps) → kernel-state reads + behavioral port checks; `/proc`-scan teardown replaces absent `pkill`.
8. Recorded, not fixed (diagnostics wart, out of scope): `create --help` scaffolds instead of printing help (C).

## Environment manifest (closure-time)

Probe output at closure: Debian 12 bookworm, x86_64, Bun 1.4.0, Rust
1.96.0 (default, minimal), tooling homes writable, ports 3000/8080
clean. Note: `fresh=no-velqu-material` is a **pre-install** property of
the image (BETA-016-A) and is intentionally false after the journey —
the journey's whole point is installing Velqu material
(`~/velqu`, `~/hello-velqu`); the probe at image level remains the
freshness gate.

## Gates (this worktree)

- `cargo test -p q-pack` — pass
- `cargo test -p velqu-runtime` — pass
- `bun test` — 434 pass / 0 fail (67 files, in `unshare -rn` netns)
- `bun run typecheck` — pass
- `cargo fmt --all --check` / `cargo clippy --workspace --all-targets -- -D warnings` — pass
- `./scripts/verify` — ALL PASS

## Disclosures

- Verification closure only; no runtime behavior modified in this packet.
- The E correction is carried in F's merged change to
  `deploy-proof-service.sh`; this packet re-ran and confirmed the
  corrected lifecycle.
- Standing CI disclosure applies (zero-step verify workflows since
  ~#714); local gates are the acceptance basis.
