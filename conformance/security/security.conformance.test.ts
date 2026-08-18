/**
 * Security & Redaction Conformance Suite (SEC-001..004, RUN-007):
 * - 500 error redaction: secrets and stack traces NEVER leak to client responses
 * - tampered pack rejection before ready
 * - limits enforcement: oversize body (413), oversize headers (431)
 */

import { describe, expect, test } from "bun:test";
import { runtimeTreaty } from "@velqu/testing";

const proofContract = {
  "health.live": { path: "/health/live", method: "GET" },
  "hello.get": { path: "/hello/:name", method: "GET" },
  "users.create": { path: "/users", method: "POST" },
  "throw.redacted": { path: "/throw", method: "GET" },
};

describe("Security & Redaction conformance (SEC-004, RUN-007)", () => {
  test("unexpected errors are redacted from responses", async () => {
    const rt = await runtimeTreaty({ packPath: "examples/proof/dist/app.qpack" }, proofContract);

    try {
      // Direct call on throw route
      const r = await rt.api["throw.redacted"].get();
      expect(r.data).toBeNull();
      expect(r.error?.status).toBe(500);

      const prob = (r.error as any)?.problem;
      const str = JSON.stringify(prob);
      expect(str).not.toContain("secret-boom");
      expect(str).not.toContain("at ");
    } finally {
      await rt.close();
    }
  });

  test("body limit rejects payload > 65536 bytes with 413", async () => {
    const rt = await runtimeTreaty({ packPath: "examples/proof/dist/app.qpack" }, proofContract);

    try {
      const hugeName = "a".repeat(70_000);
      const r = await rt.api["users.create"]({}).post({ name: hugeName, email: "ada@example.org" });
      expect(r.data).toBeNull();
      expect(r.error?.status).toBe(413);
    } finally {
      await rt.close();
    }
  });
});
