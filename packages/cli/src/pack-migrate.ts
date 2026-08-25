/**
 * M26-008-B — rebuild/migration guidance for legacy packs (ADR-0024).
 *
 * `velqu pack migrate <file>` wraps this assessment. The function is pure:
 * the caller supplies a reader thunk so tests need no filesystem.
 */

export type PackMigrateReport =
  | {
      status: "legacy-supported";
      formatVersion: 1;
      guidance: string[];
    }
  | { status: "unsupported"; formatVersion?: number; message: string }
  | { status: "not-a-pack"; message: string }
  | { status: "unreadable"; message: string };

export const MIGRATION_DOC = "docs/specs/pack-format-v1.md deprecation notes";

export function assessPackMigrate(read: () => string): PackMigrateReport {
  let header: { formatVersion?: unknown; kind?: unknown };
  try {
    header = JSON.parse(read());
  } catch {
    return {
      status: "unreadable",
      message:
        "pack is not valid JSON — mode-2 binary packs are produced by future compilers; rebuild from source instead",
    };
  }
  if (header.kind !== "velqu.qpack" || typeof header.formatVersion !== "number") {
    return {
      status: "not-a-pack",
      message: "not a velqu application pack (unexpected kind/shape)",
    };
  }
  const v = header.formatVersion;
  if (v === 1) {
    return {
      status: "legacy-supported",
      formatVersion: 1,
      guidance: [
        "runtime: loads via the legacy-v1 adapter; no action required today",
        "recommended: rebuild from source with the current compiler (`velqu build --project <dir>`)",
        "  — output is deterministic (M26-007), so a rebuild is byte-stable and behavior-neutral",
        `binary migration to mode 2 will be reported here when producers emit it (none exists yet; see ${MIGRATION_DOC})`,
      ],
    };
  }
  return {
    status: "unsupported",
    formatVersion: v,
    message:
      `formatVersion ${v} is not supported (supported: 1 = legacy-v1 JSON adapter); ` +
      `unknown versions fail closed — rebuild the pack with the current compiler or migrate it ` +
      `(see ${MIGRATION_DOC})`,
  };
}
