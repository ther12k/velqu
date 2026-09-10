Atomic Browser-WASM packet: `<path>`  
Program: `BWASM`  
Phase: `<phase>`  
Mode: `IMPLEMENT | VERIFY | VERIFY_OR_FIX | EVIDENCE | GATE_REVIEW`  
Priority: `P0 | P1`  
Optional: `YES | NO`  
Research baseline: `<commit>`  
Status: `TODO`

---

# BWASM-<ID> — <title>

## Atomic goal

<One bounded outcome.>

## Parent intent

<Why the phase exists.>

## Architecture invariant

- Rust/WASM owns compatibility-critical routing/validation/authorization.
- Browser Worker owns MVP handler execution.
- Public boundary is `Request -> Promise<Response>`.
- Native Velqu remains the production path for native-only capabilities.

## Dependencies

- `<BWASM-ID>`

## Read first

- `<path>`

## Steps

1. <Step>
2. <Step>

## Acceptance criteria

- [ ] <Observable outcome>
- [ ] <Negative-path outcome>
- [ ] <No-regression outcome>

## Targeted tests and commands

- `<command or test family>`

## Required evidence

- [ ] Exact candidate commit.
- [ ] Raw logs.
- [ ] Artifact hashes where applicable.
- [ ] Environment/browser/toolchain manifest.

## Guardrails

- Do not expand the issue without opening a dependency.
- Do not make unsupported security/performance/compatibility claims.
- Do not bypass the Rust/WASM path.
- Keep native Velqu green.

## Out of scope

- <Explicit exclusion>

## Commit / PR guidance

- Suggested commit prefix: `bwasm-<id>:`.
- One bounded PR.
- Link issue, tests, evidence, and follow-up blockers.

## Stop condition

Stop when every criterion is demonstrated and canonical verification is green. Leave the issue open with a precise blocker when owner approval or external evidence is required.

## Handoff format

```text
Issue:
Candidate commit:
Files changed:
Commands run:
Targeted tests:
Full verification:
Artifacts and SHA-256:
Browser/OS/toolchain:
Acceptance criteria:
Known limitations:
Residual risks:
Follow-up issue links:
```
