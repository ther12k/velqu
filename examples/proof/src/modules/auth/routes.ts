import { route } from "@velqu/core";
import { s } from "@velqu/schema";
import { jwtPolicy, issueToken, JWT_DEMO_SECRET, type Session } from "../../policy/jwt";

/**
 * JWT-like reference routes (M4A-009-B): an educational login fixture that
 * issues a signed reference token, and a protected profile route guarded by
 * the JWT-like policy with a typed session.
 */
export const login = route({
  id: "auth.login",
  method: "POST",
  path: "/auth/login",
  body: s.object({
    username: s.string({ minLength: 1, maxLength: 60 }),
    // Demo fixture only — never accept raw secrets for a real deployment.
    demoSecret: s.string({ minLength: 1, maxLength: 128 }),
  }),
  response: {
    200: s.object({ token: s.string(), note: s.string() }),
    401: s.object({ missing: s.boolean() }),
  },
  handle: async ({ body }) => {
    if (body.demoSecret !== JWT_DEMO_SECRET) {
      // declared 401: wrong fixture credential
      const problem = { missing: true };
      return { __problem: true, problem: "unauthorized", status: 401 as const, detail: "bad credentials", ...problem };
    }
    const token = await issueToken({ sub: `usr_${body.username}`, scope: "items:read profile:read" });
    return {
      token,
      note: "reference token for the JWT-like policy demo (fixture only)",
    };
  },
});

export const profile = route({
  id: "auth.profile",
  method: "GET",
  path: "/auth/profile",
  policy: jwtPolicy,
  response: {
    200: s.object({
      userId: s.string({ minLength: 1, maxLength: 60 }),
      scope: s.string({ maxLength: 120 }),
    }),
  },
  handle: async ({ session }) => {
    const s = session as Session;
    return { userId: s.userId, scope: s.scope };
  },
});

export default [login, profile] as const;
