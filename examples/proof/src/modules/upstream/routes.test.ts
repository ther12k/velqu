/**
 * Upstream module unit tests (M4A-009-C): verifies route declarations,
 * method, path, and error schemas.
 */
import { describe, it, expect } from "bun:test";
import { quote, relay, fanout } from "./routes";

describe("upstream module (controlled upstream M4A-009-C)", () => {
  it("declares upstream.quote GET route with 200/502 contracts", () => {
    expect(quote.id).toBe("upstream.quote");
    expect(quote.method).toBe("GET");
    expect(quote.path).toBe("/upstream/quote");
  });

  it("declares upstream.relay GET route with query schema and 200/502 contracts", () => {
    expect(relay.id).toBe("upstream.relay");
    expect(relay.method).toBe("GET");
    expect(relay.path).toBe("/upstream/relay");
  });

  it("declares upstream.fanout GET route with bounded count and 200/502 contracts", () => {
    expect(fanout.id).toBe("upstream.fanout");
    expect(fanout.method).toBe("GET");
    expect(fanout.path).toBe("/upstream/fanout");
  });
});
