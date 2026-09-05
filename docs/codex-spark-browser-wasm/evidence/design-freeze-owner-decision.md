# Browser-WASM Design Freeze — Owner Decision Record

- **Decision ID**: OD-BWASM-001 (design freeze)
- **Date**: 2026-09-05
- **Decider**: repository owner (`ther12k`), in an interactive review session
- **Scope**: ratification of the Browser-WASM design phase (BWASM-D-001..004)
- **Method**: the owner was presented the three design ADRs with their
  correction history and the exact decision consequences, and selected
  from explicit options ("Ratify as corrected", "Ratify as targets",
  "Confirm — accept"); no default or implied acceptance was used.

## Decisions

1. **ADR-0037 (product and runtime contract) — ACCEPTED.**
   The architecture invariant was owner-specified verbatim in the
   Browser-WASM packet; the owner confirmed the agent-authored
   remainder (semantics classification, lifecycle wording, capability
   table) matches intent.
2. **ADR-0038 (threat model and isolation contract) — ACCEPTED AS
   CORRECTED.** Ratified including the §5 ambient-API honesty section:
   the capability bridge mediates declared calls but cannot prevent
   trusted handler code from calling browser APIs directly;
   platform-enforceable guarantees (origin isolation, CSP
   `connect-src`) are separated from trusted-code conventions;
   untrusted-preview isolation is deployment posture (separate origin +
   sandboxed iframe), not a runtime property.
3. **ADR-0039 (support matrix, compatibility claims, budgets) —
   ACCEPTED AS TARGETS.** Numeric budgets are normative targets,
   revisable only by ADR amendment with measured evidence; silently
   missing a budget stays release-blocking. All browser lanes remain
   `experimental-untested` until BWASM-Q-002 real-browser CI evidence
   exists; no lane may be called `tested` before then.
4. **Phased registration — K-PHASE NEXT.** The owner selected
   registering only the six kernel issues (BWASM-K-001..006) after the
   freeze; later phases register as their dependencies complete. This
   re-affirms the original phased instruction (5 design issues first,
   kernel only after the freeze).

## Consequences

- The design-freeze gate is **closed→complete**: BWASM-D-003 (#1182)
  and BWASM-D-004 (#1183) acceptance criteria are met by this record;
  the K-phase is authorized to register and proceed.
- The 27 non-kernel issues (runtime, build/deploy, capabilities,
  quality/release, optional) remain **unregistered** until their phase
  gates, per decision 4.
- Budget changes require an ADR amendment carrying the measured
  evidence; lane promotions require the named evidence per lane.

## Alternatives considered (in session)

- Ratify with changes / adjust numbers / keep blocked (per ADR) — not
  selected.
- Register all 33 remaining issues at once — not selected (phased
  registration retained).
- Hold registration — not selected.

## History (honesty note)

The ADRs were originally merged on 2026-09-05 with overstated
owner-acceptance claims; corrected the same day (ADR statuses →
`proposed`, ambient-API overclaim fixed, #1182 reopened), and
**subsequently ratified properly** via this record. The correction
history is retained in the ADR frontmatter and the decisions index.
