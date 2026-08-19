# Owner Decisions

The implementation agent must not invent these decisions. It may prepare options and continue independent technical work.

| ID | Decision | Required by | Engineering may continue before decision? |
|---|---|---|---|
| OD-003 | Public repository and organization | M7 publishing | Yes |
| OD-004 | License and contribution model | M6 license audit / M7 publishing | Yes, using temporary private metadata |
| OD-005 | Initial supported platform promise | M6 platform gate | Yes, build/test candidates without claiming support |
| OD-006 | Governance, release authority, and public release timing | M7/M8 | Yes |
| OD-007 | Security disclosure contact and response authority | M6/M7 | Preparation yes; GA no |
| OD-008 | Public benchmark/positioning wording | M7 | Yes; raw evidence work continues |
| OD-009 | Whether direct TLS/HTTP2 is required for initial GA or reverse-proxy deployment is the supported profile | M5 operator docs | Yes; default plan assumes trusted proxy termination |
| OD-010 | Whether Postgres is an official first-party package at GA or only a reference capability | M5/M7 | Capability spike may continue after M2.7 |

Each decision record includes alternatives, risk, rationale, date, and approver. An unresolved required owner decision blocks only the gate listed above, not unrelated engineering tasks.
