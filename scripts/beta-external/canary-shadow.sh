#!/usr/bin/env bash
# #1419 — Canary Phase-0 synthetic-shadow driver (M7-006 machinery).
#
# Mirrors a request manifest to a BASELINE and a CANDIDATE service and
# validates response equivalence per CANARY_PROGRAM.md Phase 0: HTTP
# compliance, response equivalence, zero unhandled panics (any 5xx is a
# divergence unless the manifest declares it expected).
#
# Assignment-agnostic by design: which services it points at is the
# owner's canary-assignment decision (#1321 item 2); this harness only
# needs two base URLs and a manifest. Running it proves NOTHING about a
# canary by itself — Phase 0 evidence requires the owner-named
# baseline/candidate pair and its recorded transcript.
#
# Usage:
#   scripts/beta-external/canary-shadow.sh BASE_URL CAND_URL MANIFEST.json [passes]
#
# Manifest: [{"method":"GET","path":"/health/live","body":null,"headers":{}}, ...]
# Comparison: status (exact), content-type (normalized, exact), body
# (exact bytes after masking the RFC 9457 `instance` member — it is
# request-scoped by definition and can never match across two services).
# Manifests must otherwise avoid per-instance volatile fields (ids,
# timestamps); extend the mask if a workload needs more.
set -uo pipefail

BASE="${1:?usage: canary-shadow.sh BASE_URL CAND_URL MANIFEST.json [passes]}"
CAND="${2:?usage: canary-shadow.sh BASE_URL CAND_URL MANIFEST.json [passes]}"
MANIFEST="${3:?usage: canary-shadow.sh BASE_URL CAND_URL MANIFEST.json [passes]}"
PASSES="${4:-1}"

BUN="${BUN:-bun}"
command -v "$BUN" >/dev/null 2>&1 || { echo "canary-shadow: bun not found (set BUN=...)" >&2; exit 2; }
[ -f "$MANIFEST" ] || { echo "canary-shadow: manifest not found: $MANIFEST" >&2; exit 2; }

echo "canary-shadow: baseline=$BASE candidate=$CAND manifest=$MANIFEST passes=$PASSES"

"$BUN" -e '
const [base, cand, manifestPath, passesArg] = process.argv.slice(1);
const passes = Number(passesArg ?? "1");
const manifest = JSON.parse(await Bun.file(manifestPath).text());

// request-scoped members that can never match across services
const mask = (s) => s.replace(/"instance"\s*:\s*"[^"]*"/g, `"instance":"*"`);

const one = async (root, req) => {
  const res = await fetch(root + req.path, {
    method: req.method ?? "GET",
    headers: { ...(req.body ? { "content-type": "application/json" } : {}), ...(req.headers ?? {}) },
    body: req.body ? JSON.stringify(req.body) : undefined,
  });
  return { status: res.status, ctype: (res.headers.get("content-type") ?? "").split(";")[0], body: mask(await res.text()) };
};

let divergences = 0, unhandled5xx = 0, compared = 0;
for (let p = 1; p <= passes; p++) {
  for (const req of manifest) {
    const label = `${req.method ?? "GET"} ${req.path}`;
    let b, c;
    try {
      [b, c] = await Promise.all([one(base, req), one(cand, req)]);
    } catch (e) {
      console.error(`DIVERGENCE ${label}: transport error — ${e}`);
      divergences++; continue;
    }
    compared++;
    if (b.status >= 500 || c.status >= 500) {
      console.error(`DIVERGENCE ${label}: unhandled 5xx (baseline ${b.status}, candidate ${c.status})`);
      unhandled5xx++; divergences++; continue;
    }
    const fields = [];
    if (b.status !== c.status) fields.push(`status ${b.status} vs ${c.status}`);
    if (b.ctype !== c.ctype) fields.push(`ctype ${b.ctype} vs ${c.ctype}`);
    if (b.body !== c.body) fields.push(`body ${JSON.stringify(b.body.slice(0, 160))} vs ${JSON.stringify(c.body.slice(0, 160))}`);
    if (fields.length) {
      console.error(`DIVERGENCE ${label}: ${fields.join("; ")}`);
      divergences++;
    }
  }
}
console.log(`canary-shadow: ${compared} comparisons, ${divergences} divergences (${unhandled5xx} unhandled-5xx) over ${passes} pass(es)`);
process.exit(divergences === 0 && compared > 0 ? 0 : 1);
' "$BASE" "$CAND" "$MANIFEST" "$PASSES"
exit $?
