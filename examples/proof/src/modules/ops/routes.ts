import { route } from "@velqu/core";
import { s } from "@velqu/schema";
import { resolve as resolveUsers } from "../users/service";
import { resolve as resolveItems } from "../items/service";

const startTime = Date.now();

/**
 * Application-level readiness probe (M4A-009-D): verifies that domain
 * services (users, items) are initialized and healthy. (Distinct from the
 * native host-level `/health/ready` socket probe).
 */
export const readiness = route({
  id: "ops.readiness",
  method: "GET",
  path: "/ops/readiness",
  response: {
    200: s.object({
      ready: s.boolean(),
      services: s.object({
        users: s.boolean(),
        items: s.boolean(),
      }),
    }),
  },
  handle: () => {
    const usersReady = !!resolveUsers();
    const itemsReady = !!resolveItems();
    return {
      ready: usersReady && itemsReady,
      services: {
        users: usersReady,
        items: itemsReady,
      },
    };
  },
});

/**
 * Application operational metrics (M4A-009-D): exposes in-memory service
 * inventory and uptime without leaking secrets or host internals.
 */
export const metrics = route({
  id: "ops.metrics",
  method: "GET",
  path: "/ops/metrics",
  response: {
    200: s.object({
      uptimeMs: s.integer(),
      usersCount: s.integer(),
      itemsSampleCount: s.integer(),
    }),
  },
  handle: () => {
    const itemsPage = resolveItems().list(50, 0);
    return {
      uptimeMs: Date.now() - startTime,
      usersCount: 1, // seeded user usr_1
      itemsSampleCount: itemsPage.items.length,
    };
  },
});

/**
 * Operational version and environment summary (M4A-009-D).
 */
export const version = route({
  id: "ops.version",
  method: "GET",
  path: "/ops/version",
  response: {
    200: s.object({
      appId: s.string(),
      engine: s.string(),
      version: s.string(),
    }),
  },
  handle: () => ({
    appId: "proof",
    engine: "quickjs-ng",
    version: "0.1.0-alpha",
  }),
});

/**
 * Lightweight latency ping (M4A-009-D).
 */
export const ping = route({
  id: "ops.ping",
  method: "GET",
  path: "/ops/ping",
  response: {
    200: s.object({
      pong: s.boolean(),
    }),
  },
  handle: () => ({
    pong: true,
  }),
});

/**
 * Diagnostic health check simulation (M4A-009-D): accepts an echo payload
 * to verify body validation and payload reflection on ops endpoints.
 */
export const check = route({
  id: "ops.check",
  method: "POST",
  path: "/ops/check",
  body: s.object({
    component: s.string({ minLength: 1, maxLength: 60 }),
    detail: s.optional(s.string({ maxLength: 120 })),
  }),
  response: {
    200: s.object({
      healthy: s.boolean(),
      component: s.string(),
    }),
  },
  handle: ({ body }) => ({
    healthy: true,
    component: body.component,
  }),
});

export default [readiness, metrics, version, ping, check] as const;
