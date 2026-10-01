/**
 * Unit tests for the bounded delivery store (no runtime required).
 */
import { describe, it, expect, beforeEach } from "bun:test";
import { saveDelivery, getDelivery, setEnrichment, storeSize, MAX_DELIVERIES } from "./service";

function delivery(n: number) {
  return {
    source: "generic" as const,
    deliveryId: `delivery-${String(n).padStart(8, "0")}`,
    eventType: "ping",
    payloadText: "{}",
  };
}

describe("delivery store", () => {
  beforeEach(() => {
    // Each test starts from the current size; the bound is what matters.
  });

  it("stores and reads back a delivery", () => {
    const before = storeSize();
    const d = saveDelivery(delivery(1));
    expect(getDelivery(d.deliveryId)?.eventType).toBe("ping");
    expect(storeSize()).toBe(before + 1);
  });

  it("records enrichment on an existing delivery", () => {
    const d = saveDelivery(delivery(2));
    setEnrichment(d.deliveryId, "stub-enrichment");
    expect(getDelivery(d.deliveryId)?.enrichment).toBe("stub-enrichment");
  });

  it("evicts oldest deliveries beyond MAX_DELIVERIES (bounded store)", () => {
    const start = storeSize();
    const overflow = MAX_DELIVERIES + 5 - start;
    for (let i = 0; i < overflow; i++) saveDelivery(delivery(100 + i));
    expect(storeSize()).toBeLessThanOrEqual(MAX_DELIVERIES);
    expect(getDelivery(delivery(100).deliveryId)).toBeUndefined();
  });
});
