#!/usr/bin/env bash
# BETA-016-E — external-user deployment of the proof service.
#
# Mirrors the documented deployment shape (docs/beta/INSTALL.md +
# docker-compose.beta.yml): the proof app is built from the installed
# tree, the production runtime serves the pack in shared mode bound to
# loopback (reverse-proxy-first), an edge proxy is published on a
# loopback port by the platform operator, and the service is verified
# through the edge. Rollback removes the edge site, stops the service,
# and removes build artifacts.
#
# Subcommands (fail-closed, step-numbered):
#   app      — as the unprivileged user: build pack, run runtime (background)
#   edge     — as root: install + configure nginx edge on :8080
#   verify   — as any user: probe the edge (and direct loopback)
#   rollback — as root: remove edge site; stops service, removes artifacts
set -euo pipefail

INSTALL="${VELQU_INSTALL:-$HOME/velqu}"
APP="${VELQU_APP:-$INSTALL/examples/proof}"
PACK="$APP/dist/app.qpack"
RUNTIME="$INSTALL/target/release/velqu-runtime"
EDGE_PORT=8080
UPSTREAM_PORT=3000
NGINX_SITE=/etc/nginx/sites-available/velqu-proof
NGINX_ENABLED=/etc/nginx/sites-enabled/velqu-proof
PIDFILE="$HOME/proof-service.pid"
STEP=0
step() { STEP=$((STEP+1)); echo "== [$STEP] \$ $*"; }
fail() { echo "DEPLOY-FAIL at step $[$STEP]: $*" >&2; exit 1; }

case "${1:-}" in
  app)
    [ "$(id -un)" = "beta" ] || { echo "run 'app' as the unprivileged 'beta' user" >&2; exit 1; }
    echo "== external deploy transcript (app) =="

    step "cd $INSTALL && bun packages/cli/src/index.ts build --project examples/proof"
    (cd "$INSTALL" && bun packages/cli/src/index.ts build --project examples/proof) \
      || fail "proof pack build failed"
    [ -f "$PACK" ] || fail "app.qpack missing after build"

    step "$RUNTIME --pack $PACK --port $UPSTREAM_PORT --proxy-mode reverse-proxy (background)"
    "$RUNTIME" --pack "$PACK" --port "$UPSTREAM_PORT" --proxy-mode reverse-proxy \
      --log off >"$HOME/proof-service.log" 2>&1 &
    echo $! > "$PIDFILE"

    step "wait for readiness on 127.0.0.1:$UPSTREAM_PORT"
    for _ in $(seq 1 50); do
      curl -sf "http://127.0.0.1:$UPSTREAM_PORT/health/ready" >/dev/null 2>&1 && break
      sleep 0.2
    done
    curl -sf "http://127.0.0.1:$UPSTREAM_PORT/health/ready" >/dev/null || fail "service not ready"
    grep -Fq '"proxyMode":"reverse-proxy"' "$HOME/proof-service.log" \
      || fail "runtime did not report reverse-proxy mode"
    echo "APP-OK pid=$(cat "$PIDFILE") upstream=127.0.0.1:$UPSTREAM_PORT (loopback only)"
    ;;

  edge)
    [ "$(id -u)" = "0" ] || { echo "run 'edge' as root (operator provisioning)" >&2; exit 1; }
    echo "== external deploy transcript (edge) =="

    step "apt-get install -y nginx (operator tooling)"
    apt-get update -qq && apt-get install -y -qq nginx >/dev/null || fail "nginx install failed"

    step "write edge site $NGINX_SITE (loopback :$EDGE_PORT -> 127.0.0.1:$UPSTREAM_PORT)"
    cat > "$NGINX_SITE" <<EOF
server {
    listen 127.0.0.1:$EDGE_PORT;
    server_name _;
    location / {
        proxy_pass http://127.0.0.1:$UPSTREAM_PORT;
        proxy_set_header Host \$host;
    }
}
EOF
    ln -sf "$NGINX_SITE" "$NGINX_ENABLED"
    nginx -t 2>/dev/null || fail "nginx config invalid"
    service nginx restart >/dev/null 2>&1 || nginx >/dev/null 2>&1 || fail "nginx start failed"
    echo "EDGE-OK 127.0.0.1:$EDGE_PORT -> 127.0.0.1:$UPSTREAM_PORT"
    ;;

  verify)
    echo "== external deploy transcript (verify) =="
    step "GET http://127.0.0.1:$EDGE_PORT/health/live (through edge)"
    OUT="$(curl -sf --max-time 5 http://127.0.0.1:$EDGE_PORT/health/live)" || fail "edge /health/live failed"
    echo "$OUT"
    case "$OUT" in *'"status":"ok"'*) ;; *) fail "unexpected live body: $OUT";; esac

    step "GET http://127.0.0.1:$EDGE_PORT/hello/beta (through edge)"
    OUT="$(curl -sf --max-time 5 http://127.0.0.1:$EDGE_PORT/hello/beta)" || fail "edge /hello/beta failed"
    echo "$OUT"
    case "$OUT" in *'"message":"Hello beta"'*) ;; *) fail "unexpected hello body: $OUT";; esac

    step "GET http://127.0.0.1:$EDGE_PORT/health/ready (through edge)"
    OUT="$(curl -sf --max-time 5 http://127.0.0.1:$EDGE_PORT/health/ready)" || fail "edge /health/ready failed"
    echo "$OUT"
    case "$OUT" in *ready*) ;; *) fail "unexpected ready body: $OUT";; esac
    echo "VERIFY-OK: proof service reachable through the edge"
    ;;

  rollback)
    [ "$(id -u)" = "0" ] || { echo "run 'rollback' as root (operator provisioning)" >&2; exit 1; }
    echo "== external deploy transcript (rollback) =="
    # Paths must resolve against the OWNING user's home, not the invoking
    # user's: root's $HOME is /root, which silently no-ops the cleanup
    # (BETA-016-F correction of the BETA-016-E rollback).
    OWNER_HOME="$(getent passwd beta | cut -d: -f6)"
    USER_PIDFILE="$OWNER_HOME/proof-service.pid"
    USER_APP="$OWNER_HOME/velqu/examples/proof"
    USER_PACK="$USER_APP/dist/app.qpack"

    step "remove edge site + stop nginx"
    rm -f "$NGINX_SITE" "$NGINX_ENABLED"
    service nginx stop >/dev/null 2>&1 || true
    curl -sf --max-time 2 "http://127.0.0.1:$EDGE_PORT/health/live" >/dev/null 2>&1 \
      && fail "edge still answering after removal"
    echo "edge removed (port $EDGE_PORT closed)"

    step "SIGTERM the service (pidfile $USER_PIDFILE, owner beta)"
    [ -f "$USER_PIDFILE" ] || fail "pidfile missing at $USER_PIDFILE — cannot verify service stop"
    PID="$(cat "$USER_PIDFILE")"
    kill -TERM "$PID" 2>/dev/null || true
    for _ in $(seq 1 50); do
      if [ -d "/proc/$PID" ]; then
        # kill -0 succeeds for ZOMBIES too, and this container's init
        # (sleep infinity) never reaps — read the kernel state instead.
        STATE="$(cut -d' ' -f3 /proc/$PID/stat 2>/dev/null || echo gone)"
        [ "$STATE" = "Z" ] && break
        [ "$STATE" = "gone" ] && break
        sleep 0.1
      else
        break
      fi
    done
    if [ -d "/proc/$PID" ]; then
      STATE="$(cut -d' ' -f3 /proc/$PID/stat 2>/dev/null || echo gone)"
      { [ "$STATE" != "Z" ] && [ "$STATE" != "gone" ]; } \
        && fail "service did not exit after SIGTERM (state=$STATE)"
    fi
    echo "service stopped (graceful; kernel state $STATE)"

    step "assert upstream port released, remove build artifacts + pidfile"
    curl -sf --max-time 2 "http://127.0.0.1:$UPSTREAM_PORT/health/live" >/dev/null 2>&1 \
      && fail "upstream $UPSTREAM_PORT still answering after service stop"
    rm -rf "$USER_APP/dist" "$USER_PIDFILE"
    [ ! -f "$USER_PACK" ] || fail "pack still present after rollback"
    echo "ROLLBACK-OK: edge closed, service stopped, upstream released, artifacts removed"
    ;;

  *)
    echo "usage: deploy-proof-service.sh app|edge|verify|rollback" >&2
    exit 1
    ;;
esac
