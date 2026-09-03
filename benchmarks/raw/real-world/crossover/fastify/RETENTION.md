# Raw-sample retention

- Policy: raw per-request rows are retained as deterministic raw.jsonl.gz beside summary.json; uncompressed rows and logs stay local-only
- Rows: 1098860
- raw.jsonl: 126672058 bytes, sha256 961e1bb54a23f05069cf04156333dc7470c3deac695059bcb320942b1d3103d2
- raw.jsonl.gz: 5381632 bytes, sha256 8033bd01c0e6e1ce5f993ce9ab55ead3218df2e8c92bd9a5290927169c507af1
- Verify: `gunzip -c raw.jsonl.gz | sha256sum` must equal the raw.jsonl sha256 above.
