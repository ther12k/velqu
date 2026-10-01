/** Unit tests for pagination math (no runtime required). */
import { describe, it, expect } from "bun:test";
import { resolvePage } from "./service";

describe("resolvePage", () => {
  it("applies documented defaults", () => {
    expect(resolvePage(undefined, undefined)).toEqual({ page: 1, pageSize: 20, offset: 0 });
  });

  it("computes offset for arbitrary valid pages", () => {
    expect(resolvePage(3, 10)).toEqual({ page: 3, pageSize: 10, offset: 20 });
    expect(resolvePage(2, 50)).toEqual({ page: 2, pageSize: 50, offset: 50 });
  });

  it("honors explicit page 1 with max page size", () => {
    expect(resolvePage(1, 50)).toEqual({ page: 1, pageSize: 50, offset: 0 });
  });
});
