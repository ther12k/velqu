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

/// Environment variable names for pool limits (BETA-004-E). All optional;
/// absent values fall back to `PoolConfig::default_config()`. Present but
/// invalid values are **startup rejections, never clamps**.
pub const ENV_POOL_MAX: &str = "VELQU_PG_POOL_MAX";
pub const ENV_POOL_CONNECT_TIMEOUT_MS: &str = "VELQU_PG_POOL_CONNECT_TIMEOUT_MS";
pub const ENV_POOL_IDLE_TIMEOUT_MS: &str = "VELQU_PG_POOL_IDLE_TIMEOUT_MS";

/// Resolve pool config from an env-style lookup (injectable for tests).
pub fn pool_config_from_lookup(
    get: impl Fn(&str) -> Option<String>,
) -> Result<PoolConfig, PoolError> {
    let mut config = PoolConfig::default_config();
    if let Some(raw) = get(ENV_POOL_MAX) {
        let n: usize = raw.trim().parse().map_err(|_| PoolError::InvalidConfig {
            detail: "VELQU_PG_POOL_MAX must be an integer",
        })?;
        config.max_connections = n;
    }
    if let Some(raw) = get(ENV_POOL_CONNECT_TIMEOUT_MS) {
        let n: u64 = raw.trim().parse().map_err(|_| PoolError::InvalidConfig {
            detail: "VELQU_PG_POOL_CONNECT_TIMEOUT_MS must be an integer",
        })?;
        config.connect_timeout_ms = n;
    }
    if let Some(raw) = get(ENV_POOL_IDLE_TIMEOUT_MS) {
        let n: u64 = raw.trim().parse().map_err(|_| PoolError::InvalidConfig {
            detail: "VELQU_PG_POOL_IDLE_TIMEOUT_MS must be an integer",
        })?;
        config.idle_timeout_ms = n;
    }
    // bounds are enforced by PoolConfig::new — invalid combos reject
    PoolConfig::new(
        config.max_connections,
        config.connect_timeout_ms,
        config.idle_timeout_ms,
    )
}

/// Build the handle from a database URL plus env-configured limits.
/// Fail closed on invalid limits: the caller turns the error into a
/// startup rejection.
pub fn pool_from_url_and_env(
    url: impl Into<String>,
    get: impl Fn(&str) -> Option<String>,
) -> Result<PostgresQueryHandle, PoolError> {
    let config = pool_config_from_lookup(get)?;
    Ok(pool_from_url(url, config))
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

#[cfg(test)]
mod env_config_tests {
    use super::*;
    use std::collections::HashMap;

    fn lookup(map: &HashMap<String, String>) -> impl Fn(&str) -> Option<String> + '_ {
        move |k: &str| map.get(k).cloned()
    }

    #[test]
    fn defaults_when_env_absent() {
        let cfg = pool_config_from_lookup(|_| None).unwrap();
        assert_eq!(cfg, PoolConfig::default_config());
    }

    #[test]
    fn valid_env_values_override_defaults() {
        let mut env = HashMap::new();
        env.insert(ENV_POOL_MAX.to_string(), "4".to_string());
        env.insert(ENV_POOL_CONNECT_TIMEOUT_MS.to_string(), "2500".to_string());
        env.insert(ENV_POOL_IDLE_TIMEOUT_MS.to_string(), "10000".to_string());
        let cfg = pool_config_from_lookup(lookup(&env)).unwrap();
        assert_eq!(
            cfg,
            PoolConfig {
                max_connections: 4,
                connect_timeout_ms: 2500,
                idle_timeout_ms: 10000
            }
        );
    }

    #[test]
    fn out_of_bounds_values_reject_startup_never_clamp() {
        let mut env = HashMap::new();
        env.insert(ENV_POOL_MAX.to_string(), "1000".to_string());
        assert!(pool_config_from_lookup(lookup(&env)).is_err());
        let mut env = HashMap::new();
        env.insert(ENV_POOL_CONNECT_TIMEOUT_MS.to_string(), "0".to_string());
        assert!(pool_config_from_lookup(lookup(&env)).is_err());
        let mut env = HashMap::new();
        env.insert(ENV_POOL_MAX.to_string(), "ten".to_string());
        assert!(pool_config_from_lookup(lookup(&env)).is_err());
    }
}
