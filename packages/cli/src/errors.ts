/**
 * @velqu/cli — actionable diagnostic error formatting (M4A-002-D).
 *
 * Implements source-located, context-aware error formatting for compiler
 * errors, syntax errors, missing files, broken contracts, and runtime panics.
 * Includes code frame snippets with caret indicators and actionable hints.
 */

import { existsSync, readFileSync } from "node:fs";
import { CompileError } from "@velqu/compiler";

export interface ErrorLocation {
  file: string;
  line: number;
  column: number;
}

export type DiagnosticCode =
  | "VELQU-COMP-IMPORT"
  | "VELQU-COMP-CONTRACT"
  | "VELQU-COMP-PATH"
  | "VELQU-COMP-SCHEMA"
  | "VELQU-TOOLCHAIN"
  | "VELQU-RUNTIME"
  | "VELQU-UNKNOWN";

export interface FormattedDiagnostic {
  title: string;
  code: DiagnosticCode;
  message: string;
  location?: ErrorLocation;
  codeFrame?: string;
  hint?: string;
  raw: string;
}

/**
 * Generates a clean code frame around the error line with a caret pointing to the column.
 */
export function renderCodeFrame(filePath: string, line: number, column: number, contextLines = 2): string | null {
  if (!existsSync(filePath)) return null;
  try {
    const content = readFileSync(filePath, "utf8");
    const lines = content.split("\n");
    const startLine = Math.max(1, line - contextLines);
    const endLine = Math.min(lines.length, line + contextLines);

    const out: string[] = [];
    const maxLineNumWidth = endLine.toString().length;

    out.push("   --> " + filePath + ":" + line + ":" + column);
    out.push("   |");

    for (let l = startLine; l <= endLine; l++) {
      const lineText = lines[l - 1] ?? "";
      const isTarget = l === line;
      const numStr = l.toString().padStart(maxLineNumWidth, " ");
      const prefix = isTarget ? `${numStr} | ` : `${numStr} | `;
      out.push(` ${prefix}${lineText}`);

      if (isTarget) {
        const indent = " ".repeat(maxLineNumWidth + 3 + Math.max(0, column - 1));
        out.push(`${indent}^`);
      }
    }
    out.push("   |");
    return out.join("\n");
  } catch {
    return null;
  }
}

/**
 * Format any compiler error, syntax error, or toolchain error into an actionable diagnostic.
 */
function diagnosticCode(err: unknown): DiagnosticCode {
  const message = err instanceof Error ? err.message : String(err);
  if (/unsupported import|node:|bun:/.test(message)) return "VELQU-COMP-IMPORT";
  if (/path|parameter/.test(message)) return "VELQU-COMP-PATH";
  if (/schema|body|query/.test(message)) return "VELQU-COMP-SCHEMA";
  if (/route|contract|response|method/.test(message)) return "VELQU-COMP-CONTRACT";
  if (/toolchain|typescript|bun version/.test(message)) return "VELQU-TOOLCHAIN";
  if (/runtime|worker|quickjs|pack/.test(message)) return "VELQU-RUNTIME";
  return "VELQU-UNKNOWN";
}

export function formatActionableError(err: unknown, defaultTitle = "error"): FormattedDiagnostic {
  const code = diagnosticCode(err);
  if (err instanceof CompileError) {
    const codeFrame = err.location
      ? renderCodeFrame(err.location.file, err.location.line, err.location.column) ?? undefined
      : undefined;

    let raw = `[velqu:${defaultTitle}] ${err.message}`;
    if (codeFrame) raw += "\n" + codeFrame;
    if (err.hint) raw += `\n  hint: ${err.hint}`;

    return {
      title: defaultTitle,
      code,
      message: err.message,
      location: err.location,
      codeFrame,
      hint: err.hint,
      raw,
    };
  }

  if (err instanceof Error) {
    // Check if error message contains file:line:col pattern (e.g. from TypeScript / Bun)
    const match = err.message.match(/([^\s()]+):(\d+):(\d+)/);
    let location: ErrorLocation | undefined = undefined;
    let codeFrame: string | undefined = undefined;

    if (match) {
      const file = match[1];
      const line = parseInt(match[2], 10);
      const col = parseInt(match[3], 10);
      if (existsSync(file)) {
        location = { file, line, column: col };
        codeFrame = renderCodeFrame(file, line, col) ?? undefined;
      }
    }

    let raw = `[velqu:${defaultTitle}] ${err.message}`;
    if (codeFrame) raw += "\n" + codeFrame;

    return {
      title: defaultTitle,
      code,
      message: err.message,
      location,
      codeFrame,
      raw,
    };
  }

  return {
    title: defaultTitle,
    code,
    message: String(err),
    raw: `[velqu:${defaultTitle}] ${String(err)}`,
  };
}
