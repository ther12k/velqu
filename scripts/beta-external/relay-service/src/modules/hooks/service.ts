/**
 * Delivery store — bounded by construction (the Velqu posture applied at
 * the application layer too): insertion-order eviction keeps the map at
 * MAX_DELIVERIES regardless of intake rate. In-memory means best-effort:
 * durable delivery queues belong to an external system, by design.
 */

export interface Delivery {
  deliveryId: string;
  source: "github" | "stripe" | "generic";
  eventType: string;
  payloadText: string;
  receivedAtMs: number;
  enrichment: string | null;
}

export const MAX_DELIVERIES = 128;

/** Fixed loopback upstream the enrichment route fetches (driver binds it). */
export const UPSTREAM_BASE = "http://127.0.0.1:18971";

const deliveries = new Map<string, Delivery>();

export function saveDelivery(input: Omit<Delivery, "receivedAtMs" | "enrichment">): Delivery {
  if (deliveries.size >= MAX_DELIVERIES) {
    const oldest = deliveries.keys().next().value;
    if (oldest !== undefined) deliveries.delete(oldest);
  }
  const d: Delivery = { ...input, receivedAtMs: Date.now(), enrichment: null };
  deliveries.set(d.deliveryId, d);
  return d;
}

export function getDelivery(id: string): Delivery | undefined {
  return deliveries.get(id);
}

export function setEnrichment(id: string, enrichment: string): void {
  const d = deliveries.get(id);
  if (d) d.enrichment = enrichment;
}

export function storeSize(): number {
  return deliveries.size;
}
