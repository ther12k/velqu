import { route } from "@q/core";
import { s } from "@q/schema";

/**
 * Benchmark-contract routes required for matched comparisons
 * (benchmarks/fixtures/fixture-contract.json). Semantically part of the same
 * observable application as the PRD §13 proof routes.
 */
export const jsText = route({
  id: "js.text",
  method: "GET",
  path: "/js-text",
  response: { 200: s.string() },
  handle: async () => "plain",
});

export const jsJson = route({
  id: "js.json",
  method: "GET",
  path: "/js-json",
  response: { 200: s.object({ ok: s.boolean() }) },
  handle: async () => ({ ok: true }),
});

export const cancel = route({
  id: "async.cancel",
  method: "GET",
  path: "/cancel",
  query: s.object({
    ms: s.optional(s.integer({ minimum: 1, maximum: 5000 }), { default: 1000 }),
  }),
  response: { 200: s.object({ cancelled: s.boolean(), waited: s.integer() }) },
  handle: async ({ query, native }) => {
    const waited = await native.timer.delay(query.ms);
    return { cancelled: false, waited };
  },
});

export const throwRedacted = route({
  id: "throw.redacted",
  method: "GET",
  path: "/throw",
  response: { 200: s.object({}) },
  handle: async () => {
    throw new Error("secret-boom");
  },
});

export default [jsText, jsJson, cancel, throwRedacted] as const;
