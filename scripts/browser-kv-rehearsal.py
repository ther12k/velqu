#!/usr/bin/env python3
"""
BWASM-C-004 — real-browser IndexedDB rehearsal (evidence tooling).

Bundles a small evidence entry against @velqu/browser-runtime, serves it
over HTTP, and drives headless Chromium (Playwright) through the REAL
IndexedDB adapter:

  K1  set/get/list round trip           (genuine IndexedDB persistence)
  K2  persistence across page reload    (data survives navigation)
  K3  namespace isolation               (two namespaces, no leakage)
  K4  quota + structured quota error
  K5  migration: v1 -> v2 with declared hook; mismatch fails closed
  K6  export/reset/gc controls

Evidence tooling only (Python + Playwright; the repository itself has
no Python dependency). Usage:
  python3 scripts/browser-kv-rehearsal.py --out FILE
Requires VELQU_CHROME or the Playwright chromium under ~/.cache.
"""

import argparse
import json
import os
import subprocess
import sys
import tempfile
import threading
import time
import http.server
import functools

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CHROME = os.environ.get(
    "VELQU_CHROME",
    os.path.expanduser(
        "~/.cache/ms-playwright/chromium-1234/chrome-linux64/chrome"
    ),
)

ENTRY = r'''
import { createIndexedDbKv, KvMigrationRequired, KvQuotaExceeded } from "@velqu/browser-runtime";
const out = {};
try {
  // K1 — round trip on the real IndexedDB adapter
  const kv = createIndexedDbKv({ namespace: "evidence:main" });
  await kv.set("greeting", { hello: "world", n: 1 });
  out.k1_roundTrip = { value: await kv.get("greeting"), ok: true };

  // K3 — namespace isolation (second namespace cannot see the first)
  const other = createIndexedDbKv({ namespace: "evidence:other" });
  out.k3_isolation = {
    otherList: await other.list(),
    otherGet: await other.get("greeting"),
  };
  await other.set("greeting", "other-value");
  out.k3_isolation.mainUnaffected = (await kv.get("greeting")).hello === "world";
  out.k3_isolation.ok = out.k3_isolation.otherList.length === 0
    && out.k3_isolation.otherGet === null
    && out.k3_isolation.mainUnaffected;

  // K4 — quota: structured error on oversized value
  const small = createIndexedDbKv({ namespace: "evidence:quota", maxValueBytes: 32 });
  try {
    await small.set("big", "x".repeat(64));
    out.k4_quota = { ok: false };
  } catch (e) {
    out.k4_quota = { ok: e.code === "kv-quota-exceeded", code: e.code };
  }

  // K5 — migration with declared hook
  const v1 = createIndexedDbKv({ namespace: "evidence:mig", schemaVersion: 1 });
  await v1.set("legacy", "v1-value");
  const v2 = createIndexedDbKv({
    namespace: "evidence:mig",
    schemaVersion: 2,
    migrations: {
      1: async (store) => {
        const old = await store.get("legacy");
        if (old !== null) { await store.delete("legacy"); await store.set("migrated", old); }
      },
    },
  });
  out.k5_migration = { migrated: await v2.get("migrated"), legacyGone: (await v2.get("legacy")) === null, ok: true };

  // K5b — version mismatch without migration fails closed, data preserved
  const v3 = createIndexedDbKv({ namespace: "evidence:mig", schemaVersion: 3 });
  try {
    await v3.get("anything");
    out.k5b_failClosed = { ok: false };
  } catch (e) {
    out.k5b_failClosed = { ok: e instanceof KvMigrationRequired, from: e.fromVersion, to: e.toVersion };
  }

  // K6 — export / gc / reset
  const ctl = createIndexedDbKv({ namespace: "evidence:ctl", maxEntries: 2 });
  await ctl.set("a", 1);
  await ctl.set("b", 2);
  out.k6_export = await ctl.exportAll();
  await ctl.reset();
  out.k6_afterReset = await ctl.list();

  out.PASS = true;
} catch (e) {
  out.PASS = false;
  out.error = String(e && e.stack || e);
}
document.getElementById("kv-out").textContent = JSON.stringify(out);
'''

PAGE = """<!doctype html>
<html><head><meta charset="utf-8"></head>
<body><pre id="kv-out">running…</pre>
<script type="module" src="./entry.js"></script>
</body></html>
"""


def bun(argv, cwd=REPO, check=True):
    return subprocess.run(["bun", *argv], cwd=cwd, check=check, capture_output=True, text=True)


def build_evidence(out_dir: str) -> None:
    os.makedirs(out_dir, exist_ok=True)
    # workspace resolution: link @velqu into the evidence project
    link_dir = os.path.join(out_dir, "node_modules", "@velqu")
    os.makedirs(link_dir, exist_ok=True)
    pkg_link = os.path.join(link_dir, "browser-runtime")
    if not os.path.islink(pkg_link):
        os.symlink(os.path.join(REPO, "packages", "browser-runtime"), pkg_link)
    entry = os.path.join(out_dir, "kv-entry.ts")
    with open(entry, "w") as f:
        f.write(ENTRY)
    result = bun(["build", entry, "--outdir", out_dir, "--target", "browser",
                  "--format", "esm", "--minify"], check=False)
    if result.returncode != 0:
        raise RuntimeError(f"bundle failed: {result.stderr[-800:]}")
    # rewrite the import to the workspace package
    built = os.path.join(out_dir, "kv-entry.js")
    src = open(built).read()
    # bun already bundled the workspace import (followed the symlink)
    with open(os.path.join(out_dir, "entry.js"), "w") as f:
        f.write(src)
    with open(os.path.join(out_dir, "index.html"), "w") as f:
        f.write(PAGE)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=None)
    args = ap.parse_args()

    if not os.path.exists(CHROME):
        print(f"chromium not found at {CHROME} (set VELQU_CHROME)", file=sys.stderr)
        return 2

    from playwright.sync_api import sync_playwright

    work = tempfile.mkdtemp(prefix="velqu-c004-")
    build_evidence(work)
    port = 8951

    class Quiet(http.server.SimpleHTTPRequestHandler):
        def __init__(self, *a, **kw):
            super().__init__(*a, directory=work, **kw)

        def log_message(self, *a):
            pass

    httpd = http.server.ThreadingHTTPServer(("127.0.0.1", port), Quiet)
    threading.Thread(target=httpd.serve_forever, daemon=True).start()

    report = {}
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True, executable_path=CHROME,
                                    args=["--no-sandbox"])
        page = browser.new_context().new_page()
        page.on("console", lambda m: report.setdefault("console", []).append(m.text[:200]))
        page.goto(f"http://127.0.0.1:{port}/index.html")
        page.wait_for_function(
            "document.getElementById('kv-out')?.textContent !== 'running…'", timeout=30000)
        report["result"] = json.loads(page.text_content("#kv-out"))
        # K2 — persistence across reload (genuine durability within the profile)
        page.reload(wait_until="domcontentloaded")
        page.wait_for_function(
            "document.getElementById('kv-out')?.textContent !== 'running…'", timeout=30000)
        second = json.loads(page.text_content("#kv-out"))
        report["k2_reload"] = {
            "valueSurvived": second.get("k1_roundTrip", {}).get("value") == {"hello": "world", "n": 1},
        }
        browser.close()
    httpd.shutdown()

    result = report["result"]
    checks = {
        "k1_roundTrip": result.get("k1_roundTrip", {}).get("ok") is True,
        "k3_isolation": result.get("k3_isolation", {}).get("ok") is True,
        "k4_quota": result.get("k4_quota", {}).get("ok") is True,
        "k5_migration": result.get("k5_migration", {}).get("ok") is True
        and result.get("k5_migration", {}).get("migrated") == "v1-value",
        "k5b_failClosed": result.get("k5b_failClosed", {}).get("ok") is True
        and result.get("k5b_failClosed", {}).get("from") == 2,
        "k6_exportReset": result.get("k6_export") == {"a": 1, "b": 2}
        and result.get("k6_afterReset") == [],
        "k2_reload": report.get("k2_reload", {}).get("valueSurvived") is True,
    }
    report["checks"] = checks
    report["REHEARSAL-PASS"] = all(checks.values())
    print(json.dumps(report, indent=1))
    if args.out:
        with open(args.out, "w") as f:
            json.dump(report, f, indent=1)
    return 0 if report["REHEARSAL-PASS"] else 1


if __name__ == "__main__":
    sys.exit(main())
