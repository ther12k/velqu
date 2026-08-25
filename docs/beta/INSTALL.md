# Installing Velqu — shared mode (`velqu-runtime` + app.qpack)

Shared mode deploys two files: the runtime binary and your compiled
application pack. It suits small app updates — ship a new `app.qpack`
without touching the runtime.

## Prerequisites

- Linux x86_64 (beta target).
- The two artifacts below, from a trusted build (see ADR-0026 for why
  authenticity checks live in your deployment pipeline, not the pack).

## Files

| file | produced by | notes |
|---|---|---|
| `velqu-runtime` | `cargo build --release -p velqu-runtime` | static Rust binary; embeds quickjs-ng 0.15.1 via rquickjs 0.12.2 |
| `app.qpack` | `velqu build --project <dir> --out <dir>` | verified application pack; deterministic bytes |

## Run

```bash
velqu-runtime --pack app.qpack --port 8080
# optional: --host 127.0.0.1 --config config.json
```

The process exits non-zero with a structured diagnostic if anything
fails before ready — including a pack built for a different runtime
build ("engine mismatch … SEC-001 exact match"). A pack only runs on
the exact runtime build it was compiled against; upgrading the runtime
means rebuilding/re-shipping the pack.

## Updating

| change | action |
|---|---|
| app code/routes | rebuild + replace `app.qpack`, restart |
| runtime upgrade | rebuild both artifacts together; fingerprint must match |

## Limits

Defaults (config-overridable): body 1 MiB, header 32 KiB, URI 8 KiB,
queue 256 concurrent, heap 32 MiB, stack 512 KiB, handler deadline 5 s,
pending ops 1024.

Standalone single-file deployment (runtime with embedded pack) is a
separate M26-009 deliverable and is not part of shared mode.
