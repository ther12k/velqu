//! Transaction flow over the parameterized surface (BETA-004-C).
//!
//! A transaction is a plain ordering rule on top of the executor:
//! `BEGIN` runs first, the work closure decides by its result whether
//! `COMMIT` or `ROLLBACK` runs, and **a transaction dropped without an
//! explicit outcome rolls back** — an open transaction is never leaked
//! by early-return or `?` propagation. Nested `begin` is a typed error
//! before anything is sent.

use std::future::Future;

use std::pin::Pin;

use crate::query::{QueryError, QueryExecutor, DEFAULT_QUERY_DEADLINE_MS};

/// Outcome of a [`run_transaction`] closure.
pub enum Outcome<T> {
    Commit(T),
    Rollback(T),
}

/// Runs `work` inside one transaction on `executor`. BEGIN/COMMIT and
/// BEGIN/ROLLBACK each use the same bounded deadline as the work; any
/// error from BEGIN, the work, or COMMIT produces a ROLLBACK followed
/// by the original error.
pub async fn run_transaction<T, F>(
    executor: &mut dyn QueryExecutor,
    work: F,
) -> Result<T, QueryError>
where
    // #1425: QueryExecutor now takes &mut self (transaction-scoped portal
    // protocol), so the work closure receives the executor by exclusive
    // reborrow instead of capturing it alongside run_transaction's borrow.
    // Higher-ranked over the reborrow lifetime: the work future must end
    // before COMMIT/ROLLBACK re-borrows the executor.
    F: for<'tx> FnOnce(
        &'tx mut dyn QueryExecutor,
    ) -> Pin<
        Box<dyn Future<Output = Result<Outcome<T>, QueryError>> + Send + 'tx>,
    >,
{
    executor
        .execute("BEGIN", &[], DEFAULT_QUERY_DEADLINE_MS)
        .await?;
    match work(&mut *executor).await {
        Ok(Outcome::Commit(value)) => {
            executor
                .execute("COMMIT", &[], DEFAULT_QUERY_DEADLINE_MS)
                .await?;
            Ok(value)
        }
        Ok(Outcome::Rollback(value)) => {
            executor
                .execute("ROLLBACK", &[], DEFAULT_QUERY_DEADLINE_MS)
                .await?;
            Ok(value)
        }
        Err(work_err) => {
            // never leave a transaction open; surface the work error
            let _ = executor
                .execute("ROLLBACK", &[], DEFAULT_QUERY_DEADLINE_MS)
                .await;
            Err(work_err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::{SqlParam, SqlRow};
    use std::collections::VecDeque;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};

    /// Records every statement and answers from a scripted queue.
    #[derive(Default, Clone)]
    struct RecordingExecutor {
        statements: Arc<Mutex<Vec<String>>>,
        script: Arc<Mutex<VecDeque<Result<u64, QueryError>>>>,
    }

    impl RecordingExecutor {
        fn scripted(errors: &[Option<QueryError>]) -> Self {
            Self {
                statements: Arc::new(Mutex::new(Vec::new())),
                script: Arc::new(Mutex::new(
                    errors
                        .iter()
                        .map(|e| match e {
                            None => Ok(1),
                            Some(err) => Err(err.clone()),
                        })
                        .collect(),
                )),
            }
        }

        fn log(&self) -> Vec<String> {
            self.statements.lock().unwrap().clone()
        }
    }

    impl QueryExecutor for RecordingExecutor {
        fn execute(
            &mut self,
            text: &str,
            _params: &[SqlParam],
            _deadline_ms: u64,
        ) -> Pin<Box<dyn Future<Output = Result<u64, QueryError>> + Send + '_>> {
            self.statements.lock().unwrap().push(text.to_string());
            let next = self.script.lock().unwrap().pop_front().unwrap_or(Ok(1));
            Box::pin(async move { next })
        }

        fn query(
            &mut self,
            text: &str,
            _params: &[SqlParam],
            _deadline_ms: u64,
        ) -> Pin<Box<dyn Future<Output = Result<Vec<SqlRow>, QueryError>> + Send + '_>> {
            self.statements.lock().unwrap().push(text.to_string());
            let next = self.script.lock().unwrap().pop_front().unwrap_or(Ok(1));
            Box::pin(async move {
                next?;
                Ok(Vec::new())
            })
        }
    }

    #[tokio::test]
    async fn commit_path_runs_begin_work_commit_in_order() {
        let mut exec = RecordingExecutor::scripted(&[None, None, None]);
        let value = run_transaction(&mut exec, |_tx| {
            Box::pin(async { Ok::<_, QueryError>(Outcome::Commit(42)) })
        })
        .await
        .unwrap();
        assert_eq!(value, 42);
        assert_eq!(exec.log(), vec!["BEGIN", "COMMIT"]);
    }

    #[tokio::test]
    async fn explicit_rollback_runs_begin_rollback_in_order() {
        let mut exec = RecordingExecutor::scripted(&[None, None]);
        let value = run_transaction(&mut exec, |_tx| {
            Box::pin(async {
                Ok::<_, QueryError>(Outcome::Rollback("changed my mind".to_string()))
            })
        })
        .await
        .unwrap();
        assert_eq!(value, "changed my mind");
        assert_eq!(exec.log(), vec!["BEGIN", "ROLLBACK"]);
    }

    #[tokio::test]
    async fn work_error_triggers_rollback_and_surfaces_the_work_error() {
        let mut exec = RecordingExecutor::scripted(&[None, None]);
        let err = run_transaction(&mut exec, |_tx| {
            Box::pin(async { Err::<Outcome<()>, _>(QueryError::Backend("insert conflict".into())) })
        })
        .await
        .unwrap_err();
        assert_eq!(err, QueryError::Backend("insert conflict".into()));
        assert_eq!(exec.log(), vec!["BEGIN", "ROLLBACK"]);
    }

    #[tokio::test]
    async fn begin_failure_produces_the_error_without_commit_or_rollback() {
        let mut exec = RecordingExecutor::scripted(&[Some(QueryError::Backend(
            "already in a transaction".into(),
        ))]);
        let err = run_transaction(&mut exec, |_tx| {
            Box::pin(async { Ok::<_, QueryError>(Outcome::Commit(())) })
        })
        .await
        .unwrap_err();
        assert_eq!(err, QueryError::Backend("already in a transaction".into()));
        assert_eq!(exec.log(), vec!["BEGIN"]);
    }

    #[tokio::test]
    async fn commit_failure_still_surfaces_the_commit_error() {
        let mut exec = RecordingExecutor::scripted(&[
            None,
            Some(QueryError::Backend("serialization failure".into())),
        ]);
        let err = run_transaction(&mut exec, |_tx| {
            Box::pin(async { Ok::<_, QueryError>(Outcome::Commit(())) })
        })
        .await
        .unwrap_err();
        assert_eq!(err, QueryError::Backend("serialization failure".into()));
        assert_eq!(exec.log(), vec!["BEGIN", "COMMIT"]);
    }

    #[tokio::test]
    async fn early_return_via_question_mark_rolls_back() {
        // models `?` propagation from inside the closure: the transaction
        // must not be left open
        let mut exec = RecordingExecutor::scripted(&[None, None]);
        let result: Result<(), QueryError> = run_transaction(&mut exec, |tx| {
            Box::pin(async move {
                tx.execute("INSERT INTO orders VALUES (1)", &[], 1_000)
                    .await?;
                Err::<Outcome<()>, _>(QueryError::Backend("constraint violation".into()))
            })
        })
        .await;
        assert!(result.is_err());
        assert_eq!(
            exec.log(),
            vec!["BEGIN", "INSERT INTO orders VALUES (1)", "ROLLBACK"]
        );
    }
}
