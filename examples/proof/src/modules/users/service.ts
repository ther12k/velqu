import { defineService } from "@q/core";

export interface User {
  id: string;
  name: string;
  email: string;
}

/**
 * Lazy in-memory service: the factory runs on FIRST use at runtime (never
 * during compilation — COMP-002) and seeds the fixture user (C5). The
 * memoized `resolve` keeps state across requests within a process.
 */
export const usersService = defineService("users.service", () => {
  let nextUser = 1;
  const users = new Map<string, User>();
  users.set("usr_1", { id: "usr_1", name: "Ada", email: "ada@example.org" });
  return {
    get(id: string): User | undefined {
      return users.get(id);
    },
    create(name: string, email: string): User {
      const id = `usr_${nextUser++}`;
      const u = { id, name, email };
      users.set(id, u);
      return u;
    },
  };
});

let instance: ReturnType<typeof usersService.factory> | null = null;
export function resolve() {
  return (instance ??= usersService.factory());
}

export default usersService;
