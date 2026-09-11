#!/usr/bin/env python3
"""Self-contained: build the rehearsal's SQL entry + e7 doc into a stable
work-tree dir, serve with /__velqu_editor__/-scoped isolation headers,
and print the import result rendered by the page itself."""
import http.server, threading, os, sys, tempfile
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
os.chdir(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import importlib.util
spec = importlib.util.spec_from_file_location("reh", "scripts/browser-e2e-rehearsal.py")
reh = importlib.util.module_from_spec(spec)
spec.loader.exec_module(reh)

work = tempfile.mkdtemp(prefix="c003-probe-", dir=os.path.abspath("tmp-probe"))
project = os.path.join(work, "app")
os.makedirs(project, exist_ok=True)
build_id, browser_dir = reh.build_deployment(project, "Probe v1")
engine_assets = reh.build_sql_entry(browser_dir, work)
print("assets:", engine_assets, flush=True)

class Q(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *a, **k): super().__init__(*a, directory=browser_dir, **k)
    def log_message(self, *a): pass
    def end_headers(self):
        if self.path.startswith("/__velqu_editor__/"):
            self.send_header("Cross-Origin-Opener-Policy", "same-origin")
            self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
            self.send_header("Cross-Origin-Resource-Policy", "same-origin")
        super().end_headers()
httpd = http.server.ThreadingHTTPServer(("127.0.0.1", 9010), Q)
threading.Thread(target=httpd.serve_forever, daemon=True).start()

from playwright.sync_api import sync_playwright
with sync_playwright() as p:
    chrome = os.environ.get("VELQU_CHROME") or os.path.expanduser(
        "~/.cache/ms-playwright/chromium-1223/chrome-linux64/chrome")
    b = p.chromium.launch(headless=True, args=["--no-sandbox"], executable_path=chrome)
    page = b.new_context().new_page()
    page.on("pageerror", lambda e: print("PAGEERR:", str(e)[:300], flush=True))
    page.on("response", lambda r: print(f"HTTP{r.status}:", r.url[-70:], flush=True)
            if r.status >= 400 else None)
    page.goto("http://127.0.0.1:9010/__velqu_editor__/e7.html", timeout=30000)
    page.wait_for_timeout(20000)
    print("ST:", page.evaluate("() => document.getElementById('st').textContent"), flush=True)
    print("SQL-SET:", page.evaluate("() => typeof window.sql"), flush=True)
    r = page.evaluate("() => Promise.race([window.sql ? window.sql.run('write').then(x => JSON.stringify(x)) : Promise.resolve('NO-SQL'), new Promise(res => setTimeout(() => res('TIMEOUT-150s'), 150000))])")
    print("WRITE:", r, flush=True)
    b.close()
httpd.shutdown()
