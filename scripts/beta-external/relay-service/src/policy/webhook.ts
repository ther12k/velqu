import { definePolicy, status } from "@velqu/core";

export interface HookAuth {
  token: string;
}

/**
 * Webhook token policy (mirror of the proof app's session policy shape).
 * Uses the standard `authorization` header, which the PUBLISHED beta.1
 * packages materialize correctly. Custom policy headers (e.g.
 * `x-hook-token`) were broken by a compiler bug — extraction dropped the
 * declared `header` and emit hardcoded "authorization" — fixed in-repo
 * by #1401 and usable from the next package publish.
 *
 * The token is a FIXTURE for this external-consumer exercise — real
 * deployments take it from their secret channel, never from source.
 */
export const webhookPolicy = definePolicy({
  id: "webhook.token",
  header: "authorization",
  declares: { 401: "unauthorized" },
  provides: "hook",
  check: async (req) => {
    if (req.headers.authorization !== "Bearer q-relay-demo-token") {
      return status(401).problem("unauthorized");
    }
    return { hook: { token: "q-relay-demo-token" } satisfies HookAuth };
  },
});

export default webhookPolicy;
