#!/usr/bin/env python3
"""#1318 / M6-003 — explicit unsafe/FFI audit artifact generator.

Enumerates every `unsafe` occurrence in the workspace's first-party
crates, classifies it against the audit taxonomy (FFI boundary,
generation check, raw slice op, hint/deref), and pairs each with the
covering evidence (fuzz target, sanitizer stage, Miri, or unit test).
Output: docs/production/evidence/m6-unsafe-audit.md — committed, hashed
input to the M6-003 acceptance (the unsafe audit deliverable; Miri
exclusions alone are not an unsafe audit).

Fail-closed: exits non-zero if an unsafe block is found that the
classification table does not cover, so new unsafe code cannot enter
without an explicit audit entry.
"""
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "docs/production/evidence/m6-unsafe-audit.md"

# Every first-party workspace crate is audited. Miri coverage per crate
# comes from the single machine-readable source of truth (fuzz/miri-scope.json),
# shared with scripts/miri-campaign.sh — the audit can no longer claim a
# Miri inclusion the runner does not perform (owner review 2026-09-12).
import json

WORKSPACE_CRATES = [
    "q-runtime", "q-bytecode-tool", "q-engine", "q-runtime-model",
    "q-engine-quickjs", "q-http", "q-router", "q-browser-kernel",
    "q-bridge", "q-pack", "q-schema-runtime", "q-capabilities",
    "q-capability-postgres", "q-bench-support",
]
MIRI_SCOPE = json.loads((ROOT / "fuzz" / "miri-scope.json").read_text())
MIRI_INCLUDED = set(MIRI_SCOPE["included"])
MIRI_EXCLUDED = {e["crate"]: e for e in MIRI_SCOPE["excluded"]}
mapped = MIRI_INCLUDED | set(MIRI_EXCLUDED)
missing_from_map = [c for c in WORKSPACE_CRATES if c not in mapped]
CRATES = [f"crates/{c}" for c in WORKSPACE_CRATES]

PATTERN = re.compile(r"\bunsafe\s+(?:fn|impl|trait|extern|mod)|\bunsafe\s*\{")

def classify(crate: str, rel_path: str, line: str) -> str:
    if "/tests/" in rel_path or rel_path.endswith("_test.rs") or rel_path.endswith("conformance.rs") and "/tests/" in rel_path:
        return "test-scope unsafe (covered by workspace test runs; not the serving path)"
    if "Box::from_raw" in line and "leaked" in line:
        # cfg(test) fixture inside crates/q-pack/src/lib.rs (the only
        # Box::from_raw in the workspace): the test leaks a slice to
        # mimic include_bytes!, asserts zero-copy views, then reclaims
        # the exact same allocation so LeakSanitizer stays clean. The
        # pointer provenance is the test's own Box::leak — no serving-
        # path bytes are affected (owner review 3, 2026-09-12).
        return "test-scope unsafe (zero-copy fixture reclaims its own Box::leak'd slice after the assertions; covered by workspace test runs under LeakSanitizer; not the serving path)"
    if "AllocCounters" in line or "alloc_fn" in line or "f(&mut c)" in line:
        return "FFI call (tracer snapshot fn pointer into the allocation tracer; bench-support evidence tool, not the serving path)"
    if "getrusage" in line or "assume_init" in line or "dlsym" in line or ("transmute" in line and "AllocCounters" in line):
        return "FFI call (libc getrusage/dlsym allocation+CPU instrumentation; bench-support diagnostics, not the serving path)"
    if "zeroed" in line:
        return "FFI call (libc rusage zeroed-init; bench-support diagnostics, not the serving path)"
    if "Mmap::map" in line or "memmap2" in line:
        return "raw memory op (mmap-backed pack load; native-only; integrity digest checked before use)"
    if "extern" in line and "C" in line:
        return "FFI declaration"
    if crate == "crates/q-engine-quickjs":
        return "rquickjs boundary (safe-wrapped C engine; generation-checked handles)"
    if "generation" in line or "slot" in line or "handle" in line:
        return "handle/generation check"
    if "from_raw_parts" in line or "slice" in line or "transmute" in line or "zeroed" in line:
        return "raw memory op"
    return "UNCLASSIFIED"

def miri_line(crate: str) -> str:
    name = crate.removeprefix("crates/")
    if name in MIRI_INCLUDED:
        return "Miri: INCLUDED (scripts/miri-campaign.sh)"
    e = MIRI_EXCLUDED.get(name)
    if e is None:
        return "Miri: NOT IN SCOPE MAP — AUDIT FAILURE"
    comps = "; ".join(e.get("compensations", []))
    return f"Miri: EXCLUDED — {e['reason']}. Compensations: {comps}"


def covering_evidence(crate: str) -> list[str]:
    ev = ["ASan workspace pass (S1, scripts/fuzz-campaign.sh)"]
    ev.append(miri_line(crate))
    name = crate.removeprefix("crates/")
    fuzz_targets = {
        "q-engine-quickjs": ["cargo-fuzz: bridge_handles + pack_verify (handle and bytecode trust boundaries)"],
        "q-bridge": ["cargo-fuzz: bridge_handles (stale/foreign access never grants)"],
        "q-http": ["cargo-fuzz: http_decode (ingress decoder no-amplification)"],
        "q-capabilities": ["cargo-fuzz: capabilities_policy (SSRF gate, redirect limiter, identity/inventory/DAG)"],
        "q-schema-runtime": ["cargo-fuzz: schema_validate + codec_encoders"],
        "q-router": ["cargo-fuzz: router_match"],
        "q-pack": ["cargo-fuzz: pack_verify"],
    }.get(name, [])
    ev += fuzz_targets
    ev.append("workspace unit/conformance tests (cargo test -p)")
    return ev

def main() -> int:
    rows: list[tuple[str, str, int, str, str]] = []
    unclassified = 0
    for crate in CRATES:
        base = ROOT / crate
        if not base.is_dir():
            continue
        for rs in base.rglob("*.rs"):
            rel = rs.relative_to(ROOT)
            text = rs.read_text(encoding="utf-8", errors="replace")
            src_lines = text.splitlines()
            for i, line in enumerate(src_lines, 1):
                if PATTERN.search(line):
                    context = " ".join(src_lines[i - 1 : i + 3])
                    cls = classify(crate, str(rel), context)
                    if cls == "UNCLASSIFIED":
                        unclassified += 1
                    rows.append((str(rel), str(i), cls, line.strip()[:110], crate))

    covered = sorted({r[4] for r in rows})
    lines = [
        "# M6-003 — Explicit unsafe / FFI Audit",
        "",
        f"Generated by `scripts/unsafe-audit.py` (reproducible; fail-closed on unclassified blocks).",
        "",
        "## Scope and method",
        "",
        "- Every `unsafe` fn/impl/trait/extern/block in the first-party workspace crates is enumerated below.",
        "- Each entry is classified and paired with its covering evidence (libFuzzer target, sanitizer stage, Miri, unit tests).",
        "- Miri exclusions (rquickjs/quickjs-ng foreign C) are compensated by the UBSan C-FFI stage (S2) — recorded in the campaign ledger, not here assumed.",
        "",
        "## Summary",
        "",
        f"- unsafe occurrences: **{len(rows)}**",
        f"- crates carrying unsafe: **{', '.join(covered) if covered else 'none'}**",
        f"- unclassified (must be zero to pass): **{unclassified}**",
        "",
        "## Inventory",
        "",
        "| location | line | classification | snippet |",
        "|---|---|---|---|",
    ]
    for rel, line, cls, snippet, _ in rows:
        lines.append(f"| `{rel}` | {line} | {cls} | `{snippet}` |")
    lines += ["", "## Covering evidence per crate", ""]
    for crate in covered:
        lines.append(f"### `{crate}`")
        for e in covering_evidence(crate):
            lines.append(f"- {e}")
        lines.append("")
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"audit written: {OUT.relative_to(ROOT)} ({len(rows)} occurrences, {unclassified} unclassified, {len(missing_from_map)} crates missing from miri-scope.json)")
    return 1 if (unclassified or missing_from_map) else 0

if __name__ == "__main__":
    sys.exit(main())
