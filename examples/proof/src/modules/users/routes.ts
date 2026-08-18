import { route, status } from "@q/core";
import { s } from "@q/schema";
import { resolve } from "./service";
import { sessionPolicy } from "../../policy/session";

export const create = route({
  id: "users.create",
  method: "POST",
  path: "/users",
  body: s.object({
    name: s.string({ minLength: 1, maxLength: 60 }),
    email: s.string({ format: "email" }),
  }),
  response: {
    201: s.object({ id: s.string(), name: s.string(), email: s.string() }),
  },
  handle: async ({ body }) => {
    // resolve() runs the lazy factory on first use (C5)
    const svc = resolve();
    const u = svc.create(body.name, body.email);
    return status(201).value(u);
  },
});

export const get = route({
  id: "users.get",
  method: "GET",
  path: "/users/:id",
  policy: sessionPolicy,
  params: s.object({ id: s.string({ pattern: "^usr_[0-9]+$" }) }),
  response: {
    200: s.object({ id: s.string(), name: s.string(), email: s.string() }),
  },
  handle: async ({ params }) => {
    const svc = resolve();
    const u = svc.get(params.id);
    if (!u) return status(404).problem("not-found", { detail: "user not found" });
    return u;
  },
});

export default [create, get] as const;
