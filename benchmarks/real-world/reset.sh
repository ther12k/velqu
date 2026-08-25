#!/usr/bin/env bash
# Deterministic Postgres reset (BETA-001-A): drop, re-apply schema, re-seed.
# The result is byte-identical dataset contents on every invocation.
set -euo pipefail
cd "$(dirname "$0")"

if ! command -v docker >/dev/null 2>&1; then
  echo "reset.sh: docker is required for the real-world benchmark Postgres" >&2
  exit 1
fi

PSQL=(docker compose exec -T postgres psql -U velqu_bench -d velqu_bench -v ON_ERROR_STOP=1)

# SQL files live on the host; pipe them into the container via stdin.
"${PSQL[@]}" < postgres/reset.sql
"${PSQL[@]}" < postgres/schema.sql
"${PSQL[@]}" < postgres/seed.sql

echo "reset.sh: dataset counts"
"${PSQL[@]}" -t -A -c "SELECT 'users=' || count(*) FROM users"
"${PSQL[@]}" -t -A -c "SELECT 'products=' || count(*) FROM products"
"${PSQL[@]}" -t -A -c "SELECT 'reviews=' || count(*) FROM reviews"
"${PSQL[@]}" -t -A -c "SELECT 'electronics=' || count(*) FROM products WHERE category = 'electronics'"
"${PSQL[@]}" -t -A -c "SELECT 'orders=' || count(*) FROM orders"
