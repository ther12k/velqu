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

/// BETA-007-B: the closed `VELQU_*` environment namespace. Startup
/// rejects any `VELQU_*` name outside this list — a typo'd knob
/// (`VELQU_MAXQUEUE`) must fail before ready, never be silently
/// ignored. Values of unknown names are never read or echoed (they
/// may be secrets). Tooling-only names are recognized for dev/test
/// convenience but never consumed by the serving runtime.
pub const KNOWN_ENV_VARS: &[&str] = &[
    // runtime configuration (BETA-007-A)
    ENV_CONFIG,
    ENV_HOST,
    ENV_LOG,
    ENV_LOG_SAMPLE,
    ENV_MAX_BODY_BYTES,
    ENV_MAX_QUEUE,
    ENV_PORT,
    // postgres capability (BETA-004-E)
    "VELQU_DATABASE_URL",
    "VELQU_PG_POOL_MAX",
    "VELQU_PG_POOL_CONNECT_TIMEOUT_MS",
    "VELQU_PG_POOL_IDLE_TIMEOUT_MS",
    // build-time (embedded standalone pack)
    "VELQU_STANDALONE_PACK",
    // tooling-only (bench binaries, dev server, test gating)
    "VELQU_ALLOC_PROFILE",
    "VELQU_BENCH_DEBUG",
    "VELQU_PACK",
    "VELQU_PG_LIVE_TEST",
    "VELQU_RUNTIME",
    "VELQU_TEST_TRUST_KEYS",
];

/// Where a resolved field came from — reported per-field in the
/// startup config block so operators can see which layer won.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldSource {
    Cli,
    Env,
    File,
    Default,
}

impl FieldSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            FieldSource::Cli => "cli",
            FieldSource::Env => "env",
            FieldSource::File => "file",
            FieldSource::Default => "default",
        }
    }
}

/// Per-field provenance of the resolved configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldSources {
    pub host: FieldSource,
    pub port: FieldSource,
    pub max_body_bytes: FieldSource,
    pub max_queue: FieldSource,
    pub log: FieldSource,
    pub log_sample: FieldSource,
}

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
    /// BETA-007-B: per-field provenance, rendered in the ready line.
    pub sources: FieldSources,
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
    /// A `VELQU_*` environment name outside the closed namespace. The
    /// variable's VALUE is deliberately never read or echoed.
    UnknownEnvVar { var: String },
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
            ConfigError::UnknownEnvVar { var } => write!(
                f,
                "unknown environment variable {var}: the VELQU_* namespace is closed (see docs/beta/CONFIGURATION.md)"
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
    let mut sources = FieldSources {
        host: FieldSource::Default,
        port: FieldSource::Default,
        max_body_bytes: FieldSource::Default,
        max_queue: FieldSource::Default,
        log: FieldSource::Default,
        log_sample: FieldSource::Default,
    };
    let host = if let Some(h) = &s.cli.host {
        validate_host(h, "cli")?;
        sources.host = FieldSource::Cli;
        h.clone()
    } else if let Some(h) = (s.env)(ENV_HOST) {
        validate_host(&h, "env:VELQU_HOST")?;
        sources.host = FieldSource::Env;
        h
    } else if let Some(h) = file_host {
        validate_host(&h, &file_source(file_path))?;
        sources.host = FieldSource::File;
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
        sources.port = FieldSource::Cli;
        port_src(p as u64, "cli")?
    } else if let Some(v) = (s.env)(ENV_PORT) {
        let n = parse_env_u64(&v, ENV_PORT)?;
        sources.port = FieldSource::Env;
        port_src(n, "env:VELQU_PORT")?
    } else if let Some(v) = (s.env)(ENV_LEGACY_PORT) {
        let n = parse_env_u64(&v, ENV_LEGACY_PORT)?;
        sources.port = FieldSource::Env;
        port_src(n, "env:PORT")?
    } else if let Some(p) = file_port {
        sources.port = FieldSource::File;
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
        sources.max_body_bytes = FieldSource::Env;
        n as usize
    } else if let Some(n) = file_body {
        check_range(
            "maxBodyBytes",
            n,
            1,
            MAX_BODY_BYTES_CEILING as u64,
            &file_source(file_path),
        )?;
        sources.max_body_bytes = FieldSource::File;
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
        sources.max_queue = FieldSource::Env;
        n as usize
    } else if let Some(n) = file_queue {
        check_range(
            "maxQueue",
            n,
            1,
            MAX_QUEUE_CEILING as u64,
            &file_source(file_path),
        )?;
        sources.max_queue = FieldSource::File;
        n as usize
    } else {
        DEFAULT_MAX_QUEUE
    };

    // ---- log: cli > env > file > default (closed set, canonicalized)
    let log: &'static str = if let Some(v) = &s.cli.log {
        sources.log = FieldSource::Cli;
        parse_log(v, "cli")?
    } else if let Some(v) = (s.env)(ENV_LOG) {
        sources.log = FieldSource::Env;
        parse_log(&v, "env:VELQU_LOG")?
    } else if let Some(v) = file_log {
        sources.log = FieldSource::File;
        parse_log(&v, &file_source(file_path))?
    } else {
        DEFAULT_LOG
    };

    // ---- logSample: cli > env > file > default
    let log_sample = if let Some(v) = s.cli.log_sample {
        check_range("logSample", v, 0, LOG_SAMPLE_CEILING, "cli")?;
        sources.log_sample = FieldSource::Cli;
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
        sources.log_sample = FieldSource::Env;
        n
    } else if let Some(n) = file_sample {
        check_range(
            "logSample",
            n,
            0,
            LOG_SAMPLE_CEILING,
            &file_source(file_path),
        )?;
        sources.log_sample = FieldSource::File;
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
        sources,
    })
}

/// BETA-007-B: validate that every `VELQU_*` name in the environment is
/// on the closed allowlist. Unknown names reject startup (a typo'd knob
/// must never be silently ignored); values are never read. Names
/// outside the `VELQU_*` namespace are not the runtime's concern.
pub fn validate_env_namespace(names: &[String]) -> Result<(), ConfigError> {
    for name in names {
        if name.starts_with("VELQU_") && !KNOWN_ENV_VARS.contains(&name.as_str()) {
            return Err(ConfigError::UnknownEnvVar { var: name.clone() });
        }
    }
    Ok(())
}

/// BETA-007-B: the validated, non-secret resolved configuration as the
/// `config` block of the startup `ready` line — field values plus the
/// layer each field came from. Keys are a fixed allowlist (tested).
pub fn startup_config_json(r: &Resolved) -> serde_json::Value {
    serde_json::json!({
        "host": r.host,
        "hostSource": r.sources.host.as_str(),
        "port": r.port,
        "portSource": r.sources.port.as_str(),
        "maxBodyBytes": r.max_body_bytes,
        "maxBodyBytesSource": r.sources.max_body_bytes.as_str(),
        "maxQueue": r.max_queue,
        "maxQueueSource": r.sources.max_queue.as_str(),
        "log": r.log,
        "logSource": r.sources.log.as_str(),
        "logSample": r.log_sample,
        "logSampleSource": r.sources.log_sample.as_str(),
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

    #[test]
    fn unknown_velqu_env_name_rejected_value_never_echoed() {
        // The canonical typo story: VELQU_MAXQUEUE is not VELQU_MAX_QUEUE.
        let names = vec![
            "PATH".to_string(),
            "VELQU_MAXQUEUE".to_string(),
            "VELQU_DATABASE_URL".to_string(),
        ];
        let err = validate_env_namespace(&names).unwrap_err();
        assert!(
            matches!(err, ConfigError::UnknownEnvVar { ref var } if var == "VELQU_MAXQUEUE"),
            "{err}"
        );
        // The rendered rejection names the variable, never a value.
        let rendered = err.to_string();
        assert!(rendered.contains("VELQU_MAXQUEUE"), "{rendered}");
        assert!(!rendered.contains('='));
    }

    #[test]
    fn every_known_env_var_passes_the_namespace_check() {
        let names: Vec<String> = KNOWN_ENV_VARS.iter().map(|s| s.to_string()).collect();
        validate_env_namespace(&names).expect("the documented namespace must validate");
    }

    #[test]
    fn namespace_check_ignores_non_velqu_names() {
        let names = vec!["PATH".to_string(), "PORT".to_string(), "HOME".to_string()];
        validate_env_namespace(&names).expect("non-VELQU_ names are not ours to validate");
        // Case-sensitive: the namespace is VELQU_, not velqu_.
        let lower = vec!["velqu_port".to_string()];
        validate_env_namespace(&lower).expect("namespace prefix is case-sensitive");
    }

    #[test]
    fn resolved_sources_report_the_winning_layer() {
        // All defaults.
        let r = resolve_with(cli_default(), &no_env, &no_file).unwrap();
        assert_eq!(r.sources.host, FieldSource::Default);
        assert_eq!(r.sources.port, FieldSource::Default);
        assert_eq!(r.sources.max_queue, FieldSource::Default);

        // File wins over default; env wins over file; cli wins over env.
        let read = file_map(&[(
            "/app/velqu.config.json",
            r#"{"configVersion":1,"maxQueue":512,"log":"off"}"#,
        )]);
        let cli = CliConfig {
            config: Some("/app/velqu.config.json".into()),
            log_sample: Some(10),
            ..cli_default()
        };
        let e = env_of(&[("VELQU_MAX_QUEUE", "1024")]);
        let r = resolve_with(cli, &e, &read).unwrap();
        assert_eq!(r.sources.max_queue, FieldSource::Env);
        assert_eq!(r.sources.log, FieldSource::File);
        assert_eq!(r.sources.log_sample, FieldSource::Cli);
        assert_eq!(r.max_queue, 1024);
        assert_eq!(r.log, "off");
        assert_eq!(r.log_sample, 10);
    }

    #[test]
    fn startup_config_json_is_an_exact_field_allowlist() {
        let r = resolve_with(cli_default(), &no_env, &no_file).unwrap();
        let v = startup_config_json(&r);
        let obj = v.as_object().expect("config block must be an object");
        let expected: Vec<&str> = vec![
            "host",
            "hostSource",
            "port",
            "portSource",
            "maxBodyBytes",
            "maxBodyBytesSource",
            "maxQueue",
            "maxQueueSource",
            "log",
            "logSource",
            "logSample",
            "logSampleSource",
        ];
        let keys: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
        assert_eq!(keys, expected, "config block keys are a fixed allowlist");
        // Default provenance is visible, and no secret-shaped field exists.
        assert_eq!(obj["portSource"], "default");
        assert_eq!(obj["log"], "errors");
    }
}
