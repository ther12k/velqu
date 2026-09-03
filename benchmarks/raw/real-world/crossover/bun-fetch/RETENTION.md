# Raw-sample retention

- Policy: raw per-request rows are retained as deterministic raw.jsonl.gz beside summary.json; uncompressed rows and logs stay local-only
- Rows: 796198
- raw.jsonl: 91599746 bytes, sha256 037f5bc2a608ad688507cccab7c78541094a11d51f8b816f429a1766fda3c237
- raw.jsonl.gz: 3907901 bytes, sha256 b084bc39fd4646b204fef3ea5dc4a579070ed37ec8ad0610822ec59d58f0644d
- Verify: `gunzip -c raw.jsonl.gz | sha256sum` must equal the raw.jsonl sha256 above.
