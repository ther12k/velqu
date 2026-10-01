/**
 * Runtime-local Treaty contract test (deterministic skip pattern, #1398):
 * with no server answering, Treaty reports a status-0 network result and
 * the suite skips; anything else that answers fails loudly. Point it at
 * a running dev server / runtime with VELQU_DEV_PORT when occupied.
 */

import { describe, it, expect } from "bun:test";
import { createClient } from "./client";

const port = process.env.VELQU_DEV_PORT ?? "3000";
const api = createClient(`http://127.0.0.1:${port}`);

function refused(res: { error?: unknown }): boolean {
  const err = res.error as { status?: number } | undefined | null;
  return Boolean(err) && err.status === 0;
}

function expectRelayServer(what: string): never {
  throw new Error(
    `${what}: 127.0.0.1:${port} answered but did not behave like the relay service. ` +
      `If another service occupies the port, start the relay elsewhere and set VELQU_DEV_PORT.`,
  );
}

describe("relay API (runtime-local via Treaty)", () => {
  it("health.live answers ok", async () => {
    const res = await api.health.live.get();
    if (refused(res)) {
      console.warn(`skipping: no server on 127.0.0.1:${port}`);
      return;
    }
    if (res.error) expectRelayServer("health.live failed");
    expect(res.data?.status).toBe("ok");
  });

  it("rejects an invalid webhook token with a typed 401", async () => {
    const res = await api.hooks.receive.post({
      headers: { authorization: "Bearer wrong" },
      body: { source: "generic", deliveryId: "d12345678", eventType: "ping", payloadText: "{}" },
    });
    if (refused(res)) {
      console.warn(`skipping: no server on 127.0.0.1:${port}`);
      return;
    }
    if (!res.error) expectRelayServer("unauthorized intake unexpectedly succeeded");
    expect(res.error?.status).toBe(401);
  });
});
