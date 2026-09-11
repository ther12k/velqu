# Source Baseline (BASE-001)

```text
Archive:            velqu-m0-m2-20260819T093558Z.zip
Archive SHA-256:    03a06bbdcc7b4f7626dd5b287983c4f3b6d26ff82e4895923284d76af92debb5
Reviewed commit:    4e69049 (G0-001-A verified the archive zip and the velqu
                    4e69049 bundle against SOURCE-COMMIT.txt)
```

## Toolchain locks at baseline

- Rust: pinned stable for release builds (see `.github/workflows/verify.yml`)
- Bun: 1.4.0 (dev/package/test tooling only)
- TypeScript: 5.9.3
- Engine: quickjs-ng 0.15.1 via `rquickjs =0.12.2` (pinned exactly)

## Known M2.3-r1 gaps carried by the baseline

Documented verbatim in `docs/production/BASELINE_AND_SCOPE.md` ("It is not
accepted as full M2.3 because..."):

- no explicit numeric load contract independent of the legacy handler table;
- no exact RoutePlan/contract equivalence;
- `RouteId`, `PolicyId`, `SchemaId` not yet operational identities;
- no verified `FieldNeeds` representation;
- no compiler-emitted terminal router (runtime candidate reconstruction);
- no canonical repeated M2.3 performance/cold-start evidence;
- no executable real-world benchmark harness.

These gaps were closed by the G0/M23R2 program (`docs/beta/program/gates/G0-GATE.md`,
status PASS) and are tracked as `M23R2-001..009` in `TASKS.production.json`.

## Baseline integrity

The reviewed archive is reproducible from git history; later review never
relies only on the unversioned ZIP. `SOURCE-COMMIT.txt` carries the exact
commit hash of the reviewed source.
