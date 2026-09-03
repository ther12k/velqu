# Raw-sample retention

- Policy: raw per-request rows are retained as deterministic raw.jsonl.gz beside summary.json; uncompressed rows and logs stay local-only
- Rows: 710139
- raw.jsonl: 81836004 bytes, sha256 a9473e788006621f83f843097f3e834a6c7e366aa1fccdce1e4d96a3a0972974
- raw.jsonl.gz: 3509823 bytes, sha256 7d98d5ea46897575f2a1be3c1adcbbfaa98f7f84219d4e8a20f95c25abdafcaf
- Verify: `gunzip -c raw.jsonl.gz | sha256sum` must equal the raw.jsonl sha256 above.
