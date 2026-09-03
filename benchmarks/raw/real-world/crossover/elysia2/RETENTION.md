# Raw-sample retention

- Policy: raw per-request rows are retained as deterministic raw.jsonl.gz beside summary.json; uncompressed rows and logs stay local-only
- Rows: 1400964
- raw.jsonl: 160924216 bytes, sha256 0e87b25b7e050484a56beb7facc4ae2a9a14fb7f18b5b9cb9b03ee24bc37d10d
- raw.jsonl.gz: 6499848 bytes, sha256 e5a5b6bcd34cf00fd4dbacaad2ed7fa0ea8344360a3b7b01d868cce2cea90f7d
- Verify: `gunzip -c raw.jsonl.gz | sha256sum` must equal the raw.jsonl sha256 above.
