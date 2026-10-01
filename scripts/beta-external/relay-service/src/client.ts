/**
 * Treaty client for the relay service — hand-mirrored contract of the
 * routes above (a real consumer generates this from the build's
 * contract.json / contract.d.ts; `velqu build` emits both).
 */
import { treaty, type TreatyClient } from "@velqu/treaty";

export type RelayApi = {
  "health.live": {
    path: "/health/live";
    method: "GET";
    params: never;
    query: never;
    body: never;
    headers: never;
    responses: { 200: { status: string } };
  };
  "hooks.receive": {
    path: "/hooks";
    method: "POST";
    params: never;
    query: never;
    body: { source: "github" | "stripe" | "generic"; deliveryId: string; eventType: string; payloadText: string };
    headers: { authorization: string };
    responses: { 201: { deliveryId: string; receivedAtMs: number } };
  };
  "hooks.enrich": {
    path: "/hooks/:deliveryId/enrich";
    method: "POST";
    params: { deliveryId: string };
    query: never;
    body: never;
    headers: never;
    responses: { 200: { deliveryId: string; eventType: string; enrichment: string }; 502: { error: string } };
  };
  "hooks.get": {
    path: "/hooks/:deliveryId";
    method: "GET";
    params: { deliveryId: string };
    query: never;
    body: never;
    headers: never;
    responses: { 200: { deliveryId: string; source: string; eventType: string; enrichment: string } };
  };
}

export const contract = {
  "health.live": { path: "/health/live", method: "GET" },
  "hooks.receive": { path: "/hooks", method: "POST" },
  "hooks.enrich": { path: "/hooks/:deliveryId/enrich", method: "POST" },
  "hooks.get": { path: "/hooks/:deliveryId", method: "GET" },
} as const;

export function createClient(baseUrl = "http://127.0.0.1:3000"): TreatyClient<RelayApi> {
  return treaty<RelayApi>({
    baseUrl,
    contract,
  });
}
