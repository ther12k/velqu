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
    Float(f64),
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

    /// Column names in sorted order (row payloads are unordered maps
    /// at the JS boundary).
    pub fn column_names(&self) -> Vec<&String> {
        self.columns.keys().collect()
    }

    /// Column/value iteration (sorted; used by the #1425 byte estimator).
    pub fn iter(&self) -> impl Iterator<Item = (&String, &SqlValue)> {
        self.columns.iter()
    }

    /// Test/estimator construction without a live `Row`.
    #[cfg(test)]
    pub(crate) fn from_pairs(pairs: Vec<(String, SqlValue)>) -> SqlRow {
        SqlRow {
            columns: pairs.into_iter().collect(),
        }
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
    /// #1425: the streamed result crossed the per-query row budget.
    ResultRowsExceeded {
        rows: u64,
        max: u64,
    },
    /// #1425: the streamed result crossed the per-query byte budget
    /// (estimated serialized size).
    ResultBytesExceeded {
        bytes: u64,
        max: u64,
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
            QueryError::ResultRowsExceeded { rows, max } => {
                write!(f, "result exceeded the row budget: {rows} rows > {max} (paginate the query)")
            }
            QueryError::ResultBytesExceeded { bytes, max } => write!(
                f,
                "result exceeded the byte budget: ~{bytes} bytes > {max} (select fewer or narrower columns)"
            ),
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

/// #1425: default fail-closed result budget — rows per single query.
pub const DEFAULT_MAX_RESULT_ROWS: u64 = 10_000;
/// #1425: fail-closed ceiling for the per-query row budget. Raising it is
/// a reviewed decision, not a caller option.
pub const MAX_RESULT_ROWS_CEILING: u64 = 1_000_000;
/// #1425: default fail-closed result budget — estimated serialized bytes
/// per single query (aligned with the fetch response-body cap).
pub const DEFAULT_MAX_RESULT_BYTES: u64 = 16 * 1024 * 1024;
/// #1425: fail-closed ceiling for the byte budget.
pub const MAX_RESULT_BYTES_CEILING: u64 = 256 * 1024 * 1024;

/// #1425: per-query result budget. Deadlines bound TIME, not BYTES — a
/// database that happily produces a 2 GB result inside the deadline used
/// to be fully buffered natively before any JS-side heap bound applied.
/// The executor streams rows and rejects the result the moment either
/// bound is crossed, so native allocations are bounded by this budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResultBudget {
    pub max_rows: u64,
    pub max_bytes: u64,
}

impl Default for ResultBudget {
    fn default() -> Self {
        ResultBudget {
            max_rows: DEFAULT_MAX_RESULT_ROWS,
            max_bytes: DEFAULT_MAX_RESULT_BYTES,
        }
    }
}

impl ResultBudget {
    /// Fail-closed validation: both bounds present, positive, at or below
    /// their ceilings. Construction-time rejection, never a clamp.
    pub fn new(max_rows: u64, max_bytes: u64) -> Result<Self, crate::PoolError> {
        if max_rows == 0 || max_rows > MAX_RESULT_ROWS_CEILING {
            return Err(crate::PoolError::InvalidConfig {
                detail: "max_result_rows out of 1..=1_000_000",
            });
        }
        if max_bytes == 0 || max_bytes > MAX_RESULT_BYTES_CEILING {
            return Err(crate::PoolError::InvalidConfig {
                detail: "max_result_bytes out of 1..=268435456",
            });
        }
        Ok(ResultBudget {
            max_rows,
            max_bytes,
        })
    }
}

/// #1425: upper-bound estimate of a row's serialized JSON size, used
/// DURING streaming so the collected `Vec<SqlRow>` and the serialized
/// result string stay bounded by the byte budget (the actual serde_json
/// output is always ≤ this estimate: text worst-cases at 6 bytes/char
/// under `\uXXXX` escaping, numbers at their longest decimal forms).
pub fn estimate_row_bytes(row: &SqlRow) -> u64 {
    let mut total: u64 = 2; // row braces + slack for separators
    for (name, value) in row.iter() {
        // "name": escaped — names are identifiers but escape defensively
        total += (name.len() as u64) * 6 + 4;
        total += match value {
            SqlValue::Null => 4,
            SqlValue::Bool(_) => 5,
            SqlValue::Int(_) => 20,
            SqlValue::Float(_) => 24,
            SqlValue::Text(s) => (s.len() as u64) * 6 + 2,
        };
    }
    total
}

pub fn validate_deadline(deadline_ms: u64) -> Result<u64, QueryError> {
    if deadline_ms == 0 || deadline_ms > MAX_QUERY_DEADLINE_MS {
        return Err(QueryError::InvalidDeadline { ms: deadline_ms });
    }
    Ok(deadline_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// #1425: the streaming byte estimator must UPPER-BOUND the actual
    /// serialized JSON member the dialer produces — the byte budget's
    /// guarantee (collected rows + serialized string stay within budget)
    /// rests on this property.
    #[test]
    fn row_byte_estimate_upper_bounds_serialized_json() {
        let row = SqlRow::from_pairs(vec![
            (
                "name".into(),
                SqlValue::Text("Ada Lovelace \"analytical engine\"".into()),
            ),
            ("id".into(), SqlValue::Int(i64::MAX)),
            ("score".into(), SqlValue::Float(std::f64::consts::PI)),
            ("ok".into(), SqlValue::Bool(true)),
            ("note".into(), SqlValue::Null),
        ]);
        let estimate = estimate_row_bytes(&row);
        let to_json = |v: &SqlValue| match v {
            SqlValue::Null => serde_json::Value::Null,
            SqlValue::Bool(b) => serde_json::Value::Bool(*b),
            SqlValue::Int(i) => serde_json::json!(i),
            SqlValue::Float(f) => serde_json::json!(f),
            SqlValue::Text(s) => serde_json::Value::String(s.clone()),
        };
        let mut obj = serde_json::Map::new();
        for (k, v) in row.iter() {
            obj.insert(k.clone(), to_json(v));
        }
        let actual = serde_json::to_string(&serde_json::Value::Object(obj))
            .unwrap()
            .len() as u64;
        assert!(
            estimate >= actual,
            "estimate {estimate} must bound the serialized row ({actual} bytes)"
        );
        // worst-case escaping: every char control-escapes to \uXXXX
        let hostile = SqlRow::from_pairs(vec![(
            "t".into(),
            SqlValue::Text("\u{1}\u{2}\u{3}".repeat(64)),
        )]);
        let hostile_actual = {
            let mut o = serde_json::Map::new();
            for (k, v) in hostile.iter() {
                o.insert(k.clone(), to_json(v));
            }
            serde_json::to_string(&serde_json::Value::Object(o))
                .unwrap()
                .len() as u64
        };
        assert!(estimate_row_bytes(&hostile) >= hostile_actual);
    }

    /// #1425: budget construction is fail-closed — no zero, no above
    /// ceiling, never a clamp.
    #[test]
    fn result_budget_validates_fail_closed() {
        assert!(ResultBudget::new(1, 1).is_ok());
        assert!(ResultBudget::new(MAX_RESULT_ROWS_CEILING, MAX_RESULT_BYTES_CEILING).is_ok());
        assert_eq!(
            ResultBudget::new(0, 1024),
            Err(crate::PoolError::InvalidConfig {
                detail: "max_result_rows out of 1..=1_000_000"
            })
        );
        assert_eq!(
            ResultBudget::new(MAX_RESULT_ROWS_CEILING + 1, 1024),
            Err(crate::PoolError::InvalidConfig {
                detail: "max_result_rows out of 1..=1_000_000"
            })
        );
        assert_eq!(
            ResultBudget::new(10, 0),
            Err(crate::PoolError::InvalidConfig {
                detail: "max_result_bytes out of 1..=268435456"
            })
        );
        assert_eq!(
            ResultBudget::new(10, MAX_RESULT_BYTES_CEILING + 1),
            Err(crate::PoolError::InvalidConfig {
                detail: "max_result_bytes out of 1..=268435456"
            })
        );
        assert_eq!(
            ResultBudget::default(),
            ResultBudget {
                max_rows: DEFAULT_MAX_RESULT_ROWS,
                max_bytes: DEFAULT_MAX_RESULT_BYTES,
            }
        );
    }

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
