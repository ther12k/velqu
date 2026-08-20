# Current Reviewed Baseline

```text
Release commit: 4e6904951729ea14b48ca39a9564a950cc83e98e
Source archive: source-4e69049.zip
Git bundle:     velqu-4e69049.bundle
Target:         0.1.0-beta.1
Current gate:   G0 / M23R2 trusted numeric graph and evidence truth
```

## Known baseline contradiction

The release `SOURCE-COMMIT.txt` identifies `4e69049`, while the supplied review and evidence indexes identify `e2b379d`. Treat those indexes as stale until G0 fixes and regenerates them.

## Reviewed open G0 items

- semantic function manifest must be mandatory in current numeric mode;
- serialized router must be semantically verified, not merely bounds-checked;
- current numeric startup must load the verified router without semantic reconstruction;
- RouteId, PolicyId, and SchemaId must be complete operational identities;
- public contract hash must exclude implementation details and include observable request/security semantics;
- beta/production ledgers and review/evidence indexes must identify the current commit and actual evidence;
- canonical benchmark evidence still needs repeated randomized runs and allocation/startup profiles.

## Explicit carry-forward

Response-schema validation may remain on an optional slow path until M2.5, provided it is documented and does not weaken correctness.
