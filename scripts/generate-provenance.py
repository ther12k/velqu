#!/usr/bin/env python3
"""Generate or verify the release provenance statement (M6-005).

The statement records, for a set of release artifacts: the source commit,
the builder environment identity, the exact toolchain versions, the build
parameters that affect output bytes, every artifact's SHA-256, and the
disclosed non-reproducible inputs. Layout is in-toto/SLSA-inspired but is
a Velqu-defined schema (velqu-provenance-v1); it does not claim SLSA
attestation level, isolated build provenance, or independent-builder
reproducibility — that is M6-006 and remains unclaimed here.

Modes:
  generate (default)  scripts/generate-provenance.py --out release/provenance.json
                      [--artifacts release] [--commit <sha>]
  verify              scripts/generate-provenance.py --verify release/provenance.json
                      re-hashes every listed artifact, checks the commit against
                      SOURCE-COMMIT.txt when present, and requires every
                      declared field to be present and finite/typed.

The generator refuses a dirty tree (same rule as scripts/release-packet):
provenance for a dirty tree would misdescribe the source.
"""
import argparse
import hashlib
import json
import os
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCHEMA = "velqu-provenance-v1"

NON_REPRODUCIBLE_INPUTS = [
    "generatedAt (wall-clock timestamp of the statement itself; not embedded in any artifact except the packet indexes, which disclose their own generatedAt)",
    "REVIEW_INDEX.json/EVIDENCE_INDEX.json generatedAt fields inside the release packet (recorded, timestamped metadata — not build outputs)",
    "git bundle/zip embed creation order and timestamps of the creating process",
    "absolute build paths ARE sanitized: RUSTFLAGS --remap-path-prefix and CFLAGS -ffile-prefix-map/-fdebug-prefix-map rewrite them to /velqu-src (see REBUILD.md); any residue would break compare-builds",
]


def run(cmd, cwd=None):
    return subprocess.run(cmd, capture_output=True, text=True, cwd=cwd or ROOT, check=False)


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def collect_toolchain():
    def v(cmd):
        r = run(cmd)
        return r.stdout.strip() if r.returncode == 0 else f"UNAVAILABLE({cmd[0]})"
    toolchain = {
        "rustc": v(["rustc", "--version"]),
        "cargo": v(["cargo", "--version"]),
        "bun": v(["bun", "--version"]),
        "git": v(["git", "--version"]),
        "gpg": v(["gpg", "--version"]).splitlines()[0] if run(["gpg", "--version"]).returncode == 0 else "UNAVAILABLE(gpg)",
    }
    # Pinned toolchain file, when present, is normative — record it verbatim.
    pinned = ROOT / "rust-toolchain.toml"
    toolchain["rustToolchainFile"] = pinned.read_text().strip() if pinned.exists() else "ABSENT"
    # Pinned engine + binding from the lock (normative per AGENTS.md constraint 1).
    lock = (ROOT / "Cargo.lock").read_text()
    engine = binding = None
    blocks = lock.split("[[package]]")
    for b in blocks:
        if '\nname = "quickjs-ng"' in b or b.startswith('name = "quickjs-ng"'):
            for line in b.splitlines():
                if line.startswith("version = "):
                    engine = line.split('"')[1]
        if 'name = "rquickjs-core"' in b or b.startswith('name = "rquickjs-core"'):
            for line in b.splitlines():
                if line.startswith("version = "):
                    binding = line.split('"')[1]
    toolchain["pinnedEngine"] = {
        "engine": f"quickjs-ng {engine}" if engine else "quickjs-ng (version unresolved)",
        "binding": f"rquickjs-core {binding}" if binding else "rquickjs-core (version unresolved)",
    }
    return toolchain


def collect_builder():
    uname = run(["uname", "-srm"])
    return {
        "platform": uname.stdout.strip() if uname.returncode == 0 else "UNKNOWN",
        "hostnameDisclosed": False,
        "builderType": "single documented host build (no isolation claim; independent-builder reproducibility is M6-006 and is NOT claimed by this statement)",
        "user": None,
    }


def collect_parameters():
    return {
        "profile": "release",
        "target": "host (rustc default target)",
        "RUSTFLAGS": os.environ.get("RUSTFLAGS", ""),
        "CFLAGS": os.environ.get("CFLAGS", ""),
        "remapNote": "release builds must carry --remap-path-prefix=$(pwd)=/velqu-src and C prefix/debug maps (scripts/verify, REBUILD.md); recorded values are the ambient values of THIS invocation",
        "frozenLockfiles": True,
    }


def generate(args):
    status = run(["git", "status", "--porcelain"])
    if status.stdout.strip():
        print("generate-provenance: working tree is dirty — provenance would "
              "misdescribe the source; refusing", file=sys.stderr)
        return 2
    commit = args.commit or run(["git", "rev-parse", "HEAD"]).stdout.strip()

    art_dir = ROOT / args.artifacts
    if not art_dir.is_dir():
        print(f"generate-provenance: artifact directory not found: {art_dir}",
              file=sys.stderr)
        return 2
    artifacts = {}
    for p in sorted(art_dir.rglob("*")):
        rel = p.relative_to(art_dir).as_posix()
        if p.is_file() and rel not in ("provenance.json", "provenance.json.asc"):
            artifacts[rel] = sha256_file(p)

    statement = {
        "schema": SCHEMA,
        "generatedAt": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "source": {
            "commit": commit,
            "treeStatus": "clean",
            "archive": None,
        },
        "builder": collect_builder(),
        "toolchain": collect_toolchain(),
        "parameters": collect_parameters(),
        "artifacts": artifacts,
        "nonReproducibleInputs": NON_REPRODUCIBLE_INPUTS,
        "verification": {
            "rebuildDoc": "docs/production/REBUILD.md",
            "selfCheck": "scripts/generate-provenance.py --verify",
            "checksumManifest": "SHA256SUMS.txt in the same directory (when produced by scripts/release-packet)",
        },
        "claimsNotMade": [
            "SLSA attestation level",
            "isolated or hermetic build",
            "independent-builder reproducibility (M6-006)",
            "artifact signing (out-of-band, --sign / M8-003)",
        ],
    }
    # Source-commit identity of the packet, when generated inside one.
    src_txt = art_dir / "SOURCE-COMMIT.txt"
    if src_txt.is_file():
        statement["source"]["archive"] = f"release packet bound to {src_txt.read_text().strip()}"

    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(statement, indent=1, sort_keys=True) + "\n")
    print(f"provenance: {out} ({len(artifacts)} artifacts, commit {commit[:12]})")
    return 0


def verify(args):
    path = Path(args.path)
    if not path.is_file():
        print(f"verify-provenance: not found: {path}", file=sys.stderr)
        return 2
    try:
        st = json.loads(path.read_text())
    except json.JSONDecodeError as exc:
        print(f"verify-provenance: invalid JSON: {exc}", file=sys.stderr)
        return 2
    problems = []
    if st.get("schema") != SCHEMA:
        problems.append(f"schema {st.get('schema')!r} != {SCHEMA!r}")
    for section in ("source", "builder", "toolchain", "parameters", "artifacts",
                    "nonReproducibleInputs"):
        if not isinstance(st.get(section), dict if section != "nonReproducibleInputs" else list) \
                or not st.get(section):
            problems.append(f"missing/empty {section}")
    commit = (st.get("source") or {}).get("commit", "")
    if len(commit) != 40 or any(c not in "0123456789abcdef" for c in commit):
        problems.append("source.commit is not a 40-hex sha")
    src_txt = path.parent / "SOURCE-COMMIT.txt"
    if src_txt.is_file() and src_txt.read_text().strip() != commit:
        problems.append("source.commit does not match SOURCE-COMMIT.txt")
    toolchain = st.get("toolchain") or {}
    for key in ("rustc", "cargo", "bun"):
        val = toolchain.get(key, "")
        if not val or val.startswith("UNAVAILABLE"):
            problems.append(f"toolchain.{key} unavailable at generation time")
    bad_hashes = []
    base = path.parent
    for rel, digest in (st.get("artifacts") or {}).items():
        f = base / rel
        if not f.is_file():
            bad_hashes.append(f"{rel}: missing")
        elif sha256_file(f) != digest:
            bad_hashes.append(f"{rel}: hash mismatch")
    problems.extend(bad_hashes)
    if problems:
        print("verify-provenance: FAIL")
        for p in problems:
            print(f"  - {p}")
        return 1
    n = len(st.get("artifacts") or {})
    print(f"verify-provenance: PASS ({n} artifact hashes recomputed, commit {commit[:12]}, "
          "toolchain fields present)")
    return 0


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", default="release/provenance.json")
    ap.add_argument("--artifacts", default="release")
    ap.add_argument("--commit")
    ap.add_argument("--verify", metavar="PATH", dest="verify_path")
    args = ap.parse_args()
    if args.verify_path:
        return verify(argparse.Namespace(path=args.verify_path))
    return generate(args)


if __name__ == "__main__":
    sys.exit(main())
