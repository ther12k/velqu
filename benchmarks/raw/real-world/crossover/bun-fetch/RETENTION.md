# Raw-sample retention

- Policy: raw per-request rows are retained as deterministic raw.jsonl.gz beside summary.json; uncompressed rows and logs stay local-only
- Rows: 1262402
- raw.jsonl: 145184142 bytes, sha256 898df6e408b1beadabfa84ff48fab4d6acfd7f2275536e0f4f2c0216313f721b
- raw.jsonl.gz: 6072488 bytes, sha256 87b79a037e189d0d779be144d371a44b30bd8ccad8512bb3030d2dba5c43e563
- Verify: `gunzip -c raw.jsonl.gz | sha256sum` must equal the raw.jsonl sha256 above.
