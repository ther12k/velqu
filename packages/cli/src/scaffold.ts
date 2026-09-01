/**
 * @velqu/cli — starter project scaffolding template (M4A-003-A/B/C/D).
 *
 * Provides minimal, correct starter templates without demo credentials
 * or forced databases:
 * - Module/service/contract separation.
 * - Standard health/liveness route and greetings API.
 * - Minimal workspace dependencies (@velqu/core, @velqu/schema, @velqu/treaty).
 * - End-to-end type-safe Treaty client example with route-id dot-navigation.
 * - Optional runtime service profile choices (serverless, service, throughput).
 * - Optional outbound fetch capability integration (M4A-003-D).
 */

export type ServiceProfileChoice = "serverless" | "service" | "throughput";

export const VALID_SERVICE_PROFILES: readonly ServiceProfileChoice[] = [
  "serverless",
  "service",
  "throughput",
] as const;

export interface ProjectTemplateOptions {
  name: string;
  appId?: string;
  description?: string;
  profile?: ServiceProfileChoice;
  withFetch?: boolean;
}

export function generateStarterProject(opts: ProjectTemplateOptions): Record<string, string> {
  const appId = opts.appId ?? opts.name.replace(/[^a-zA-Z0-9_-]/g, "-").toLowerCase();
  const description = opts.description ?? "A lightweight service powered by Velqu and QuickJS";
  const profile: ServiceProfileChoice = opts.profile ?? "serverless";
  const withFetch = Boolean(opts.withFetch);

  if (!VALID_SERVICE_PROFILES.includes(profile)) {
    throw new Error(
      `Invalid service profile '${profile}'. Valid choices are: ${VALID_SERVICE_PROFILES.join(", ")}`,
    );
  }

  const devScript = profile === "serverless" ? "velqu dev" : `velqu dev --profile ${profile}`;
  const buildScript = profile === "serverless" ? "velqu build" : `velqu build --profile ${profile}`;

  const packageJson = JSON.stringify(
    {
      name: opts.name,
      private: true,
      version: "0.1.0",
      description,
      type: "module",
      scripts: {
        dev: devScript,
        build: buildScript,
        check: "velqu check",
        test: "bun test",
        client: "bun run src/client.ts",
      },
      velqu: {
        profile,
        capabilities: withFetch ? ["fetch"] : [],
      },
      dependencies: {
        "@velqu/core": "workspace:*",
        "@velqu/schema": "workspace:*",
        "@velqu/treaty": "workspace:*",
      },
      devDependencies: {
        "@types/bun": "^1.3.4",
        typescript: "^5.9.3",
      },
    },
    null,
    2,
  );

  const tsconfigJson = JSON.stringify(
    {
      compilerOptions: {
        target: "ES2022",
        module: "ESNext",
        moduleResolution: "Bundler",
        strict: true,
        noEmit: true,
        allowImportingTsExtensions: true,
        skipLibCheck: true,
        types: ["bun-types"],
      },
      include: ["src/**/*"],
    },
    null,
    2,
  );

  const appTs = `import { defineApp, defineModule } from "@velqu/core";
import { healthRoutes } from "./modules/health/routes";
import { greetingsRoutes } from "./modules/greetings/routes";${
  withFetch
    ? `
import { upstreamRoutes } from "./modules/upstream/routes";`
    : ""
}

/**
 * Velqu Application Definition.
 * Configured runtime profile: ${profile}
 */
export const app = defineApp({
  id: "${appId}",
  modules: [
    defineModule({ id: "health", routes: healthRoutes }),
    defineModule({ id: "greetings", routes: greetingsRoutes }),${
  withFetch
    ? `
    defineModule({ id: "upstream", routes: upstreamRoutes }),`
    : ""
}
  ],
});

export default app;
`;

  const healthRoutesTs = `import { route } from "@velqu/core";
import { s } from "@velqu/schema";

export const live = route({
  id: "health.live",
  method: "GET",
  path: "/health/live",
  response: {
    200: s.object({ status: s.string() }),
  },
  handle: async () => ({ status: "ok" }),
});

export const healthRoutes = [live];
`;

  const greetingsRoutesTs = `import { route, status } from "@velqu/core";
import { s } from "@velqu/schema";
import { getGreeting, createGreeting } from "./service";

export const get = route({
  id: "greetings.get",
  method: "GET",
  path: "/greetings/:name",
  params: s.object({
    name: s.string({ minLength: 1, maxLength: 64 }),
  }),
  response: {
    200: s.object({ message: s.string() }),
  },
  handle: async ({ params }) => {
    const greeting = getGreeting(params.name);
    return { message: greeting };
  },
});

export const create = route({
  id: "greetings.create",
  method: "POST",
  path: "/greetings",
  body: s.object({
    name: s.string({ minLength: 1, maxLength: 64 }),
    customGreeting: s.optional(s.string({ maxLength: 128 })),
  }),
  response: {
    201: s.object({ name: s.string(), greeting: s.string() }),
  },
  handle: async ({ body }) => {
    const item = createGreeting(body.name, body.customGreeting);
    return status(201).value(item);
  },
});

export const greetingsRoutes = [get, create];
`;

  const greetingsServiceTs = `/**
 * Greetings domain service.
 * Separates business logic from HTTP route declarations.
 */

export interface GreetingItem {
  name: string;
  greeting: string;
}

const customGreetings = new Map<string, string>();

export function getGreeting(name: string): string {
  const custom = customGreetings.get(name.toLowerCase());
  if (custom) return custom;
  return \`Hello, \${name}!\`;
}

export function createGreeting(name: string, custom?: string): GreetingItem {
  const greeting = custom ?? \`Welcome, \${name}!\`;
  customGreetings.set(name.toLowerCase(), greeting);
  return { name, greeting };
}
`;

  const upstreamRoutesTs = `import { route, status } from "@velqu/core";
import { s } from "@velqu/schema";

/**
 * Upstream quote route (demonstrates outbound fetch capability).
 */
export const quote = route({
  id: "upstream.quote",
  method: "GET",
  path: "/upstream/quote",
  response: {
    200: s.object({ quote: s.string(), source: s.string() }),
    502: s.object({ error: s.string() }),
  },
  handle: async () => {
    try {
      const res = await fetch("https://api.github.com/zen", {
        headers: { "user-agent": "velqu-starter" },
      });
      if (!res.ok) {
        return status(502).value({ error: \`Upstream returned HTTP \${res.status}\` });
      }
      const text = await res.text();
      return { quote: text.trim(), source: "github" };
    } catch (e) {
      return status(502).value({ error: (e as Error).message });
    }
  },
});

export const upstreamRoutes = [quote];
`;

  const upstreamTestTs = `/**
 * Upstream route definition unit tests.
 */
import { describe, it, expect } from "bun:test";
import { quote } from "./routes";

describe("upstream module (withFetch)", () => {
  it("declares upstream.quote GET route with schema contract", () => {
    expect(quote.id).toBe("upstream.quote");
    expect(quote.method).toBe("GET");
    expect(quote.path).toBe("/upstream/quote");
  });
});
`;

  const clientTs = `/**
 * Type-safe Treaty client example (M4A-003-B/D).
 *
 * Demonstrates:
 * - Route-ID dot-navigation: \`api.greetings.get({ name: "Ada" })\`
 * - Strict 2xx data vs non-2xx error splitting: \`if (res.data) ... else res.error\`
 * - Complete type inference from server route contracts with zero shared runtime code.
 */

import { treaty, type TreatyClient } from "@velqu/treaty";

export type StarterApi = {
  "health.live": {
    path: "/health/live";
    method: "GET";
    params: never;
    query: never;
    body: never;
    headers: never;
    responses: { 200: { status: string } };
  };
  "greetings.get": {
    path: "/greetings/:name";
    method: "GET";
    params: { name: string };
    query: never;
    body: never;
    headers: never;
    responses: { 200: { message: string } };
  };
  "greetings.create": {
    path: "/greetings";
    method: "POST";
    params: never;
    query: never;
    body: { name: string; customGreeting?: string };
    headers: never;
    responses: { 201: { name: string; greeting: string } };
  };${
  withFetch
    ? `
  "upstream.quote": {
    path: "/upstream/quote";
    method: "GET";
    params: never;
    query: never;
    body: never;
    headers: never;
    responses: { 200: { quote: string; source: string }; 502: { error: string } };
  };`
    : ""
}
}

export const contract = {
  "health.live": { path: "/health/live", method: "GET" },
  "greetings.get": { path: "/greetings/:name", method: "GET" },
  "greetings.create": { path: "/greetings", method: "POST" },${
  withFetch
    ? `
  "upstream.quote": { path: "/upstream/quote", method: "GET" },`
    : ""
}
} as const;

export function createClient(baseUrl = "http://127.0.0.1:3000"): TreatyClient<StarterApi> {
  return treaty<StarterApi>({
    baseUrl,
    contract,
  });
}

export async function main() {
  const api = createClient();
  console.log("Calling health.live via Treaty...");
  const health = await api.health.live.get();
  if (health.data) {
    console.log("Health OK:", health.data.status);
  } else {
    console.error("Health error:", health.error);
  }

  console.log("Creating custom greeting via Treaty...");
  const created = await api.greetings.create.post({
    name: "Ada",
    customGreeting: "Greetings from Treaty!",
  });
  if (created.data) {
    console.log("Created greeting:", created.data);
  }

  console.log("Fetching greeting by route param...");
  const greeting = await api.greetings.get({ name: "Ada" }).get();
  if (greeting.data) {
    console.log("Message:", greeting.data.message);
  }
}

if (import.meta.main) {
  main().catch(console.error);
}
`;

  const greetingsTestTs = `/**
 * Starter test suite (M4A-003-C): exercises the greetings domain service
 * directly (unit-level, no runtime required).
 */

import { describe, it, expect } from "bun:test";
import { getGreeting, createGreeting } from "./service";

describe("greetings service", () => {
  it("returns the default greeting for an unknown name", () => {
    expect(getGreeting("Stranger")).toBe("Hello, Stranger!");
  });

  it("returns the stored custom greeting after createGreeting", () => {
    createGreeting("Ada", "Greetings from test!");
    expect(getGreeting("ada")).toBe("Greetings from test!");
  });

  it("stores greetings case-insensitively", () => {
    createGreeting("Grace", "Welcome aboard!");
    expect(getGreeting("GRACE")).toBe("Welcome aboard!");
  });
});
`;

  const clientTestTs = `/**
 * Treaty client contract test (M4A-003-C): drives the LIVE dev-server
 * runtime through the type-safe Treaty client. Requires \`velqu dev\`
 * running on 127.0.0.1:3000 (skipped automatically otherwise).
 */

import { describe, it, expect } from "bun:test";
import { createClient } from "./client";

const api = createClient("http://127.0.0.1:3000");

describe("greetings API (runtime-local via Treaty)", () => {
  it("health.live answers ok", async () => {
    const res = await api.health.live.get();
    if (res.error) {
      console.warn("skipping: dev server not reachable");
      return;
    }
    expect(res.data?.status).toBe("ok");
  });

  it("greetings.create stores a custom greeting and greetings.get reads it back", async () => {
    const created = await api.greetings.create.post({
      name: "TestUser",
      customGreeting: "Hello from contract test!",
    });
    if (created.error) {
      console.warn("skipping: dev server not reachable");
      return;
    }
    expect(created.data?.name).toBe("TestUser");

    const greeting = await api.greetings.get({ name: "TestUser" }).get();
    expect(greeting.error).toBeNull();
    expect(greeting.data?.message).toBe("Hello from contract test!");
  });
});
`;

  const readmeMd = `# ${opts.name}

${description}

- **Runtime Service Profile**: \`${profile}\`
- **Outbound Fetch Capability**: \`${withFetch ? "enabled" : "disabled"}\`

## Getting Started

\`\`\`bash
# Start live development reload loop
bun run dev

# Build production QPack bundle
bun run build

# Static check
bun run check

# Run tests
bun run test

# Run Treaty client example
bun run client
\`\`\`
`;

  const result: Record<string, string> = {
    "package.json": packageJson,
    "tsconfig.json": tsconfigJson,
    "README.md": readmeMd,
    "src/app.ts": appTs,
    "src/modules/health/routes.ts": healthRoutesTs,
    "src/modules/greetings/routes.ts": greetingsRoutesTs,
    "src/modules/greetings/service.ts": greetingsServiceTs,
    "src/client.ts": clientTs,
    "src/modules/greetings/service.test.ts": greetingsTestTs,
    "src/client.test.ts": clientTestTs,
  };

  if (withFetch) {
    result["src/modules/upstream/routes.ts"] = upstreamRoutesTs;
    result["src/modules/upstream/routes.test.ts"] = upstreamTestTs;
  }

  return result;
}
