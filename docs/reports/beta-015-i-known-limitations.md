# BETA-015-I — Known Limitations

## Overview

Implements the **known limitations** deliverable of the beta release packet:

- New canonical inventory `docs/beta/KNOWN-LIMITATIONS.md` (indexed in `docs/beta/INDEX.md`) — 18 numbered limitations grouped by runtime/platform, performance boundaries, deployment/operations, packaging/publication, and evidence posture. Every entry names its evidence source (ramp losses artifact, performance methodology, RELEASE_AUTHORITY, BETA-009-B scan disclosure, etc.).
- `scripts/release-packet` now ships `KNOWN-LIMITATIONS.md` in the packet (conditional copy, checksummed by the unified manifest).

## Coverage highlights

- Platform promise (Linux x86_64 glibc only) and trusted-code-only boundary.
- Pack↔runtime exact-match coupling and the PACK_FORMAT v1 pin (owner decision pending, carried in both packet indexes).
- Measured performance boundaries with the honest-loss numbers (2.29× C0 steady floor; no raw-rust overtake in horizon; bytecode≠JIT; no cloud cold-start extrapolation; warm fixture coverage gaps).
- Deployment posture (reverse-proxy first, bounded fail-closed defaults, defer semantics, untrusted forwarded headers, no dynamic code execution).
- Packaging posture (private npm packages, open license decision, advisory scanning unavailable, Owner-gated publication).
- Standing CI disclosure as a first-class limitation.

## Guardrail mapping

- **No stale historical metadata is current** — the inventory reflects the current beta state including every open item carried in the packet indexes.
- **Artifacts map to one source commit** — the file ships inside the commit-bound, checksummed packet.
- **Checksums verify from release directory** — covered by the unified `SHA256SUMS.txt`.

## Gates

- `cargo test -p q-pack` — pass (100+2)
- `cargo test -p q-http` — pass (15)
- `cargo test -p q-schema-runtime` — pass (58)
- `bun test` — 434 pass / 0 fail (67 files)
- `bun run typecheck` — pass
- `cargo fmt --all --check` / `cargo clippy -D warnings` — pass
- `./scripts/validate-okf` — pass
- `./scripts/verify` — ALL PASS (M0–M2 + M2.2.1 + M2.3 + M23R2-GATE-CLOSE verified)

## Disclosures

- Documentation deliverable; no runtime behavior modified.
- Standing CI disclosure: verify workflows stall/fail with zero executed steps at PR creation since roughly #714; local gates/evidence are acceptance basis.
