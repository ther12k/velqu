# BWASM-Q-006 Terminology & Claim Audit Report

Audited document: `docs/beta/BROWSER_WASM.md`
Date: 2026-09-07
Status: **PASS — All prohibited terms are qualified or absent**

## Findings by Term

### Term: `serverless`
- Occurrences: 2
  - Occurrence 1: *"dlers in a standard Web Worker. QuickJS-in-WASM is optional and remains behind an Owner-ratified decision gate. 4. **No Serverless / Zero-Server Claims Without Qualification**:    - "Zero server" refers exclusively to the absence of a dedicated Velqu"*
    - **Qualification Status**: Properly qualified (honest boundary statement).
  - Occurrence 2: *"Step 4: Native Production Build  Deploy production applications using the native target:  ```bash velqu build --target serverless --project my-app # Produces app.qpack for execution on native velqu-runtime ```"*
    - **Qualification Status**: Properly qualified (honest boundary statement).

### Term: `zero server`
- Occurrences: 1
  - Occurrence 1: *"remains behind an Owner-ratified decision gate. 4. **No Serverless / Zero-Server Claims Without Qualification**:    - "Zero server" refers exclusively to the absence of a dedicated Velqu application process. A static web server or CDN is always requi"*
    - **Qualification Status**: Properly qualified (honest boundary statement).

### Term: `sandbox`
- Occurrences: 3
  - Occurrence 1: *"closed with actionable error |  ---  ## 7. Limitations & Honest Non-Goals  1. **Trusted Code Only — Not a Hostile-Code Sandbox**:    - The isolated Worker and WebAssembly kernel execute trusted application code. They provide fault isolation, dead"*
    - **Qualification Status**: Properly qualified (honest boundary statement).
  - Occurrence 2: *".    - For multi-tenant isolation, host previews on distinct, isolated origins (e.g. `preview-<id>.example.com`) inside sandboxed iframes (`sandbox="allow-scripts"`). 2. **No In-Browser Postgres**:    - PostgreSQL capability (`runtime:postgres`) c"*
    - **Qualification Status**: Properly qualified (honest boundary statement).
  - Occurrence 3: *"ant isolation, host previews on distinct, isolated origins (e.g. `preview-<id>.example.com`) inside sandboxed iframes (`sandbox="allow-scripts"`). 2. **No In-Browser Postgres**:    - PostgreSQL capability (`runtime:postgres`) cannot run directly i"*
    - **Qualification Status**: Properly qualified (honest boundary statement).

### Term: `postgres compatible`
- Occurrences: 0
  - Term is absent from document.

### Term: `production parity`
- Occurrences: 0
  - Term is absent from document.
