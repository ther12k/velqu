# Changelog & Migration Notes — Velqu 0.1.0-beta.1

All notable changes and migration requirements for Velqu public beta releases are documented here.
This project adheres to [Semantic Versioning 2.0.0](https://semver.org/).
As defined in `docs/beta/01_BETA_DEFINITION.md`, prerelease versions carry no backward API/ABI stability guarantees, and every breaking change requires an explicit migration note.

---

## [0.1.0-beta.1] — 2026-09-04

### Initial Public Beta Release

The first public beta release of Velqu (Project Q) provides a high-performance HTTP runtime combining a native Rust host with QuickJS-NG (`0.15.1` via `rquickjs =0.12.2`) executing TypeScript application handlers compiled into immutable QPack artifacts.

#### Highlights & Capabilities
- **Single Contract Model**: One unified schema contract drives route types, runtime validation, Eden-style Treaty client dot-navigation, OpenAPI generation, and contract locking (`contract.lock.json`).
- **Zero-Copy Ingress & Lazy Materialization**: Rust routes by method and path natively; request fields materialize lazily only when accessed by handlers.
- **Strict Bounded Execution**: Queues, request bodies, stack (512 KiB), heap (32 MiB), and per-route execution deadlines are strictly bounded and fail closed.
- **No Dynamic Code Execution**: Pre-eval lockdown disables `eval`, `new Function`, and prototype constructor routes before application code executes.
- **Observability Baseline**: Bounded route metrics, structured completion logs with field allowlist, and worker operations status.
- **Deployment Boundaries**: Reverse-proxy-first loopback default (`proxyMode: "reverse-proxy"`), graceful drain within a 5-second budget with active cancellation and zero orphaned invocations.
- **First-Party Capabilities**:
  - `runtime:postgres@1`: Lazy zero-I/O connection pool with extended-protocol-only parameterized queries and secret redaction.
  - `@velqu/capability-auth-jwt`: HS256-only five-gate fail-closed token verification and keyring management.

---

### Migration Notes & Breaking Changes (from Alpha / M4A)

1. **Mandatory Versioned Configuration (`configVersion: 1`)**
   - **Change**: Unversioned configuration files passed via `--config` or `VELQU_CONFIG` are rejected fail-closed before ready.
   - **Migration**: Add `"configVersion": 1` to all `velqu.config.json` files.
   - **Example**:
     ```json
     {
       "configVersion": 1,
       "host": "127.0.0.1",
       "port": 3000,
       "proxyMode": "reverse-proxy"
     }
     ```

2. **Closed Environment Namespace (`VELQU_*`)**
   - **Change**: Any environment variable prefixed with `VELQU_` that is not recognized in the allowlist (`KNOWN_ENV_VARS`) will cause the runtime to reject startup immediately (exit code 2).
   - **Migration**: Verify and clean environment variables before launching `velqu-runtime`. Typos such as `VELQU_MAXQUEUE` must be corrected to `VELQU_MAX_QUEUE`.

3. **Disabled Dynamic Code Execution (`eval` / `Function`)**
   - **Change**: Invoking `eval(...)` or `new Function(...)` throws a `TypeError: velqu: dynamic code execution is disabled (...)`.
   - **Migration**: Remove dynamic code generation or `eval` patterns. Applications must rely on static TypeScript/JavaScript compilation into the verified QPack bundle.

4. **Reverse-Proxy Loopback Enforcement**
   - **Change**: By default, `velqu-runtime` binds to `127.0.0.1` and will reject public interface binds (such as `0.0.0.0`) unless `proxyMode: "direct"` is explicitly selected.
   - **Migration**: Terminate public TLS at a trusted reverse proxy (e.g. Nginx, Envoy, Caddy) and forward plain HTTP to loopback. Use `proxyMode: "direct"` only if the operator takes full responsibility for edge security.

5. **Forwarded Headers are Data, Never Identity**
   - **Change**: Ingress headers like `X-Forwarded-For`, `X-Forwarded-Proto`, and `Host` are treated as ordinary unauthenticated data. They are never used by the runtime for client identity, authorization, or routing.
   - **Migration**: When cross-proxy client identity is required, use signed application-layer tokens (such as `@velqu/capability-auth-jwt`).
