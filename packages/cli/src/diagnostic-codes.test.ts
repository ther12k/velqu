import { describe, it, expect } from "bun:test";
import { CompileError } from "@velqu/compiler";
import { formatActionableError } from "./errors";

describe("Structured diagnostic codes (M4A-006-A)", () => {
  it("assigns stable catalog codes to representative compile failures", () => {
    expect(formatActionableError(new CompileError("unsupported import 'node:fs'")).code).toBe("VELQU-COMP-IMPORT");
    expect(formatActionableError(new CompileError("route is missing 'response'")).code).toBe("VELQU-COMP-CONTRACT");
    expect(formatActionableError(new CompileError("route path must start with '/'")).code).toBe("VELQU-COMP-PATH");
    expect(formatActionableError(new CompileError("schema body must be an object")).code).toBe("VELQU-COMP-SCHEMA");
    expect(formatActionableError(new Error("runtime worker failed")).code).toBe("VELQU-RUNTIME");
    expect(formatActionableError(new Error("unexpected failure")).code).toBe("VELQU-UNKNOWN");
  });

  it("keeps diagnostic code separate from untrusted message content", () => {
    const diag = formatActionableError(new Error("token=secret-value runtime failed"));
    expect(diag.code).toBe("VELQU-RUNTIME");
    expect(diag.message).toContain("token=secret-value");
    expect(diag.raw).toContain("[velqu:error]");
  });
});
