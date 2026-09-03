//! Production [`QueryExecutor`] over a pooled tokio-postgres client
//! (BETA-004-C).
//!
//! Every statement runs through the extended protocol with bound
//! parameters (no string interpolation). The placeholder scan is a
//! conservative deterministic `$N` scan: it over-counts if a statement
//! embeds `$<digit>` inside a literal, which fails the query typed
//! instead of binding fewer parameters — parameterized texts keep
//! positional placeholders outside literals.

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use tokio_postgres::types::{ToSql, Type};

use crate::query::{
    validate_deadline, validate_query, QueryError, QueryExecutor, SqlParam, SqlRow,
};

/// Binds our closed parameter set through tokio-postgres's ToSql.
impl ToSql for SqlParam {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut tokio_postgres::types::private::BytesMut,
    ) -> Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        match self {
            SqlParam::Null => None::<i32>.to_sql(ty, out),
            SqlParam::Bool(b) => b.to_sql(ty, out),
            // width matches the declared column type in binary format
            SqlParam::Int(i) => match *ty {
                Type::INT2 => (*i as i16).to_sql(ty, out),
                Type::INT4 => (*i as i32).to_sql(ty, out),
                _ => i.to_sql(ty, out),
            },
            SqlParam::Text(s) => s.to_sql(ty, out),
        }
    }

    fn accepts(_ty: &Type) -> bool {
        // the backend coerces untyped text-format parameters
        true
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut tokio_postgres::types::private::BytesMut,
    ) -> Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.to_sql(ty, out)
    }
}

fn map_pg_error(e: &tokio_postgres::Error) -> String {
    // server-side errors carry their SQLSTATE + message; anything else
    // is rendered with URL fragments stripped (defense in depth)
    match e.as_db_error() {
        Some(db) => format!("{} (sqlstate {})", db.message(), db.code().code()),
        None => {
            let msg = e.to_string();
            match msg.find("postgres://") {
                Some(idx) => format!("{}<redacted-url>", &msg[..idx]),
                None => msg,
            }
        }
    }
}

/// Validates and materializes owned parameter bindings (they move into
/// the returned future, so no borrowed-local lifetime games).
fn bind_boxed(
    text: &str,
    params: &[SqlParam],
) -> Result<Vec<Box<dyn ToSql + Sync + Send>>, QueryError> {
    let placeholders = validate_query(text, params)?;
    Ok(params
        .iter()
        .take(placeholders)
        .map(|p| Box::new(p.clone()) as Box<dyn ToSql + Sync + Send>)
        .collect())
}

async fn bounded<'f, T>(
    deadline_ms: u64,
    fut: Pin<Box<dyn Future<Output = Result<T, tokio_postgres::Error>> + Send + 'f>>,
) -> Result<T, QueryError> {
    let deadline = validate_deadline(deadline_ms)?;
    match tokio::time::timeout(Duration::from_millis(deadline), fut).await {
        Ok(Ok(v)) => Ok(v),
        Ok(Err(e)) => Err(QueryError::Backend(map_pg_error(&e))),
        Err(_) => Err(QueryError::DeadlineExceeded { ms: deadline }),
    }
}

/// Executor bound to one leased connection. Cheap to construct; all
/// I/O happens per call under the caller's deadline.
pub struct ClientExecutor<'c> {
    client: &'c tokio_postgres::Client,
}

impl<'c> ClientExecutor<'c> {
    pub fn new(client: &'c tokio_postgres::Client) -> Self {
        ClientExecutor { client }
    }
}

impl<'c> QueryExecutor for ClientExecutor<'c> {
    fn execute(
        &self,
        text: &str,
        params: &[SqlParam],
        deadline_ms: u64,
    ) -> Pin<Box<dyn Future<Output = Result<u64, QueryError>> + Send + '_>> {
        let bound = match bind_boxed(text, params) {
            Ok(b) => b,
            Err(e) => return Box::pin(async move { Err(e) }),
        };
        let client = self.client;
        // the future owns its text and bindings: no borrow outlives the
        // shortest input lifetime
        let owned_text = text.to_string();
        Box::pin(async move {
            let refs: Vec<&(dyn ToSql + Sync)> =
                bound.iter().map(|p| &**p as &(dyn ToSql + Sync)).collect();
            bounded(
                deadline_ms,
                Box::pin(client.execute(owned_text.as_str(), &refs)),
            )
            .await
        })
    }

    fn query(
        &self,
        text: &str,
        params: &[SqlParam],
        deadline_ms: u64,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<SqlRow>, QueryError>> + Send + '_>> {
        let bound = match bind_boxed(text, params) {
            Ok(b) => b,
            Err(e) => return Box::pin(async move { Err(e) }),
        };
        let client = self.client;
        let owned_text = text.to_string();
        Box::pin(async move {
            let refs: Vec<&(dyn ToSql + Sync)> =
                bound.iter().map(|p| &**p as &(dyn ToSql + Sync)).collect();
            let rows = bounded(
                deadline_ms,
                Box::pin(client.query(owned_text.as_str(), &refs)),
            )
            .await?;
            let mut out = Vec::with_capacity(rows.len());
            for row in rows {
                out.push(SqlRow::from_pg_row(row)?);
            }
            Ok(out)
        })
    }
}
