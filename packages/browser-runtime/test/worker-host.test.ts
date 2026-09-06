/**
 * BWASM-R-004 — Worker execution host tests: infinite-loop termination
 * with post-recovery usability, stale/foreign message drops, deadline
 * enforcement, abort races, oversized payloads, bounded logs. Runs
 * against an in-process Worker double implementing the same protocol
 * the real browser Worker bootstrap uses.
 */
import { describe, it, expect } from "bun:test";
import {
  WorkerHost,
  WorkerProtocolError,
  WORKER_PROTOCOL_VERSION,
  workerBootstrapSource,
} from "../src/worker-host";
import type { KernelInvokePlan, KernelPlanResult } from "../src/index";

type Msg = Record<string, unknown>;

/** Scriptable Worker double: executes a script per received invoke. */
function makeScriptableWorker(
  script: (msg: Msg, post: (m: Msg) => void) => void,
): { worker: import("../src/worker-host").WorkerLike; delivered: Msg[]; terminateCount: () => number } {
  const listeners: Array<(e: { data: unknown }) => void> = [];
  let terminated = 0;
  const delivered: Msg[] = [];
  const worker = {
    postMessage(message: unknown): void {
      delivered.push(message as Msg);
      // Deliver asynchronously so the supervisor's promise is pending.
      setTimeout(() => script(message as Msg, (m) => listeners.forEach((l) => l({ data: m }))), 0);
    },
    terminate(): void {
      terminated += 1;
    },
    addEventListener(_type: "message", listener: (e: { data: unknown }) => void): void {
      listeners.push(listener);
    },
  };
  return { worker, delivered, terminateCount: () => terminated };
}

const basePlan: KernelInvokePlan = {
  kind: "invoke",
  abiVersion: 1,
  routeId: 0,
  handlerKey: "health.live",
  allowedStatuses: [200],
  defaultStatus: 200,
  deadlineMs: 200,
};

const okResult = { kind: "response" as const, status: 200, headers: [], body: { ok: true } };
const SESSION = "sess-test-1";

function postResult(
  post: (m: Msg) => void,
  invocationId: number,
  result: unknown,
  sessionId: string = SESSION,
): void {
  post({ v: WORKER_PROTOCOL_VERSION, type: "result", sessionId, invocationId, result });
}

describe("R-004 Worker execution host", () => {
  it("normal invoke resolves with the handler result", async () => {
    const { worker } = makeScriptableWorker((msg, post) => {
      if (msg.type === "invoke") postResult(post, msg.invocationId as number, okResult);
    });
    const host = new WorkerHost(SESSION, () => worker);
    const result = await host.execute(basePlan);
    expect(result).toEqual(okResult);
    host.dispose();
  });

  it("infinite loops are stopped by deadline termination and the host REMAINS USABLE", async () => {
    let terminateCount = 0;
    const { worker } = makeScriptableWorker((msg, post) => {
      if (msg.type === "invoke") {
        // Handler that never responds (infinite loop in the Worker).
        if ((msg.plan as KernelInvokePlan).handlerKey === "loop") return;
        postResult(post, msg.invocationId as number, okResult);
      }
    });
    // Wrap to count terminations.
    const counted = {
      postMessage: worker.postMessage.bind(worker),
      terminate(): void {
        terminateCount += 1;
        worker.terminate();
      },
      addEventListener: worker.addEventListener.bind(worker),
    };
    const host = new WorkerHost(SESSION, () => counted);

    const loopPlan = { ...basePlan, handlerKey: "loop", deadlineMs: 50 };
    await expect(host.execute(loopPlan)).rejects.toThrow();
    expect(terminateCount).toBeGreaterThan(0);
    expect(host.hardRecoveries).toBeGreaterThan(0);

    // The runtime remains usable: a fresh Worker serves the next call.
    const result = await host.execute(basePlan);
    expect(result).toEqual(okResult);
    host.dispose();
  });

  it("late messages from a terminated/stale run are ignored", async () => {
    let respond: ((m: Msg) => void) | null = null;
    const { worker } = makeScriptableWorker((msg, post) => {
      if (msg.type === "invoke") respond = () => postResult(post, msg.invocationId as number, okResult);
    });
    const host = new WorkerHost(SESSION, () => worker);
    const slow = host.execute(basePlan);
    // Let the invoke be delivered, then dispose (hard terminate —
    // dispose deterministically rejects the pending invocation).
    await new Promise((r) => setTimeout(r, 10));
    void slow.catch(() => {}); // the pending call rejects with FATAL by design
    host.dispose();
    // The late result arrives after termination:
    respond?.();
    await new Promise((r) => setTimeout(r, 10));
    expect(host.staleMessagesDropped).toBeGreaterThanOrEqual(1);
  });

  it("cross-session invocation results cannot be received (no cross-project theft)", async () => {
    const { worker } = makeScriptableWorker((msg, post) => {
      if (msg.type === "invoke") {
        // Replies with the FOREIGN session id.
        postResult(post, msg.invocationId as number, okResult, "sess-other-project");
      }
    });
    const host = new WorkerHost(SESSION, () => worker);
    await expect(host.execute(basePlan)).rejects.toThrow(); // deadline fires; result was dropped
    expect(host.staleMessagesDropped).toBeGreaterThanOrEqual(1);
    host.dispose();
  });

  it("abort before deadline cancels the invocation (AbortError)", async () => {
    const { worker } = makeScriptableWorker(() => {
      /* never responds */
    });
    const host = new WorkerHost(SESSION, () => worker);
    const controller = new AbortController();
    const pending = host.execute(basePlan, controller.signal);
    setTimeout(() => controller.abort(), 20);
    await expect(pending).rejects.toMatchObject({ name: "AbortError" });
    host.dispose();
  });

  it("pre-aborted signal rejects immediately without posting invoke", async () => {
    const { worker, delivered } = makeScriptableWorker(() => {});
    const host = new WorkerHost(SESSION, () => worker);
    const controller = new AbortController();
    controller.abort();
    await expect(host.execute(basePlan, controller.signal)).rejects.toMatchObject({ name: "AbortError" });
    const invokes = delivered.filter((m) => m.type === "invoke");
    expect(invokes).toHaveLength(0);
    host.dispose();
  });

  it("high-log-volume workers are hard-terminated (bounded log forwarding)", async () => {
    // The factory returns a FRESH Worker per spawn (matching real
    // Worker construction): the first Worker floods logs, its
    // replacement serves normally.
    let crashed = false;
    const host = new WorkerHost(SESSION, () => {
      const { worker } = makeScriptableWorker((msg, post) => {
        if (msg.type === "invoke") {
          if (crashed) {
            postResult(post, msg.invocationId as number, okResult);
            return;
          }
          crashed = true;
          for (let i = 0; i < 100; i++) {
            post({
              v: WORKER_PROTOCOL_VERSION,
              type: "log",
              sessionId: SESSION,
              invocationId: msg.invocationId,
              lines: ["spam"],
            });
          }
        }
      });
      return worker;
    });
    const outcome = await host.execute(basePlan).then(
      () => "resolved",
      (e) => (e instanceof WorkerProtocolError ? e.code : "other"),
    );
    expect(outcome).toBe("FATAL");
    expect(host.hardRecoveries).toBe(1);
    // Usable afterward: the fresh Worker serves the next call.
    const result = await host.execute(basePlan);
    expect(result).toEqual(okResult);
    host.dispose();
  });

  it("oversized results become structured errors, never silent truncation", async () => {
    const big = { body: "x".repeat(1 << 20 + 1) };
    const { worker } = makeScriptableWorker((msg, post) => {
      if (msg.type === "invoke") postResult(post, msg.invocationId as number, big);
    });
    const host = new WorkerHost(SESSION, () => worker);
    await expect(host.execute(basePlan)).rejects.toMatchObject({
      code: "OVERSIZED_PAYLOAD",
    });
    host.dispose();
  });

  it("crash/restart: fatal messages trigger hard recovery and the next call works", async () => {
    let crashed = false;
    const { worker } = makeScriptableWorker((msg, post) => {
      if (msg.type === "invoke" && !crashed) {
        crashed = true;
        post({ v: WORKER_PROTOCOL_VERSION, type: "fatal", sessionId: SESSION, detail: "simulated crash" });
        return;
      }
      if (msg.type === "invoke") postResult(post, msg.invocationId as number, okResult);
    });
    const host = new WorkerHost(SESSION, () => worker);
    await expect(host.execute(basePlan)).rejects.toMatchObject({ code: "FATAL" });
    const result = await host.execute(basePlan);
    expect(result).toEqual(okResult);
    host.dispose();
  });

  it("no parent-realm references cross the protocol (messages are plain data)", () => {
    const src = workerBootstrapSource();
    expect(src).not.toContain("document");
    expect(src).not.toContain("window");
    expect(src).toContain("redact"); // stack redaction is part of the bootstrap
    expect(src).toContain("hostile-code sandbox");
  });

  it("bootstrap source carries the isolation-honesty statement", () => {
    expect(workerBootstrapSource()).toContain("NOT a hostile-code sandbox");
  });
});
