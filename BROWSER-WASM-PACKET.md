# Velqu Browser-WASM GitHub Issue Packet

This packet converts the Browser-WASM proposal into **38 GitHub-ready issues** for `ther12k/velqu`.

## Start here

1. Read [`docs/codex-spark-browser-wasm/MASTER_PLAN.md`](docs/codex-spark-browser-wasm/MASTER_PLAN.md).
2. Resolve [`OWNER_DECISIONS.md`](docs/codex-spark-browser-wasm/OWNER_DECISIONS.md).
3. Review [`TASK_INDEX.md`](docs/codex-spark-browser-wasm/TASK_INDEX.md) and [`DEPENDENCY_GRAPH.md`](docs/codex-spark-browser-wasm/DEPENDENCY_GRAPH.md).
4. Validate the packet:
   ```bash
   python3 scripts/browser-wasm/validate_packet.py
   ```
5. Preview GitHub operations without writing:
   ```bash
   python3 scripts/browser-wasm/create_github_issues.py --repo ther12k/velqu
   ```
6. Register only the program/design phase first:
   ```bash
   python3 scripts/browser-wasm/create_github_issues.py \
     --repo ther12k/velqu \
     --phase 00_program \
     --phase 01_design \
     --apply
   ```

## Architecture in one sentence

Velqu emits static browser artifacts in which a **Rust/WASM kernel** owns compatibility-critical routing/validation/authorization, while generated handlers run in an isolated browser Worker; native Velqu remains the production target for production-only capabilities.

## Packet contents

- master plan and definition of done;
- one combined [`ALL_ISSUES.md`](docs/codex-spark-browser-wasm/ALL_ISSUES.md) plus standalone issue files;
- owner decision checklist;
- 38 individual issue bodies;
- issue/label manifests in JSON and CSV;
- Mermaid dependency graph;
- issue and blocker templates;
- packet validator;
- dry-run-by-default GitHub CLI registration script;
- SHA-256 inventory.

## Important boundaries

“Runs in browser” means no Velqu application server after static deployment. It still needs static HTTPS hosting, and applications may still call an external model/API gateway. The MVP does not claim native Hyper/Tokio transport parity, full PostgreSQL parity, or a proven hostile-code sandbox.

Research baseline: `ther12k/velqu@84740c54242a116ad8424dc4a14cca8d3af2dd93` (2026-09-04). Re-check repository state before implementation and refresh decisions where upstream code has changed.
