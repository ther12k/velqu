# Beta program ledger (moved)

This ledger previously lived at `docs/codex-spark-beta/` and moved to
`docs/beta/program/` in the consolidation that followed PR #1297. If you
arrived from a GitHub issue or report citing the old path, the same file
is here with the path prefix changed:

| Old path | New path |
| --- | --- |
| `docs/codex-spark-beta/WORKFLOW.md` | `docs/beta/program/WORKFLOW.md` |
| `docs/codex-spark-beta/STATUS.md` | `docs/beta/program/STATUS.md` |
| `docs/codex-spark-beta/tasks/…` | `docs/beta/program/tasks/…` |
| `docs/codex-spark-beta/indexes/…` | `docs/beta/program/indexes/…` |

`docs/browser-wasm/` remains in place until after the
Browser-WASM GO/NO-GO gate review: the release-candidate packet binds
claim hashes to exact paths there, and moving evidence bytes before the
verdict would invalidate the frozen candidate (#1179).

Closed GitHub issues and pull requests keep citing the old paths as
historical text; that is expected and does not affect validity.

## Browser-WASM program tree (2026-09-11)

`docs/codex-spark-browser-wasm/` moved to `docs/browser-wasm/` at the
BWASM-GATE close (owner GO, issue #1179). Same file set, path prefix
changed:

| Old path | New path |
| --- | --- |
| `docs/codex-spark-browser-wasm/tasks/…` | `docs/browser-wasm/tasks/…` |
| `docs/codex-spark-browser-wasm/evidence/…` | `docs/browser-wasm/evidence/…` |
| `docs/codex-spark-browser-wasm/gates/…` | `docs/browser-wasm/gates/…` |
| `docs/codex-spark-browser-wasm/manifests/…` | `docs/browser-wasm/manifests/…` |

Registration-time snapshot files (`SHA256SUMS`, `DRY_RUN_ALL.txt`, task
records, manifests, transcripts) intentionally keep their original bytes
with the old path strings inside — they are historical records; the gate
record discloses the mechanical packet deltas (path strings in
`candidate-index.json`/`checksums.sha256`).
