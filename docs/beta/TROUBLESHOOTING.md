# Troubleshooting (public beta)

Every symptom below was reproduced against this repository's build; the
messages and exit codes shown are the actual ones. Startup rejections
are fail-closed by design: nothing listens, and the process exits with
code 2 plus one structured `startup.rejected` line naming the stage.

## Read the ready line first

A successful start prints one JSON `ready` line naming the app, route
count, bind address, mode, engine, config sources, and per-stage
timings. When something is wrong you get `startup.rejected` instead —
the `stage` field tells you which layer refused.

## Startup rejections (exit code 2)

### Missing or unreadable pack

```text
{"level":"error","event":"startup.rejected","stage":"pack.load",
 "error":"pack io error: No such file or directory (os error 2)"}
```

Check the `--pack` path and that the file shipped with the correct
deployment artifact (shared mode needs `app.qpack` next to the binary;
standalone embeds it).

### Configuration file without `configVersion`

```text
{"level":"error","event":"startup.rejected","stage":"config.resolve",
 "error":"config file '/tmp/velqu.json' rejected: missing field `configVersion` at line 1 column 32"}
```

Add `"configVersion": 1` to the file (see `docs/beta/CONFIGURATION.md`).

### Unknown `VELQU_*` environment variable

```text
{"level":"error","event":"startup.rejected","stage":"config.resolve",
 "error":"unknown environment variable VELQU_BOOGABOOGA: the VELQU_* namespace is closed (see docs/beta/CONFIGURATION.md)"}
```

The namespace is closed: typos fail startup instead of being silently
ignored. Fix the name or remove the variable.

### Public bind in the default reverse-proxy mode

```text
{"level":"error","event":"startup.rejected","stage":"config.resolve",
 "error":"reverse-proxy mode requires a loopback bind; host \"0.0.0.0\" is public (use proxyMode=direct only when the operator owns the direct boundary)"}
```

Keep the runtime on `127.0.0.1` behind your proxy; `proxyMode: "direct"`
is an explicit operator opt-in (`docs/beta/DEPLOYMENT-REVERSE-PROXY.md`).

### Corrupted or tampered pack

```text
{"level":"error","event":"startup.rejected","stage":"pack.load",
 "error":"pack is not valid JSON: missing field `id` at line 1 column 41011"}
```

Packs are verified fail-closed; a flipped byte, truncation, or a pack
from a different build never "mostly works". Rebuild and re-ship the
pack (`velqu build`), or the full artifact pair after a runtime upgrade
(engine fingerprint is exact-match; see `docs/beta/INSTALL.md`).

## Build and toolchain errors

### `toolchain mismatch — byte-identical packs require the pinned toolchain`

The compiler refuses to emit packs unless the pinned toolchain matches,
so byte-identical rebuilds stay possible. Run `bun install
--frozen-lockfile` inside the checkout and build with the repository's
pinned versions.

### Scaffold cannot resolve `@velqu/*` or fails to bundle

`@velqu/*` packages are workspace-resolved (not on npm). Scaffold
inside a Velqu checkout and link the workspace packages:

```bash
mkdir -p hello-velqu/node_modules/@velqu
for p in core schema treaty; do
  ln -sfn "$(pwd)/packages/$p" "hello-velqu/node_modules/@velqu/$p"
done
```

(Reproduced during the beta docs work: scaffolding outside the checkout
fails with the toolchain-mismatch guard, by design.)

## Runtime behavior questions

- **`/health/ready` returns 503 `engine quarantined`** — the engine is
  unavailable and the runtime refuses traffic; see the health/drain
  section of `docs/beta/DEPLOYMENT-REVERSE-PROXY.md`. Do not retry
  blindly at the edge.
- **503 + `Retry-After: 1` during shutdown** — the drain gate flipped;
  in-flight work is finishing within the bounded budget.
- **Handler threw / deadline passed** — responses are typed problems
  (RFC 9457); unexpected errors are redacted before leaving the host,
  so the response will not contain internal details. Check the
  structured completion logs (field allowlist, sampled) for the
  internal view.
- **`eval`/`new Function` throws `TypeError: velqu: dynamic code
  execution is disabled`** — intentional (no dynamic code execution);
  remove dynamic code generation and rely on the compiled bundle.

## Still stuck

- `docs/beta/INSTALL.md` — build, run, deployment modes.
- `docs/beta/CONFIGURATION.md` — config layers, env namespace, secrets.
- `docs/beta/DEPLOYMENT-REVERSE-PROXY.md` — proxy posture, health,
  drain, rollout.
- `docs/beta/LIMITS-AND-NON-GOALS.md` — what the beta does not promise
  (non-SLA; no production-readiness claim; QPack bytecode is not
  native-machine-code JIT).
