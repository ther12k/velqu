# PARTIAL CAMPAIGN — SUPERSEDED CANDIDATE

Candidate: `47ddc3fc8aca2ee92ddd284661be4b943d7a7b8a`.

This campaign run found a real defect before completing: the
`codec_encoders` target crashed on `{"\xff":0,"status":{}}` — RFC 9457
problem extensions could shadow frozen envelope members, so the emitted
problem parsed back with `status` as `{}` instead of the declared 422.

Disposition (PR #1339, merged as `b8fee349`): reserved envelope member
names (type/title/status/instance/detail/errors) are now skipped at all
three layers — `q-schema-runtime` `ProblemProgram::encode`, `q-runtime`
`problems::body`, and the `q-engine-quickjs` worker extraction reserved
set. Regression test `problem_encoder_skips_envelope_named_extensions`.

- `disposition/crash-d370c25c69550dca3034052889f73fd2560a5a9e` — the
  original crash artifact, retained as audit trail. Replayed against the
  fix: passes (2026-09-14). The campaign input bytes are also recorded in
  PR #1339.
- A sustained confirmation rerun of `codec_encoders` alone
  (seed 1789340122, 6,289,132 execs / 901s, 0 findings) ran clean on the
  fix before merge.

This directory is the superseded attempt's evidence. It is NOT a
completed campaign and must not be cited as one. The restarted chain
binds to `b8fee349`.
