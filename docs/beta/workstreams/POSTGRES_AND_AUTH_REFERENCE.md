---
type: Workstream
title: Optional Postgres and Authentication Reference
status: draft
tags:
- postgres
- auth
- capability

---

# Optional Postgres and Authentication Reference

## Principle

Database and authentication must make Velqu useful without enlarging core or defining a hidden platform.

## Postgres beta package

- Built only after the capability ABI passes.
- Lazy bounded pool.
- Parameterized query API and transactions.
- Route-deadline and AbortSignal cancellation.
- Graceful shutdown and pool metrics.
- No ORM in core.
- No driver loaded for apps that do not declare the capability.

## JWT reference package

- One explicit approved algorithm/profile for beta.
- Issuer, audience, expiry, and clock-skew checks.
- Key loading/rotation hooks.
- Typed 401/403 RFC 9457 problems.
- Secret redaction and no token logging.
- Algorithm confusion and `none` behavior rejected.

## Proof workloads

- W1 authenticated primary-key read.
- W2 transactional order write.
- W3 paginated join and aggregation.
- Unauthorized, expired, wrong audience, pool timeout, transaction rollback, and shutdown cases.

## Non-goals

- ORM, migrations framework, identity provider, session UI, or universal database abstraction.
