/** CJS twin of shared.ts for the Node/Fastify candidate. */
const UPSTREAM = process.env.UPSTREAM_URL ?? "";
if (!UPSTREAM) {
  console.error(
    JSON.stringify({ event: "candidate.rejected", reason: "UPSTREAM_URL is required" }),
  );
  process.exit(3);
}
function validateMs(raw) {
  if (raw === null || raw === undefined || !/^\d{1,4}$/.test(raw)) return null;
  const ms = Number(raw);
  if (ms > 1000) return null;
  return ms;
}
module.exports = { UPSTREAM, PORT: Number(process.env.PORT ?? 0), validateMs };
