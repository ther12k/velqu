# @velqu/browser-runtime

Browser-WASM runtime: `fetch(Request) => Promise<Response>` over the Rust/WASM kernel with generated handlers in an isolated Worker. Browser-only (no Bun.*/node:* imports); ships the hash-pinned `kernel/` assets.

Part of [Velqu](https://github.com/ther12k/velqu) — a Rust HTTP runtime that runs TypeScript handlers. See the [repo README](https://github.com/ther12k/velqu) for the 30-second example and docs index.

**Beta:** version `0.1.0-beta.1`. Requires Bun 1.4+ where noted. Licensed MIT.
