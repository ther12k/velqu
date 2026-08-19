# Review Protocol

## Purpose

The implementation agent may execute the whole ordered plan, but every milestone leaves an independently reviewable checkpoint. The final reviewer evaluates frozen acceptance gates, not an ever-expanding wish list.

## Milestone checkpoint package

Every gate commit must contain:

```text
docs/reports/<milestone>-report.md
evidence/<milestone>/manifest.json
evidence/<milestone>/test-output/
evidence/<milestone>/raw/           # when benchmarks are required
evidence/<milestone>/summary.json
REVIEW_REQUEST.md
SOURCE-COMMIT.txt
SHA256SUMS.txt
```

External delivery includes:

```text
commit-named clean source ZIP
git bundle or complete format-patch series
archive checksum file
build/test environment manifest
all raw evidence referenced by the report
```

A source ZIP without Git provenance is insufficient for final history review.

## Claim hierarchy

When sources disagree, trust them in this order:

1. source code and exact dependency locks;
2. executable tests and captured command output;
3. raw benchmark/conformance data;
4. generated reports;
5. handoff summary prose.

The verification command must fail when a higher layer contradicts a lower layer.

## Finite review rule

A reviewer finding blocks the current milestone only when it is:

- P0 or P1 under the frozen milestone invariant;
- a required acceptance item lacking evidence;
- a source/evidence contradiction;
- an unauthorized scope or trust-boundary change.

New desirable features that are outside the frozen gate become later backlog items rather than repeated closure revisions. P2 findings do not block a milestone unless the owner explicitly promotes them.

## Review request contents

The agent must state:

- baseline commit and ending commit;
- exact changed files;
- tasks claimed PASS;
- commands and captured results;
- benchmark protocols and raw-data paths;
- known failures, waivers, and limitations;
- security-sensitive invariants changed;
- performance-sensitive paths changed;
- archive name and SHA-256;
- clean working-tree evidence.

## Final review

The final review packet includes a machine-readable `REVIEW_INDEX.json` mapping every production gate to:

```text
source files
test names
raw evidence
reports
owner decision or waiver
```

The reviewer may sample or rerun any gate. “Production ready” is not used until `M8-GATE` is approved.
