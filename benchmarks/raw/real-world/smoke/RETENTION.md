# Raw-sample retention

- Policy: raw per-request rows are retained as deterministic raw.jsonl.gz beside summary.json; uncompressed rows and logs stay local-only
- Rows: 323505
- raw.jsonl: 35906767 bytes, sha256 7aaecb20f951680106e1ef1f41e72935f2b74012c332ef214fdae0185f71628c
- raw.jsonl.gz: 1794557 bytes, sha256 ca2ded2124d9522b45e0c68b8506d662da662348270a0a157c76c949e9a8e7ec
- Verify: `gunzip -c raw.jsonl.gz | sha256sum` must equal the raw.jsonl sha256 above.
