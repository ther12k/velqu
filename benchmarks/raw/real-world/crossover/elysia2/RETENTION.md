# Raw-sample retention

- Policy: raw per-request rows are retained as deterministic raw.jsonl.gz beside summary.json; uncompressed rows and logs stay local-only
- Rows: 842349
- raw.jsonl: 96753148 bytes, sha256 513ff17b4d35bfe6123c80aec8ab246f47ef90c21941882cd016d5089e7492fe
- raw.jsonl.gz: 3987056 bytes, sha256 a5608c08de2bd051fcd75c760c68cde3218a2694ea8b297005b2a0e760e49a2f
- Verify: `gunzip -c raw.jsonl.gz | sha256sum` must equal the raw.jsonl sha256 above.
