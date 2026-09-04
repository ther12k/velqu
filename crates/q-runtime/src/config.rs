//! BETA-007-A: typed environment/file configuration for the runtime.
//!
//! Every operational field resolves through the same layer stack, first
//! hit wins:
//!
//! ```text
//! CLI flag > environment > config file > built-in default
//! ```
//!
//! The file surface is versioned: `configVersion` is required and only
//! version 1 is accepted. Unknown file fields, wrong types, and
//! out-of-range values fail closed BEFORE the engine or listener exist
//! — startup exits 2 with a typed `startup.rejected` line. Values are
//! never clamped into range: a rejected value is a rejected startup
//! (GLOBAL_INVARIANTS: bounded configuration, no silent limits).
//!
//! Secrets are deliberately NOT part of this surface: capability
//! credentials (e.g. VELQU_DATABASE_URL) stay environment-only, are
//! never read by this module, and never appear in configuration errors
//! (redaction-tested).

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::serve::LogMode;

/// The only supported config-file schema version.
pub const CONFIG_VERSION: u64 = 1;

/// Built-in defaults (the historical `Limits::default()` posture).
pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 3000;
pub const DEFAULT_LOG: &str = "errors";
pub const DEFAULT_MAX_BODY_BYTES: usize = 1 << 20; // 1 MiB
pub const DEFAULT_MAX_QUEUE: usize = 256;

/// Fail-closed ceilings — resolved values outside these ranges reject
/// startup; they are never clamped.
pub const MAX_BODY_BYTES_CEILING: usize = 64 << 20; // 64 MiB
pub const MAX_QUEUE_CEILING: usize = 10_000;
pub const LOG_SAMPLE_CEILING: u64 = 1_000_000_000;
pub const HOST_MAX_BYTES: usize = 253;

pub const ENV_HOST: &str = "VELQU_HOST";
pub const ENV_PORT: &str = "VELQU_PORT";
/// Legacy port variable, kept for compatibility below `VELQU_PORT`.
pub const ENV_LEGACY_PORT: &str = "PORT";
pub const ENV_LOG: &str = "VELQU_LOG";
pub const ENV_LOG_SAMPLE: &str = "VELQU_LOG_SAMPLE";
pub const ENV_MAX_BODY_BYTES: &str = "VELQU_MAX_BODY_BYTES";
pub const ENV_MAX_QUEUE: &str = "VELQU_MAX_QUEUE";
pub const ENV_CONFIG: &str = "VELQU_CONFIG";

/// Versioned, typed config-file schema (`--config` / `VELQU_CONFIG`).
///
/// `configVersion` is mandatory; unknown fields are rejected outright so
/// a typo can never silently disable a limit.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileConfig {
    #[serde(rename = "configVersion")]
    config_version: u64,
    host: Option<String>,
    port: Option<u16>,
    #[serde(rename = "maxBodyBytes")]
    max_body_bytes: Option<u64>,
    #[serde(rename = "maxQueue")]
    max_queue: Option<u64>,
    log: Option<String>,
    #[serde(rename = "logSample")]
    log_sample: Option<u64>,
}

/// Explicit CLI layer (already parsed; `None` = flag not given).
#[derive(Debug, Clone, Default)]
pub struct CliConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub config: Option<PathBuf>,
    pub log: Option<String>,
    pub log_sample: Option<u64>,
}

/// Injected environment/file lookups so resolution is unit-testable
/// without touching the process environment.
pub struct Sources<'a> {
    pub cli: CliConfig,
    pub env: &'a dyn Fn(&str) -> Option<String>,
    pub read_file: &'a dyn Fn(&Path) -> std::io::Result<String>,
}

/// The fully resolved, validated configuration consumed by the startup
/// pipeline. `log` is the canonical lowercase mode string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolved {
    pub host: String,
    pub port: u16,
    pub max_body_bytes: usize,
    pub max_queue: usize,
    pub log: &'static str,
    pub log_sample: u64,
}

/// Typed configuration rejection. Rendered reasons name the field, the
/// source layer, and the expected shape — they never echo file contents
/// or values of environment variables outside this typed surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    /// The selected config file could not be read (path + IO detail only).
    FileRead { path: String, detail: String },
    /// The config file is not valid JSON, violates the schema, or omits
    /// the required `configVersion`.
    FileSchema { path: String, detail: String },
    /// `configVersion` present but not supported.
    UnsupportedConfigVersion { path: String, found: u64 },
    /// An environment value did not parse into the field's type.
    EnvParse {
        var: &'static str,
        value: String,
        expected: &'static str,
    },
    /// A resolved numeric value is outside its declared range.
    OutOfBounds {
        field: &'static str,
        value: u64,
        range: String,
        source: String,
    },
    /// host violated shape constraints (empty / too long / whitespace).
    InvalidHost { source: String, reason: String },
    /// log mode outside the closed off|errors|full set.
    InvalidLogMode { source: String, value: String },
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileRead { path, detail } => {
                write!(f, "config file '{path}' could not be read: {detail}")
            }
            ConfigError::FileSchema { path, detail } => {
                write!(f, "config file '{path}' rejected: {detail}")
            }
            ConfigError::UnsupportedConfigVersion { path, found } => write!(
                f,
                "config file '{path}' rejected: unsupported configVersion {found} (supported: {CONFIG_VERSION})"
            ),
            ConfigError::EnvParse {
                var,
                value,
                expected,
            } => write!(f, "environment variable {var} must be {expected}, got {value:?}"),
            ConfigError::OutOfBounds {
                field,
                value,
                range,
                source,
            } => write!(
                f,
                "{field} from {source} must be in range {range}, got {value} (values are never clamped)"
            ),
            ConfigError::InvalidHost { source, reason } => {
                write!(f, "host from {source} is invalid: {reason}")
            }
            ConfigError::InvalidLogMode { source, value } => write!(
                f,
                "log mode from {source} must be off|errors|full, got {value:?}"
            ),
        }
    }
}

fn validate_host(h: &str, source: &str) -> Result<(), ConfigError> {
    if h.is_empty() {
        return Err(ConfigError::InvalidHost {
            source: source.to_string(),
            reason: "empty".to_string(),
        });
    }
    if h.len() > HOST_MAX_BYTES {
        return Err(ConfigError::InvalidHost {
            source: source.to_string(),
            reason: format!("longer than {HOST_MAX_BYTES} bytes"),
        });
    }
    // Printable ASCII only: no whitespace/control characters, so the
    // resolved host can never smuggle structure into the startup line.
    if h.bytes().any(|b| !(0x21..=0x7e).contains(&b)) {
        return Err(ConfigError::InvalidHost {
            source: source.to_string(),
            reason: "contains whitespace or control characters".to_string(),
        });
    }
    Ok(())
}

fn parse_env_u64(s: &str, var: &'static str) -> Result<u64, ConfigError> {
    s.trim().parse::<u64>().map_err(|_| ConfigError::EnvParse {
        var,
        value: s.to_string(),
        expected: "a non-negative integer",
    })
}

fn check_range(
    field: &'static str,
    value: u64,
    min: u64,
    max: u64,
    source: &str,
) -> Result<(), ConfigError> {
    if value < min || value > max {
        return Err(ConfigError::OutOfBounds {
            field,
            value,
            range: format!("{min}..={max}"),
            source: source.to_string(),
        });
    }
    Ok(())
}

/// Resolve the full configuration from the CLI, environment, and (optionally)
/// the versioned config file. Fails closed on any invalid layer.
pub fn resolve(s: Sources) -> Result<Resolved, ConfigError> {
    // ---- layer: config file (selected by --config, else VELQU_CONFIG)
    let selected: Option<(String, PathBuf)> = match (&s.cli.config, (s.env)(ENV_CONFIG)) {
        (Some(p), _) => Some((p.display().to_string(), p.clone())),
        (None, Some(v)) if !v.trim().is_empty() => Some((v.clone(), PathBuf::from(&v))),
        (None, _) => None,
    };
    let file: Option<(String, FileConfig)> = match selected {
        None => None,
        Some((display, path)) => {
            let text = (s.read_file)(&path).map_err(|e| ConfigError::FileRead {
                path: display.clone(),
                detail: e.to_string(),
            })?;
            let parsed: FileConfig =
                serde_json::from_str(&text).map_err(|e| ConfigError::FileSchema {
                    path: display.clone(),
                    detail: e.to_string(),
                })?;
            if parsed.config_version != CONFIG_VERSION {
                return Err(ConfigError::UnsupportedConfigVersion {
                    path: display,
                    found: parsed.config_version,
                });
            }
            Some((display, parsed))
        }
    };
    let (file_path, file_host, file_port, file_body, file_queue, file_log, file_sample) =
        match &file {
            Some((p, f)) => (
                Some(p.as_str()),
                f.host.clone(),
                f.port,
                f.max_body_bytes,
                f.max_queue,
                f.log.clone(),
                f.log_sample,
            ),
            None => (None, None, None, None, None, None, None),
        };

    // ---- host: cli > env > file > default
    let host = if let Some(h) = &s.cli.host {
        validate_host(h, "cli")?;
        h.clone()
    } else if let Some(h) = (s.env)(ENV_HOST) {
        validate_host(&h, "env:VELQU_HOST")?;
        h
    } else if let Some(h) = file_host {
        validate_host(&h, &file_source(file_path))?;
        h
    } else {
        DEFAULT_HOST.to_string()
    };

    // ---- port: cli > VELQU_PORT > PORT > file > default
    let port_src = |v: u64, source: &str| -> Result<u16, ConfigError> {
        check_range("port", v, 1, u16::MAX as u64, source)?;
        Ok(v as u16)
    };
    let port = if let Some(p) = s.cli.port {
        port_src(p as u64, "cli")?
    } else if let Some(v) = (s.env)(ENV_PORT) {
        let n = parse_env_u64(&v, ENV_PORT)?;
        port_src(n, "env:VELQU_PORT")?
    } else if let Some(v) = (s.env)(ENV_LEGACY_PORT) {
        let n = parse_env_u64(&v, ENV_LEGACY_PORT)?;
        port_src(n, "env:PORT")?
    } else if let Some(p) = file_port {
        port_src(p as u64, &file_source(file_path))?
    } else {
        DEFAULT_PORT
    };

    // ---- maxBodyBytes: env > file > default
    let max_body_bytes = if let Some(v) = (s.env)(ENV_MAX_BODY_BYTES) {
        let n = parse_env_u64(&v, ENV_MAX_BODY_BYTES)?;
        check_range(
            "maxBodyBytes",
            n,
            1,
            MAX_BODY_BYTES_CEILING as u64,
            "env:VELQU_MAX_BODY_BYTES",
        )?;
        n as usize
    } else if let Some(n) = file_body {
        check_range(
            "maxBodyBytes",
            n,
            1,
            MAX_BODY_BYTES_CEILING as u64,
            &file_source(file_path),
        )?;
        n as usize
    } else {
        DEFAULT_MAX_BODY_BYTES
    };

    // ---- maxQueue: env > file > default
    let max_queue = if let Some(v) = (s.env)(ENV_MAX_QUEUE) {
        let n = parse_env_u64(&v, ENV_MAX_QUEUE)?;
        check_range(
            "maxQueue",
            n,
            1,
            MAX_QUEUE_CEILING as u64,
            "env:VELQU_MAX_QUEUE",
        )?;
        n as usize
    } else if let Some(n) = file_queue {
        check_range(
            "maxQueue",
            n,
            1,
            MAX_QUEUE_CEILING as u64,
            &file_source(file_path),
        )?;
        n as usize
    } else {
        DEFAULT_MAX_QUEUE
    };

    // ---- log: cli > env > file > default (closed set, canonicalized)
    let log: &'static str = if let Some(v) = &s.cli.log {
        parse_log(v, "cli")?
    } else if let Some(v) = (s.env)(ENV_LOG) {
        parse_log(&v, "env:VELQU_LOG")?
    } else if let Some(v) = file_log {
        parse_log(&v, &file_source(file_path))?
    } else {
        DEFAULT_LOG
    };

    // ---- logSample: cli > env > file > default
    let log_sample = if let Some(v) = s.cli.log_sample {
        check_range("logSample", v, 0, LOG_SAMPLE_CEILING, "cli")?;
        v
    } else if let Some(v) = (s.env)(ENV_LOG_SAMPLE) {
        let n = parse_env_u64(&v, ENV_LOG_SAMPLE)?;
        check_range(
            "logSample",
            n,
            0,
            LOG_SAMPLE_CEILING,
            "env:VELQU_LOG_SAMPLE",
        )?;
        n
    } else if let Some(n) = file_sample {
        check_range(
            "logSample",
            n,
            0,
            LOG_SAMPLE_CEILING,
            &file_source(file_path),
        )?;
        n
    } else {
        0
    };

    Ok(Resolved {
        host,
        port,
        max_body_bytes,
        max_queue,
        log,
        log_sample,
    })
}

fn file_source(path: Option<&str>) -> String {
    match path {
        Some(p) => format!("file:{p}"),
        None => "file".to_string(),
    }
}

fn parse_log(v: &str, source: &str) -> Result<&'static str, ConfigError> {
    LogMode::parse_checked(v)
        .map(|m| m.as_str())
        .map_err(|_| ConfigError::InvalidLogMode {
            source: source.to_string(),
            value: v.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_env(_: &str) -> Option<String> {
        None
    }

    fn env_of<'a>(vars: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        move |k: &str| {
            vars.iter()
                .find(|(name, _)| *name == k)
                .map(|(_, v)| v.to_string())
        }
    }

    fn no_file(_: &Path) -> std::io::Result<String> {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "no config file layer",
        ))
    }

    fn file_map<'a>(
        files: &'a [(&'a str, &'a str)],
    ) -> impl Fn(&Path) -> std::io::Result<String> + 'a {
        move |p: &Path| {
            let key = p.display().to_string();
            files
                .iter()
                .find(|(name, _)| *name == key)
                .map(|(_, body)| body.to_string())
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "file not in map"))
        }
    }

    fn cli_default() -> CliConfig {
        CliConfig::default()
    }

    fn resolve_with(
        cli: CliConfig,
        env: &dyn Fn(&str) -> Option<String>,
        read: &dyn Fn(&Path) -> std::io::Result<String>,
    ) -> Result<Resolved, ConfigError> {
        resolve(Sources {
            cli,
            env,
            read_file: read,
        })
    }

    #[test]
    fn defaults_when_nothing_is_configured() {
        let r = resolve_with(cli_default(), &no_env, &no_file).unwrap();
        assert_eq!(r.host, DEFAULT_HOST);
        assert_eq!(r.port, DEFAULT_PORT);
        assert_eq!(r.max_body_bytes, DEFAULT_MAX_BODY_BYTES);
        assert_eq!(r.max_queue, DEFAULT_MAX_QUEUE);
        assert_eq!(r.log, "errors");
        assert_eq!(r.log_sample, 0);
    }

    #[test]
    fn file_overrides_defaults() {
        let read = file_map(&[(
            "/app/velqu.config.json",
            r#"{"configVersion":1,"maxBodyBytes":2097152,"maxQueue":512}"#,
        )]);
        let cli = CliConfig {
            config: Some("/app/velqu.config.json".into()),
            ..cli_default()
        };
        let r = resolve_with(cli, &no_env, &read).unwrap();
        assert_eq!(r.max_body_bytes, 2_097_152);
        assert_eq!(r.max_queue, 512);
        assert_eq!(r.host, DEFAULT_HOST);
    }

    #[test]
    fn env_overrides_file() {
        let read = file_map(&[(
            "/app/velqu.config.json",
            r#"{"configVersion":1,"maxQueue":512}"#,
        )]);
        let e = env_of(&[("VELQU_MAX_QUEUE", "1024")]);
        let cli = CliConfig {
            config: Some("/app/velqu.config.json".into()),
            ..cli_default()
        };
        let r = resolve_with(cli, &e, &read).unwrap();
        assert_eq!(r.max_queue, 1024);
        // file still supplies what env does not set
        let e2 = env_of(&[]);
        let cli2 = CliConfig {
            config: Some("/app/velqu.config.json".into()),
            ..cli_default()
        };
        let r2 = resolve_with(cli2, &e2, &read).unwrap();
        assert_eq!(r2.max_queue, 512);
    }

    #[test]
    fn cli_overrides_env() {
        let e = env_of(&[("VELQU_PORT", "4000"), ("VELQU_LOG", "off")]);
        let cli = CliConfig {
            port: Some(5000),
            log: Some("full".to_string()),
            ..cli_default()
        };
        let r = resolve_with(cli, &e, &no_file).unwrap();
        assert_eq!(r.port, 5000);
        assert_eq!(r.log, "full");
    }

    #[test]
    fn legacy_port_env_still_works_and_velqu_port_wins() {
        let both = env_of(&[("PORT", "4001"), ("VELQU_PORT", "4002")]);
        let r = resolve_with(cli_default(), &both, &no_file).unwrap();
        assert_eq!(r.port, 4002);
        let legacy = env_of(&[("PORT", "4001")]);
        let r = resolve_with(cli_default(), &legacy, &no_file).unwrap();
        assert_eq!(r.port, 4001);
    }

    #[test]
    fn unknown_file_field_fails_closed() {
        let read = file_map(&[(
            "/app/velqu.config.json",
            r#"{"configVersion":1,"maxBodyBytes":1048576,"maxBodyByets":1}"#,
        )]);
        let cli = CliConfig {
            config: Some("/app/velqu.config.json".into()),
            ..cli_default()
        };
        let err = resolve_with(cli, &no_env, &read).unwrap_err();
        assert!(
            err.to_string().contains("unknown field"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn missing_or_unsupported_config_version_fails_closed() {
        // Legacy unversioned files (maxBodyBytes/maxQueue only) are
        // rejected: configuration is versioned as of BETA-007-A.
        let legacy = file_map(&[("/app/old.json", r#"{"maxBodyBytes":1048576}"#)]);
        let cli = CliConfig {
            config: Some("/app/old.json".into()),
            ..cli_default()
        };
        let err = resolve_with(cli.clone(), &no_env, &legacy).unwrap_err();
        assert!(err.to_string().contains("configVersion"), "{err}");

        let v2 = file_map(&[("/app/v2.json", r#"{"configVersion":2}"#)]);
        let cli = CliConfig {
            config: Some("/app/v2.json".into()),
            ..cli_default()
        };
        let err = resolve_with(cli, &no_env, &v2).unwrap_err();
        assert!(
            matches!(err, ConfigError::UnsupportedConfigVersion { found: 2, .. }),
            "{err}"
        );
    }

    #[test]
    fn file_type_errors_fail_closed() {
        let read = file_map(&[(
            "/app/velqu.config.json",
            r#"{"configVersion":1,"maxQueue":"many"}"#,
        )]);
        let cli = CliConfig {
            config: Some("/app/velqu.config.json".into()),
            ..cli_default()
        };
        let err = resolve_with(cli, &no_env, &read).unwrap_err();
        assert!(matches!(err, ConfigError::FileSchema { .. }), "{err}");
    }

    #[test]
    fn out_of_bounds_values_rejected_never_clamped() {
        let read = file_map(&[(
            "/app/big.json",
            r#"{"configVersion":1,"maxBodyBytes":999999999999}"#,
        )]);
        let cli = CliConfig {
            config: Some("/app/big.json".into()),
            ..cli_default()
        };
        let err = resolve_with(cli, &no_env, &read).unwrap_err();
        assert!(
            err.to_string().contains("never clamped") && err.to_string().contains("maxBodyBytes"),
            "{err}"
        );

        let e = env_of(&[("VELQU_MAX_QUEUE", "0")]);
        let err = resolve_with(cli_default(), &e, &no_file).unwrap_err();
        assert!(
            matches!(err, ConfigError::OutOfBounds { value: 0, .. }),
            "{err}"
        );

        let e = env_of(&[("VELQU_MAX_QUEUE", "10001")]);
        let err = resolve_with(cli_default(), &e, &no_file).unwrap_err();
        assert!(
            matches!(err, ConfigError::OutOfBounds { value: 10001, .. }),
            "{err}"
        );
    }

    #[test]
    fn invalid_env_values_fail_closed() {
        for (var, val) in [
            ("VELQU_PORT", "http"),
            ("PORT", "0x30"),
            ("VELQU_LOG_SAMPLE", "-1"),
            ("VELQU_MAX_BODY_BYTES", "1MiB"),
        ] {
            let pair = [(var, val)];
            let e = env_of(&pair);
            let err = resolve_with(cli_default(), &e, &no_file).unwrap_err();
            assert!(
                matches!(err, ConfigError::EnvParse { .. }),
                "{var}={val}: {err}"
            );
        }
    }

    #[test]
    fn port_zero_rejected_everywhere() {
        let e = env_of(&[("PORT", "0")]);
        let err = resolve_with(cli_default(), &e, &no_file).unwrap_err();
        assert!(matches!(err, ConfigError::OutOfBounds { .. }), "{err}");
    }

    #[test]
    fn invalid_host_rejected() {
        let e = env_of(&[("VELQU_HOST", "bad host with spaces")]);
        let err = resolve_with(cli_default(), &e, &no_file).unwrap_err();
        assert!(matches!(err, ConfigError::InvalidHost { .. }), "{err}");
        let e = env_of(&[("VELQU_HOST", "")]);
        let err = resolve_with(cli_default(), &e, &no_file).unwrap_err();
        assert!(matches!(err, ConfigError::InvalidHost { .. }), "{err}");
    }

    #[test]
    fn log_mode_canonicalized_case_insensitively() {
        let e = env_of(&[("VELQU_LOG", "FULL")]);
        let r = resolve_with(cli_default(), &e, &no_file).unwrap();
        assert_eq!(r.log, "full");
    }

    #[test]
    fn unknown_log_mode_fails_closed_everywhere() {
        // Environment layer: typed rejection, never the old silent
        // fallback to "errors".
        let e = env_of(&[("VELQU_LOG", "verbose")]);
        let err = resolve_with(cli_default(), &e, &no_file).unwrap_err();
        assert!(matches!(err, ConfigError::InvalidLogMode { .. }), "{err}");
        // File layer: same closed set.
        let read = file_map(&[(
            "/app/velqu.config.json",
            r#"{"configVersion":1,"log":"chatty"}"#,
        )]);
        let cli = CliConfig {
            config: Some("/app/velqu.config.json".into()),
            ..cli_default()
        };
        let err = resolve_with(cli, &no_env, &read).unwrap_err();
        assert!(matches!(err, ConfigError::InvalidLogMode { .. }), "{err}");
    }

    #[test]
    fn velqu_config_env_selects_file_cli_wins() {
        let read = file_map(&[
            ("/env selected.json", r#"{"configVersion":1,"maxQueue":77}"#),
            ("/cli selected.json", r#"{"configVersion":1,"maxQueue":88}"#),
        ]);
        let e = env_of(&[("VELQU_CONFIG", "/env selected.json")]);
        let r = resolve_with(cli_default(), &e, &read).unwrap();
        assert_eq!(r.max_queue, 77);
        let cli = CliConfig {
            config: Some("/cli selected.json".into()),
            ..cli_default()
        };
        let r = resolve_with(cli, &e, &read).unwrap();
        assert_eq!(r.max_queue, 88);
    }

    #[test]
    fn velqu_config_env_missing_file_fails_closed() {
        let e = env_of(&[("VELQU_CONFIG", "/does/not/exist.json")]);
        let err = resolve_with(cli_default(), &e, &no_file).unwrap_err();
        assert!(matches!(err, ConfigError::FileRead { .. }), "{err}");
    }

    #[test]
    fn redaction_unrelated_env_never_appears_in_errors() {
        // A capability secret in the environment must never leak into a
        // configuration rejection, whatever the invalid field is.
        let e = env_of(&[
            (
                "VELQU_DATABASE_URL",
                "postgres://bench:s3cret-pw@127.0.0.1:5433/db",
            ),
            ("VELQU_LOG", "verbose"),
        ]);
        let err = resolve_with(cli_default(), &e, &no_file).unwrap_err();
        let rendered = err.to_string();
        assert!(!rendered.contains("s3cret-pw"), "{rendered}");
        assert!(!rendered.contains("postgres://"), "{rendered}");
        assert!(matches!(err, ConfigError::InvalidLogMode { .. }));
    }

    #[test]
    fn redaction_file_read_error_reports_path_not_contents() {
        let read = file_map(&[]); // nothing readable
        let cli = CliConfig {
            config: Some("/srv/velqu/secret-config.json".into()),
            ..cli_default()
        };
        let err = resolve_with(cli, &no_env, &read).unwrap_err();
        let rendered = err.to_string();
        assert!(
            rendered.contains("/srv/velqu/secret-config.json"),
            "{rendered}"
        );
        // IO detail names the failure class, never any file content.
        assert!(!rendered.contains("configVersion"), "{rendered}");
        assert!(!rendered.contains('{'), "{rendered}");
    }

    #[test]
    fn example_config_file_parses() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/config/velqu.config.json");
        let r = resolve(Sources {
            cli: CliConfig {
                config: Some(path.clone()),
                ..cli_default()
            },
            env: &no_env,
            read_file: &|p| std::fs::read_to_string(p),
        })
        .unwrap_or_else(|e| panic!("example config must parse: {e}"));
        // Values match the documented example file.
        assert_eq!(r.max_body_bytes, 2_097_152);
        assert_eq!(r.max_queue, 512);
        assert_eq!(r.port, 3000);
        assert_eq!(r.host, "127.0.0.1");
        assert_eq!(r.log, "errors");
        assert_eq!(r.log_sample, 0);
    }
}
