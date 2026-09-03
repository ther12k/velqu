# Raw-sample retention

- Policy: raw per-request rows are retained as deterministic raw.jsonl.gz beside summary.json; uncompressed rows and logs stay local-only
- Rows: 1213549
- raw.jsonl: 139656201 bytes, sha256 80eb9f6091076c1fb6c280ecb9b3dbdd480f8869511ce3c43d8716e4ac46a10e
- raw.jsonl.gz: 5914463 bytes, sha256 1c1d0815797e3db040db78120cc56e67487a731f71147c509aaeb851b8cd83a2
- Verify: `gunzip -c raw.jsonl.gz | sha256sum` must equal the raw.jsonl sha256 above.
