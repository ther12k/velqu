import { defineApp, defineModule } from "@velqu/core";
import { healthRoutes } from "./modules/health/routes";
import { notesRoutes } from "./modules/notes/routes";

/**
 * External consumer application (#1404): notes CRUD over Postgres.
 * Profile: serverless. The postgres grant comes from handler usage
 * (ctx.native.postgres) — the compiler detects it and puts an exact
 * runtime:postgres v1 requirement in the pack.
 */
export const app = defineApp({
  id: "notes-service",
  modules: [
    defineModule({ id: "health", routes: healthRoutes }),
    defineModule({ id: "notes", routes: notesRoutes }),
  ],
});

export default app;
