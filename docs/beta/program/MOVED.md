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

`docs/codex-spark-browser-wasm/` remains in place until after the
Browser-WASM GO/NO-GO gate review: the release-candidate packet binds
claim hashes to exact paths there, and moving evidence bytes before the
verdict would invalidate the frozen candidate (#1179).

Closed GitHub issues and pull requests keep citing the old paths as
historical text; that is expected and does not affect validity.
