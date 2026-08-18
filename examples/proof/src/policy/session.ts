import { definePolicy, status } from "@velqu/core";

export interface Session {
  userId: string;
}

/** C4 vertical slice: bearer fixture, typed 401, session context (SCHEMA-004). */
export const sessionPolicy = definePolicy({
  id: "auth.session",
  header: "authorization",
  declares: { 401: "unauthorized" },
  provides: "session",
  check: async (req) => {
    if (req.headers.authorization !== "Bearer q-demo-token") {
      return status(401).problem("unauthorized");
    }
    return { session: { userId: "usr_1" } satisfies Session };
  },
});

export default sessionPolicy;
