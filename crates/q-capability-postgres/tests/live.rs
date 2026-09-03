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
        let conn = pool.acquire(5_000).await.unwrap();
        let exec = ClientExecutor::new(conn.get());
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
        run_transaction(&exec, || async {
            exec.execute(
                "INSERT INTO pgcap_tx_test (id, qty) VALUES ($1, $2)",
                &[SqlParam::text("tx_commit"), SqlParam::Int(1)],
                5_000,
            )
            .await?;
            Ok::<_, q_capability_postgres::QueryError>(Outcome::Commit(()))
        })
        .await
        .unwrap();

        // ROLLBACK path: work inside a transaction does not persist
        run_transaction(&exec, || async {
            exec.execute(
                "INSERT INTO pgcap_tx_test (id, qty) VALUES ($1, $2)",
                &[SqlParam::text("tx_rollback"), SqlParam::Int(1)],
                5_000,
            )
            .await?;
            Ok::<_, q_capability_postgres::QueryError>(Outcome::Rollback(()))
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
