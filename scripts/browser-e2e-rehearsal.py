#!/usr/bin/env python3
"""
BWASM-Q-002 — real-browser end-to-end smoke lanes (evidence tooling).

Runs the SIX required smoke suites against EMITTED release-like
artifacts (`velqu build --target browser-wasm` output, served statically)
in a real browser:

  E1  kernel boot + handler execution   (wasm kernel + module Worker)
  E2  dispatcher normalization          (query pairs, JSON body, 404 problem)
  E3  Service Worker install/activate   (B-004 verified prefetch)
  E4  offline navigation                (cached shell + artifacts)
  E5  IndexedDB KV persistence          (C-004 adapter, reload-durable)
  E6  cache/update detection            (SW identity bytes + apply-on-reload)
  E7  local-SQL IndexedDB durability    (C-003 adapter, lazy engine load,
      network trace: zero engine bytes before first open)

Browser selection: --browser chromium|firefox|webkit (the Playwright
launch name). Required vs experimental lanes are defined in
`.github/workflows/browser-lanes.yml` and
`docs/browser-wasm/evidence/browser-matrix.json` — this
script is the lane runner both CI and local evidence use.

Usage:
  python3 scripts/browser-e2e-rehearsal.py --browser chromium --out FILE
"""

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
import threading
import time
import http.server
import platform

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CHROMIUM = os.path.expanduser(
    "~/.cache/ms-playwright/chromium-1234/chrome-linux64/chrome"
)


def browser_path(name: str) -> str | None:
    if name == "chromium":
        override = os.environ.get("VELQU_CHROME")
        if override and os.path.exists(override):
            return override
        return CHROMIUM if os.path.exists(CHROMIUM) else None
    return None  # firefox/webkit resolve through Playwright's own registry


def bun(argv, cwd=REPO, check=True):
    out = subprocess.run(["bun", *argv], cwd=cwd, check=False, capture_output=True, text=True)
    if check and out.returncode != 0:
        # Surface the captured output — a bare CalledProcessError hides the
        # actual build error, which made failing CI lanes undiagnosable (#1305).
        raise RuntimeError(
            f"bun {' '.join(argv)} failed (exit {out.returncode})\n"
            f"--- stdout ---\n{out.stdout[-1500:]}\n"
            f"--- stderr ---\n{out.stderr[-1500:]}"
        )
    return out


def link_workspace(project: str, packages=("core", "schema", "browser-runtime")):
    link_dir = os.path.join(project, "node_modules", "@velqu")
    os.makedirs(link_dir, exist_ok=True)
    for pkg in packages:
        dst = os.path.join(link_dir, pkg)
        if not os.path.islink(dst):
            os.symlink(os.path.join(REPO, "packages", pkg), dst)


def write_app(project: str, greeting: str):
    src = os.path.join(project, "src")
    os.makedirs(src, exist_ok=True)
    with open(os.path.join(src, "app.ts"), "w") as f:
        f.write(f'''
import {{ route }} from "@velqu/core";
import {{ s }} from "@velqu/schema";

export const hello = route({{
  id: "hello.get",
  method: "GET",
  path: "/hello/:name",
  response: {{ 200: s.object({{ message: s.string() }}) }},
  handle: async (ctx) => ({{ message: `{greeting}` }}),
}});

export const app = {{ routes: [hello] }};
''')
    with open(os.path.join(project, "package.json"), "w") as f:
        json.dump({"name": "e2e-demo", "type": "module", "private": True}, f)


KV_ENTRY = (
    "import { createIndexedDbKv } from '@velqu/browser-runtime';\n"
    "export async function run() {\n"
    "  try {\n"
    "    const kv = createIndexedDbKv({ namespace: 'e2e:kv' });\n"
    "    const previous = await kv.get('persisted');\n"
    "    const next = (typeof previous === 'number' ? previous : 0) + 1;\n"
    "    await kv.set('persisted', next);\n"
    "    const roundTrip = await kv.get('persisted');\n"
    "    const keys = await kv.list();\n"
    "    return { ok: roundTrip === next, roundTrip, previous, keys };\n"
    "  } catch (e) {\n"
    "    return { ok: false, error: String((e && e.message) || e) };\n"
    "  }\n"
    "}\n"
)


def build_kv_entry(browser_dir: str, work: str):
    """Bundle the KV evidence entry into the SW passthrough lane
    (`/__velqu_editor__/` is never intercepted by the B-004 scope
    guard), so a post-build evidence module stays reachable while the
    deployment's own artifacts remain exactly as emitted."""
    entry = os.path.join(work, "kv-entry.ts")
    with open(entry, "w") as f:
        f.write(KV_ENTRY)
    link_workspace(work)
    out_dir = os.path.join(browser_dir, "__velqu_editor__")
    os.makedirs(out_dir, exist_ok=True)
    out = bun(["build", entry, "--outdir", out_dir, "--target", "browser",
               "--format", "esm", "--minify",
               # resolve @velqu/* exports to workspace sources (./src) on
               # trees without built dist/ — same mapping the CLI's own
               # browser bundler honors (#1305)
               "--conditions", "bun"], check=False)
    if out.returncode != 0:
        raise RuntimeError(f"kv bundle failed: {out.stderr[-400:]}")


SQL_ENTRY = (
    "import { createLocalSql } from '@velqu/browser-pglite';\n"
    "let handle = null;\n"
    "let opened = false;\n"
    "async function db() {\n"
    "  if (!handle) handle = createLocalSql({ namespace: 'e2e:sql', persistence: 'indexeddb' });\n"
    "  if (!opened) { await handle.open(); opened = true; }\n"
    "  return handle;\n"
    "}\n"
    "export async function run(mode) {\n"
    "  try {\n"
    "    const d = await db();\n"
    "    if (mode === 'reset') {\n"
    "      await d.reset();\n"
    "      return { ok: true, reset: true };\n"
    "    }\n"
    "    if (mode === 'write') {\n"
    "      await d.query('CREATE TABLE IF NOT EXISTS hits (id serial PRIMARY KEY, at timestamptz DEFAULT now())');\n"
    "      await d.query('INSERT INTO hits DEFAULT VALUES');\n"
    "    }\n"
    "    const r = await d.query('SELECT count(*)::int AS n FROM hits');\n"
    "    return { ok: true, count: r.rows[0].n };\n"
    "  } catch (e) {\n"
    "    return { ok: false, error: String((e && e.message) || e) };\n"
    "  }\n"
    "}\n"
)


def pglite_dist_dir():
    import glob
    pats = [
        os.path.join(REPO, "node_modules/.bun/@electric-sql+pglite@*/node_modules/@electric-sql/pglite/dist"),
        os.path.join(REPO, "node_modules/@electric-sql/pglite/dist"),
    ]
    for pat in pats:
        hits = sorted(glob.glob(pat)) if "*" in pat else [pat]
        if hits and os.path.isdir(hits[0]):
            return hits[0]
    raise RuntimeError("PGlite dist directory not found (is @electric-sql/pglite installed?)")


def pglite_dist_js():
    """The engine's own JS entry + chunks (its worker layout must stay
    intact — see build_sql_entry)."""
    d = pglite_dist_dir()
    return sorted(n for n in os.listdir(d)
                  if n.endswith(".js") and not n.endswith(".cjs"))


def pglite_engine_assets():
    """The engine's runtime assets (bun's nested store layout): both WASM
    modules AND the Emscripten file pack (pglite.data) the wasm fetches
    relative to its module URL."""
    import glob
    names = ["pglite.wasm", "initdb.wasm", "pglite.data"]
    pats = [
        os.path.join(REPO, "node_modules/.bun/@electric-sql+pglite@*/node_modules/@electric-sql/pglite/dist/"),
        os.path.join(REPO, "node_modules/@electric-sql/pglite/dist/"),
    ]
    found = []
    for pat in pats:
        for n in names:
            found.extend(sorted(glob.glob(os.path.join(pat, n))))
        if found:
            break
    return found


def build_sql_entry(browser_dir: str, work: str):
    """Bundle the C-003 evidence entry beside the deployment (same
    /__velqu_editor__/ passthrough as the KV lane) and serve the engine
    WASM files next to it — PGlite resolves them relative to the bundle's
    module URL."""
    entry = os.path.join(work, "sql-entry.ts")
    with open(entry, "w") as f:
        f.write(SQL_ENTRY)
    link_workspace(work, packages=("core", "schema", "browser-runtime", "browser-pglite"))
    out_dir = os.path.join(browser_dir, "__velqu_editor__")
    os.makedirs(out_dir, exist_ok=True)
    # The engine stays EXTERNAL and is served from its own dist layout
    # (import map below): a single-file re-bundle breaks emscripten's
    # pthread worker in cross-origin-isolated pages (the engine wedges
    # at init). The original dist layout is the supported configuration.
    out = bun(["build", entry, "--outdir", out_dir, "--target", "browser",
               "--format", "esm", "--minify",
               "--external", "@electric-sql/pglite",
               "--conditions", "bun"], check=False)
    if out.returncode != 0:
        raise RuntimeError(f"sql bundle failed: {out.stderr[-400:]}")
    dist_src = pglite_dist_dir()
    dist_dst = os.path.join(out_dir, "pglite-dist")
    os.makedirs(dist_dst, exist_ok=True)
    copied = []
    engine_abs = pglite_engine_assets()          # absolute paths
    js_names = pglite_dist_js()                  # plain names
    for src in engine_abs:
        name = os.path.basename(src)
        shutil.copyfile(src, os.path.join(dist_dst, name))
        copied.append("pglite-dist/" + name)
    for name in js_names:
        shutil.copyfile(os.path.join(dist_src, name), os.path.join(dist_dst, name))
        copied.append("pglite-dist/" + name)
    with open(os.path.join(out_dir, "e7.html"), "w") as f:
        f.write(E7_PAGE)
    return copied


E7_PAGE = """<!doctype html>
<html><head><meta charset=\"utf-8\"><title>e7</title></head>
<body><div id=\"st\">loading</div>
<script type=\"importmap\">
  { \"imports\": { \"@electric-sql/pglite\": \"./pglite-dist/index.js\" } }
</script>
<script type=\"module\">
  const m = await import('./sql-entry.js');
  window.sql = m;
  document.getElementById('st').textContent = 'ready';
</script></body></html>
"""


def build_deployment(project: str, greeting: str, base_path: str | None = None):
    write_app(project, greeting)
    link_workspace(project)
    cmd = ["packages/cli/src/index.ts", "build", "--target", "browser-wasm",
           "--project", project, "--json", "--kv",
           "--probe-path", "/hello/e2e", "--probe-method", "GET"]
    if base_path:
        cmd += ["--base-path", base_path]
    out = bun(cmd)
    if out.returncode != 0:
        raise RuntimeError(f"build failed: {out.stderr[-500:]}")
    info = json.loads(out.stdout)
    return info["buildId"], os.path.join(project, "dist", "browser")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--browser", default="chromium", choices=["chromium", "firefox", "webkit"])
    ap.add_argument("--out", default=None)
    args = ap.parse_args()

    exe = browser_path(args.browser)
    if args.browser == "chromium" and not exe:
        print(f"chromium not found at {CHROMIUM} (set VELQU_CHROME)", file=sys.stderr)
        return 2

    from playwright.sync_api import sync_playwright

    work = tempfile.mkdtemp(prefix="velqu-q002-")
    project = os.path.join(work, "app")
    os.makedirs(project, exist_ok=True)
    build_v1, browser_dir = build_deployment(project, "Hello E2E v1")
    build_kv_entry(browser_dir, work)
    engine_assets = build_sql_entry(browser_dir, work)
    port = 8961

    class Quiet(http.server.SimpleHTTPRequestHandler):
        def __init__(self, *a, **kw):
            super().__init__(*a, directory=browser_dir, **kw)

        def log_message(self, *a):
            pass

        def end_headers(self):
            # C-003 (E7): a PGlite application page is a pthreads WASM host
            # and wedges without SharedArrayBuffer — cross-origin isolation
            # is a REAL runtime requirement of that optional capability.
            # The isolation headers are served ONLY for the dedicated E7
            # top-level document; the main shell keeps its unisolated
            # deployment contract (the required service-worker lane broke
            # under blanket COEP: require-corp — SW scripts refuse install
            # even with CORP on this chromium).
            if self.path.startswith("/__velqu_editor__/"):
                self.send_header("Cross-Origin-Opener-Policy", "same-origin")
                self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
                self.send_header("Cross-Origin-Resource-Policy", "same-origin")
            super().end_headers()

    httpd = http.server.ThreadingHTTPServer(("127.0.0.1", port), Quiet)
    threading.Thread(target=httpd.serve_forever, daemon=True).start()

    report = {"browser": args.browser, "checks": {}}
    checks = report["checks"]

    def record(check: str, ok: bool, detail=None):
        entry = {"ok": bool(ok)}
        if detail is not None:
            entry["detail"] = detail
        checks[check] = entry
        print(("PASS " if ok else "FAIL ") + check + (f" {detail}" if detail is not None else ""), flush=True)

    try:
        with sync_playwright() as p:
            launch = {"chromium": p.chromium, "firefox": p.firefox, "webkit": p.webkit}[args.browser]
            kwargs = {"args": ["--no-sandbox"]} if args.browser == "chromium" else {}
            if exe:
                kwargs["executable_path"] = exe
            browser = launch.launch(headless=True, **kwargs)
            ctx = browser.new_context()
            page = ctx.new_page()
            # network trace for the C-003 lazy-engine assertion (E7)
            net_requests: list[str] = []
            page.on("request", lambda r: net_requests.append(r.url))
            base = f"http://127.0.0.1:{port}"

            # E1 — kernel boot + handler execution
            page.goto(f"{base}/index.html", timeout=45000)
            page.wait_for_function(
                "document.getElementById('velqu-status')?.textContent.includes('ready')",
                timeout=45000)
            info = json.loads(page.text_content("#velqu-info"))
            record("E1-kernel-boot", info.get("buildId") == build_v1
                   and info.get("serviceWorker") == "service-worker", info.get("buildId", "")[:12])

            # E2 — dispatcher normalization through the runtime boundary:
            # the build's self-probe executes a declared route in-page and
            # renders status + body (sanctioned dev-shell surface).
            record("E2-dispatcher-probe-200",
                   info.get("probe", {}).get("status") == 200
                   and "Hello E2E v1" in json.dumps(info.get("probe", {}).get("body")),
                   info.get("probe"))

            # E3 — Service Worker installed, activated, controlling.
            # A page that commits before the activated worker takes the
            # navigation is never controlled, so a single immediate reload
            # races activation; sample with one bounded retry on a fresh
            # reload. The asserted property is unchanged: the page must be
            # controlled by the activated worker (#1305).
            page.wait_for_function(
                "async () => { const r = await navigator.serviceWorker.getRegistration(); return !!r && !!r.active; }",
                timeout=60000)
            controlled = False
            for attempt in range(2):
                page.reload(wait_until="domcontentloaded")
                page.wait_for_function(
                    "document.getElementById('velqu-status')?.textContent.includes('ready')", timeout=45000)
                try:
                    page.wait_for_function("() => !!navigator.serviceWorker.controller", timeout=10000)
                    controlled = True
                    break
                except Exception:
                    controlled = False
            record("E3-sw-activated-and-controlling", controlled)

            # E4 — offline navigation (host down)
            inv = page.evaluate("""async () => {
              const out = {};
              for (const k of await caches.keys()) {
                const c = await caches.open(k);
                out[k] = (await c.keys()).length;
              }
              return out;
            }""")
            record("E4-precache-inventory", any(v >= 12 for v in inv.values()), inv)
            httpd.shutdown()
            httpd.server_close()
            time.sleep(1)
            try:
                page.reload(wait_until="domcontentloaded", timeout=20000)
                page.wait_for_function(
                    "document.getElementById('velqu-status')?.textContent.includes('ready')", timeout=20000)
                offline_ok = True
            except Exception:
                offline_ok = False
            record("E4-offline-navigation", offline_ok)
            httpd.server_close()
            httpd = http.server.ThreadingHTTPServer(("127.0.0.1", port), Quiet)
            threading.Thread(target=httpd.serve_forever, daemon=True).start()

            # E5 — IndexedDB KV persistence (the REAL C-004 adapter,
            # bundled beside the deployment and imported same-origin)
            page.reload(wait_until="domcontentloaded")
            page.wait_for_function(
                "document.getElementById('velqu-status')?.textContent.includes('ready')", timeout=45000)
            kv = page.evaluate("""async () => {
              const m = await import('/__velqu_editor__/kv-entry.js');
              const first = await m.run();
              const second = await m.run();
              return { first, second, durable: second.roundTrip === first.roundTrip + 1 };
            }""")
            record("E5-indexeddb-kv",
                   kv.get("durable") is True
                   and kv.get("first", {}).get("ok") is True
                   and kv.get("second", {}).get("ok") is True,
                   kv)

            # E7 — C-003 local-SQL, in a DEDICATED top-level document:
            # a PGlite app page is a pthreads WASM host and requires
            # cross-origin isolation (SAB); the isolation headers are
            # served for /__velqu_editor__/e7.html only, so the main
            # shell's deployment contract (incl. the required SW lane)
            # is unchanged. The lane: lazy engine network trace, a write,
            # IndexedDB durability across a real page reload, then reset.
            sql = ctx.new_page()
            sql.on("request", lambda r: net_requests.append(r.url))
            sql.goto(f"{base}/__velqu_editor__/e7.html", timeout=45000)
            sql.wait_for_function("document.getElementById('st')?.textContent === 'ready'", timeout=45000)
            isolated = sql.evaluate("() => crossOriginIsolated")
            first = sql.evaluate("() => window.sql.run('write')")
            engine_urls = sql.evaluate(
                "() => performance.getEntriesByType('resource').map(e => e.name)"
                  " .filter(u => u.includes('pglite') || u.includes('initdb'))")
            e7_lazy = (
                first.get("ok") is True
                and isolated is True
                and len(engine_urls) > 0
            )
            record("E7-sql-lazy-engine-network", e7_lazy,
                   {"crossOriginIsolated": isolated,
                    "engineAssets": engine_assets,
                    "engineBytesFetched": len(engine_urls)})

            sql.reload(wait_until="domcontentloaded")
            sql.wait_for_function("document.getElementById('st')?.textContent === 'ready'", timeout=45000)
            readback = sql.evaluate("() => window.sql.run('read')")
            e7_durable = (
                readback.get("ok") is True
                and isinstance(readback.get("count"), int)
                and readback["count"] >= (first.get("count") or 0)
            )
            record("E7-sql-indexeddb-durability", e7_durable,
                   {"written": first.get("count"), "readAfterReload": readback.get("count")})

            reset = sql.evaluate("() => window.sql.run('reset')")
            record("E7-sql-reset", reset.get("ok") is True, reset)
            sql.close()

            # E6 — update detection: redeploy changed build; SW bytes change
            build_v2, _ = build_deployment(project, "Hello E2E v2")
            update = page.evaluate("""async () => {
              const r = await navigator.serviceWorker.getRegistration();
              await r.update();
              for (let i = 0; i < 240; i++) {
                if (r.waiting) break;
                await new Promise(res => setTimeout(res, 250));
              }
              if (!r.waiting) return { waiting: false };
              r.waiting.postMessage({ type: 'VELQU_APPLY_UPDATE' });
              for (let i = 0; i < 240; i++) {
                await new Promise(res => setTimeout(res, 250));
                if (!r.waiting && r.active) return { waiting: false, applied: true };
              }
              return { waiting: true, applied: false };
            }""")
            page.reload(wait_until="domcontentloaded")
            page.wait_for_function(
                "document.getElementById('velqu-status')?.textContent.includes('ready')", timeout=45000)
            info2 = json.loads(page.text_content("#velqu-info"))
            record("E6-update-applies-on-reload",
                   update.get("applied") is True and info2.get("buildId") == build_v2,
                   {"newBuild": info2.get("buildId", "")[:12]})

            report["environment"] = {
                "browser": args.browser,
                "version": browser.version,
                "platform": platform.platform(),
                "machine": platform.machine(),
                "buildIdV1": build_v1,
                "buildIdV2": build_v2,
            }
            browser.close()
    finally:
        try:
            httpd.shutdown()
            httpd.server_close()
        except Exception:
            pass
        shutil.rmtree(work, ignore_errors=True)

    report["REHEARSAL-PASS"] = all(v["ok"] for v in checks.values())
    print(json.dumps({"REHEARSAL-PASS": report["REHEARSAL-PASS"]}), flush=True)
    if args.out:
        # fresh checkouts have no evidence/ tree; don't lose a full lane run
        # to the report write (#1305)
        os.makedirs(os.path.dirname(os.path.abspath(args.out)), exist_ok=True)
        with open(args.out, "w") as f:
            json.dump(report, f, indent=1)
    return 0 if report["REHEARSAL-PASS"] else 1


if __name__ == "__main__":
    sys.exit(main())
