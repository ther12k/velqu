/**
 * Treaty client for the proof application (M4A-009-E).
 *
 * Imports published contract metadata from `dist/contract.json` (or uses
 * static route mapping) and typed RouteContracts from `dist/contract.d.ts`.
 * Exercises end-to-end type safety across all proof service modules.
 */

import { treaty, type TreatyClient, treatyRoutes } from "@velqu/treaty";
import type { Api as ProofApi } from "../dist/contract";

export type { ProofApi };

/**
 * Static route contract map for the proof service.
 * Used when connecting to remote instances where dist/contract.json is not bundled.
 */
export const proofContractRoutes = {
  "health.live": { path: "/health/live", method: "GET" },
  "hello.get": { path: "/hello/:name", method: "GET" },
  "users.create": { path: "/users", method: "POST" },
  "users.get": { path: "/users/:id", method: "GET" },
  "items.list": { path: "/items", method: "GET" },
  "items.create": { path: "/items", method: "POST" },
  "items.get": { path: "/items/:id", method: "GET" },
  "items.update": { path: "/items/:id", method: "PATCH" },
  "items.delete": { path: "/items/:id", method: "DELETE" },
  "auth.login": { path: "/auth/login", method: "POST" },
  "auth.profile": { path: "/auth/profile", method: "GET" },
  "upstream.quote": { path: "/upstream/quote", method: "GET" },
  "upstream.relay": { path: "/upstream/relay", method: "GET" },
  "upstream.fanout": { path: "/upstream/fanout", method: "GET" },
  "ops.readiness": { path: "/ops/readiness", method: "GET" },
  "ops.metrics": { path: "/ops/metrics", method: "GET" },
  "ops.version": { path: "/ops/version", method: "GET" },
  "ops.ping": { path: "/ops/ping", method: "GET" },
  "ops.check": { path: "/ops/check", method: "POST" },
  "async.timer": { path: "/async", method: "GET" },
  "js.text": { path: "/js-text", method: "GET" },
  "js.json": { path: "/js-json", method: "GET" },
  "async.cancel": { path: "/cancel", method: "GET" },
  "throw.redacted": { path: "/throw", method: "GET" },
} as const;

export interface ProofClientOptions {
  baseUrl?: string;
  fetch?: typeof globalThis.fetch;
}

/**
 * Creates a type-safe Treaty client for the proof service.
 */
export function createProofClient(opts: ProofClientOptions = {}): TreatyClient<ProofApi> {
  const baseUrl = opts.baseUrl ?? "http://127.0.0.1:3000";
  return treaty<ProofApi>({
    baseUrl,
    contract: proofContractRoutes,
    fetch: opts.fetch,
  });
}

/**
 * Creates a tree-shaken Treaty client exposing only selected routes.
 */
export function createProofClientSubset<const R extends readonly (keyof ProofApi & string)[]>(
  routes: R,
  opts: ProofClientOptions = {},
): TreatyClient<Pick<ProofApi, R[number]>> {
  const baseUrl = opts.baseUrl ?? "http://127.0.0.1:3000";
  return treatyRoutes<ProofApi, R>(
    {
      baseUrl,
      contract: proofContractRoutes,
      fetch: opts.fetch,
    },
    routes,
  );
}
