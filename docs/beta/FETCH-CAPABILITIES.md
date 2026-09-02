# Fetch and capabilities

Velqu's private-alpha fetch surface is an explicit capability. It is not
available by accident: a project opts in with the CLI scaffold and the
compiler records the capability in its build artifacts.

## Opt in from a scaffold

```bash
bun packages/cli/src/index.ts init /tmp/velqu-fetch \
  --name velqu-fetch --with-fetch
cd /tmp/velqu-fetch
bun install
bun run check
bun run test
```

`--with-fetch` adds an `upstream` module and records `fetch` in the generated
`velqu.capabilities` metadata. The generated route is a fixture demonstrating
an outbound request; it is not a promise that a public endpoint is available
or that network egress is enabled in every deployment.

## Handler example

```ts
import { route, status } from "@velqu/core";
import { s } from "@velqu/schema";

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
      const response = await fetch("https://api.github.com/zen", {
        headers: { "user-agent": "velqu-starter" },
      });
      if (!response.ok) return status(502).value({ error: `upstream ${response.status}` });
      return { quote: (await response.text()).trim(), source: "github" };
    } catch {
      return status(502).value({ error: "upstream unavailable" });
    }
  },
});
```

Keep upstream failures typed and bounded. Do not expose exception text or
secrets in a public response. Use explicit timeouts, response-size limits, and
an allowlisted egress policy for a real integration; the scaffold example is a
small educational fixture.

## Capability and security boundary

Fetch is subject to the runtime capability linker and the deployment's
network-egress policy. The host owns DNS/TLS, cancellation, deadlines, body
limits, and resource accounting; application code receives the Web-compatible
surface only. Requests are not a general proxy and SSRF protections remain a
runtime/deployment concern.

The capability manifest emitted by `velqu build` is evidence of what the pack
requests, not proof that an external network is reachable. A project without
fetch does not initialize fetch-specific resources.

## Verify

From the repository root:

```bash
bun install --frozen-lockfile
bun test packages/cli/src/scaffold.test.ts conformance/web-api
bun run typecheck
bun packages/cli/src/index.ts build --project examples/proof
```

The scaffold tests verify the optional module and capability metadata; the
proof build verifies artifact generation. This private-alpha documentation is
not a production-readiness claim. Same-process QuickJS executes trusted code
only and is not a hostile-code sandbox.
