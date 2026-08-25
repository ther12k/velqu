#!/usr/bin/env bash
# Real-world benchmark one-command harness (BETA-001-A).
#
#   ./run.sh            prepare -> smoke -> report
#   ./run.sh prepare    compose up + deterministic dataset reset
#   ./run.sh smoke      2s load-gen smoke against the controlled upstream,
#                       result-schema validation, report generation
#
# Requires docker with the compose plugin. Fails fast and honestly when
# prerequisites are missing; no hidden fallbacks.
set -euo pipefail
cd "$(dirname "$0")"

SMOKE_DIR="../raw/real-world/smoke"
UPSTREAM_PORT="${UPSTREAM_PORT:-8791}"

cmd_prepare() {
  if ! command -v docker >/dev/null 2>&1; then
    echo "run.sh: docker with compose plugin is required" >&2
    exit 1
  fi
  echo "== real-world: starting pinned Postgres =="
  docker compose up -d --wait
  echo "== real-world: pinned versions (versions.json) =="
  bun -e "const v = await Bun.file('versions.json').json(); console.log(JSON.stringify(v, null, 2));"
  ./reset.sh
}

cmd_smoke() {
  local out_dir="$SMOKE_DIR"
  rm -rf "$out_dir"
  mkdir -p "$out_dir"

  echo "== real-world: starting controlled upstream on :$UPSTREAM_PORT =="
  PORT="$UPSTREAM_PORT" bun upstream.ts >"$out_dir/upstream.log" 2>&1 &
  UPSTREAM_PID=$!
  trap 'if [ -n "${UPSTREAM_PID:-}" ]; then kill "$UPSTREAM_PID" 2>/dev/null || true; fi' EXIT

  local ready=0
  for _ in $(seq 1 100); do
    if curl -fsS "http://127.0.0.1:$UPSTREAM_PORT/health" >/dev/null 2>&1; then ready=1; break; fi
    sleep 0.1
  done
  if [ "$ready" -ne 1 ]; then
    echo "run.sh: controlled upstream failed to become ready" >&2
    cat "$out_dir/upstream.log" >&2 || true
    exit 1
  fi

  echo "== real-world: 2s load-gen smoke (W4 cells against the upstream) =="
  bun load.ts \
    --base-url "http://127.0.0.1:$UPSTREAM_PORT" \
    --upstream-url "http://127.0.0.1:$UPSTREAM_PORT" \
    --out-dir "$out_dir" \
    --duration 2 \
    --concurrency 1,10

  echo "== real-world: report =="
  bun report.ts --summary "$out_dir/summary.json" --out "$out_dir/report.md"

  echo "== real-world: result-schema validation =="
  bun -e "
    const s = await Bun.file('$out_dir/summary.json').json();
    const cfg = await Bun.file('workloads.json').json();
    const { validateRealWorldSummary } = await import('./result-schema.ts');
    const errs = validateRealWorldSummary(s, cfg.workloads.map(w => w.id), s.concurrencyLevels);
    if (errs.length) { for (const e of errs) console.error('  - ' + e); process.exit(1); }
    console.log('result-schema: PASS');
  "

  kill "$UPSTREAM_PID" 2>/dev/null || true
  UPSTREAM_PID=""
  trap - EXIT
}

case "${1:-all}" in
  prepare) cmd_prepare ;;
  smoke) cmd_smoke ;;
  all) cmd_prepare; cmd_smoke ;;
  *) echo "usage: ./run.sh [prepare|smoke|all]" >&2; exit 2 ;;
esac
