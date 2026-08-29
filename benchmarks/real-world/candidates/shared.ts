/**
 * Shared W4 candidate contract (M28-011-A).
 *
 * Every candidate implements the identical surface:
 *   GET /api/bench/io?ms=N -> proxies to ${UPSTREAM}/io?ms=N via the
 *   runtime's NATIVE fetch/client and relays the upstream JSON + status.
 *
 * The upstream URL comes from UPSTREAM_URL (no default: explicit wiring).
 * Malformed ms fails fast with 400 without touching the upstream.
 */

export const UPSTREAM = process.env.UPSTREAM_URL ?? "";
if (!UPSTREAM) {
  console.error(
    JSON.stringify({ event: "candidate.rejected", reason: "UPSTREAM_URL is required" }),
  );
  process.exit(3);
}

export const PORT = Number(process.env.PORT ?? 0);

export function validateMs(raw: string | null): number | null {
  if (raw === null || !/^\d{1,4}$/.test(raw)) return null;
  const ms = Number(raw);
  if (ms > 1000) return null;
  return ms;
}
