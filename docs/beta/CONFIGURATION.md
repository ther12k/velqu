# Runtime Configuration (BETA-007-A)

Typed configuration for the production hosts (`velqu-runtime`,
`velqu-standalone`). Every operational field resolves through the same
layer stack — first hit wins:

```text
CLI flag > environment > config file > built-in default
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
| `VELQU_CONFIG`         | (file selector)| ignored when `--config` is given           |

Invalid environment values (wrong type, out of range, unknown log
mode) reject startup — they are never silently ignored and never fall
back to a default. Historical note: an unparsable `PORT` used to be
silently ignored; as of BETA-007-A it is a typed startup rejection.

## What is NOT configurable here

- **Secrets.** Capability credentials such as `VELQU_DATABASE_URL`
  stay environment-only; they are never read by the configuration
  layer, never written to a config file, and never echoed in
  configuration errors (redaction-tested). Secret *value* handling has
  its own packet (BETA-007-C).
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
