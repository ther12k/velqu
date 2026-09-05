#!/usr/bin/env python3
"""Register Velqu Browser-WASM issues with GitHub CLI.

Dry-run is the default. Add --apply to perform writes.
"""

from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Any


def packet_root() -> Path:
    return Path(__file__).resolve().parents[2]


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def run_gh(args: list[str], *, capture: bool = True) -> str:
    command = ["gh", *args]
    result = subprocess.run(
        command,
        check=False,
        text=True,
        capture_output=capture,
    )
    if result.returncode != 0:
        stderr = result.stderr.strip() if result.stderr else ""
        stdout = result.stdout.strip() if result.stdout else ""
        detail = stderr or stdout or f"exit {result.returncode}"
        raise RuntimeError(f"command failed: {' '.join(command)}\n{detail}")
    return result.stdout.strip() if capture else ""


def select_issues(
    issues: list[dict[str, Any]],
    phases: set[str],
    ids: set[str],
    include_optional: bool,
) -> list[dict[str, Any]]:
    selected = []
    for issue in issues:
        if phases and issue["phase"] not in phases:
            continue
        if ids and issue["id"] not in ids:
            continue
        if issue.get("optional") and not include_optional:
            continue
        selected.append(issue)
    return selected


def print_plan(
    repo: str,
    root: Path,
    selected: list[dict[str, Any]],
    labels_to_create: list[dict[str, Any]],
) -> None:
    print("DRY RUN — no network calls or writes will be performed.")
    print(f"Repository: {repo}")
    print(f"Packet root: {root}")
    print("\nLabels declared for creation when missing:")
    for label in labels_to_create:
        print(
            f"  - {label['name']} "
            f"(#{label['color']}): {label.get('description', '')}"
        )
    print(f"\nIssues selected: {len(selected)}")
    selected_ids = {issue["id"] for issue in selected}
    for issue in selected:
        deps_outside = [
            dep for dep in issue.get("dependencies", []) if dep not in selected_ids
        ]
        optional = " [OPTIONAL]" if issue.get("optional") else ""
        print(f"\n  {issue['id']}{optional}")
        print(f"    title: {issue['title']}")
        print(f"    body: {issue['body_file']}")
        print(f"    labels: {', '.join(issue['labels'])}")
        if deps_outside:
            print(
                "    dependencies outside this selection: "
                + ", ".join(deps_outside)
            )
    print("\nRe-run with --apply only after reviewing this plan.")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", required=True, help="GitHub repository, e.g. ther12k/velqu")
    parser.add_argument(
        "--phase",
        action="append",
        default=[],
        help="Select a phase; may be repeated.",
    )
    parser.add_argument(
        "--id",
        action="append",
        default=[],
        help="Select an exact packet issue ID; may be repeated.",
    )
    parser.add_argument(
        "--include-optional",
        action="store_true",
        help="Include issues marked optional.",
    )
    parser.add_argument(
        "--apply",
        action="store_true",
        help="Perform GitHub writes. Without this flag the script is offline dry-run.",
    )
    parser.add_argument(
        "--skip-label-create",
        action="store_true",
        help="Do not create packet-specific labels; require all labels to exist.",
    )
    parser.add_argument(
        "--root",
        type=Path,
        default=packet_root(),
        help="Packet root.",
    )
    args = parser.parse_args()

    root = args.root.resolve()
    issues_manifest = load_json(
        root / "docs/codex-spark-browser-wasm/manifests/issues.json"
    )
    labels_manifest = load_json(
        root / "docs/codex-spark-browser-wasm/manifests/labels.json"
    )
    issues = issues_manifest["issues"]
    known_phases = {issue["phase"] for issue in issues}
    known_ids = {issue["id"] for issue in issues}

    requested_phases = set(args.phase)
    requested_ids = set(args.id)
    unknown_phases = requested_phases - known_phases
    unknown_ids = requested_ids - known_ids
    if unknown_phases:
        parser.error(f"unknown phase(s): {', '.join(sorted(unknown_phases))}")
    if unknown_ids:
        parser.error(f"unknown issue ID(s): {', '.join(sorted(unknown_ids))}")

    selected = select_issues(
        issues,
        requested_phases,
        requested_ids,
        args.include_optional,
    )
    if not selected:
        parser.error("selection is empty")

    labels_to_create = labels_manifest.get("create", [])
    if not args.apply:
        print_plan(args.repo, root, selected, labels_to_create)
        return 0

    if shutil.which("gh") is None:
        print("ERROR: GitHub CLI 'gh' was not found.", file=sys.stderr)
        return 2

    # Validate auth/repository before any write. Exercise the token
    # directly: `gh auth status` enumerates every keyring account and
    # fails when any unrelated stored entry is invalid, which would
    # block registration even with a fully working active account.
    run_gh(["api", "user"])
    run_gh(["repo", "view", args.repo, "--json", "nameWithOwner"])

    existing_label_rows = json.loads(
        run_gh([
            "label",
            "list",
            "--repo",
            args.repo,
            "--limit",
            "1000",
            "--json",
            "name",
        ])
        or "[]"
    )
    existing_labels = {row["name"] for row in existing_label_rows}

    if not args.skip_label_create:
        for label in labels_to_create:
            name = label["name"]
            if name in existing_labels:
                print(f"SKIP label exists: {name}")
                continue
            print(f"CREATE label: {name}")
            run_gh([
                "label",
                "create",
                name,
                "--repo",
                args.repo,
                "--color",
                label["color"],
                "--description",
                label.get("description", ""),
            ])
            existing_labels.add(name)

    all_required_labels = {
        label for issue in selected for label in issue.get("labels", [])
    }
    missing_labels = sorted(all_required_labels - existing_labels)
    if missing_labels:
        print(
            "ERROR: required labels do not exist: "
            + ", ".join(missing_labels),
            file=sys.stderr,
        )
        print(
            "Create/rename them deliberately or update labels.json; "
            "the script will not guess replacements.",
            file=sys.stderr,
        )
        return 3

    existing_issue_rows = json.loads(
        run_gh([
            "issue",
            "list",
            "--repo",
            args.repo,
            "--state",
            "all",
            "--limit",
            "1000",
            "--json",
            "number,title,url,state",
        ])
        or "[]"
    )
    existing_by_title = {row["title"]: row for row in existing_issue_rows}

    results: list[dict[str, Any]] = []
    for issue in selected:
        title = issue["title"]
        existing = existing_by_title.get(title)
        if existing:
            print(
                f"SKIP issue exists: #{existing['number']} "
                f"{issue['id']} {existing['url']}"
            )
            results.append({
                "id": issue["id"],
                "action": "skipped-existing",
                **existing,
            })
            continue

        body_path = (root / issue["body_file"]).resolve()
        try:
            body_path.relative_to(root)
        except ValueError:
            raise RuntimeError(
                f"body path escapes packet root: {issue['body_file']}"
            )
        if not body_path.is_file():
            raise RuntimeError(f"body file missing: {body_path}")

        command = [
            "issue",
            "create",
            "--repo",
            args.repo,
            "--title",
            title,
            "--body-file",
            str(body_path),
        ]
        for label in issue["labels"]:
            command.extend(["--label", label])

        print(f"CREATE issue: {issue['id']} — {title}")
        url = run_gh(command)
        results.append({
            "id": issue["id"],
            "action": "created",
            "title": title,
            "url": url,
        })

    output = (
        root
        / "docs/codex-spark-browser-wasm/manifests/registration-results.json"
    )
    output.write_text(
        json.dumps(
            {
                "repository": args.repo,
                "results": results,
            },
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )
    print(f"\nRegistration result written to: {output}")
    print(
        f"Created: {sum(r['action'] == 'created' for r in results)}; "
        f"skipped existing: "
        f"{sum(r['action'] == 'skipped-existing' for r in results)}"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except RuntimeError as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        raise SystemExit(1)
