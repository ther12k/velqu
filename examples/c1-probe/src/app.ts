/**
 * C1-A diagnostic probe app (non-canonical, measurement only).
 *
 * Four routes isolating the response-path difference between C1 (text)
 * and C2 (JSON) with the Promise-settlement variable split out:
 *   diag.text.sync  /diag/text-sync   () => "plain"          s.string()
 *   diag.text.async /diag/text-async  async () => "plain"    s.string()
 *   diag.json.sync  /diag/json-sync   () => ({ok:true})      s.object({ok})
 *   diag.json.async /diag/json-async  async () => ({ok:true}) same
 *
 * The canonical fixtures (examples/proof /js-text, /js-json) are untouched;
 * this app never enters gate thresholds, conformance, or benchmark manifests.
 */
import { route } from "@velqu/core";
import { s } from "@velqu/schema";

export const textSync = route({
  id: "diag.text.sync",
  method: "GET",
  path: "/diag/text-sync",
  response: { 200: s.string() },
  handle: () => "plain",
});

export const textAsync = route({
  id: "diag.text.async",
  method: "GET",
  path: "/diag/text-async",
  response: { 200: s.string() },
  handle: async () => "plain",
});

export const jsonSync = route({
  id: "diag.json.sync",
  method: "GET",
  path: "/diag/json-sync",
  response: { 200: s.object({ ok: s.boolean() }) },
  handle: () => ({ ok: true }),
});

export const jsonAsync = route({
  id: "diag.json.async",
  method: "GET",
  path: "/diag/json-async",
  response: { 200: s.object({ ok: s.boolean() }) },
  handle: async () => ({ ok: true }),
});

export const app = { routes: [textSync, textAsync, jsonSync, jsonAsync] };
