import { defineApp, defineModule } from "@velqu/core";
import health from "./modules/health/routes";
import hello from "./modules/hello/routes";
import usersRoutes from "./modules/users/routes";
import itemsRoutes from "./modules/items/routes";
import authRoutes from "./modules/auth/routes";
import asyncRoutes from "./modules/async/routes";
import benchRoutes from "./modules/bench/routes";
import { sessionPolicy } from "./policy/session";
import { jwtPolicy } from "./policy/jwt";

export const app = defineApp({
  id: "proof",
  modules: [
    defineModule({ id: "health", routes: [health] }),
    defineModule({ id: "hello", routes: [hello] }),
    defineModule({ id: "users", routes: [...usersRoutes] }),
    defineModule({ id: "items", routes: [...itemsRoutes] }),
    defineModule({ id: "auth", routes: [...authRoutes] }),
    defineModule({ id: "async", routes: [asyncRoutes] }),
    defineModule({ id: "bench", routes: [...benchRoutes] }),
  ],
});

export default app;
export const policies = [sessionPolicy, jwtPolicy];
