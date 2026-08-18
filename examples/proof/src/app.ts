import { defineApp, defineModule } from "@velqu/core";
import health from "./modules/health/routes";
import hello from "./modules/hello/routes";
import usersRoutes from "./modules/users/routes";
import asyncRoutes from "./modules/async/routes";
import benchRoutes from "./modules/bench/routes";
import { sessionPolicy } from "./policy/session";

export const app = defineApp({
  id: "proof",
  modules: [
    defineModule({ id: "health", routes: [health] }),
    defineModule({ id: "hello", routes: [hello] }),
    defineModule({ id: "users", routes: [...usersRoutes] }),
    defineModule({ id: "async", routes: [asyncRoutes] }),
    defineModule({ id: "bench", routes: [...benchRoutes] }),
  ],
});

export default app;
export const policies = [sessionPolicy];
