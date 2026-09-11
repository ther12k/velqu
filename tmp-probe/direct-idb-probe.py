import http.server, threading, os, glob
d = sorted(glob.glob(os.path.join(os.path.dirname(os.path.abspath(__file__)), "c003-probe-*")))[-1]
d = os.path.join(d, "app/dist/browser")
class Q(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *a, **k): super().__init__(*a, directory=d, **k)
    def log_message(self,*a): pass
    def end_headers(self):
        if self.path.startswith("/__velqu_editor__/"):
            self.send_header("Cross-Origin-Opener-Policy", "same-origin")
            self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
            self.send_header("Cross-Origin-Resource-Policy", "same-origin")
        super().end_headers()
httpd = http.server.ThreadingHTTPServer(("127.0.0.1", 9011), Q)
threading.Thread(target=httpd.serve_forever, daemon=True).start()
from playwright.sync_api import sync_playwright
with sync_playwright() as p:
    b = p.chromium.launch(headless=True, args=["--no-sandbox"], executable_path="/home/ther12k/.cache/ms-playwright/chromium-1223/chrome-linux64/chrome")
    page = b.new_context().new_page()
    page.on("pageerror", lambda e: print("PAGEERR:", str(e)[:200], flush=True))
    page.goto("http://127.0.0.1:9011/__velqu_editor__/direct-idb.html", timeout=30000)
    for i in range(30):
        t = page.evaluate("() => document.getElementById('out').textContent")
        if t.startswith(("IDB-OK","IDB-FAIL")):
            print(t[:300], flush=True); break
        page.wait_for_timeout(5000)
    else:
        print("IDB-TIMEOUT", flush=True)
    b.close()
httpd.shutdown()
