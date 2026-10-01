import { definePolicy, status } from "@velqu/core";

export interface NoteAuth {
  actor: string;
}

/**
 * Write-path policy over the standard authorization header (the header
 #1401 fixed custom names for; authorization works on published beta.1).
 * The token is a FIXTURE for this external-consumer exercise — real
 * deployments take it from their secret channel, never from source.
 */
export const notesWritePolicy = definePolicy({
  id: "notes.write",
  header: "authorization",
  declares: { 401: "unauthorized" },
  provides: "auth",
  check: async (req) => {
    if (req.headers.authorization !== "Bearer q-notes-demo-token") {
      return status(401).problem("unauthorized");
    }
    return { auth: { actor: "demo" } satisfies NoteAuth };
  },
});

export default notesWritePolicy;
