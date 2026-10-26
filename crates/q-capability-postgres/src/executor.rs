//! Production [`QueryExecutor`] over a pooled tokio-postgres client
//! (BETA-004-C).
//!
//! Every statement runs through the extended protocol with bound
//! parameters (no string interpolation). The placeholder scan is a
//! conservative deterministic `$N` scan: it over-counts if a statement
//! embeds `$<digit>` inside a literal, which fails the query typed
//! instead of binding fewer parameters — parameterized texts keep
//! positional placeholders outside literals.
//!
//! #1425 result retrieval is PAGE-WISE portal execution (see `query`):
//! tokio-postgres delivers a request's backend messages only at
//! ReadyForQuery (verified with a trace-instrumented probe against
//! PostgreSQL 17 — `query_raw` resolved only after the whole portal had
//! executed), so a single Execute buffers the ENTIRE wire result inside
//! the connection no matter how lazily the executor consumes it. Each
//! portal fetch is its own request, so fetching in bounded pages is what
//! actually bounds client-side buffering.

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use futures_util::StreamExt;
use tokio_postgres::types::{ToSql, Type};

use crate::query::{
    validate_deadline, validate_query, QueryError, QueryExecutor, ResultBudget, SqlParam, SqlRow,
};

/// #1425 (measured basis): rows fetched per portal Execute. Each fetch is
/// one request whose backend messages are delivered only at its
/// ReadyForQuery, so this — not the byte budget — is the bound on
/// client-side buffering per round trip: PAGE_ROWS × the largest row's
/// wire frame (a single row's frame is bounded only by the server's own
/// field limits and the deadline — disclosed transport caveat). 256 keeps
/// a page at a few MB for typical rows while a 10k-row result costs ~40
/// loopback-trivial round trips.
const FETCH_PAGE_ROWS: i32 = 256;

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
            SqlParam::Float(f) => match *ty {
                Type::FLOAT4 => (*f as f32).to_sql(ty, out),
                _ => f.to_sql(ty, out),
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

/// Executor bound to one leased connection. Holds the connection
/// mutably because #1425's page-wise portal execution drives
/// transaction-scoped protocol (BEGIN / bind / bounded fetches / COMMIT)
/// through `Client::transaction`, which requires `&mut`. Cheap to
/// construct; all I/O happens per call under ONE absolute caller
/// deadline and the #1425 result budget (rows + estimated serialized
/// bytes, enforced per page — see `query`).
pub struct ClientExecutor<'c> {
    client: &'c mut tokio_postgres::Client,
    budget: ResultBudget,
}

impl<'c> ClientExecutor<'c> {
    pub fn new(client: &'c mut tokio_postgres::Client) -> Self {
        ClientExecutor {
            client,
            budget: ResultBudget::default(),
        }
    }

    /// Executor with an explicit per-query result budget (#1425).
    pub fn with_budget(client: &'c mut tokio_postgres::Client, budget: ResultBudget) -> Self {
        ClientExecutor { client, budget }
    }
}

/// #1425 (review hardening): upper-bound estimate of a row's serialized
/// JSON size taken from the row's BORROWED wire data — text-family
/// columns estimate from `&str` borrows (`String::accepts` delegates to
/// `&str::accepts` in tokio-postgres, so accept/reject matches the owned
/// conversion exactly), and a budget-rejected row is never copied into
/// owned strings. The per-value formula is identical to
/// `estimate_row_bytes` (6 bytes/char worst case under JSON escaping),
/// so the estimate always upper-bounds the serialized output. Columns
/// whose type cannot convert estimate 0: `from_pg_row` rejects them with
/// the typed ColumnConversion error before anything is collected.
fn estimate_pg_row_bytes(row: &tokio_postgres::Row) -> u64 {
    let mut total: u64 = 2;
    for (idx, column) in row.columns().iter().enumerate() {
        total += (column.name().len() as u64) * 6 + 4;
        total += match *column.type_() {
            Type::BOOL => 5,
            Type::INT2 | Type::INT4 | Type::INT8 => 20,
            Type::FLOAT4 | Type::FLOAT8 => 24,
            _ => match row.try_get::<_, Option<&str>>(idx) {
                Ok(Some(s)) => (s.len() as u64) * 6 + 2,
                Ok(None) => 4,
                // mirrors from_pg_row's catch-all: an unsupported type is a
                // typed conversion failure, not a size question
                Err(_) => 0,
            },
        };
    }
    total
}

/// Bounds one protocol stage (begin/bind/fetch/commit) by the query's
/// REMAINING absolute deadline — every stage together can never exceed
/// deadline_ms. Stage errors map to the typed backend error.
async fn stage_timeout<T>(
    abs_deadline: tokio::time::Instant,
    deadline_ms: u64,
    fut: impl Future<Output = Result<T, tokio_postgres::Error>>,
) -> Result<T, QueryError> {
    let left = abs_deadline.saturating_duration_since(tokio::time::Instant::now());
    if left.is_zero() {
        return Err(QueryError::DeadlineExceeded { ms: deadline_ms });
    }
    tokio::time::timeout(left, fut)
        .await
        .map_err(|_| QueryError::DeadlineExceeded { ms: deadline_ms })?
        .map_err(|e| QueryError::Backend(map_pg_error(&e)))
}

impl<'c> QueryExecutor for ClientExecutor<'c> {
    fn execute(
        &mut self,
        text: &str,
        params: &[SqlParam],
        deadline_ms: u64,
    ) -> Pin<Box<dyn Future<Output = Result<u64, QueryError>> + Send + '_>> {
        let bound = match bind_boxed(text, params) {
            Ok(b) => b,
            Err(e) => return Box::pin(async move { Err(e) }),
        };
        let client = &mut *self.client;
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
        &mut self,
        text: &str,
        params: &[SqlParam],
        deadline_ms: u64,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<SqlRow>, QueryError>> + Send + '_>> {
        let bound = match bind_boxed(text, params) {
            Ok(b) => b,
            Err(e) => return Box::pin(async move { Err(e) }),
        };
        let client = &mut *self.client;
        let budget = self.budget;
        let owned_text = text.to_string();
        Box::pin(async move {
            // #1425, hardened on measured evidence: tokio-postgres delivers
            // a request's backend messages only at its ReadyForQuery (a
            // trace-instrumented probe against PostgreSQL 17 showed
            // `query_raw` resolving only after the whole portal executed,
            // with every row then arriving in microseconds), so a single
            // Execute buffers the ENTIRE wire result inside the connection
            // regardless of how lazily the executor consumes it — and no
            // budget applied at the executor can bound that buffering.
            //
            // The fix is PAGE-WISE PORTAL EXECUTION: bind the statement to
            // a portal inside an explicit transaction, fetch
            // FETCH_PAGE_ROWS per Execute, and commit. Each fetch is its
            // own request, so client-side buffering per round trip is
            // bounded by PAGE_ROWS × the largest row's wire frame (a
            // single row's frame is bounded only by the server's field
            // limits — disclosed). The row/byte budget is enforced per row
            // WITHIN each page (byte estimate from the row's borrowed wire
            // data BEFORE conversion, so a rejected row is never copied
            // into owned strings), and ONE ABSOLUTE deadline bounds every
            // await by its remaining time: begin + bind + all fetches +
            // commit together can never exceed deadline_ms. The lease is
            // exclusive for the duration; ANY error is followed by a full
            // lease discard (connection close → implicit server rollback),
            // so a partial page never leaks backend state.
            let deadline_ms = validate_deadline(deadline_ms)?;
            let abs_deadline = tokio::time::Instant::now() + Duration::from_millis(deadline_ms);
            let (tx, portal) = {
                let tx = stage_timeout(abs_deadline, deadline_ms, client.transaction()).await?;
                let refs: Vec<&(dyn ToSql + Sync)> =
                    bound.iter().map(|p| &**p as &(dyn ToSql + Sync)).collect();
                let portal = stage_timeout(
                    abs_deadline,
                    deadline_ms,
                    tx.bind_raw(owned_text.as_str(), refs),
                )
                .await?;
                (tx, portal)
            };

            let mut out: Vec<SqlRow> = Vec::new();
            let mut total_bytes: u64 = 0;
            loop {
                let mut stream = Box::pin(
                    stage_timeout(
                        abs_deadline,
                        deadline_ms,
                        tx.query_portal_raw(&portal, FETCH_PAGE_ROWS),
                    )
                    .await?,
                );
                let mut fetched = 0usize;
                loop {
                    let next = {
                        let left =
                            abs_deadline.saturating_duration_since(tokio::time::Instant::now());
                        if left.is_zero() {
                            return Err(QueryError::DeadlineExceeded { ms: deadline_ms });
                        }
                        let mut stream = std::pin::pin!(stream.as_mut());
                        tokio::time::timeout(left, stream.next()).await
                    };
                    match next {
                        Err(_) => return Err(QueryError::DeadlineExceeded { ms: deadline_ms }),
                        Ok(None) => break,
                        Ok(Some(Err(e))) => return Err(QueryError::Backend(map_pg_error(&e))),
                        Ok(Some(Ok(row))) => {
                            fetched += 1;
                            if (out.len() as u64) >= budget.max_rows {
                                return Err(QueryError::ResultRowsExceeded {
                                    rows: out.len() as u64 + 1,
                                    max: budget.max_rows,
                                });
                            }
                            let row_bytes = estimate_pg_row_bytes(&row);
                            if total_bytes + row_bytes > budget.max_bytes {
                                return Err(QueryError::ResultBytesExceeded {
                                    bytes: total_bytes + row_bytes,
                                    max: budget.max_bytes,
                                });
                            }
                            total_bytes += row_bytes;
                            out.push(SqlRow::from_pg_row(row)?);
                        }
                    }
                }
                // A short fetch means the portal is exhausted; an exact
                // page costs one final empty fetch to confirm the end.
                if fetched < FETCH_PAGE_ROWS as usize {
                    break;
                }
            }
            stage_timeout(abs_deadline, deadline_ms, tx.commit()).await?;
            Ok(out)
        })
    }
}
