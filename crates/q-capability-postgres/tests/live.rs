//! Live-Postgres pool verification (BETA-004-B).
//!
//! Operator-run: requires the benchmark stack
//!   cd benchmarks/real-world && docker compose up -d --wait && ./reset.sh
//! and `VELQU_PG_LIVE_TEST=1`. Skipped (with a notice) otherwise so
//! `cargo test` stays deterministic everywhere.

use q_capability_postgres::{PoolConfig, PostgresPool};

fn live_url() -> Option<String> {
    if std::env::var("VELQU_PG_LIVE_TEST").ok()?.as_str() != "1" {
        return None;
    }
    Some(
        std::env::var("VELQU_DATABASE_URL").unwrap_or_else(|_| {
            "postgres://velqu_bench:velqu_bench@127.0.0.1:5433/velqu_bench".into()
        }),
    )
}

#[tokio::test]
async fn live_pool_connects_queries_and_stays_bounded() {
    let Some(url) = live_url() else {
        eprintln!("live: skipped (set VELQU_PG_LIVE_TEST=1 and start the benchmark stack)");
        return;
    };
    // bounded below the matched posture to prove the ceiling under load
    let pool = PostgresPool::postgres(url, PoolConfig::new(4, 5_000, 30_000).unwrap());

    // lazy: nothing connected yet
    assert_eq!(pool.stats().created_total, 0);

    // a real round trip against the seeded dataset
    {
        let conn = pool.acquire(5_000).await.expect("acquire within deadline");
        let messages = conn
            .get()
            .simple_query("SELECT count(*) AS n FROM users")
            .await
            .expect("SELECT count(*) FROM users");
        let n: usize = messages
            .iter()
            .find_map(|m| match m {
                tokio_postgres::SimpleQueryMessage::Row(row) => {
                    row.get(0).and_then(|v| v.parse().ok())
                }
                _ => None,
            })
            .expect("a row with the count");
        assert_eq!(n, 1_000, "seeded user count");
    }
    let after_first = pool.stats();
    assert_eq!((after_first.created_total, after_first.idle), (1, 1));

    // sequential acquires reuse the idle connection
    {
        let _conn = pool.acquire(5_000).await.expect("second acquire");
    }
    assert_eq!(
        pool.stats().created_total,
        1,
        "idle reuse: no second connect"
    );

    // the ceiling is real: 4 leases held, the 5th acquire fails typed
    let c1 = pool.acquire(1_000).await.unwrap();
    let c2 = pool.acquire(1_000).await.unwrap();
    let c3 = pool.acquire(1_000).await.unwrap();
    let c4 = pool.acquire(1_000).await.unwrap();
    let err = pool.acquire(100).await.unwrap_err();
    assert!(
        matches!(
            err,
            q_capability_postgres::PoolError::AtCapacity { max: 4, .. }
        ),
        "expected typed at-capacity, got {err}"
    );
    drop((c1, c2, c3, c4));

    // bounded connect timeout against an unroutable address fails typed
    let bad = PostgresPool::postgres(
        "postgres://velqu_bench:velqu_bench@127.0.0.1:54330/velqu_bench",
        PoolConfig::default_config(),
    );
    let err = bad.acquire(300).await.expect_err("unroutable must fail");
    assert!(
        matches!(
            err,
            q_capability_postgres::PoolError::ConnectTimeout { .. }
                | q_capability_postgres::PoolError::ConnectRejected(_)
        ),
        "typed connect failure, got {err}"
    );

    pool.begin_shutdown();
    assert!(matches!(
        pool.acquire(100).await.unwrap_err(),
        q_capability_postgres::PoolError::ShuttingDown
    ));
}

/// BETA-004-C: parameterized queries and transactions against the live
/// backend — parameter binding (extended protocol), typed row
/// conversion, transactional commit and rollback with early-return
/// safety.
#[tokio::test]
async fn live_parameterized_queries_and_transactions() {
    use q_capability_postgres::{
        run_transaction, ClientExecutor, Outcome, QueryExecutor, SqlParam,
    };
    let Some(url) = live_url() else {
        eprintln!("live: skipped (set VELQU_PG_LIVE_TEST=1 and start the benchmark stack)");
        return;
    };
    let pool = PostgresPool::postgres(url, PoolConfig::default_config());

    // scratch table (idempotent)
    {
        let mut conn = pool.acquire(5_000).await.unwrap();
        let mut exec = ClientExecutor::new(conn.get_mut());
        exec.execute(
            "CREATE TABLE IF NOT EXISTS pgcap_tx_test (id TEXT PRIMARY KEY, qty INT NOT NULL)",
            &[],
            5_000,
        )
        .await
        .unwrap();
        exec.execute("DELETE FROM pgcap_tx_test", &[], 5_000)
            .await
            .unwrap();

        // parameterized insert + typed select-back
        exec.execute(
            "INSERT INTO pgcap_tx_test (id, qty) VALUES ($1, $2)",
            &[SqlParam::text("item_1"), SqlParam::Int(7)],
            5_000,
        )
        .await
        .unwrap();
        let rows = exec
            .query(
                "SELECT id, qty FROM pgcap_tx_test WHERE id = $1",
                &[SqlParam::text("item_1")],
                5_000,
            )
            .await
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(
            rows[0].get("id"),
            Some(&q_capability_postgres::SqlValue::Text("item_1".into()))
        );
        assert_eq!(
            rows[0].get("qty"),
            Some(&q_capability_postgres::SqlValue::Int(7))
        );

        // unbound placeholder fails typed before anything is sent
        let err = exec
            .query(
                "SELECT * FROM pgcap_tx_test WHERE id = $1 AND qty = $2",
                &[SqlParam::Int(1)],
                5_000,
            )
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            q_capability_postgres::QueryError::ParamCountMismatch {
                placeholders: 2,
                bound: 1
            }
        ));

        // COMMIT path: work inside a transaction persists
        run_transaction(&mut exec, |tx| {
            Box::pin(async move {
                tx.execute(
                    "INSERT INTO pgcap_tx_test (id, qty) VALUES ($1, $2)",
                    &[SqlParam::text("tx_commit"), SqlParam::Int(1)],
                    5_000,
                )
                .await?;
                Ok::<_, q_capability_postgres::QueryError>(Outcome::Commit(()))
            })
        })
        .await
        .unwrap();

        // ROLLBACK path: work inside a transaction does not persist
        run_transaction(&mut exec, |tx| {
            Box::pin(async move {
                tx.execute(
                    "INSERT INTO pgcap_tx_test (id, qty) VALUES ($1, $2)",
                    &[SqlParam::text("tx_rollback"), SqlParam::Int(1)],
                    5_000,
                )
                .await?;
                Ok::<_, q_capability_postgres::QueryError>(Outcome::Rollback(()))
            })
        })
        .await
        .unwrap();

        let rows = exec
            .query("SELECT id FROM pgcap_tx_test ORDER BY id", &[], 5_000)
            .await
            .unwrap();
        let ids: Vec<&str> = rows
            .iter()
            .filter_map(|r| match r.get("id") {
                Some(q_capability_postgres::SqlValue::Text(s)) => Some(s.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(
            ids,
            vec!["item_1", "tx_commit"],
            "rollback must not persist"
        );

        exec.execute("DROP TABLE IF EXISTS pgcap_tx_test", &[], 5_000)
            .await
            .unwrap();
    }
    assert!(pool.stats().created_total >= 1);
}

/// #1425: the per-query result budget rejects oversized results WHILE
/// ROWS STREAM — never after full collection. Operator-run (same stack
/// and VELQU_PG_LIVE_TEST=1 gating as the suite above): a small
/// generate_series result crosses a deliberately tiny budget and fails
/// typed on either axis; a within-budget result succeeds.
#[tokio::test]
async fn live_result_budget_rejects_oversized_results_while_streaming() {
    let Some(url) = live_url() else {
        eprintln!("live: skipped (set VELQU_PG_LIVE_TEST=1 and start the benchmark stack)");
        return;
    };
    use q_capability_postgres::{ClientExecutor, QueryExecutor, ResultBudget, SqlParam};

    let pool = PostgresPool::postgres(url, PoolConfig::new(2, 5_000, 30_000).unwrap());
    let mut lease = pool.acquire(5_000).await.expect("acquire");

    // row budget: 5-row result against a 2-row budget
    let mut exec =
        ClientExecutor::with_budget(lease.get_mut(), ResultBudget::new(2, 1_048_576).unwrap());
    let err = exec
        .query(
            "SELECT i AS n FROM generate_series(1, $1::int8) AS i",
            &[SqlParam::Int(5)],
            5_000,
        )
        .await
        .expect_err("5 rows must breach the 2-row budget");
    assert!(
        matches!(
            &err,
            q_capability_postgres::QueryError::ResultRowsExceeded { rows: 3, max: 2 }
        ),
        "typed row-budget breach, got {err}"
    );

    // byte budget: 4 KiB texts against a 8 KiB budget (3 rows ≈ 12 KiB)
    let mut exec =
        ClientExecutor::with_budget(lease.get_mut(), ResultBudget::new(1000, 8192).unwrap());
    let err = exec
        .query(
            "SELECT repeat('x', 4096) AS payload FROM generate_series(1, $1::int8) AS i",
            &[SqlParam::Int(3)],
            5_000,
        )
        .await
        .expect_err("3×4KiB rows must breach the 8KiB byte budget");
    assert!(
        matches!(
            &err,
            q_capability_postgres::QueryError::ResultBytesExceeded { max: 8192, .. }
        ),
        "typed byte-budget breach, got {err}"
    );

    // within budget: same shapes succeed and return every row
    let mut exec = ClientExecutor::with_budget(lease.get_mut(), ResultBudget::default());
    let rows = exec
        .query(
            "SELECT i AS n FROM generate_series(1, $1::int8) AS i",
            &[SqlParam::Int(5)],
            5_000,
        )
        .await
        .expect("within-budget result succeeds");
    assert_eq!(rows.len(), 5);
}

/// #1425 hardening: ONE absolute deadline bounds setup plus ALL row
/// waits together. `pg_sleep(0.05)` per row × 10 rows ≈ 500 ms total;
/// under a per-row restarted timeout every row individually settles
/// well within the 300 ms deadline and the query would (wrongly)
/// succeed. With the absolute deadline it must fail typed with
/// DeadlineExceeded — this test discriminates the two designs.
#[tokio::test]
async fn live_absolute_deadline_bounds_total_query_time() {
    let Some(url) = live_url() else {
        eprintln!("live: skipped (set VELQU_PG_LIVE_TEST=1 and start the benchmark stack)");
        return;
    };
    use q_capability_postgres::{ClientExecutor, QueryExecutor, SqlParam};

    let pool = PostgresPool::postgres(url, PoolConfig::new(2, 5_000, 30_000).unwrap());
    let mut lease = pool.acquire(5_000).await.expect("acquire");
    let mut exec = ClientExecutor::new(lease.get_mut());

    let started = std::time::Instant::now();
    let err = exec
        .query(
            "SELECT i AS n, pg_sleep(0.05) FROM generate_series(1, $1::int8) AS i",
            &[SqlParam::Int(10)],
            300,
        )
        .await
        .expect_err("10×50ms rows must breach the 300ms total deadline");
    let elapsed = started.elapsed();
    assert!(
        matches!(
            &err,
            q_capability_postgres::QueryError::DeadlineExceeded { ms: 300 }
        ),
        "typed total-deadline breach expected, got {err}"
    );
    assert!(
        elapsed >= std::time::Duration::from_millis(250),
        "deadline fired suspiciously early: {elapsed:?}"
    );
    assert!(
        elapsed < std::time::Duration::from_secs(3),
        "absolute deadline must fire near 300ms, not after full collection: {elapsed:?}"
    );
}

/// #1425 hardening: a single oversized row is rejected from its BORROWED
/// wire data — typed byte-budget breach, and a within-budget result
/// still round-trips completely.
#[tokio::test]
async fn live_single_oversized_row_rejected_before_owned_conversion() {
    let Some(url) = live_url() else {
        eprintln!("live: skipped (set VELQU_PG_LIVE_TEST=1 and start the benchmark stack)");
        return;
    };
    use q_capability_postgres::{ClientExecutor, QueryExecutor, ResultBudget};

    let pool = PostgresPool::postgres(url, PoolConfig::new(2, 5_000, 30_000).unwrap());
    let mut lease = pool.acquire(5_000).await.expect("acquire");

    // 100 KiB text against an 8 KiB budget: rejected from the borrowed
    // wire view (no owned String copy of the row is needed to decide).
    let mut exec =
        ClientExecutor::with_budget(lease.get_mut(), ResultBudget::new(1000, 8192).unwrap());
    let err = exec
        .query("SELECT repeat('x', 100000) AS payload", &[], 5_000)
        .await
        .expect_err("a single 100KiB row must breach the 8KiB byte budget");
    assert!(
        matches!(
            &err,
            q_capability_postgres::QueryError::ResultBytesExceeded { max: 8192, .. }
        ),
        "typed byte-budget breach expected, got {err}"
    );

    // Same row under the default budget (16 MiB): succeeds in full.
    let mut exec = ClientExecutor::new(lease.get_mut());
    let rows = exec
        .query("SELECT repeat('x', 100000) AS payload", &[], 5_000)
        .await
        .expect("within-budget single row succeeds");
    assert_eq!(rows.len(), 1);
    match rows[0].get("payload") {
        Some(q_capability_postgres::SqlValue::Text(s)) => assert_eq!(s.len(), 100_000),
        other => panic!("expected the full payload text, got {other:?}"),
    }
}

/// #1425 hardening: a result LARGER than one fetch page (256 rows) is
/// collected correctly across multiple portal fetches — proves the
/// page-wise loop suspends and resumes the portal without losing or
/// duplicating rows, and that an exactly-page-multiple boundary works.
#[tokio::test]
async fn live_large_result_spans_multiple_fetch_pages() {
    let Some(url) = live_url() else {
        eprintln!("live: skipped (set VELQU_PG_LIVE_TEST=1 and start the benchmark stack)");
        return;
    };
    use q_capability_postgres::{ClientExecutor, QueryExecutor, SqlParam};

    let pool = PostgresPool::postgres(url, PoolConfig::new(2, 5_000, 30_000).unwrap());
    let mut lease = pool.acquire(5_000).await.expect("acquire");
    let mut exec = ClientExecutor::new(lease.get_mut());

    // 700 rows across 3 fetches (256 + 256 + 188), non-multiple boundary
    let rows = exec
        .query(
            "SELECT i AS n FROM generate_series(1, $1::int8) AS i",
            &[SqlParam::Int(700)],
            5_000,
        )
        .await
        .expect("multi-page result collected");
    assert_eq!(rows.len(), 700, "every row across every page present");
    let first = rows[0].get("n");
    let last = rows[699].get("n");
    assert!(matches!(
        first,
        Some(q_capability_postgres::SqlValue::Int(1))
    ));
    assert!(matches!(
        last,
        Some(q_capability_postgres::SqlValue::Int(700))
    ));

    // exact page multiple: 512 rows = 2 full pages, boundary must not
    // duplicate or drop the final row
    let rows = exec
        .query(
            "SELECT i AS n FROM generate_series(1, $1::int8) AS i",
            &[SqlParam::Int(512)],
            5_000,
        )
        .await
        .expect("page-multiple result collected");
    assert_eq!(rows.len(), 512);
    assert!(matches!(
        rows[511].get("n"),
        Some(q_capability_postgres::SqlValue::Int(512))
    ));
}
