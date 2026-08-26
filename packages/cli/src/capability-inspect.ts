import { capabilityInventoryHash } from "../../compiler/src/emit";

/**
 * M27-002-D: `velqu inspect capabilities` accuracy. Reads the
 * capability manifest and the compiled pack, verifies the inventory
 * hash (fail loud on mismatch), and renders the human report.
 * Extracted from index.ts so accuracy is unit-testable.
 */
export function inspectCapabilities(input: {
  declared: string[];
  perRoute: Record<string, string[]>;
  nativeOps: Record<string, string>;
  pack: {
    capabilityInventory?: Array<{ id: string; version: number }>;
    capabilityInventorySha256?: string;
  };
}): string[] {
  const lines: string[] = [];
  lines.push(`declared: ${input.declared.join(", ") || "(none)"}`);
  for (const [route, list] of Object.entries(input.perRoute)) {
    if (list.length) lines.push(`  ${route}: ${list.join(", ")}`);
  }
  lines.push(`native ops: ${JSON.stringify(input.nativeOps)}`);

  const inv = input.pack.capabilityInventory ?? [];
  const declared = input.pack.capabilityInventorySha256;
  if (declared === undefined) {
    lines.push("linked: unknown (pack predates capability inventory)");
  } else {
    const computed = capabilityInventoryHash(inv);
    if (computed !== declared) {
      throw new Error(
        `capability inventory hash mismatch in pack: declares ${declared}, computed ${computed} — artifact is corrupt or was hand-edited`,
      );
    }
    // sorted + unique check mirrors q-pack verify()
    for (let i = 1; i < inv.length; i++) {
      if (inv[i - 1].id >= inv[i].id) {
        throw new Error(
          `capability inventory is not sorted ascending/unique at '${inv[i].id}'`,
        );
      }
    }
    lines.push(
      `linked: ${inv.map((e) => `${e.id}@${e.version}`).join(", ") || "(none — zero linked modules)"}`,
    );
    lines.push(`inventory sha256: ${declared}`);
  }
  return lines;
}
