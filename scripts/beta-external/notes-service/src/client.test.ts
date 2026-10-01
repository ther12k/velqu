/**
 * Runtime-local Treaty contract test (deterministic skip pattern, #1398).
 * Requires `velqu build` first (the client imports generated artifacts);
 * with no server answering, Treaty reports a status-0 network result and
 * the suite skips — anything else that answers fails loudly.
 */
import { describe, it, expect } from "bun:test";
import { createClient } from "./client";

const port = process.env.VELQU_DEV_PORT ?? "3000";
const api = createClient(`http://127.0.0.1:${port}`);

function refused(res: { error?: unknown }): boolean {
  const err = res.error as { status?: number } | undefined | null;
  return Boolean(err) && err.status === 0;
}

describe("notes API (runtime-local via Treaty, generated contract)", () => {
  it("health.live answers ok", async () => {
    const res = await api.health.live.get();
    if (refused(res)) {
      console.warn(`skipping: no server on 127.0.0.1:${port}`);
      return;
    }
    expect(res.data?.status).toBe("ok");
  });

  it("rejects a bad query with a typed 422 (generated contract round-trip)", async () => {
    const res = await api.notes.list.get({ query: { pageSize: 999 } });
    if (refused(res)) {
      console.warn(`skipping: no server on 127.0.0.1:${port}`);
      return;
    }
    expect(res.error?.status).toBe(422);
  });
});
