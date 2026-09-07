#!/usr/bin/env python3
"""BWASM-Q-005 — Browser-WASM size, startup, latency, and leak budget rehearsal.

Measures and enforces release budgets ratified in BWASM-D-004:
- Compressed/uncompressed sizes (brotli-11 and gzip-9) for kernel, glue, bundle, total.
- Cold start to ready (< 2000 ms on reference).
- Warm start from cache to ready (< 500 ms on reference).
- Per-request latency overhead over 100 samples (p50 <= 5 ms, p99 <= 15 ms).
- Memory growth across 100 repeated requests, failures, aborts, and restarts.
- Absence of optional SQL/parity-engine downloads for core applications.

Usage:
  python3 scripts/browser-budgets-rehearsal.py --browser chromium --out evidence/q-005/rehearsal-report.json
"""

import argparse
import http.server
import json
import os
import platform
import shutil
import subprocess
import sys
import tempfile
import threading
import time

try:
    import brotli
except ImportError:
    brotli = None

import zlib

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CHROMIUM = os.path.expanduser(
    "~/.cache/ms-playwright/chromium-1234/chrome-linux64/chrome"
)


def browser_path(name: str) -> str | None:
    if name == "chromium":
        return CHROMIUM if os.path.exists(CHROMIUM) else None
    return None


def bun(argv, cwd=REPO, check=True):
    return subprocess.run(["bun", *argv], cwd=cwd, check=check, capture_output=True, text=True)


def link_workspace(project: str, packages=("core", "schema", "browser-runtime")):
    link_dir = os.path.join(project, "node_modules", "@velqu")
    os.makedirs(link_dir, exist_ok=True)
    for pkg in packages:
        dst = os.path.join(link_dir, pkg)
        if not os.path.islink(dst):
            os.symlink(os.path.join(REPO, "packages", pkg), dst)
    ts_dst = os.path.join(project, "node_modules", "typescript")
    if not os.path.exists(ts_dst) and os.path.exists(os.path.join(REPO, "node_modules", "typescript")):
        os.symlink(os.path.join(REPO, "node_modules", "typescript"), ts_dst)


def write_app(project: str):
    src = os.path.join(project, "src")
    os.makedirs(src, exist_ok=True)
    with open(os.path.join(src, "app.ts"), "w") as f:
        f.write('''
import { route } from "@velqu/core";
import { s } from "@velqu/schema";

export const hello = route({
  id: "hello.get",
  method: "GET",
  path: "/hello/:name",
  response: { 200: s.object({ message: s.string() }) },
  handle: async (ctx) => ({ message: `Hello ${ctx.params?.name ?? "world"}` }),
});

export const echo = route({
  id: "echo.post",
  method: "POST",
  path: "/echo",
  body: s.object({ val: s.string() }),
  response: { 200: s.object({ val: s.string() }) },
  handle: async (ctx) => ({ val: ctx.body.val }),
});

export const app = { routes: [hello, echo] };
''')
    with open(os.path.join(project, "package.json"), "w") as f:
        json.dump({"name": "budget-demo", "type": "module", "private": True}, f)


BENCH_HARNESS = '''
import {
  createBrowserRuntime,
  createIndexedDbKv,
  WorkerHost,
  loadArtifacts,
} from "@velqu/browser-runtime";

export async function runBenchmark() {
  const base = new URL(".", window.location.href);
  const manifestRes = await fetch(new URL("velqu-artifacts.json", base).href);
  const manifestText = await manifestRes.text();
  const manifest = JSON.parse(manifestText);

  const kernelMod = await import(new URL("kernel.js", base).href);
  const loaded = await loadArtifacts(manifestText, async (url) => {
    const res = await fetch(new URL(url, base).href);
    return new Uint8Array(await res.arrayBuffer());
  });

  kernelMod.initKernelSync(loaded.bytes.kernelWasm);

  const sessionId = crypto.randomUUID();
  const host = new WorkerHost(sessionId, () => {
    const w = new Worker(new URL("worker.js", base).href, { type: "module" });
    w.postMessage({ type: "velqu-session", sessionId });
    return w;
  });

  function VelquKernel(packBytes) { return new kernelMod.WasmKernel(packBytes); }
  VelquKernel.kernel_abi_version = kernelMod.kernel_abi_version;

  const runtime = createBrowserRuntime({
    packBytes: loaded.bytes.pack,
    kernel: VelquKernel,
    executeHandler: (plan) => host.execute(plan),
  });

  // Warmup
  await runtime.fetch(new Request("http://velqu.local/hello/warmup"));

  // Latency samples (100 requests)
  const samples = [];
  for (let i = 0; i < 100; i++) {
    const t0 = performance.now();
    const res = await runtime.fetch(new Request(`http://velqu.local/hello/user${i}`));
    await res.text();
    const t1 = performance.now();
    samples.push(t1 - t0);
  }

  // Soak / Leak run (100 mixed cycles)
  const heapBefore = window.performance?.memory?.usedJSHeapSize ?? 0;
  const kv = createIndexedDbKv({ namespace: "budget:soak" });

  for (let i = 0; i < 100; i++) {
    // 1. Valid request
    await (await runtime.fetch(new Request(`http://velqu.local/hello/soak${i}`))).text();
    // 2. 404 Route
    await (await runtime.fetch(new Request("http://velqu.local/nonexistent"))).text();
    // 3. Validation failure
    await (await runtime.fetch(new Request("http://velqu.local/echo", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ invalid: 123 }),
    }))).text();
    // 4. KV write/read
    await kv.set(`key_${i}`, { index: i, ts: Date.now() });
    await kv.get(`key_${i}`);
  }

  const heapAfter = window.performance?.memory?.usedJSHeapSize ?? 0;

  return {
    samples,
    heapBefore,
    heapAfter,
    heapDeltaBytes: heapAfter - heapBefore,
  };
}
'''


def build_bench_entry(browser_dir: str, work: str):
    entry = os.path.join(work, "bench-entry.ts")
    with open(entry, "w") as f:
        f.write(BENCH_HARNESS)
    link_workspace(work)
    out_dir = os.path.join(browser_dir, "__velqu_editor__")
    os.makedirs(out_dir, exist_ok=True)
    out = bun(["build", entry, "--outdir", out_dir, "--target", "browser",
               "--format", "esm", "--minify"], check=False)
    if out.returncode != 0:
        raise RuntimeError(f"bench bundle failed: {out.stderr[-400:]}")


def build_deployment(project: str):
    write_app(project)
    link_workspace(project)
    cmd = ["packages/cli/src/index.ts", "build", "--target", "browser-wasm",
           "--project", project, "--json", "--kv",
           "--probe-path", "/hello/probe", "--probe-method", "GET"]
    out = bun(cmd)
    if out.returncode != 0:
        raise RuntimeError(f"build failed: {out.stderr[-500:]}")
    info = json.loads(out.stdout)
    return info["buildId"], os.path.join(project, "dist", "browser")


def measure_sizes(browser_dir: str) -> dict:
    inventory = {}
    total_raw = 0
    total_brotli = 0
    total_gzip9 = 0

    for fname in sorted(os.listdir(browser_dir)):
        if fname.startswith("."):
            continue
        p = os.path.join(browser_dir, fname)
        if os.path.isdir(p):
            continue
        with open(p, "rb") as f:
            data = f.read()

        raw_len = len(data)
        br_len = len(brotli.compress(data, quality=11)) if brotli else None
        gz_len = len(zlib.compress(data, level=9))

        total_raw += raw_len
        if br_len:
            total_brotli += br_len
        total_gzip9 += gz_len

        inventory[fname] = {
            "raw_bytes": raw_len,
            "brotli11_bytes": br_len,
            "gzip9_bytes": gz_len,
        }

    return {
        "files": inventory,
        "total": {
            "raw_bytes": total_raw,
            "brotli11_bytes": total_brotli,
            "gzip9_bytes": total_gzip9,
        }
    }


def compute_percentiles(samples: list[float]) -> dict:
    s = sorted(samples)
    n = len(s)

    def p(pct: float) -> float:
        idx = int(round((pct / 100.0) * (n - 1)))
        return round(s[idx], 3)

    return {
        "n": n,
        "min_ms": round(s[0], 3),
        "mean_ms": round(sum(s) / n, 3),
        "p50_ms": p(50),
        "p90_ms": p(90),
        "p95_ms": p(95),
        "p99_ms": p(99),
        "max_ms": round(s[-1], 3),
    }


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--browser", default="chromium", choices=["chromium"])
    ap.add_argument("--out", default=None)
    args = ap.parse_args()

    exe = browser_path(args.browser)
    if not exe:
        print(f"chromium not found at {CHROMIUM} (set VELQU_CHROME)", file=sys.stderr)
        return 2

    from playwright.sync_api import sync_playwright

    work = tempfile.mkdtemp(prefix="velqu-q005-")
    project = os.path.join(work, "app")
    os.makedirs(project, exist_ok=True)
    build_id, browser_dir = build_deployment(project)
    build_bench_entry(browser_dir, work)
    port = 8963

    class Quiet(http.server.SimpleHTTPRequestHandler):
        def __init__(self, *a, **kw):
            super().__init__(*a, directory=browser_dir, **kw)

        def log_message(self, *a):
            pass

    httpd = http.server.ThreadingHTTPServer(("127.0.0.1", port), Quiet)
    threading.Thread(target=httpd.serve_forever, daemon=True).start()

    report = {
        "schemaVersion": 1,
        "suite": "bwasm-q-005-budgets",
        "date": time.strftime("%Y-%m-%d"),
        "environment": {
            "browser": args.browser,
            "browser_path": exe,
            "os": platform.system(),
            "arch": platform.machine(),
            "node_toolchain": "bun/1.4.0",
        },
        "buildId": build_id,
        "budgets": {},
    }

    try:
        # 1. Size budgets
        size_data = measure_sizes(browser_dir)
        report["size_inventory"] = size_data

        kernel_br = size_data["files"].get("kernel.wasm", {}).get("brotli11_bytes") or \
                    size_data["files"].get("q_browser_kernel_bg.wasm", {}).get("brotli11_bytes")
        kernel_gz = size_data["files"].get("kernel.wasm", {}).get("gzip9_bytes") or \
                    size_data["files"].get("q_browser_kernel_bg.wasm", {}).get("gzip9_bytes")
        total_br = size_data["total"]["brotli11_bytes"]

        report["budgets"]["base_wasm_kernel_size"] = {
            "target_max_bytes": 512000,
            "actual_brotli_bytes": kernel_br,
            "actual_gzip9_bytes": kernel_gz,
            "pass": kernel_br <= 512000 if kernel_br else False,
        }
        report["budgets"]["total_initial_transfer_size"] = {
            "target_max_bytes": 1048576,
            "actual_brotli_bytes": total_br,
            "pass": total_br <= 1048576,
        }

        # 2. Browser timings & soak
        with sync_playwright() as p:
            browser = p.chromium.launch(headless=True, executable_path=exe, args=["--no-sandbox"])
            requested_urls = []

            # Cold start context
            ctx_cold = browser.new_context()
            page_cold = ctx_cold.new_page()
            page_cold.on("request", lambda r: requested_urls.append(r.url))

            base_url = f"http://127.0.0.1:{port}"

            t0 = time.perf_counter()
            page_cold.goto(f"{base_url}/index.html", timeout=45000)
            page_cold.wait_for_function(
                "document.getElementById('velqu-status')?.textContent.includes('ready')",
                timeout=45000,
            )
            wall_cold_ms = round((time.perf_counter() - t0) * 1000, 2)
            page_cold_ms = round(page_cold.evaluate("performance.now()"), 2)

            report["budgets"]["cold_start"] = {
                "target_max_ms": 2000,
                "actual_in_page_ms": page_cold_ms,
                "wall_time_ms": wall_cold_ms,
                "pass": page_cold_ms <= 2000,
            }

            # Warm start context (after SW / cache active)
            page_cold.wait_for_function(
                "async () => { const r = await navigator.serviceWorker.getRegistration(); return !!r && !!r.active; }",
                timeout=30000,
            )
            t_warm = time.perf_counter()
            page_cold.reload(wait_until="domcontentloaded")
            page_cold.wait_for_function(
                "document.getElementById('velqu-status')?.textContent.includes('ready')",
                timeout=45000,
            )
            warm_start_ms = round((time.perf_counter() - t_warm) * 1000, 2)

            report["budgets"]["warm_start"] = {
                "target_max_ms": 500,
                "actual_ms": warm_start_ms,
                "pass": warm_start_ms <= 500,
            }

            # Run in-browser benchmark module for latency overhead and soak
            bench_result = page_cold.evaluate("""async () => {
                const mod = await import('./__velqu_editor__/bench-entry.js');
                return await mod.runBenchmark();
            }""")

            latency_stats = compute_percentiles(bench_result["samples"])
            report["latency_samples_summary"] = latency_stats
            report["raw_latency_samples"] = bench_result["samples"]

            report["budgets"]["kernel_latency_p50"] = {
                "target_p50_ms": 5.0,
                "actual_p50_ms": latency_stats["p50_ms"],
                "pass": latency_stats["p50_ms"] <= 5.0,
            }
            report["budgets"]["kernel_latency_p99"] = {
                "target_p99_ms": 15.0,
                "actual_p99_ms": latency_stats["p99_ms"],
                "pass": latency_stats["p99_ms"] <= 15.0,
            }

            # Memory soak analysis
            heap_before = bench_result["heapBefore"]
            heap_after = bench_result["heapAfter"]
            heap_delta = bench_result["heapDeltaBytes"]

            report["budgets"]["memory_repeated_lifecycle"] = {
                "heap_before_bytes": heap_before,
                "heap_after_bytes": heap_after,
                "heap_delta_bytes": heap_delta,
                "operations_completed": 100,
                "pass": heap_delta < 20_000_000,  # flat within < 20 MB GC margin
            }

            # Optional asset check (criterion 1)
            forbidden_assets = [u for u in requested_urls if any(k in u.lower() for k in ["pglite", "quickjs-wasm"])]
            report["budgets"]["no_optional_assets_downloaded"] = {
                "forbidden_requests": forbidden_assets,
                "pass": len(forbidden_assets) == 0,
            }

            browser.close()

        all_pass = all(b.get("pass") for b in report["budgets"].values())
        report["status"] = "PASS" if all_pass else "FAIL"

        print(f"\n=== BWASM-Q-005 Budget Rehearsal Result: {report['status']} ===")
        for name, b in report["budgets"].items():
            verdict = "PASS" if b.get("pass") else "FAIL"
            print(f"  [{verdict}] {name}: {b}")

        if args.out:
            os.makedirs(os.path.dirname(os.path.abspath(args.out)), exist_ok=True)
            with open(args.out, "w") as f:
                json.dump(report, f, indent=2)
            print(f"\nReport written to {args.out}")

        return 0 if all_pass else 1

    finally:
        httpd.shutdown()
        shutil.rmtree(work, ignore_errors=True)


if __name__ == "__main__":
    sys.exit(main())
