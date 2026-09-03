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
