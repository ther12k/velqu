import { defineApp, defineModule } from "@velqu/core";
import { healthRoutes } from "./modules/health/routes";
import { hooksRoutes } from "./modules/hooks/routes";

/**
 * External consumer application (#1400): webhook relay.
 * Profile: serverless. Capabilities: fetch (declared, exercised).
 */
export const app = defineApp({
  id: "relay-service",
  modules: [
    defineModule({ id: "health", routes: healthRoutes }),
    defineModule({ id: "hooks", routes: hooksRoutes }),
  ],
});

export default app;
