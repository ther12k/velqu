/**
 * BWASM-R-002 — dispatcher conformance: body forms, abort contract,
 * HEAD/OPTIONS behavior, header/query policies, and the
 * no-JS-fast-path guarantee (every decision flows through the kernel
 * double, which records what it was asked).
 */
import { describe, it, expect } from "bun:test";
import {
  dispatchFetchRequest,
  DispatcherError,
  UNSUPPORTED_SEMANTICS,
  DEFAULT_MAX_BODY_BYTES,
} from "../src/dispatcher";
import type { KernelInstance, KernelPlanRequest, KernelPlanResult } from "../src/index";

interface Recorded {
  plans: KernelPlanRequest[];
}

/** Kernel double that records every plan message it receives. */
function recordingKernel(
  planFor: (req: KernelPlanRequest) => KernelPlanResult,
): { kernel: KernelInstance; recorded: Recorded } {
  const recorded: Recorded = { plans: [] };
  const kernel: KernelInstance = {
    plan_request(requestJson: string): string {
      const req = JSON.parse(requestJson) as KernelPlanRequest;
      recorded.plans.push(req);
      return JSON.stringify(planFor(req));
    },
    complete_invocation(completionJson: string): string {
      const c = JSON.parse(completionJson) as { result: { status: number } };
      if (c.result.status !== 200) {
        return JSON.stringify({
          kind: "problem",
          problem: {
            problemId: "internal",
            type: "https://velqu.dev/problems/internal",
            title: "Internal",
            status: 500,
            detail: `handler returned undeclared status ${c.result.status}`,
          },
        });
      }
      return JSON.stringify({
        kind: "response",
        status: 200,
        headers: [["content-type", "application/json"]],
        body: { ok: true },
      });
    },
    authorize_capability(): string {
      return JSON.stringify({ authorized: true });
    },
    dispose(): void {},
  };
  return { kernel, recorded };
}

const okPlan: KernelPlanResult = {
  kind: "invoke",
  abiVersion: 1,
  routeId: 0,
  handlerKey: "health.live",
  allowedStatuses: [200],
  defaultStatus: 200,
  deadlineMs: 5000,
};

describe("R-002 dispatcher: normalization crosses the kernel boundary", () => {
  it("URL query, headers, and method normalize into the plan message", async () => {
    const { kernel, recorded } = recordingKernel(() => okPlan);
    await dispatchFetchRequest(kernel, 1, new Request("https://x.example/a/b?k1=v1&k2=v2", {
      headers: { "x-custom": "1", "content-type": "text/plain" },
    }));
    expect(recorded.plans).toHaveLength(1);
    const plan = recorded.plans[0];
    expect(plan.method).toBe("GET");
    expect(plan.path).toBe("/a/b");
    expect(plan.query).toEqual([["k1", "v1"], ["k2", "v2"]]);
    expect(plan.headers).toContainEqual(["x-custom", "1"]);
    expect(plan.abiVersion).toBe(1);
  });

  it("duplicate request headers cross as the platform-joined form (deterministic)", async () => {
    const { kernel, recorded } = recordingKernel(() => okPlan);
    const request = new Request("https://x.example/", {
      headers: [["x-dup", "a"], ["x-dup", "b"]],
    });
    await dispatchFetchRequest(kernel, 1, request);
    const header = recorded.plans[0].headers!.find(([k]) => k === "x-dup");
    expect(header).toEqual(["x-dup", "a, b"]);
  });

  it("JSON bodies cross as the kernel `body` field verbatim", async () => {
    const { kernel, recorded } = recordingKernel(() => okPlan);
    await dispatchFetchRequest(kernel, 1, new Request("https://x.example/echo", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ name: "Ada" }),
    }));
    expect(recorded.plans[0].body).toBe(JSON.stringify({ name: "Ada" }));
  });

  it("URL-encoded bodies parse into a string record (last-wins) crossing as JSON", async () => {
    const { kernel, recorded } = recordingKernel(() => okPlan);
    await dispatchFetchRequest(kernel, 1, new Request("https://x.example/form", {
      method: "POST",
      headers: { "content-type": "application/x-www-form-urlencoded" },
      body: "a=1&b=2&a=3",
    }));
    expect(JSON.parse(recorded.plans[0].body!)).toEqual({ a: "3", b: "2" });
  });

  it("multipart bodies cross as bounded metadata only", async () => {
    const { kernel, recorded } = recordingKernel(() => okPlan);
    const boundary = "XBOUND";
    const body = [
      `--${boundary}`,
      'Content-Disposition: form-data; name="field1"',
      "",
      "value1",
      `--${boundary}`,
      'Content-Disposition: form-data; name="file1"',
      "Content-Type: text/plain",
      "",
      "file contents",
      `--${boundary}--`,
    ].join("\r\n");
    await dispatchFetchRequest(kernel, 1, new Request("https://x.example/upload", {
      method: "POST",
      headers: { "content-type": `multipart/form-data; boundary=${boundary}` },
      body,
    }));
    const meta = JSON.parse(recorded.plans[0].body!) as {
      parts: Array<{ name: string; contentType?: string }>;
    };
    expect(meta.parts).toEqual([
      { name: "field1" },
      { name: "file1", contentType: "text/plain" },
    ]);
  });

  it("binary bodies reject fail-closed with the unsupported inventory entry", async () => {
    const { kernel, recorded } = recordingKernel(() => okPlan);
    try {
      await dispatchFetchRequest(kernel, 1, new Request("https://x.example/bin", {
        method: "POST",
        headers: { "content-type": "application/octet-stream" },
        body: "\x00\x01\x02",
      }));
      throw new Error("unreachable");
    } catch (e) {
      const err = e as DispatcherError;
      expect(err).toBeInstanceOf(DispatcherError);
      expect(err.rejection.kind).toBe("unsupported");
      expect(err.rejection.kind === "unsupported" && err.rejection.detail).toContain(
        "application/octet-stream",
      );
    }
    expect(recorded.plans).toHaveLength(0); // rejected BEFORE the kernel saw it
  });

  it("oversized bodies reject with the limit problem before dispatch", async () => {
    const { kernel } = recordingKernel(() => okPlan);
    try {
      await dispatchFetchRequest(
        kernel,
        1,
        new Request("https://x.example/big", {
          method: "POST",
          headers: { "content-type": "text/plain" },
          body: "x".repeat(DEFAULT_MAX_BODY_BYTES + 1),
        }),
        { maxBodyBytes: DEFAULT_MAX_BODY_BYTES },
      );
      throw new Error("unreachable");
    } catch (e) {
      expect((e as DispatcherError).rejection.kind).toBe("limit");
    }
  });

  it("query beyond the pair cap rejects with a limit problem", async () => {
    const { kernel } = recordingKernel(() => okPlan);
    const qs = Array.from({ length: 300 }, (_, i) => `k${i}=v`).join("&");
    try {
      await dispatchFetchRequest(kernel, 1, new Request(`https://x.example/?${qs}`));
      throw new Error("unreachable");
    } catch (e) {
      expect((e as DispatcherError).rejection.kind).toBe("limit");
    }
  });
});

describe("R-002 dispatcher: HEAD, OPTIONS, and abort contract", () => {
  it("HEAD dispatches through the kernel (HEAD→GET routing is kernel-side) and returns no body", async () => {
    const { kernel, recorded } = recordingKernel((req) => {
      // Kernel-side HEAD policy: router maps HEAD→GET (K-003).
      expect(req.method).toBe("HEAD");
      return okPlan;
    });
    const response = await dispatchFetchRequest(kernel, 1, new Request("https://x.example/health/live", { method: "HEAD" }));
    expect(response.status).toBe(200);
    expect(await response.text()).toBe("");
    expect(recorded.plans).toHaveLength(1);
  });

  it("OPTIONS on an unmatched method maps the kernel 405 problem with Allow", async () => {
    const { kernel } = recordingKernel(() => ({
      kind: "problem",
      problem: {
        problemId: "method",
        type: "https://velqu.dev/problems/method",
        title: "Method Not Allowed",
        status: 405,
        allow: ["GET", "HEAD", "OPTIONS"],
      },
    }));
    const response = await dispatchFetchRequest(kernel, 1, new Request("https://x.example/health/live", { method: "OPTIONS" }));
    expect(response.status).toBe(405);
    expect(response.headers.get("allow")).toBe("GET, HEAD, OPTIONS");
    expect(response.headers.get("content-type")).toBe("application/problem+json");
  });

  it("abort before dispatch rejects with AbortError", async () => {
    const { kernel, recorded } = recordingKernel(() => okPlan);
    const controller = new AbortController();
    controller.abort();
    try {
      await dispatchFetchRequest(kernel, 1, new Request("https://x.example/"), {
        signal: controller.signal,
      });
      throw new Error("unreachable");
    } catch (e) {
      expect((e as DOMException).name).toBe("AbortError");
    }
    expect(recorded.plans).toHaveLength(0);
  });

  it("abort during dispatch (body normalization in flight) rejects with AbortError, kernel untouched", async () => {
    const controller = new AbortController();
    const { kernel, recorded } = recordingKernel(() => okPlan);
    const pending = dispatchFetchRequest(kernel, 1, new Request("https://x.example/slow", {
      method: "POST",
      headers: { "content-type": "text/plain" },
      body: "payload",
    }), { signal: controller.signal });
    // Abort while the body read is still resolving: dispatch lands on
    // the during-dispatch check (after normalization, before the kernel).
    controller.abort();
    try {
      await pending;
      throw new Error("unreachable");
    } catch (e) {
      expect((e as DOMException).name).toBe("AbortError");
    }
    expect(recorded.plans).toHaveLength(0);
  });
});

describe("R-002 dispatcher: kernel authority (no JS fast path)", () => {
  it("no decision happens client-side: 404/405/problems all come from the kernel", async () => {
    const seen: string[] = [];
    const { kernel } = recordingKernel((req) => {
      seen.push(req.path);
      if (req.path === "/gone") {
        return {
          kind: "problem",
          problem: {
            problemId: "not-found",
            type: "https://velqu.dev/problems/not-found",
            title: "Not Found",
            status: 404,
          },
        };
      }
      return okPlan;
    });
    const notFound = await dispatchFetchRequest(kernel, 1, new Request("https://x.example/gone"));
    expect(notFound.status).toBe(404);
    const ok = await dispatchFetchRequest(kernel, 1, new Request("https://x.example/health/live"));
    expect(ok.status).toBe(200);
    expect(seen).toEqual(["/gone", "/health/live"]); // every outcome traversed the kernel
  });

  it("undeclared handler statuses become kernel contract-violation problems", async () => {
    const { kernel } = recordingKernel(() => okPlan);
    const response = await dispatchFetchRequest(kernel, 1, new Request("https://x.example/other"), {
      executeHandler: async () => ({ kind: "response", status: 418, headers: [], body: {} }),
    });
    expect(response.status).toBe(500);
    const problem = (await response.json()) as { problemId: string; detail: string };
    expect(problem.problemId).toBe("internal");
    expect(problem.detail).toContain("undeclared status 418");
  });

  it("UNSUPPORTED_SEMANTICS inventory is the documented four entries", () => {
    expect(UNSUPPORTED_SEMANTICS.map((u) => u.feature)).toEqual([
      "request body streaming (ReadableStream)",
      "binary request bodies (non-text media types)",
      "full multipart/form-data parsing",
      "response streaming",
    ]);
  });
});
