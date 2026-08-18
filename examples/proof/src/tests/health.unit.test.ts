import { describe, expect, test } from "bun:test";
import { live } from "../modules/health/routes";

// UNIT-LOCAL (TRT-005): executes the handler directly; not runtime conformance.
describe("health module (unit-local)", () => {
  test("handler returns the frozen liveness body", async () => {
    const out = await live.handle(undefined as never);
    expect(JSON.stringify(out)).toBe('{"status":"ok"}');
  });
});
