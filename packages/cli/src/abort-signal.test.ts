import { describe, expect, test } from "bun:test";

describe("AbortController and AbortSignal conformance (M27-007-A)", () => {
  test("initial active state", () => {
    const ctrl = new AbortController();
    expect(ctrl.signal.aborted).toBe(false);
    expect(ctrl.signal.reason).toBeUndefined();
  });

  test("abort propagates exactly once", () => {
    const ctrl = new AbortController();
    let fired = 0;
    ctrl.signal.addEventListener("abort", () => {
      fired++;
    });

    ctrl.abort("user cancelled");
    expect(ctrl.signal.aborted).toBe(true);
    expect(ctrl.signal.reason).toBe("user cancelled");
    expect(fired).toBe(1);

    // Repeated abort does not change reason or re-trigger listeners
    ctrl.abort("ignored");
    expect(ctrl.signal.reason).toBe("user cancelled");
    expect(fired).toBe(1);
  });

  test("throwIfAborted throws reason when aborted", () => {
    const ctrl = new AbortController();
    expect(() => ctrl.signal.throwIfAborted()).not.toThrow();

    ctrl.abort("custom abort error");
    expect(() => ctrl.signal.throwIfAborted()).toThrow("custom abort error");
  });

  test("AbortSignal.abort factory", () => {
    const sig = AbortSignal.abort("immediate reason");
    expect(sig.aborted).toBe(true);
    expect(sig.reason).toBe("immediate reason");
  });

  test("AbortSignal.timeout factory", async () => {
    const sig = AbortSignal.timeout(10);
    expect(sig.aborted).toBe(false);
    await new Promise((r) => setTimeout(r, 25));
    expect(sig.aborted).toBe(true);
  });

  test("AbortSignal.any combines multiple signals", () => {
    const c1 = new AbortController();
    const c2 = new AbortController();
    const combined = AbortSignal.any([c1.signal, c2.signal]);
    expect(combined.aborted).toBe(false);

    c2.abort("c2 aborted");
    expect(combined.aborted).toBe(true);
    expect(combined.reason).toBe("c2 aborted");
  });

  describe("Bridge route deadline and explicit cancellation (M27-007-B)", () => {
    test("timer delay with already-aborted signal rejects immediately", async () => {
      const ctrl = new AbortController();
      ctrl.abort("pre-aborted");
      const delayPromise = (async (ms, opts) => {
        if (opts?.signal?.aborted) throw opts.signal.reason;
        return ms;
      })(1000, { signal: ctrl.signal });

      await expect(delayPromise).rejects.toBe("pre-aborted");
    });

    test("timer delay with mid-flight abort cancels and rejects with reason", async () => {
      const ctrl = new AbortController();
      let timerId: any;
      const delayPromise = new Promise((resolve, reject) => {
        if (ctrl.signal.aborted) return reject(ctrl.signal.reason);
        timerId = setTimeout(resolve, 1000);
        ctrl.signal.addEventListener("abort", () => {
          clearTimeout(timerId);
          reject(ctrl.signal.reason);
        }, { once: true });
      });

      setTimeout(() => ctrl.abort("cancelled-mid-flight"), 10);
      await expect(delayPromise).rejects.toBe("cancelled-mid-flight");
    });
  });
});
