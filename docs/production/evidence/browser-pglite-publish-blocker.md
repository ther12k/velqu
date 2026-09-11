# @velqu/browser-pglite publish — auth blocker (2026-09-11)

Owner approved the publish (review 2026-09-11): `@velqu/browser-pglite@0.1.0-beta.1`,
dist-tag `beta`, not `latest`, via `bash scripts/publish-beta.sh --only browser-pglite`.

The publish was attempted and is blocked on npm authentication, not on tooling:

```
$ npm whoami
npm error 401 Unauthorized - GET https://registry.npmjs.org/-/whoami
```

`~/.npmrc` holds `//registry.npmjs.org/:_authToken=<redacted>` but the stored token
is rejected (stale/revoked/expired since the 2026-09-09 publishing session).
`npm login` is interactive (browser + 2FA) and can only be completed by the owner.

## Owner steps when convenient

1. `npm login` (or refresh a granular Automation token in `~/.npmrc`).
2. `bash scripts/publish-beta.sh --only browser-pglite`   # real mode; skip-check is live
3. Record here: registry version, `sha512` integrity digest from
   `npm view @velqu/browser-pglite@0.1.0-beta.1 dist.integrity`.
4. Clean-install check: `cd $(mktemp -d) && npm init -y >/dev/null && npm i @velqu/browser-pglite@beta`
   and confirm `node -e "import('@velqu/browser-pglite').then(m => console.log(typeof m.createLocalSql))"`.
5. Append the resolution line to `docs/open-decisions.md` OD-010 and
   `docs/beta/PUBLISHING.md` status header.

Everything else in the publish path is ready and qualified (#1317): the script is
idempotent, `--only` is dependency-safe (no `@velqu/*` deps asserted), packaging
invariants are test-enforced.
