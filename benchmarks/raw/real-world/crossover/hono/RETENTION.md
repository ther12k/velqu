# Raw-sample retention

- Policy: raw per-request rows are retained as deterministic raw.jsonl.gz beside summary.json; uncompressed rows and logs stay local-only
- Rows: 820040
- raw.jsonl: 94259455 bytes, sha256 414cb5c451ec39be9e874d5dfb178a8664c27f22a1e85a4e6f909a73d84a7df5
- raw.jsonl.gz: 3985489 bytes, sha256 f93cce3c6c102580fcba70d7592f9bdddca9014113e4390f1bc2f4032a12f4de
- Verify: `gunzip -c raw.jsonl.gz | sha256sum` must equal the raw.jsonl sha256 above.
