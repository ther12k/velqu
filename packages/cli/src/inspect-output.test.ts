import { describe, it, expect } from "bun:test";
import { $ } from "bun";

describe("Inspect output fidelity (M4A-006-D)", () => {
  it("reports route plan fields and route identity in JSON", async () => {
    const result = await $`bun packages/cli/src/index.ts inspect routes --project examples/proof --json`.text();
    const output = JSON.parse(result);
    expect(output.status).toBe("ok");
    expect(output.routeCount).toBe(output.routes.length);
    expect(output.routes.every((route: Record<string, unknown>) =>
      typeof route.id === "string" && typeof route.validationCodec === "string" &&
      typeof route.responseCodec === "string" && typeof route.bridge === "string" &&
      Array.isArray(route.capabilities),
    )).toBeTrue();
  });

  it("reports actual strategy distribution rather than route-count placeholders", async () => {
    const result = await $`bun packages/cli/src/index.ts inspect fallbacks --project examples/proof --json`.text();
    const output = JSON.parse(result);
    expect(output.routeCount).toBe(24);
    expect(output.strategyDistribution.nativeValidationRoutes + output.strategyDistribution.jsValidationRoutes).toBe(24);
    expect(output.strategyDistribution.nativeResponseRoutes + output.strategyDistribution.jsResponseRoutes).toBe(24);
    expect(output.activeFallbacksCount).toBe(output.fallbacks.length);
  });

  it("reports capability inventory counts and debug names", async () => {
    const result = await $`bun packages/cli/src/index.ts inspect capabilities --project examples/proof --json`.text();
    const output = JSON.parse(result);
    expect(output.declaredCount).toBe(output.declared.length);
    expect(output.perRoute).toBeDefined();
    expect(output.linkedModules).toBeDefined();
  });
});
