# BWASM-Q-005 Budget Disposition & Resolution of Carried Findings

Date: `2026-09-07`

## 1. Carried Finding: Kernel WASM Size Under Gzip-9 vs Brotli

- **Carried Finding**: During K-006 and B-006, the vendored `q_browser_kernel_bg.wasm` had an interim gzip-9 size of `572,711 bytes`, which exceeded the 512,000 byte target.
- **Budget Definition in BWASM-D-004**: `docs/codex-spark-browser-wasm/evidence/budgets.json` explicitly states:
  *"compression codec must be brotli (server default for wasm); gzip-9 numbers are interim proxies until brotli tooling is wired into measurement"*.
- **Disposition**:
  - Brotli-11 compression tooling (`python-brotli` and `node:zlib` brotliCompressSync) is now integrated and wired into measurement.
  - Measured Brotli-11 size is **400,229 bytes** (390.8 KiB).
  - The kernel satisfies the ratified budget of 512,000 bytes with **111,771 bytes of headroom**.
  - No budget amendment is required because the ratified standard was Brotli transfer bytes.

## 2. Process for Intentional Budget Changes (Acceptance Criterion 6)

Any proposed change to the release budgets defined in `budgets.json` must follow this process:
1. Open an ADR under `docs/okf/decisions/` or a formal issue proposal.
2. Provide before/after empirical measurement data with raw samples across the reference device tiers.
3. Require explicit Owner sign-off / ratification before any threshold is increased.
4. No automated gate or CI threshold may be loosened without an associated Owner decision reference.
