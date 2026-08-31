/**
 * @velqu/cli — stable exit code definitions (M4A-002-B).
 *
 * Provides deterministic exit codes for all CLI commands:
 * - 0: Success / clean execution / no breaking contract drift.
 * - 1: General error, user error, compilation error, file not found, test failure.
 * - 2: Breaking contract difference detected by `contract diff`.
 * - 3: Unsupported format or migration required (e.g. invalid pack version).
 */

export const ExitCode = {
  SUCCESS: 0,
  GENERAL_ERROR: 1,
  BREAKING_CONTRACT: 2,
  UNSUPPORTED_FORMAT: 3,
} as const;

export type ExitCodeValue = (typeof ExitCode)[keyof typeof ExitCode];
