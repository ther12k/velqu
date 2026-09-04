# Runtime Configuration (BETA-007-A / BETA-008-A)

Typed configuration for the production hosts (`velqu-runtime`,
`velqu-standalone`). Every operational field resolves through the same
layer stack — first hit wins:

```text
CLI flag > environment > active profile > config file > built-in default
```

Invalid configuration **fails closed before ready**: startup prints a
typed `startup.rejected` line (`stage: "config.resolve"`) and exits
with code 2. Values are never clamped into an allowed range — an
out-of-range value is a rejected startup, not a silent correction.

## Built-in defaults

| Field          | Default     | Allowed range                          |
| -------------- | ----------- | -------------------------------------- |
| `host`         | `127.0.0.1` | 1–253 bytes printable ASCII, no whitespace/control characters |
| `port`         | `3000`      | `1..=65535` (0 is rejected)            |
| `maxBodyBytes` | `1048576` (1 MiB) | `1..=67108864` (64 MiB)          |
| `maxQueue`     | `256`       | `1..=10000`                            |
| `log`          | `errors`    | closed set: `off` \| `errors` \| `full` (case-insensitive) |
| `logSample`    | `0` (disabled) | `0..=1000000000`                    |
| `proxyMode`    | `reverse-proxy` | `reverse-proxy` (loopback-only) \| `direct` (operator opt-in) |

## Config file (versioned)

Selected with `--config <path>` or the `VELQU_CONFIG` environment
variable (`--config` wins). The file is JSON, and `configVersion` is
**required** — unversioned legacy files are rejected. Unknown fields
are rejected outright so a typo can never silently disable a limit.

Example (`examples/config/velqu.config.json` — parse-tested in CI):

```json
{
  "configVersion": 1,
  "host": "127.0.0.1",
  "port": 3000,
  "maxBodyBytes": 2097152,
  "maxQueue": 512,
  "log": "errors",
  "logSample": 0
}
```

Every field except `configVersion` is optional; omitted fields fall
through to the next layer. The only supported version is `1`; a file
declaring any other version is rejected at startup.

## Environment variables

| Variable               | Field          | Notes                                      |
| ---------------------- | -------------- | ------------------------------------------ |
| `VELQU_PORT`           | `port`         | wins over the legacy `PORT` variable       |
| `PORT`                 | `port`         | legacy compatibility, still strict         |
| `VELQU_HOST`           | `host`         |                                            |
| `VELQU_LOG`            | `log`          | `off` \| `errors` \| `full`                |
| `VELQU_LOG_SAMPLE`     | `logSample`    | sample successful completions every N      |
| `VELQU_MAX_BODY_BYTES` | `maxBodyBytes` |                                            |
| `VELQU_MAX_QUEUE`      | `maxQueue`     |                                            |
| `VELQU_PROFILE`        | (profile select)| BETA-007-D; wins over the file's `activeProfile` |
| `VELQU_PROXY_MODE`     | `proxyMode`    | `reverse-proxy` (default) or `direct`       |
| `VELQU_CONFIG`         | (file selector)| ignored when `--config` is given           |

Invalid environment values (wrong type, out of range, unknown log
mode) reject startup — they are never silently ignored and never fall
back to a default. Historical note: an unparsable `PORT` used to be
silently ignored; as of BETA-007-A it is a typed startup rejection.

## Closed environment namespace (BETA-007-B)

The `VELQU_*` namespace is **closed**: at startup the runtime checks
every `VELQU_*` variable in the environment against the allowlist
below, and an unknown name rejects startup with a typed error. A
typo'd knob (`VELQU_MAXQUEUE`) must fail before ready, never be
silently ignored. Values of unknown names are never read or echoed —
they may be secrets. Names outside the `VELQU_*` namespace are not the
runtime's concern, and the check is case-sensitive.

| Group | Names |
| ------------------- | ------------------------------------------------ |
| Runtime configuration | `VELQU_CONFIG`, `VELQU_HOST`, `VELQU_LOG`, `VELQU_LOG_SAMPLE`, `VELQU_MAX_BODY_BYTES`, `VELQU_MAX_QUEUE`, `VELQU_PORT`, `VELQU_PROFILE`, `VELQU_PROXY_MODE` |
| Postgres capability | `VELQU_DATABASE_URL`, `VELQU_PG_POOL_MAX`, `VELQU_PG_POOL_CONNECT_TIMEOUT_MS`, `VELQU_PG_POOL_IDLE_TIMEOUT_MS` |
| Build-time | `VELQU_STANDALONE_PACK` |
| Tooling-only (never consumed by the serving runtime; recognized for dev/test convenience) | `VELQU_ALLOC_PROFILE`, `VELQU_BENCH_DEBUG`, `VELQU_PACK`, `VELQU_PG_LIVE_TEST`, `VELQU_RUNTIME`, `VELQU_TEST_TRUST_KEYS` |

```text
{"level":"error","event":"startup.rejected","stage":"config.resolve",
 "error":"unknown environment variable VELQU_MAXQUEUE: the VELQU_* namespace is closed (see docs/beta/CONFIGURATION.md)"}
```

## Startup validation report

After configuration validates, the `ready` line carries a `config`
block — the resolved, non-secret values plus the layer each field came
from (`cli` | `env` | `file` | `default`) — so a mis-applied layer is
visible at startup:

```text
"config": {
  "host": "127.0.0.1", "hostSource": "default",
  "port": 8080, "portSource": "env",
  "maxBodyBytes": 1048576, "maxBodyBytesSource": "default",
  "maxQueue": 512, "maxQueueSource": "file",
  "log": "errors", "logSource": "default",
  "logSample": 0, "logSampleSource": "default"
}
```

The block's keys are a fixed allowlist (test-enforced); it never
contains credentials or file contents. It also reports the deployment
boundary as `proxyMode` and `proxyModeSource`.

## Trusted proxy boundary (BETA-008-A)

The safe default is `proxyMode: "reverse-proxy"`, which requires a loopback
bind and is intended for public TLS termination at a trusted edge proxy.
`proxyMode: "direct"` is an explicit operator opt-in; it does not make
forwarded headers trusted and does not add runtime TLS. See
`docs/beta/governance/TRUSTED_PROXY_RUNBOOK.md` for the boundary checklist,
rollout, shutdown, and failure diagnosis.

## Profile-specific settings (BETA-007-D)

The config file may declare named profile blocks and select one as
active. The active profile overlays the file's base fields; the layer
stack stays `CLI > env > profile > file > default`.

```json
{
  "configVersion": 1,
  "activeProfile": "production",
  "maxQueue": 256,
  "profiles": {
    "production": { "log": "errors", "maxQueue": 512 },
    "development": { "log": "full", "maxQueue": 64 }
  }
}
```

- `profiles` maps names to blocks accepting the same optional fields
  as the file (`host`, `port`, `maxBodyBytes`, `maxQueue`, `log`,
  `logSample`). Unknown fields inside a block — including a nested
  `profiles` — reject the file.
- The active profile is selected by `VELQU_PROFILE` (wins) or the
  file's `activeProfile`. Selecting an undeclared name rejects
  startup with a typed error naming the declared set.
- Profile names are a closed shape: 1..=32 characters of `a-z`,
  `0-9`, `-`; all declared names are validated even when inactive.
- Every profile field value passes the same bounds and closed sets as
  anywhere else (out-of-range rejects, never clamps).
- The `config` block of the ready line reports the applied profile
  (`activeProfile`) and `profile` provenance on the fields it set.
- With no `VELQU_PROFILE` and no `activeProfile`, no profile layer
  applies — the plain file behavior is unchanged.

## Secret value wrapper (BETA-007-C)

Capability secrets (the database URL) are wrapped in a typed
`SecretString` the moment they are read from the environment:

- `Debug` and `Display` always render `[redacted]` — any accidental
  log, error, or inspect of a holder cannot disclose the value.
- The only read path is the explicit `expose()` (grep-auditable); the
  value flows from the wrapper straight into the pool constructor.
- No `Clone`, no equality: secret material is neither duplicated nor
  compared.
- The startup `ready`/`startup.rejected` lines name the variable, not
  the value; pool-side error rendering additionally redacts URL
  fragments (BETA-004).

Memory zeroization on drop is deliberately not claimed — the
guarantee is redaction across formatting and serialization paths.

## What is NOT configurable here

- **Secrets.** Capability credentials such as `VELQU_DATABASE_URL`
  stay environment-only; they are never read by the configuration
  layer, never written to a config file, and never echoed in
  configuration errors (redaction-tested).
- **Behavioral profiles.** `--service-profile` and
  `--context-profile` remain explicit CLI-only decisions; there is no
  profile file (BETA-007-D covers profile-specific settings).

## Examples

```bash
# env-only deployment (no config file):
VELQU_PORT=8080 VELQU_MAX_QUEUE=1024 VELQU_LOG=full velqu-runtime --pack app.qpack

# file-based deployment:
velqu-runtime --pack app.qpack --config /etc/velqu/velqu.config.json

# mixed: file supplies limits, env overrides the port
VELQU_PORT=8080 velqu-runtime --pack app.qpack --config /etc/velqu/velqu.config.json
```

Rejected startup example:

```text
{"level":"error","event":"startup.rejected","stage":"config.resolve",
 "error":"maxQueue from env:VELQU_MAX_QUEUE must be in range 1..=10000, got 10001 (values are never clamped)"}
```
