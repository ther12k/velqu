/** W4 candidate: Elysia 2 (AOT) on Bun + native fetch (pinned elysia@2.0.0-beta.4). */
import { Elysia } from "elysia";
import { PORT, UPSTREAM, validateMs } from "./shared";

const app = new Elysia({ aot: true })
  .get("/api/bench/io", async ({ query, set }) => {
    const ms = validateMs(query.ms ?? null);
    if (ms === null) {
      set.status = 400;
      return { error: "invalid ms" };
    }
    const upstream = await fetch(`${UPSTREAM}/io?ms=${ms}`);
    const body = await upstream.text();
    set.status = upstream.status;
    set.headers["content-type"] =
      upstream.headers.get("content-type") ?? "application/json";
    return body;
  })
  .listen(PORT);

console.log(
  JSON.stringify({ event: "candidate.ready", candidate: "elysia2", port: app.server?.port }),
);
