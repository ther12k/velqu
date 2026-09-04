#!/usr/bin/env python3
"""Validate the Velqu Browser-WASM GitHub issue packet without network access."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from collections import Counter, defaultdict, deque
from pathlib import Path
from typing import Any

ID_RE = re.compile(r"^BWASM-(?:EPIC|GATE|[DKRBCQX]-\d{3})$")
ALLOWED_PHASES = {
    "00_program",
    "01_design",
    "02_kernel",
    "03_runtime",
    "04_build_deploy",
    "05_capabilities",
    "06_quality_release",
    "07_optional_parity",
    "08_gate",
}
ALLOWED_MODES = {
    "IMPLEMENT",
    "VERIFY",
    "VERIFY_OR_FIX",
    "EVIDENCE",
    "GATE",
    "GATE_REVIEW",
}
ALLOWED_PRIORITIES = {"P0", "P1"}
REQUIRED_HEADINGS = [
    "## Atomic goal",
    "## Parent intent",
    "## Architecture invariant",
    "## Dependencies",
    "## Read first",
    "## Steps",
    "## Acceptance criteria",
    "## Targeted tests and commands",
    "## Required evidence",
    "## Guardrails",
    "## Out of scope",
    "## Commit / PR guidance",
    "## Stop condition",
    "## Handoff format",
]
REQUIRED_ROOT_FILES = [
    "README.md",
    "docs/codex-spark-browser-wasm/MASTER_PLAN.md",
    "docs/codex-spark-browser-wasm/TASK_INDEX.md",
    "docs/codex-spark-browser-wasm/DEPENDENCY_GRAPH.md",
    "docs/codex-spark-browser-wasm/OWNER_DECISIONS.md",
    "docs/codex-spark-browser-wasm/DEFINITION_OF_DONE.md",
    "docs/codex-spark-browser-wasm/RESEARCH_NOTES.md",
    "docs/codex-spark-browser-wasm/GITHUB_REGISTRATION.md",
    "docs/codex-spark-browser-wasm/context/milestones/BWASM.md",
    "docs/codex-spark-browser-wasm/manifests/issues.json",
    "docs/codex-spark-browser-wasm/manifests/labels.json",
    "docs/codex-spark-browser-wasm/manifests/phases.json",
]


def packet_root() -> Path:
    # scripts/browser-wasm/validate_packet.py -> packet root
    return Path(__file__).resolve().parents[2]


def load_json(path: Path) -> dict[str, Any]:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise RuntimeError(f"cannot load {path}: {exc}") from exc


def find_cycle(ids: set[str], dependencies: dict[str, list[str]]) -> list[str] | None:
    indegree = {issue_id: 0 for issue_id in ids}
    children: dict[str, list[str]] = defaultdict(list)
    for issue_id, deps in dependencies.items():
        for dep in deps:
            if dep in ids:
                indegree[issue_id] += 1
                children[dep].append(issue_id)

    queue = deque(sorted(k for k, degree in indegree.items() if degree == 0))
    visited: list[str] = []
    while queue:
        current = queue.popleft()
        visited.append(current)
        for child in children[current]:
            indegree[child] -= 1
            if indegree[child] == 0:
                queue.append(child)

    if len(visited) == len(ids):
        return None
    return sorted(k for k, degree in indegree.items() if degree > 0)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--root",
        type=Path,
        default=packet_root(),
        help="Packet root (defaults to path inferred from this script).",
    )
    parser.add_argument(
        "--verify-checksums",
        action="store_true",
        help="Verify SHA256SUMS when present.",
    )
    args = parser.parse_args()
    root = args.root.resolve()

    errors: list[str] = []
    warnings: list[str] = []

    for rel in REQUIRED_ROOT_FILES:
        if not (root / rel).is_file():
            errors.append(f"missing required file: {rel}")

    issues_path = root / "docs/codex-spark-browser-wasm/manifests/issues.json"
    labels_path = root / "docs/codex-spark-browser-wasm/manifests/labels.json"
    phases_path = root / "docs/codex-spark-browser-wasm/manifests/phases.json"
    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        return 1

    manifest = load_json(issues_path)
    labels_manifest = load_json(labels_path)
    phases_manifest = load_json(phases_path)
    issues = manifest.get("issues")
    if not isinstance(issues, list) or not issues:
        errors.append("issues.json must contain a non-empty issues array")
        issues = []

    ids = [str(x.get("id", "")) for x in issues]
    titles = [str(x.get("title", "")) for x in issues]
    id_counts = Counter(ids)
    title_counts = Counter(titles)

    for issue_id, count in id_counts.items():
        if count != 1:
            errors.append(f"duplicate issue id {issue_id!r}: {count}")
        if not ID_RE.fullmatch(issue_id):
            errors.append(f"invalid issue id: {issue_id!r}")

    for title, count in title_counts.items():
        if count != 1:
            errors.append(f"duplicate issue title {title!r}: {count}")

    all_ids = set(ids)
    deps_by_id: dict[str, list[str]] = {}
    body_paths: list[str] = []
    used_labels: set[str] = set()

    for item in issues:
        issue_id = str(item.get("id", ""))
        phase = item.get("phase")
        mode = item.get("mode")
        priority = item.get("priority")
        optional = item.get("optional")
        deps = item.get("dependencies", [])
        labels = item.get("labels", [])
        body_file = item.get("body_file")
        title = item.get("title", "")

        if phase not in ALLOWED_PHASES:
            errors.append(f"{issue_id}: unknown phase {phase!r}")
        if mode not in ALLOWED_MODES:
            errors.append(f"{issue_id}: unknown mode {mode!r}")
        if priority not in ALLOWED_PRIORITIES:
            errors.append(f"{issue_id}: unknown priority {priority!r}")
        if not isinstance(optional, bool):
            errors.append(f"{issue_id}: optional must be boolean")
        if not isinstance(deps, list):
            errors.append(f"{issue_id}: dependencies must be a list")
            deps = []
        if issue_id in deps:
            errors.append(f"{issue_id}: self dependency")
        missing_deps = sorted(set(deps) - all_ids)
        if missing_deps:
            errors.append(f"{issue_id}: unknown dependencies {missing_deps}")
        deps_by_id[issue_id] = list(deps)

        if not isinstance(labels, list) or not labels:
            errors.append(f"{issue_id}: labels must be a non-empty list")
            labels = []
        used_labels.update(str(x) for x in labels)
        expected_mode_label = f"mode:{str(mode).lower()}"
        expected_priority = str(priority).lower()
        for expected in ["milestone:browser-wasm", expected_mode_label, expected_priority]:
            if expected not in labels:
                errors.append(f"{issue_id}: missing label {expected}")
        if issue_id in {"BWASM-EPIC", "BWASM-GATE"}:
            if "spark-gate" not in labels:
                errors.append(f"{issue_id}: gate issue missing spark-gate")
        elif "spark-task" not in labels:
            errors.append(f"{issue_id}: task issue missing spark-task")

        if not isinstance(body_file, str) or not body_file:
            errors.append(f"{issue_id}: missing body_file")
            continue
        body_paths.append(body_file)
        body_path = (root / body_file).resolve()
        try:
            body_path.relative_to(root)
        except ValueError:
            errors.append(f"{issue_id}: body_file escapes packet root: {body_file}")
            continue
        if not body_path.is_file():
            errors.append(f"{issue_id}: body file missing: {body_file}")
            continue
        body = body_path.read_text(encoding="utf-8")
        expected_h1 = f"# {issue_id} —"
        if expected_h1 not in body:
            errors.append(f"{issue_id}: body does not contain expected H1")
        for heading in REQUIRED_HEADINGS:
            if heading not in body:
                errors.append(f"{issue_id}: body missing heading {heading}")
        if title and not str(title).startswith(f"[{issue_id}]"):
            errors.append(f"{issue_id}: GitHub title must start with [{issue_id}]")
        if "Status: `TODO`" not in body:
            warnings.append(f"{issue_id}: body does not contain TODO status")
        if len(body.splitlines()) < 70:
            warnings.append(f"{issue_id}: unusually short body ({len(body.splitlines())} lines)")

    if len(body_paths) != len(set(body_paths)):
        errors.append("multiple issues reference the same body_file")

    cycle = find_cycle(all_ids, deps_by_id)
    if cycle:
        errors.append(f"dependency cycle involving: {cycle}")

    create_labels = {
        str(x.get("name"))
        for x in labels_manifest.get("create", [])
        if isinstance(x, dict)
    }
    reuse_labels = set(labels_manifest.get("reuse_existing", []))
    declared_labels = create_labels | reuse_labels
    undeclared = sorted(used_labels - declared_labels)
    if undeclared:
        errors.append(f"used labels absent from labels manifest: {undeclared}")

    packet_label_list = set(labels_manifest.get("used_by_packet", []))
    if packet_label_list != used_labels:
        errors.append("labels.json used_by_packet does not exactly match issue labels")

    phase_items = phases_manifest.get("phases", [])
    manifest_phase_ids = {x.get("id") for x in phase_items if isinstance(x, dict)}
    if manifest_phase_ids != ALLOWED_PHASES:
        errors.append(
            "phases.json phase IDs do not match allowed phases: "
            f"got {sorted(manifest_phase_ids)}"
        )
    for phase in phase_items:
        if not isinstance(phase, dict):
            continue
        expected = {
            x["id"] for x in issues if x.get("phase") == phase.get("id")
        }
        actual = set(phase.get("issue_ids", []))
        if expected != actual:
            errors.append(
                f"phase {phase.get('id')}: issue_ids mismatch; "
                f"missing {sorted(expected-actual)}, extra {sorted(actual-expected)}"
            )

    expected_optional = {"BWASM-C-003", "BWASM-X-001"}
    actual_optional = {x["id"] for x in issues if x.get("optional") is True}
    if actual_optional != expected_optional:
        errors.append(
            f"optional issue policy changed: expected {sorted(expected_optional)}, "
            f"got {sorted(actual_optional)}"
        )

    gate = next((x for x in issues if x.get("id") == "BWASM-GATE"), None)
    if gate is None:
        errors.append("BWASM-GATE missing")
    else:
        gate_deps = set(gate.get("dependencies", []))
        mandatory_evidence = {
            "BWASM-D-004",
            "BWASM-K-006",
            "BWASM-R-006",
            "BWASM-B-006",
            "BWASM-C-005",
            "BWASM-Q-001",
            "BWASM-Q-002",
            "BWASM-Q-003",
            "BWASM-Q-005",
            "BWASM-Q-006",
            "BWASM-Q-007",
            "BWASM-Q-008",
        }
        missing = mandatory_evidence - gate_deps
        if missing:
            errors.append(f"BWASM-GATE missing mandatory dependencies: {sorted(missing)}")
        optional_gate_deps = gate_deps & expected_optional
        if optional_gate_deps:
            errors.append(
                "optional issues unexpectedly block gate: "
                f"{sorted(optional_gate_deps)}"
            )

    epic = next((x for x in issues if x.get("id") == "BWASM-EPIC"), None)
    if epic is None:
        errors.append("BWASM-EPIC missing")
    if len(issues) != 38:
        errors.append(f"unexpected issue count: expected 38, got {len(issues)}")

    if args.verify_checksums:
        checksum_path = root / "SHA256SUMS"
        if not checksum_path.is_file():
            errors.append("SHA256SUMS missing")
        else:
            for line_number, line in enumerate(
                checksum_path.read_text(encoding="utf-8").splitlines(), 1
            ):
                if not line.strip():
                    continue
                try:
                    expected_hash, rel = line.split("  ", 1)
                except ValueError:
                    errors.append(f"SHA256SUMS:{line_number}: malformed line")
                    continue
                path = root / rel
                if not path.is_file():
                    errors.append(f"SHA256SUMS:{line_number}: missing {rel}")
                    continue
                actual_hash = hashlib.sha256(path.read_bytes()).hexdigest()
                if actual_hash != expected_hash:
                    errors.append(
                        f"SHA256SUMS:{line_number}: mismatch for {rel}"
                    )

    for warning in warnings:
        print(f"WARNING: {warning}", file=sys.stderr)
    for error in errors:
        print(f"ERROR: {error}", file=sys.stderr)

    if errors:
        print(
            f"Packet validation FAILED: {len(errors)} error(s), "
            f"{len(warnings)} warning(s).",
            file=sys.stderr,
        )
        return 1

    phase_counts = Counter(x["phase"] for x in issues)
    optional_count = sum(bool(x["optional"]) for x in issues)
    print(
        "Packet validation PASS\n"
        f"  root: {root}\n"
        f"  issues: {len(issues)} "
        f"({len(issues)-optional_count} mandatory, {optional_count} optional)\n"
        f"  labels used: {len(used_labels)}\n"
        f"  phases: {dict(sorted(phase_counts.items()))}\n"
        f"  warnings: {len(warnings)}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
