// M26-007-B: single source of truth for the pinned release toolchain.
// Byte-identical packs require identical toolchain inputs (bundler,
// TypeScript emit, recorded builtBy); the build refuses to run on any
// other version and names the mismatch, so non-reproducibility is
// diagnosed at build time rather than discovered in artifact diffs.
export const PINNED_TOOLCHAIN = {
  compiler: "0.1.0",
  typescript: "5.9.3",
  bun: "1.4.0",
} as const;

export class ToolchainError extends Error {}

export function assertPinnedToolchain(running: {
  bun: string;
  typescript: string;
}): void {
  const mismatches: string[] = [];
  if (running.bun !== PINNED_TOOLCHAIN.bun) {
    mismatches.push(`bun ${running.bun} != pinned ${PINNED_TOOLCHAIN.bun}`);
  }
  if (running.typescript !== PINNED_TOOLCHAIN.typescript) {
    mismatches.push(
      `typescript ${running.typescript} != pinned ${PINNED_TOOLCHAIN.typescript}`,
    );
  }
  if (mismatches.length > 0) {
    throw new ToolchainError(
      `toolchain mismatch — byte-identical packs require the pinned toolchain (${mismatches.join("; ")})`,
    );
  }
}
