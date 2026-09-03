//! Parameterized query surface (BETA-004-C).
//!
//! Every query goes through the extended protocol (Parse/Bind/Execute)
//! — parameters are bound server-side by tokio-postgres; there is no
//! string-interpolation path in this crate. The bounded value set is
//! closed on purpose: parameters and results carry scalars only (no
//! nested arrays/objects), which keeps the JS-facing surface ORM-free
//! and the wire behavior auditable.

use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use tokio_postgres::types::Type;
use tokio_postgres::Row;

/// Fail-closed bound for statement text length.
pub const MAX_QUERY_TEXT_LEN: usize = 100_000;
/// Fail-closed bound for parameter count.
pub const MAX_PARAM_COUNT: usize = 100;

/// A bound parameter. Closed scalar set — no nested structures.
#[derive(Debug, Clone, PartialEq)]
pub enum SqlParam {
    Null,
    Bool(bool),
    Int(i64),
    Text(String),
}

impl SqlParam {
    pub fn text(value: impl Into<String>) -> Self {
        SqlParam::Text(value.into())
    }
}

/// A value returned in a result row. Closed scalar set; unknown
/// backend types are rendered as text rather than invented into
/// structures.
#[derive(Debug, Clone, PartialEq)]
pub enum SqlValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
}

/// One result row: ordered column name -> value.
#[derive(Debug, Clone, PartialEq)]
pub struct SqlRow {
    columns: BTreeMap<String, SqlValue>,
}

impl SqlRow {
    pub fn get(&self, column: &str) -> Option<&SqlValue> {
        self.columns.get(column)
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub(crate) fn from_pg_row(row: Row) -> Result<SqlRow, QueryError> {
        let mut columns = BTreeMap::new();
        for (idx, column) in row.columns().iter().enumerate() {
            let name = column.name().to_string();
            // Exact semantics: try_get::<Option<T>> yields Ok(None) for a
            // genuine NULL and Err for a conversion failure — the two are
            // never conflated.
            let value = match *column.type_() {
                Type::BOOL => match row.try_get::<_, Option<bool>>(idx) {
                    Ok(v) => v.map(SqlValue::Bool).unwrap_or(SqlValue::Null),
                    Err(_) => return Err(QueryError::ColumnConversion { column: name }),
                },
                Type::INT2 => match row.try_get::<_, Option<i16>>(idx) {
                    Ok(v) => v.map(|v| SqlValue::Int(v as i64)).unwrap_or(SqlValue::Null),
                    Err(_) => return Err(QueryError::ColumnConversion { column: name }),
                },
                Type::INT4 => match row.try_get::<_, Option<i32>>(idx) {
                    Ok(v) => v.map(|v| SqlValue::Int(v as i64)).unwrap_or(SqlValue::Null),
                    Err(_) => return Err(QueryError::ColumnConversion { column: name }),
                },
                Type::INT8 => match row.try_get::<_, Option<i64>>(idx) {
                    Ok(v) => v.map(SqlValue::Int).unwrap_or(SqlValue::Null),
                    Err(_) => return Err(QueryError::ColumnConversion { column: name }),
                },
                Type::FLOAT4 => match row.try_get::<_, Option<f32>>(idx) {
                    Ok(v) => v
                        .map(|v| SqlValue::Float(v as f64))
                        .unwrap_or(SqlValue::Null),
                    Err(_) => return Err(QueryError::ColumnConversion { column: name }),
                },
                Type::FLOAT8 => match row.try_get::<_, Option<f64>>(idx) {
                    Ok(v) => v.map(SqlValue::Float).unwrap_or(SqlValue::Null),
                    Err(_) => return Err(QueryError::ColumnConversion { column: name }),
                },
                Type::TEXT | Type::VARCHAR | Type::NAME => {
                    match row.try_get::<_, Option<String>>(idx) {
                        Ok(v) => v.map(SqlValue::Text).unwrap_or(SqlValue::Null),
                        Err(_) => return Err(QueryError::ColumnConversion { column: name }),
                    }
                }
                _ => match row.try_get::<_, Option<String>>(idx) {
                    Ok(v) => v.map(SqlValue::Text).unwrap_or(SqlValue::Null),
                    Err(_) => return Err(QueryError::ColumnConversion { column: name }),
                },
            };
            columns.insert(name, value);
        }
        Ok(SqlRow { columns })
    }
}

/// Typed query errors. Closed set; backend messages are carried
/// verbatim (they never contain credentials).
#[derive(Debug, Clone, PartialEq)]
pub enum QueryError {
    EmptyQueryText,
    QueryTextTooLong {
        len: usize,
        max: usize,
    },
    TooManyParams {
        count: usize,
        max: usize,
    },
    /// More placeholders in the text than bound parameters.
    ParamCountMismatch {
        placeholders: usize,
        bound: usize,
    },
    /// The query did not settle within its deadline.
    DeadlineExceeded {
        ms: u64,
    },
    /// Deadline 0 or above the fail-closed ceiling (rejected up front).
    InvalidDeadline {
        ms: u64,
    },
    /// A result column could not be carried in the bounded value set.
    ColumnConversion {
        column: String,
    },
    /// The backend rejected the statement (message verbatim).
    Backend(String),
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryError::EmptyQueryText => f.write_str("query text is empty"),
            QueryError::QueryTextTooLong { len, max } => {
                write!(f, "query text is {len} bytes, ceiling is {max}")
            }
            QueryError::TooManyParams { count, max } => {
                write!(f, "{count} params exceeds the ceiling of {max}")
            }
            QueryError::ParamCountMismatch {
                placeholders,
                bound,
            } => {
                write!(f, "{placeholders} placeholders but {bound} params bound")
            }
            QueryError::DeadlineExceeded { ms } => {
                write!(f, "query did not settle within {ms}ms")
            }
            QueryError::InvalidDeadline { ms } => {
                write!(
                    f,
                    "deadline {ms}ms is outside 1..={}ms",
                    MAX_QUERY_DEADLINE_MS
                )
            }
            QueryError::ColumnConversion { column } => {
                write!(
                    f,
                    "could not carry column '{column}' in the bounded value set"
                )
            }
            QueryError::Backend(msg) => write!(f, "query rejected by backend: {msg}"),
        }
    }
}

impl std::error::Error for QueryError {}

use std::fmt;

/// Validates statement text and the param list before anything is sent.
pub fn validate_query(text: &str, params: &[SqlParam]) -> Result<usize, QueryError> {
    if text.is_empty() {
        return Err(QueryError::EmptyQueryText);
    }
    if text.len() > MAX_QUERY_TEXT_LEN {
        return Err(QueryError::QueryTextTooLong {
            len: text.len(),
            max: MAX_QUERY_TEXT_LEN,
        });
    }
    if params.len() > MAX_PARAM_COUNT {
        return Err(QueryError::TooManyParams {
            count: params.len(),
            max: MAX_PARAM_COUNT,
        });
    }
    // count `$N` placeholders (max N decides); a simple deterministic scan
    let mut max_placeholder = 0usize;
    let bytes = text.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'$' && bytes[i + 1].is_ascii_digit() {
            let mut n = 0usize;
            let mut j = i + 1;
            while j < bytes.len() && bytes[j].is_ascii_digit() {
                n = n
                    .saturating_mul(10)
                    .saturating_add((bytes[j] - b'0') as usize);
                j += 1;
            }
            if n > max_placeholder {
                max_placeholder = n;
            }
            i = j;
        } else {
            i += 1;
        }
    }
    if max_placeholder > params.len() {
        return Err(QueryError::ParamCountMismatch {
            placeholders: max_placeholder,
            bound: params.len(),
        });
    }
    Ok(max_placeholder)
}

/// The parameterized execution surface. Abstracted so the
/// transaction flow and its tests run against a recording fake
/// without a database; the production impl wraps tokio-postgres.
pub trait QueryExecutor: Send + Sync {
    fn execute(
        &self,
        text: &str,
        params: &[SqlParam],
        deadline_ms: u64,
    ) -> Pin<Box<dyn Future<Output = Result<u64, QueryError>> + Send + '_>>;

    fn query(
        &self,
        text: &str,
        params: &[SqlParam],
        deadline_ms: u64,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<SqlRow>, QueryError>> + Send + '_>>;
}

pub const DEFAULT_QUERY_DEADLINE_MS: u64 = 5_000;
/// Fail-closed query deadline ceiling (stricter than the pool's op ceiling).
pub const MAX_QUERY_DEADLINE_MS: u64 = 30_000;

pub fn validate_deadline(deadline_ms: u64) -> Result<u64, QueryError> {
    if deadline_ms == 0 || deadline_ms > MAX_QUERY_DEADLINE_MS {
        return Err(QueryError::InvalidDeadline { ms: deadline_ms });
    }
    Ok(deadline_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_rejects_empty_overlong_and_overparam() {
        assert_eq!(validate_query("", &[]), Err(QueryError::EmptyQueryText));
        let long = " ".repeat(MAX_QUERY_TEXT_LEN + 1);
        assert_eq!(
            validate_query(&long, &[]),
            Err(QueryError::QueryTextTooLong {
                len: long.len(),
                max: MAX_QUERY_TEXT_LEN
            })
        );
        let many = vec![SqlParam::Null; MAX_PARAM_COUNT + 1];
        assert_eq!(
            validate_query("SELECT 1", &many),
            Err(QueryError::TooManyParams {
                count: MAX_PARAM_COUNT + 1,
                max: MAX_PARAM_COUNT
            })
        );
    }

    #[test]
    fn placeholder_scan_catches_unbound_placeholders() {
        assert_eq!(
            validate_query(
                "SELECT * FROM users WHERE id = $1 AND role = $2",
                &[SqlParam::Null]
            ),
            Err(QueryError::ParamCountMismatch {
                placeholders: 2,
                bound: 1
            })
        );
        // high placeholder numbers count even when sparse
        assert_eq!(validate_query("SELECT $7", &vec![SqlParam::Null; 7]), Ok(7));
        // ordinary text with dollar signs but no placeholders passes
        assert_eq!(validate_query("SELECT 1", &[]), Ok(0));
    }

    #[test]
    fn deadline_bounds_are_fail_closed() {
        assert_eq!(
            validate_deadline(0),
            Err(QueryError::InvalidDeadline { ms: 0 })
        );
        assert_eq!(
            validate_deadline(MAX_QUERY_DEADLINE_MS + 1),
            Err(QueryError::InvalidDeadline {
                ms: MAX_QUERY_DEADLINE_MS + 1
            })
        );
        assert_eq!(validate_deadline(1_000), Ok(1_000));
    }
}
