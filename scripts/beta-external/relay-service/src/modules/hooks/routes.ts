import { route, status } from "@velqu/core";
import { s } from "@velqu/schema";
import { webhookPolicy } from "../../policy/webhook";
import { saveDelivery, getDelivery, setEnrichment, UPSTREAM_BASE } from "./service";

// NOTE: schemas and their option values must be inline literals — the
// compiler's static extraction rejects schema variables and non-literal
// options (fail-closed, source-located diagnostics).

/** Intake: policy-authenticated, validated, stored bounded. */
export const receive = route({
  id: "hooks.receive",
  method: "POST",
  path: "/hooks",
  policy: webhookPolicy,
  body: s.object({
    source: s.enum(["github", "stripe", "generic"]),
    deliveryId: s.string({ pattern: "^[A-Za-z0-9_-]{8,64}$" }),
    eventType: s.string({ minLength: 1, maxLength: 64 }),
    // Arbitrary webhook payloads stay a bounded JSON text field — the
    // schema IR has no arbitrary-record validator, and an unbounded blob
    // would violate the bounded-by-construction posture.
    payloadText: s.string({ maxLength: 8192 }),
  }),
  response: {
    201: s.object({ deliveryId: s.string(), receivedAtMs: s.number() }),
  },
  handle: async ({ body }) => {
    const d = saveDelivery(body);
    return status(201).value({ deliveryId: d.deliveryId, receivedAtMs: d.receivedAtMs });
  },
});

/** Enrichment: outbound fetch to the loopback upstream, typed failures. */
export const enrich = route({
  id: "hooks.enrich",
  method: "POST",
  path: "/hooks/:deliveryId/enrich",
  params: s.object({ deliveryId: s.string({ pattern: "^[A-Za-z0-9_-]{8,64}$" }) }),
  response: {
    200: s.object({ deliveryId: s.string(), eventType: s.string(), enrichment: s.string() }),
    502: s.object({ error: s.string() }),
  },
  handle: async ({ params }) => {
    const d = getDelivery(params.deliveryId);
    if (!d) return status(404).problem("not-found", { detail: "delivery not found" });
    try {
      const url = `${UPSTREAM_BASE}/enrich?delivery=${encodeURIComponent(params.deliveryId)}`;
      const res = await fetch(url);
      if (!res.ok) return status(502).value({ error: `upstream ${res.status}` });
      const body = (await res.json()) as { enrichment?: string };
      const enrichment = body.enrichment ?? "none";
      setEnrichment(params.deliveryId, enrichment);
      return { deliveryId: d.deliveryId, eventType: d.eventType, enrichment };
    } catch {
      return status(502).value({ error: "upstream unavailable" });
    }
  },
});

/** Read-back: delivery state, typed 404 when unknown. */
export const get = route({
  id: "hooks.get",
  method: "GET",
  path: "/hooks/:deliveryId",
  params: s.object({ deliveryId: s.string({ pattern: "^[A-Za-z0-9_-]{8,64}$" }) }),
  response: {
    200: s.object({
      deliveryId: s.string(),
      source: s.string(),
      eventType: s.string(),
      enrichment: s.string(),
    }),
  },
  handle: async ({ params }) => {
    const d = getDelivery(params.deliveryId);
    if (!d) return status(404).problem("not-found", { detail: "delivery not found" });
    return {
      deliveryId: d.deliveryId,
      source: d.source,
      eventType: d.eventType,
      enrichment: d.enrichment ?? "pending",
    };
  },
});

export const hooksRoutes = [receive, enrich, get];
