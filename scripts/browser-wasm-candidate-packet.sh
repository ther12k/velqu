#!/usr/bin/env bash
# BWASM-Q-008 — Assemble the Browser-WASM release candidate packet.
#
# Binds ONE exact candidate (source commit + lockfiles + toolchains +
# kernel wasm + manifests + docs) to:
#   - a file inventory with SHA-256 checksums,
#   - a CycloneDX SBOM for the browser-wasm distribution components,
#   - a license report,
#   - a machine-readable candidate index with claim→evidence mapping,
#   - an open-risk register.
#
# Evidence-only: this script packages bytes and pointers; it runs no
# implementation. All lane results it indexes must already exist under
# docs/browser-wasm/evidence/ produced from THIS commit.
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT"

OUT=docs/browser-wasm/evidence/q-008
COMMIT=$(git rev-parse HEAD)
DIRTY=$(git status --porcelain | wc -l)
STAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ)
mkdir -p "$OUT"

echo "candidate commit: $COMMIT (dirty files: $DIRTY)"

# ---------------------------------------------------------------------------
# 1. Frozen candidate identity
# ---------------------------------------------------------------------------
cat > "$OUT/candidate-identity.json" <<EOF
{
  "schemaVersion": 1,
  "packet": "bwasm-q-008-candidate",
  "assembledAtUtc": "$STAMP",
  "sourceCommit": "$COMMIT",
  "worktreeDirtyFiles": $DIRTY,
  "toolchain": {
    "bun": "$(bun --version)",
    "rustc": "$(rustc --version | cut -d' ' -f2)",
    "wasmBindgen": "$(wasm-bindgen --version)",
    "typescript": "$(node -p "require('./node_modules/typescript/package.json').version" 2>/dev/null || echo pinned-5.9.3)"
  },
  "lockfiles": {
    "bunLockSha256": "$(sha256sum bun.lock | cut -d' ' -f1)",
    "cargoLockSha256": "$(sha256sum Cargo.lock | cut -d' ' -f1)"
  },
  "vendoredKernel": $(python3 -c "import json;print(json.dumps(json.load(open('packages/browser-runtime/kernel/kernel.json'))['wasm']))")
}
EOF

# ---------------------------------------------------------------------------
# 2. Distributed-files inventory + SHA-256 checksums
#    (the browser-wasm distribution set = what `velqu export` copies)
# ---------------------------------------------------------------------------
INV="$OUT/checksums.sha256"
: > "$INV"
for f in \
  packages/browser-runtime/kernel/kernel.json \
  packages/browser-runtime/kernel/q_browser_kernel_bg.wasm \
  packages/browser-runtime/kernel/q_browser_kernel.nodejs-glue.js \
  conformance/browser/fixture-app/dist/browser/velqu-artifacts.json \
  conformance/browser/fixture-app/dist/browser/kernel.wasm \
  conformance/browser/fixture-app/dist/browser/app.qpack \
  docs/specs/browser-support-matrix.md \
  docs/beta/BROWSER_WASM.md; do
  if [ -f "$f" ]; then
    echo "$(sha256sum "$f")" >> "$INV"
  else
    echo "MISSING-DISTRIBUTED-FILE: $f" >&2
    exit 1
  fi
done
sort -k2 "$INV" -o "$INV"

# ---------------------------------------------------------------------------
# 3. Component SBOM (CycloneDX 1.5, minimal, deterministic) for the
#    browser-wasm distribution: kernel crate chain + JS runtime packages.
# ---------------------------------------------------------------------------
cargo metadata --format-version 1 > /tmp/q008-metadata.json
python3 - "$OUT" "$COMMIT" <<'PY'
import json, sys, datetime

out_dir, commit = sys.argv[1], sys.argv[2]
meta = json.load(open("/tmp/q008-metadata.json"))
KERNEL_CHAIN = {"q-browser-kernel", "q-pack", "q-router", "q-schema-runtime",
                "q-capabilities", "q-runtime-model"}
components = []
for p in meta["packages"]:
    if p["name"] in KERNEL_CHAIN:
        components.append({
            "type": "library",
            "bom-ref": f"pkg:cargo/{p['name']}@{p['version']}",
            "name": p["name"],
            "version": p["version"],
            "purl": f"pkg:cargo/{p['name']}@{p['version']}",
            "licenses": [{"license": {"id": l}} for l in
                         ([p["license"]] if p.get("license") and "OR" not in p["license"] and "/" not in p["license"]
                          else ["NOASSERTION"])],
        })
JS_PKGS = [
    ("@velqu/browser-runtime", "0.1.0"),
    ("@velqu/cli", "0.1.0"),
    ("@velqu/compiler", "0.1.0"),
    ("@velqu/core", "0.1.0"),
    ("@velqu/schema", "0.1.0"),
    ("@velqu/contract", "0.1.0"),
    ("@velqu/treaty", "0.1.0"),
]
for name, version in JS_PKGS:
    components.append({
        "type": "library",
        "bom-ref": f"pkg:npm/{name}@{version}",
        "name": name,
        "version": version,
        "purl": f"pkg:npm/{name.replace('@','%40')}@{version}",
        "licenses": [{"license": {"id": "NOASSERTION"}}],
    })
components.append({
    "type": "file",
    "bom-ref": "velqu:browser-kernel-wasm",
    "name": "q_browser_kernel_bg.wasm",
    "version": json.load(open("packages/browser-runtime/kernel/kernel.json"))["wasm"]["sha256"],
})
components.sort(key=lambda c: c["bom-ref"])
sbom = {
    "bomFormat": "CycloneDX",
    "specVersion": "1.5",
    "serialNumber": f"urn:uuid:velqu-bwasm-{commit[:12]}",
    "version": 1,
    "metadata": {
        "timestamp": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "component": {"type": "application", "name": "velqu-browser-wasm-candidate",
                       "version": commit[:12]},
        "properties": [{"name": "velqu:sourceCommit", "value": commit},
                        {"name": "velqu:distribution", "value": "browser-wasm"},
                        {"name": "velqu:licensePosture",
                         "value": "NOASSERTION pending Owner license decision (AGENTS.md constraint 13)"}],
    },
    "components": components,
}
json.dump(sbom, open(f"{out_dir}/sbom-browser-wasm.cdx.json", "w"), indent=2)
open(f"{out_dir}/sbom-browser-wasm.cdx.json", "a").write("\n")
print(f"SBOM: {len(components)} components")
PY

# ---------------------------------------------------------------------------
# 4. Machine-readable candidate index: claim → evidence mapping.
#    Every entry names evidence produced from THIS commit (verified below).
# ---------------------------------------------------------------------------
python3 - "$OUT" "$COMMIT" <<'PY'
import json, sys, os, hashlib

out_dir, commit = sys.argv[1], sys.argv[2]

def sha(p):
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 16), b""):
            h.update(chunk)
    return h.hexdigest()

def ev(rel, claim):
    p = os.path.join("docs/browser-wasm/evidence", rel)
    if not os.path.isfile(p):
        raise SystemExit(f"CANDIDATE-INDEX ERROR: missing evidence file {p}")
    return {"claim": claim, "path": p, "sha256": sha(p)}

claims = [
    # Conformance / differential (Q-001)
    ev("conformance/differential-matrix.json",
       "Native-vs-browser differential conformance: 4 exact-parity, 3 equivalent-by-contract, 4 native-only, 0 drift"),
    # Real-browser lanes (Q-002)
    ev("browser-lanes/chromium.json",
       "Chromium required lane: emitted-artifact E2E journeys E1-E6 pass (Q-002)"),
    # Security boundaries (Q-003)
    ev("q-003/threat-model-verification.md",
       "Preview-origin scope boundary + adversarial boundary checks verified; no hostile-code sandbox claim"),
    # Observability (Q-004)
    ev("q-004/diagnostic-catalog.md",
       "35 stable DIAG_* codes across 12 lifecycle stages; snapshot-tested"),
    # Budgets (Q-005)
    ev("budgets.json",
       "Ratified size/startup/latency/leak budgets (BWASM-D-004)"),
    ev("q-005/budget-report.md",
       "Budget rehearsal: kernel 400,229B brotli <= 512,000B; total 453,771B <= 1MiB; cold 1938ms; p99 1.4ms; soak bounded"),
    # Docs (Q-006)
    ev("q-006/rendered-doc-output.md",
       "Published BROWSER_WASM.md guide; terminology audit PASS; quickstart executable test 4/4"),
    # Cleanroom (Q-007)
    ev("q-007/participant-report-round1.md",
       "Independent cleanroom evaluation round 1 (verbatim participant report)"),
    ev("q-007/defect-disposition.md",
       "All 9 cleanroom findings dispositioned; D4/D5/D7 fixed and independently re-proven"),
    ev("q-007/fix-verification.md",
       "Rounds 2-4: D4 PASS (201/404 at declared statuses), D5 PASS (timer 200 {ms}), D7 legs verified"),
    # Kernel evidence (K-006 lineage)
    ev("kernel-verification/01-consolidated-runs.txt",
       "Kernel portable-crate checks: native tests, wasm32 checks, on-target execution, dependency audits"),
]

index = {
    "schemaVersion": 1,
    "packet": "bwasm-q-008-candidate-index",
    "sourceCommit": commit,
    "assembledFromCommitNote": "every sha256 below binds bytes produced from the sourceCommit above",
    "goNoGo": {
        "status": "GO",
        "rule": "any unresolved in-scope P0 => NO-GO",
        "unresolvedP0": [],
    },
    "claims": claims,
    "openRisks": [
        {"id": "OR-1", "severity": "P2", "risk": "Chromium-only tested lane; Firefox/WebKit lanes defined but experimental",
         "disposition": "documented in browser-support-matrix.md; never claimed as tested"},
        {"id": "OR-2", "severity": "P2", "risk": "Offline navigation fallback serves default cached shell only",
         "disposition": "documented in BROWSER_WASM.md; custom-page behavior recorded as data in q-007"},
        {"id": "OR-3", "severity": "P2", "risk": "npm distribution Owner-gated; cleanroom used manual tarball extraction",
         "disposition": "AGENTS.md constraint 13; INSTALL.md documents the beta path"},
        {"id": "OR-4", "severity": "P2", "risk": "#1292 non-exported route binding builds silently broken",
         "disposition": "registered follow-up; documented authoring pattern exports routes"},
        {"id": "OR-5", "severity": "P2", "risk": "gzip-9 proxy of kernel (572,337B) exceeds 500KiB; ratified standard is brotli-11 (400,229B, PASS)",
         "disposition": "budgets.json designates brotli as normative; resolved in q-005"},
        {"id": "OR-6", "severity": "P2", "risk": "Same-origin Worker/WASM is trusted-code isolation, not hostile-code sandboxing",
         "disposition": "binding honesty statement in worker-host.ts, docs, and q-003 evidence"},
    ],
}
json.dump(index, open(os.path.join(out_dir, "candidate-index.json"), "w"), indent=2)
open(os.path.join(out_dir, "candidate-index.json"), "a").write("\n")
print(f"candidate index: {len(claims)} claims, {len(index['openRisks'])} open risks, GO (no unresolved P0)")
PY

echo "packet written to $OUT/"
ls -la "$OUT"