# Deployment behind a reverse proxy

The private-alpha deployment profile is **reverse-proxy first**. Terminate
public TLS at a trusted edge proxy and keep the Velqu runtime on plain HTTP,
normally bound to loopback. This is a deployment posture, not a production
readiness or availability guarantee.

## Build and run the runtime privately

```bash
bun install --frozen-lockfile
bun packages/cli/src/index.ts build --project examples/proof
cargo build --release -p velqu-runtime
./target/release/velqu-runtime \
  --pack examples/proof/dist/app.qpack \
  --host 127.0.0.1 --port 3000
```

The runtime owns routing, readiness, request limits, deadlines, and graceful
shutdown. It does not load certificates or private keys in this beta.

## Example Nginx boundary

The following minimal server terminates TLS at Nginx and forwards only to a
loopback runtime. Replace the certificate paths and hostname for your
controlled environment:

```nginx
server {
    listen 443 ssl;
    server_name api.example.test;

    ssl_certificate     /etc/letsencrypt/live/api.example.test/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/api.example.test/privkey.pem;

    client_max_body_size 1m;
    proxy_connect_timeout 2s;
    proxy_read_timeout 30s;
    proxy_send_timeout 30s;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Forwarded-Proto https;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }

    location = /health/live {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
    }

    location = /health/ready {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
    }
}
```

The proxy must be the only public path to the runtime. Do not expose a plain
HTTP listener bound to `0.0.0.0` as a secure public deployment. Forwarded
headers are trusted only at the proxy boundary; the runtime does not add
forwarded-header parsing in this beta.

## Health, readiness, and drain

Use `/health/live` for process/listener liveness and `/health/ready` for
traffic admission. A deployment controller should remove a worker from the
proxy upstream before sending its shutdown signal, then allow in-flight
requests to drain within the configured budget. A readiness failure is not a
license to retry requests blindly; preserve the client's idempotency and
retry policy at the edge.

A safe rollout sequence is:

1. Start the new runtime on a private port and wait for `/health/ready`.
2. Add it to the proxy upstream and verify a real typed route.
3. Stop admitting traffic to the old runtime.
4. Send SIGTERM and wait for its bounded shutdown report.
5. Remove the old process only after the drain completes or the deployment
   timeout expires.

## Boundaries and limitations

- The proxy owns public TLS, certificate rotation, edge access logs, request
  size limits, connection limits, and external health checks.
- The runtime owns application routing, schema validation, bounded body/queue
  work, readiness, and shutdown.
- Native runtime TLS, HTTP/2 termination, certificate handling inside Velqu,
  and direct public exposure are not supported promises in this beta.
- Same-process QuickJS executes trusted application code only; it is not a
  hostile-code sandbox.
- `defer` is in-memory best-effort work, never a durable job queue.

## Verify locally

From the repository root:

```bash
bun install --frozen-lockfile
bun packages/cli/src/index.ts build --project examples/proof
cargo test -p velqu-runtime
bun run typecheck
bun run verify
```

The commands verify the runtime and proof artifacts. The Nginx block is a
configuration example; test it with your installed proxy and deployment
configuration before use. This guide does not claim production readiness.
