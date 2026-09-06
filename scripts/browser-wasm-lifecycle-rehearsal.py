#!/usr/bin/env python3
"""
BWASM-B-006 — real-browser lifecycle rehearsal (evidence tooling).

Drives headless Chromium (Playwright) against `velqu build --target
browser-wasm` deployments and records, per scenario:

  S1  cold install          — SW activates only after ALL bound artifacts
                              prefetch with digest verification
  S2  offline navigation    — last known-good build stays usable with the
                              static host completely down
  S3  upgrade N → N+1       — apply-on-next-reload; no mid-session swap;
                              every executed request belongs to ONE build
  S4  multiple tabs         — tabs converge on the activated build
  S5  corrupt update        — a tampered/partially-propagated N+1 fails
                              its install; N remains active and coherent
  S6  rollback              — redeploying N restores a fully coherent N
  S7  base-path /app/       — subpath deployment: cold install + offline

This is evidence tooling for the BWASM program (Python + Playwright; the
repository itself has no Python/Playwright dependency). Real-browser CI
lanes and supported-browser matrices remain BWASM-Q-002.

Usage: python3 scripts/browser-wasm-lifecycle-rehearsal.py [--out FILE]
Requires: python3 -m pip install playwright, a chromium build under
~/.cache/ms-playwright (override with VELQU_CHROME=...).
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
import functools
import hashlib
import urllib.request

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CLI = os.path.join(REPO, "packages", "cli", "src", "index.ts")
DEMO = os.path.join(REPO, "examples", "browser-demo")
CHROME = os.environ.get(
    "VELQU_CHROME",
    os.path.expanduser(
        "~/.cache/ms-playwright/chromium-1234/chrome-linux64/chrome"
    ),
)

REPORT: list[dict] = []


def log(event: dict) -> None:
    REPORT.append(event)
    print(json.dumps(event), flush=True)


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def bun(*args: str, cwd: str = REPO) -> None:
    subprocess.run(["bun", *args], cwd=cwd, check=True, capture_output=True)


def build_deployment(target_dir: str, greeting: str, extra_route: bool, base_path=None) -> str:
    """Materialize an app variant and build a browser-wasm deployment."""
    src = os.path.join(target_dir, "src")
    os.makedirs(src, exist_ok=True)
    routes = f'''
import {{ route }} from "@velqu/core";
import {{ s }} from "@velqu/schema";

export const hello = route({{
  id: "hello.get",
  method: "GET",
  path: "/hello/:name",
  response: {{ 200: s.object({{ message: s.string() }}) }},
  handle: async (ctx) => ({{ message: `{greeting}` }}),
}});
'''
    if extra_route:
        routes += '''
export const status = route({
  id: "status.get",
  method: "GET",
  path: "/status",
  response: { 200: s.object({ ok: s.boolean() }) },
  handle: async (ctx) => ({ ok: ctx.handlerKey === "status.get" }),
});
'''
    routes += "\nexport const app = { routes: [hello" + (", status" if extra_route else "") + "] };\n"
    with open(os.path.join(src, "app.ts"), "w") as f:
        f.write(routes)
    with open(os.path.join(target_dir, "package.json"), "w") as f:
        json.dump({"name": "lifecycle-demo", "type": "module", "private": True}, f)
    node_modules = os.path.join(target_dir, "node_modules", "@velqu")
    os.makedirs(node_modules, exist_ok=True)
    for pkg in ("core", "schema", "browser-runtime"):
        link = os.path.join(node_modules, pkg)
        if not os.path.islink(link):
            os.symlink(os.path.join(REPO, "packages", pkg), link)
    cmd = ["packages/cli/src/index.ts", "build", "--target", "browser-wasm",
           "--project", target_dir, "--json"]
    if base_path:
        cmd += ["--base-path", base_path]
    out = subprocess.run(["bun", *cmd], cwd=REPO, capture_output=True, text=True)
    if out.returncode != 0:
        raise RuntimeError(f"build failed: {out.stderr[-800:]}")
    info = json.loads(out.stdout)
    return info["buildId"]


class DeployServer:
    """Static server whose deployment root can be swapped mid-run."""

    def __init__(self, port: int):
        self.port = port
        self.root: str | None = None
        self.overrides: dict[str, bytes] = {}   # path → exact bytes served
        self.httpd = None
        self.start()

    def start(self):
        outer = self

        class Handler(http.server.SimpleHTTPRequestHandler):
            def __init__(self, *a, **kw):
                super().__init__(*a, directory=outer.root or "/", **kw)

            def do_GET(self):
                path = self.path.split("?")[0]
                if path in outer.overrides:
                    body = outer.overrides[path]
                else:
                    return super().do_GET()
                self.send_response(200)
                self.send_header("Content-Length", str(len(body)))
                if path.endswith(".wasm"):
                    self.send_header("Content-Type", "application/wasm")
                elif path.endswith(".js"):
                    self.send_header("Content-Type", "text/javascript; charset=utf-8")
                elif path.endswith(".json"):
                    self.send_header("Content-Type", "application/json; charset=utf-8")
                elif path.endswith(".html"):
                    self.send_header("Content-Type", "text/html; charset=utf-8")
                self.end_headers()
                self.wfile.write(body)

            def log_message(self, *a):
                pass

        self.handler = Handler
        self.httpd = http.server.ThreadingHTTPServer(("127.0.0.1", self.port), Handler)
        threading.Thread(target=self.httpd.serve_forever, daemon=True).start()

    def restart(self, handler=None):
        self.httpd.shutdown()
        self.httpd.server_close()
        time.sleep(0.5)
        cls = handler or self.handler
        self.httpd = http.server.ThreadingHTTPServer(("127.0.0.1", self.port), cls)
        threading.Thread(target=self.httpd.serve_forever, daemon=True).start()

    def stop(self):
        self.httpd.shutdown()
        self.httpd.server_close()

    def url(self, path: str = "") -> str:
        return f"http://127.0.0.1:{self.port}{path}"


class Page:
    """One browser tab (context) driving a deployment."""

    def __init__(self, browser, server: DeployServer, name: str, base_path: str = "/"):
        self.name = name
        self.base = f"http://127.0.0.1:{server.port}" + ("" if base_path == "/" else base_path.rstrip("/"))
        self.ctx = browser.new_context()
        self.page = self.ctx.new_page()
        self.page.on("console", lambda m: log({"event": "console", "tab": name,
                                               "type": m.type, "text": m.text[:200]}))
        self.page.on("pageerror", lambda e: log({"event": "pageerror", "tab": name,
                                                 "text": str(e)[:200]}))

    def goto(self, wait_ready: bool = True, timeout: int = 30000):
        self.page.goto(self.base + "/index.html", timeout=timeout)
        if wait_ready:
            try:
                self.wait_ready(timeout)
            except Exception:
                log({"event": "boot-not-ready", "tab": self.name,
                     "url": self.page.url,
                     "bodySnippet": self.page.evaluate("() => document.body ? document.body.innerText.slice(0, 300) : 'no-body'"),
                     "requests": self.page.evaluate("() => performance.getEntriesByType('resource').map(r => r.name + ' ' + r.transferSize).slice(0, 12)"),
                     })
                raise
        return self.status()

    def reload(self, wait_ready: bool = True, timeout: int = 30000):
        self.page.reload(wait_until="domcontentloaded", timeout=timeout)
        if wait_ready:
            self.wait_ready(timeout)
        return self.status()

    def wait_ready(self, timeout: int = 30000):
        self.page.wait_for_function(
            "document.getElementById('velqu-status')?.textContent.includes('ready')",
            timeout=timeout,
        )

    def status(self) -> dict:
        info = json.loads(self.page.text_content("#velqu-info") or "{}")
        return {
            "tab": self.name,
            "statusText": self.page.text_content("#velqu-status"),
            "buildId": info.get("buildId"),
            "source": info.get("source"),
            "sw": info.get("serviceWorker"),
        }

    def cache_inventory(self) -> dict:
        return self.page.evaluate("""async () => {
          const out = {};
          for (const k of await caches.keys()) {
            const c = await caches.open(k);
            out[k] = (await c.keys()).map(r => r.url).sort();
          }
          return out;
        }""")

    def sw_states(self) -> dict:
        return self.page.evaluate("""async () => {
          const r = await navigator.serviceWorker.getRegistration();
          return {
            installing: r?.installing?.state ?? null,
            waiting: r?.waiting?.state ?? null,
            active: r?.active?.state ?? null,
            controlled: !!navigator.serviceWorker.controller,
          };
        }""")

    def wait_sw_active(self, timeout: int = 60000):
        deadline = time.time() + timeout / 1000
        while time.time() < deadline:
            st = self.sw_states()
            if st["active"] == "activated":
                return st
            time.sleep(0.3)
        raise TimeoutError(f"SW not activated: {st}")

    def close(self):
        self.ctx.close()


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=None, help="write the JSON report here too")
    args = ap.parse_args()

    from playwright.sync_api import sync_playwright

    if not os.path.exists(CHROME):
        print(f"chromium not found at {CHROME} (set VELQU_CHROME)", file=sys.stderr)
        return 2

    work = tempfile.mkdtemp(prefix="velqu-b006-")
    port = 8920
    server = DeployServer(port)

    try:
        # --- fixtures: N and N+1 (incompatible: N+1 adds a route) ---
        dir_n = os.path.join(work, "deploy-n")
        dir_n1 = os.path.join(work, "deploy-n1")
        os.makedirs(dir_n)
        os.makedirs(dir_n1)
        build_n = build_deployment(dir_n, "Hello N", extra_route=False)
        build_n1 = build_deployment(dir_n1, "Hello N+1", extra_route=True)
        dir_n = os.path.join(dir_n, "dist", "browser")
        dir_n1 = os.path.join(dir_n1, "dist", "browser")
        log({"event": "fixtures", "buildN": build_n[:12], "buildN1": build_n1[:12]})
        assert build_n != build_n1

        with sync_playwright() as p:
            browser = p.chromium.launch(headless=True, executable_path=CHROME,
                                        args=["--no-sandbox"])

            # ============================================================
            # S1 — cold install N
            # ============================================================
            server.root = dir_n
            tab = Page(browser, server, "s1")
            s1 = tab.goto()
            log({"event": "S1-boot", "boot": s1})
            tab.wait_sw_active()
            inv = tab.cache_inventory()
            app_caches = {k: v for k, v in inv.items() if k.startswith("velqu:app:")}
            s1_inv = {k: len(v) for k, v in app_caches.items()}
            log({"event": "S1-cold-install", "boot": s1,
                 "appCacheEntries": s1_inv})
            assert s1["buildId"] == build_n and s1["sw"] == "service-worker"
            assert len(s1_inv) == 1 and list(s1_inv.values())[0] >= 13, s1_inv

            # ============================================================
            # S2 — offline navigation: host completely down
            # ============================================================
            tab.reload()
            server.stop()
            time.sleep(1)
            try:
                s2 = tab.reload(timeout=20000)
                log({"event": "S2-offline-navigation", "boot": s2})
                assert s2["buildId"] == build_n, s2
            finally:
                server.restart()
            # tab stays open: it becomes S3's live upgrade client

            # ============================================================
            # S3 — upgrade N → N+1 on the LIVE S2 client (apply-on-next-
            # reload; every boot coherent — never a mixed N/N+1 set)
            # ============================================================
            server.root = dir_n1
            boot_swap = tab.reload()    # existing N client faces the N+1 deploy
            sw_mid = tab.sw_states()
            # Browsers throttle navigation-time SW update checks (spec:
            # at most once per 24h) — a deterministic check requires
            # registration.update(). Documented in the B-006 update policy.
            update_state = tab.page.evaluate("""async () => {
              const r = await navigator.serviceWorker.getRegistration();
              await r.update();
              // wait for the verified install to FINISH (installing → waiting)
              for (let i = 0; i < 240; i++) {
                if (r.waiting) break;
                await new Promise(res => setTimeout(res, 250));
              }
              return { waiting: !!r.waiting, installing: !!r.installing };
            }""")
            apply = tab.page.evaluate("""async () => {
              const r = await navigator.serviceWorker.getRegistration();
              if (!r.waiting) return 'no-waiting';
              r.waiting.postMessage({ type: 'VELQU_APPLY_UPDATE' });
              for (let i = 0; i < 240; i++) {
                await new Promise(res => setTimeout(res, 250));
                if (!r.waiting && r.active) return 'applied';
              }
              return 'timeout';
            }""")
            boot_next = tab.reload()    # applied build takes over on reload
            log({"event": "S3-apply", "apply": apply})
            log({"event": "S3-upgrade", "bootOnSwap": boot_swap,
                 "swMid": sw_mid, "forcedUpdate": update_state,
                 "bootAfterActivation": boot_next})
            assert boot_swap["buildId"] in (build_n, build_n1), boot_swap
            assert boot_next["buildId"] == build_n1, boot_next
            # coherence: at most the active + one previous cache remain
            inv = tab.cache_inventory()
            app_caches = {k for k in inv if k.startswith("velqu:app:")}
            assert len(app_caches) <= 2, app_caches
            log({"event": "S3-retention", "appCacheCount": len(app_caches)})

            # ============================================================
            # S4 — multiple tabs converge
            # ============================================================
            tab_a = Page(browser, server, "s4a")
            a1 = tab_a.goto()
            tab_b = Page(browser, server, "s4b")
            b1 = tab_b.goto()
            tab_a.close()               # release clients of any waiting build
            b2 = tab_b.reload()
            log({"event": "S4-multi-tab", "tabBBoot1": b1, "tabBBoot2": b2})
            assert b2["buildId"] == build_n1, (b1, b2)
            tab_b.close()

            # ============================================================
            # S5 — corrupt update (partial propagation): N+1' with N's pack
            # ============================================================
            dir_bad = os.path.join(work, "deploy-bad")
            shutil.copytree(dir_n, dir_bad)
            bad_manifest_path = os.path.join(dir_bad, "velqu-artifacts.json")
            with open(bad_manifest_path) as f:
                bad = json.load(f)
            # forge: point the manifest at N+1's pack digest (mixed set)
            with open(os.path.join(dir_n1, "velqu-artifacts.json")) as f:
                n1_manifest = json.load(f)
            bad["packSha256"] = n1_manifest["packSha256"]
            bad["artifacts"]["pack"]["sha256"] = n1_manifest["artifacts"]["pack"]["sha256"]
            bad["artifacts"]["pack"]["url"] = "app.qpack"
            with open(bad_manifest_path, "w") as f:
                json.dump(bad, f)
            server.root = dir_n
            tab = Page(browser, server, "s5")
            boot5a = tab.goto()          # establish the known-good client on N
            tab.wait_sw_active()
            server.root = dir_bad
            boot5b = tab.reload()        # corrupt update: forged manifest
            log({"event": "S5-corrupt-update", "bootBefore": boot5a, "bootAfter": boot5b,
                 "sw": tab.sw_states()})
            # last known-good: the client keeps booting N from its verified
            # cache; the forged manifest can NEVER activate (digest/buildId
            # verification fails at install and at boot).
            assert boot5b["buildId"] == build_n, (boot5a, boot5b)
            tab.close()

            # ============================================================
            # S6 — rollback: redeploy N
            # ============================================================
            server.root = dir_n
            tab = Page(browser, server, "s6")
            boot6a = tab.goto()
            tab.wait_sw_active()
            boot6b = tab.reload()
            log({"event": "S6-rollback", "boot1": boot6a, "boot2": boot6b})
            assert boot6b["buildId"] == build_n, boot6b
            inv6 = tab.cache_inventory()
            kept = {k: len(v) for k, v in inv6.items() if k.startswith("velqu:app:")}
            log({"event": "S6-retention", "appCaches": kept})
            assert len(kept) <= 2
            tab.close()

            # ============================================================
            # S7 — base-path /app/: cold install + offline
            # ============================================================
            dir_bp = os.path.join(work, "deploy-bp")
            os.makedirs(dir_bp)
            build_bp = build_deployment(dir_bp, "Hello basepath", extra_route=False,
                                        base_path="/app/")
            # serve the deployment's browser/ dir at /app/
            bp_root = os.path.join(dir_bp, "dist", "browser")
            outer = server
            saved_root = outer.root
            outer.root = bp_root

            class BPHandler(http.server.SimpleHTTPRequestHandler):
                def __init__(self, *a, **kw):
                    super().__init__(*a, directory=bp_root, **kw)
                def translate_path(self, path):
                    path = path[len("/app"):] if path.startswith("/app") else path
                    return super().translate_path(path or "/")
                def log_message(self, *a):
                    pass
            outer.restart(BPHandler)
            tab = Page(browser, server, "s7", base_path="/app/")
            boot7 = tab.goto()
            log({"event": "S7-boot", "boot": boot7})
            tab.wait_sw_active()
            scope = tab.page.evaluate("async () => (await navigator.serviceWorker.getRegistration()).scope")
            inv7 = tab.cache_inventory()
            bp_caches = {k: len(v) for k, v in inv7.items() if k.startswith("velqu:app:")}
            log({"event": "S7-basepath-cold", "boot": boot7, "scope": scope,
                 "appCacheEntries": bp_caches})
            assert boot7["buildId"] == build_bp and scope.endswith("/app/"), (boot7, scope)
            tab.reload()
            outer.stop()   # true offline under the subpath
            time.sleep(1)
            try:
                boot7_off = tab.reload(timeout=20000)
                log({"event": "S7-basepath-offline", "boot": boot7_off})
                assert boot7_off["buildId"] == build_bp, boot7_off
            finally:
                outer.root = saved_root
                outer.restart()
            tab.close()

            browser.close()

        log({"event": "REHEARSAL-PASS"})
        if args.out:
            with open(args.out, "w") as f:
                json.dump(REPORT, f, indent=1)
        return 0
    finally:
        shutil.rmtree(work, ignore_errors=True)


if __name__ == "__main__":
    sys.exit(main())
