#!/usr/bin/env python3
"""Regression tests for the #1429 decision logic in validate-benchmark-evidence.py.

Exercises the module-level helpers against synthetic git repositories and
pure-function cases:

  - runtime_input_drift: no drift at the capture commit, drift after a
    runtime-input change, None for unresolvable/malformed capture commits,
    no drift for changes OUTSIDE the runtime inputs
  - runtime_inputs_dirty: clean tree vs uncommitted runtime-input change
  - classify_target_mismatch: expected-staleness note under drift, undecidable
    notes, reproducibility ERROR when a mismatch has no drift
  - binding_finding: v3 strict errors (missing recording, mismatched binary),
    v2 loud notes, agreement passes everywhere

Run: python3 scripts/validate-benchmark-evidence.test.py  (exit 0 = all pass)
"""
import contextlib
import importlib.util
import io
import json
import subprocess
import sys
import tempfile
from pathlib import Path

SCRIPT = Path(__file__).resolve().parent / "validate-benchmark-evidence.py"
spec = importlib.util.spec_from_file_location("validate_benchmark_evidence", SCRIPT)
validator = importlib.util.module_from_spec(spec)
# Importing executes the script body against the real tree (loads summaries,
# prints notes); only the exit is __main__-guarded. Silence the side-effect
# output — the assertions below are on the module's decision helpers.
with contextlib.redirect_stdout(io.StringIO()):
    spec.loader.exec_module(validator)

FAILURES = []


def check(name, condition, detail=""):
    if condition:
        print(f"  ok: {name}")
    else:
        print(f"  FAIL: {name} {detail}")
        FAILURES.append(name)


def git(repo, *args):
    r = subprocess.run(["git", "-c", "user.name=test", "-c", "user.email=test@example.com",
                        "-c", "commit.gpgsign=false", *args], cwd=repo, capture_output=True, text=True)
    if r.returncode != 0:
        raise RuntimeError(f"git {' '.join(args)} failed: {r.stderr}")
    return r.stdout.strip()


def make_repo(root):
    """A repository whose HEAD is the capture commit; returns (repo, head)."""
    repo = root / "repo"
    repo.mkdir(parents=True)
    git(repo, "init", "-q")
    (repo / "crates").mkdir()
    (repo / "crates" / "lib.rs").write_text("fn a() {}\n")
    (repo / "Cargo.toml").write_text("[workspace]\n")
    (repo / "docs").mkdir()
    (repo / "docs" / "note.md").write_text("docs only\n")
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", "capture state")
    return repo, git(repo, "rev-parse", "HEAD")


def test_drift(root):
    print("runtime_input_drift:")
    repo, head = make_repo(root)

    # At the capture commit itself: no drift.
    check("no drift at capture commit", validator.runtime_input_drift(repo, head) is False)

    # Malformed / unresolvable capture commits: undecidable.
    check("short hash is undecidable", validator.runtime_input_drift(repo, "abc123") is None)
    check("non-hex is undecidable", validator.runtime_input_drift(repo, "z" * 40) is None)
    check("unknown commit is undecidable",
          validator.runtime_input_drift(repo, "0" * 40) is None)

    # A runtime-input change after the capture commit: drift.
    (repo / "crates" / "lib.rs").write_text("fn a() { changed }\n")
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", "runtime change")
    check("crates change is drift", validator.runtime_input_drift(repo, head) is True)

    # Cargo.lock changes are drift too.
    (repo / "Cargo.lock").write_text("version = 4\n")
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", "lock change")
    check("lockfile change is drift", validator.runtime_input_drift(repo, head) is True)

    # A docs-only repo across two commits: no drift.
    repo2, head2 = make_repo(root / "second")
    (repo2 / "docs" / "note.md").write_text("docs changed only\n")
    git(repo2, "add", "-A")
    git(repo2, "commit", "-q", "-m", "docs change")
    check("docs-only change is not drift", validator.runtime_input_drift(repo2, head2) is False)


def test_dirty(root):
    print("runtime_inputs_dirty:")
    repo, _head = make_repo(root / "dirty")
    check("clean tree is not dirty", validator.runtime_inputs_dirty(repo) is False)
    (repo / "crates" / "lib.rs").write_text("uncommitted runtime change\n")
    check("uncommitted crates change is dirty", validator.runtime_inputs_dirty(repo) is True)
    (repo / "crates" / "lib.rs").write_text("fn a() {}\n")
    (repo / "docs" / "note.md").write_text("uncommitted docs change\n")
    check("uncommitted docs-only change is not dirty", validator.runtime_inputs_dirty(repo) is False)


def test_classify():
    print("classify_target_mismatch:")
    kind, msg = validator.classify_target_mismatch("qRuntimeRelease", True, False)
    check("drift -> note", kind == "note" and "expected staleness" in msg)
    kind, msg = validator.classify_target_mismatch("qRuntimeRelease", False, False)
    check("no drift -> error", kind == "error" and "reproducibility failure" in msg)
    kind, msg = validator.classify_target_mismatch("qRuntimeRelease", None, False)
    check("undecidable -> note", kind == "note" and "undecidable" in msg)
    kind, msg = validator.classify_target_mismatch("qRuntimeRelease", False, True)
    check("dirty tree -> note", kind == "note" and "undecidable" in msg)
    # A dirty tree must never produce the error path: byte identity is not
    # decidable against the capture until the tree is committed or restored.
    check("dirty tree never errors", validator.classify_target_mismatch("x", True, True)[0] == "note")


def test_binding():
    print("binding_finding:")
    h1 = "a" * 64
    h2 = "b" * 64
    v3 = "velqu-benchmark-manifest-v3"
    v2 = "velqu-benchmark-manifest-v2"

    check("agreement passes (v3)", validator.binding_finding(v3, h1, h1, "s.json") == ("ok", ""))
    check("agreement passes (v2)", validator.binding_finding(v2, h1, h1, "s.json") == ("ok", ""))
    check("absent recording passes (v2)", validator.binding_finding(v2, None, h1, "s.json") == ("ok", ""))

    kind, msg = validator.binding_finding(v3, h2, h1, "s.json")
    check("mismatch errors (v3)", kind == "error" and "recapture" in msg)
    kind, msg = validator.binding_finding(v2, h2, h1, "s.json")
    check("mismatch notes (v2)", kind == "note" and "predates binding-integrity" in msg)
    kind, msg = validator.binding_finding(v3, None, h1, "s.json")
    check("missing recording errors (v3)", kind == "error" and "binaryHashes" in msg)


def test_real_tree_smoke():
    """The shipped tree's helpers must at least classify the live state.

    On the real repository at any commit: drift is a tri-state and the live
    route-count summary either agrees with the manifest (post-recapture) or
    reports through the v2/v3 paths without raising.
    """
    print("live-tree smoke:")
    manifest = json.loads((validator.ROOT / "benchmarks/manifest.json").read_text())
    captured = manifest.get("commit") or ""
    drift = validator.runtime_input_drift(validator.ROOT, captured)
    check("live drift is tri-state", drift in (True, False, None), f"got {drift!r}")
    summary = json.loads((validator.ROOT / "benchmarks/raw/route-count/summary.json").read_text())
    recorded = (summary.get("binaryHashes") or {}).get("target/release/velqu-runtime")
    manifest_runtime = (manifest.get("artifacts", {}).get("qRuntimeRelease") or {}).get("sha256") or ""
    kind, _msg = validator.binding_finding(manifest.get("format"), recorded, manifest_runtime,
                                           "benchmarks/raw/route-count/summary.json")
    check("live binding decision is a known kind", kind in ("ok", "note", "error"), f"got {kind!r}")


def main():
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        test_drift(root)
        test_dirty(root)
    test_classify()
    test_binding()
    test_real_tree_smoke()
    if FAILURES:
        print(f"\n{len(FAILURES)} FAILURE(S): {', '.join(FAILURES)}")
        sys.exit(1)
    print("\nvalidate-benchmark-evidence tests: ALL PASS")


if __name__ == "__main__":
    main()
