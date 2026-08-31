/**
 * @velqu/cli — starter project scaffolding template (M4A-003-A).
 *
 * Provides minimal, correct starter templates without demo credentials
 * or forced databases:
 * - Module/service/contract separation.
 * - Standard health/liveness route and greetings API.
 * - Minimal workspace dependencies (@velqu/core, @velqu/schema).
 */

export interface ProjectTemplateOptions {
  name: string;
  appId?: string;
  description?: string;
}

export function generateStarterProject(opts: ProjectTemplateOptions): Record<string, string> {
  const appId = opts.appId ?? opts.name.replace(/[^a-zA-Z0-9_-]/g, "-").toLowerCase();
  const description = opts.description ?? "A lightweight service powered by Velqu and QuickJS";

  const packageJson = JSON.stringify(
    {
      name: opts.name,
      private: true,
      version: "0.1.0",
      description,
      type: "module",
      scripts: {
        dev: "velqu dev",
        build: "velqu build",
        check: "velqu check",
        test: "velqu test",
      },
      dependencies: {
        "@velqu/core": "workspace:*",
        "@velqu/schema": "workspace:*",
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
import { greetingsRoutes } from "./modules/greetings/routes";

export const app = defineApp({
  id: "${appId}",
  modules: [
    defineModule({ id: "health", routes: healthRoutes }),
    defineModule({ id: "greetings", routes: greetingsRoutes }),
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

  const readmeMd = `# ${opts.name}

${description}

## Getting Started

\`\`\`bash
# Start live development reload loop
bun run dev

# Build production QPack bundle
bun run build

# Static check
bun run check
\`\`\`
`;

  return {
    "package.json": packageJson,
    "tsconfig.json": tsconfigJson,
    "README.md": readmeMd,
    "src/app.ts": appTs,
    "src/modules/health/routes.ts": healthRoutesTs,
    "src/modules/greetings/routes.ts": greetingsRoutesTs,
    "src/modules/greetings/service.ts": greetingsServiceTs,
  };
}
