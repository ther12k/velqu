//! Engine-facing query dialer (BETA-004-D).
//!
//! The contract the worker binds as `__velquPostgresQuery`: one call,
//! one bounded parameterized query, rows serialized as a JSON array of
//! row objects. The dialer owns the deadline across acquire **and**
//! execution, and — critically for safety — a lease that ends in an
//! error is **discarded, never parked**: a connection whose query
//! failed or timed out mid-flight may still hold backend state (the
//! server may keep streaming rows after a client-side cancel), so the
//! only safe release is close. Reuse is reserved for leases that
//! completed cleanly.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{json, Map, Value};

use crate::executor::ClientExecutor;
use crate::query::{QueryExecutor, SqlParam, SqlValue, MAX_PARAM_COUNT, MAX_QUERY_TEXT_LEN};
use crate::{LazyPool, PoolConfig, PoolError, TokioConnector};

/// Maximum parameters accepted through the JSON path (mirrors the
/// executor ceiling; the constant lives in `query`).
pub const MAX_JSON_PARAMS: usize = 100;

/// One bounded parameterized query. `params_json` is a JSON array of
/// scalars (`null`, boolean, number, string). Returns a JSON array of
/// row objects (column name -> scalar), or an error string (redacted,
/// typed-prefix). Implementations must be Send+Sync and safe to call
/// concurrently.
pub trait PostgresQueryDialer: Send + Sync {
    fn query_json(
        &self,
        text: String,
        params_json: String,
        deadline_ms: u64,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send>>;
}

/// Parse the JSON parameter array into the closed `SqlParam` set.
/// Numbers bind as integers when integral, floats otherwise; any other
/// JSON shape is a typed rejection (objects/arrays never silently
/// stringify — that would be an interpolation door).
pub fn parse_params_json(params_json: &str) -> Result<Vec<SqlParam>, PoolError> {
    let value: Value = serde_json::from_str(params_json).map_err(|_| PoolError::InvalidConfig {
        detail: "params must be a JSON array",
    })?;
    let arr = value.as_array().ok_or(PoolError::InvalidConfig {
        detail: "params must be a JSON array",
    })?;
    if arr.len() > MAX_PARAM_COUNT {
        return Err(PoolError::InvalidConfig {
            detail: "too many params",
        });
    }
    arr.iter()
        .map(|v| match v {
            Value::Null => Ok(SqlParam::Null),
            Value::Bool(b) => Ok(SqlParam::Bool(*b)),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(SqlParam::Int(i))
                } else {
                    n.as_f64()
                        .map(SqlParam::Float)
                        .ok_or(PoolError::InvalidConfig {
                            detail: "number out of range",
                        })
                }
            }
            Value::String(s) => Ok(SqlParam::Text(s.clone())),
            Value::Object(_) | Value::Array(_) => Err(PoolError::InvalidConfig {
                detail: "nested params are not supported (scalars only)",
            }),
        })
        .collect()
}

fn value_to_json(v: &SqlValue) -> Value {
    match v {
        SqlValue::Null => Value::Null,
        SqlValue::Bool(b) => Value::Bool(*b),
        SqlValue::Int(i) => json!(i),
        SqlValue::Float(f) => json!(f),
        SqlValue::Text(s) => Value::String(s.clone()),
    }
}

fn rows_to_json(rows: Vec<crate::query::SqlRow>) -> String {
    let arr: Vec<Value> = rows
        .iter()
        .map(|row| {
            let mut obj = Map::new();
            for column in row.column_names() {
                obj.insert(
                    column.clone(),
                    value_to_json(row.get(column).expect("column exists")),
                );
            }
            Value::Object(obj)
        })
        .collect();
    serde_json::to_string(&arr).unwrap_or_else(|_| "[]".to_string())
}

impl PostgresQueryDialer for LazyPool<TokioConnector> {
    fn query_json(
        &self,
        text: String,
        params_json: String,
        deadline_ms: u64,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send>> {
        let this = self.clone();
        Box::pin(async move {
            if text.is_empty() || text.len() > MAX_QUERY_TEXT_LEN {
                return Err("postgres: statement text empty or over the ceiling".into());
            }
            let params = parse_params_json(&params_json).map_err(|e: PoolError| e.to_string())?;
            if params.len() > MAX_JSON_PARAMS {
                return Err("postgres: too many params".into());
            }
            let mut lease = this.acquire(deadline_ms).await.map_err(|e| e.to_string())?;
            let executor = ClientExecutor::new(lease.get());
            match executor.query(&text, &params, deadline_ms).await {
                Ok(rows) => Ok(rows_to_json(rows)),
                // mid-flight failure or timeout: the connection may hold
                // backend state — discard (close), never park
                Err(e) => {
                    lease.discard();
                    Err(format!("postgres: {e}"))
                }
            }
        })
    }
}

/// Typed handle the engine holds; mirrors `FetchDialerHandle`.
#[derive(Clone)]
pub struct PostgresQueryHandle(pub Arc<dyn PostgresQueryDialer>);

impl std::fmt::Debug for PostgresQueryHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PostgresQueryHandle")
    }
}

impl From<LazyPool<TokioConnector>> for PostgresQueryHandle {
    fn from(pool: LazyPool<TokioConnector>) -> Self {
        PostgresQueryHandle(Arc::new(pool))
    }
}

/// Construction helper preserving the A/B cost posture: no URL parse,
/// no I/O — laziness is structural (see `PoolConfig`).
pub fn pool_from_url(url: impl Into<String>, config: PoolConfig) -> PostgresQueryHandle {
    PostgresQueryHandle::from(LazyPool::<TokioConnector>::postgres(url, config))
}

// silence unused-import lint for PooledConnection in non-test builds
#[cfg(test)]
mod tests {
    #[test]
    fn json_params_reject_nested_shapes() {
        use super::parse_params_json;
        assert!(parse_params_json("[1, \"a\", null, true]").is_ok());
        assert!(parse_params_json("[{\"nested\": 1}]").is_err());
        assert!(parse_params_json("{\"not\": \"array\"}").is_err());
        assert!(parse_params_json("not json").is_err());
    }
}
