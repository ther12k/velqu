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
/// BETA-007-D: selects the active profile from the config file's
/// `profiles` map; wins over the file's own `activeProfile`.
pub const ENV_PROFILE: &str = "VELQU_PROFILE";

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
    ENV_PROFILE,
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
    /// BETA-007-D: value came from the active profile block.
    Profile,
    File,
    Default,
}

impl FieldSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            FieldSource::Cli => "cli",
            FieldSource::Env => "env",
            FieldSource::Profile => "profile",
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
    /// BETA-007-D: the profile applied unless VELQU_PROFILE selects one.
    #[serde(rename = "activeProfile")]
    active_profile: Option<String>,
    /// BETA-007-D: named override blocks; each block accepts the same
    /// optional fields as the file itself (minus versioning/nesting).
    profiles: Option<std::collections::BTreeMap<String, ProfileBlock>>,
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

/// BETA-007-D: one profile block. Unknown fields inside a block are
/// rejected exactly like at the file level; nesting profiles inside a
/// profile is structurally impossible (unknown field).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProfileBlock {
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
    /// BETA-007-D: the applied profile, when one is active.
    pub active_profile: Option<String>,
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
    /// The selected profile is not declared in the config file.
    UnknownProfile { name: String, declared: Vec<String> },
    /// A profile name violates the closed shape (1..=32 of a-z 0-9 '-').
    InvalidProfileName { name: String },
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
            ConfigError::UnknownProfile { name, declared } => {
                if declared.is_empty() {
                    write!(
                        f,
                        "active profile '{name}' is not declared: the config file declares no profiles"
                    )
                } else {
                    write!(
                        f,
                        "active profile '{name}' is not declared (declared: {})",
                        declared.join(", ")
                    )
                }
            }
            ConfigError::InvalidProfileName { name } => write!(
                f,
                "profile name '{name}' is invalid (expected 1..=32 characters of a-z, 0-9, '-')"
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
    let file_path = file.as_ref().map(|(p, _)| p.as_str());
    let base = file.as_ref().map(|(_, f)| f);

    // ---- layer: profile (BETA-007-D). Selected by VELQU_PROFILE, else
    // the file's activeProfile; overlays the file's base fields. Env
    // and CLI still win above it. Declared names and the selected name
    // are shape-validated; an undeclared active profile rejects startup.
    if let Some(f) = base {
        if let Some(profiles) = &f.profiles {
            for name in profiles.keys() {
                if !valid_profile_name(name) {
                    return Err(ConfigError::InvalidProfileName { name: name.clone() });
                }
            }
        }
    }
    let env_profile = (s.env)(ENV_PROFILE)
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty());
    if let Some(name) = &env_profile {
        if !valid_profile_name(name) {
            return Err(ConfigError::InvalidProfileName { name: name.clone() });
        }
    }
    let active_profile: Option<String> = match env_profile {
        Some(p) => Some(p),
        None => base.and_then(|f| f.active_profile.clone()),
    };
    let profile: Option<ProfileBlock> = match &active_profile {
        None => None,
        Some(name) => {
            let declared = base.and_then(|f| f.profiles.as_ref());
            match declared.and_then(|m| m.get(name)) {
                Some(b) => Some(b.clone()),
                None => {
                    let mut names: Vec<String> = declared
                        .map(|m| m.keys().cloned().collect())
                        .unwrap_or_default();
                    names.sort();
                    return Err(ConfigError::UnknownProfile {
                        name: name.clone(),
                        declared: names,
                    });
                }
            }
        }
    };

    // File-layer value + provenance per field (the profile overlays the
    // file's base fields).
    macro_rules! overlay {
        ($profile_field:ident, $base_field:ident) => {
            match (
                profile.as_ref().and_then(|b| b.$profile_field.clone()),
                base.and_then(|f| f.$base_field.clone()),
            ) {
                (Some(v), _) => Some((v, FieldSource::Profile)),
                (None, Some(v)) => Some((v, FieldSource::File)),
                (None, None) => None,
            }
        };
    }
    let fl_host = overlay!(host, host);
    let fl_port: Option<(u16, FieldSource)> = match (
        profile.as_ref().and_then(|b| b.port),
        base.and_then(|f| f.port),
    ) {
        (Some(v), _) => Some((v, FieldSource::Profile)),
        (None, Some(v)) => Some((v, FieldSource::File)),
        (None, None) => None,
    };
    let fl_body: Option<(u64, FieldSource)> = overlay!(max_body_bytes, max_body_bytes);
    let fl_queue: Option<(u64, FieldSource)> = overlay!(max_queue, max_queue);
    let fl_log = overlay!(log, log);
    let fl_sample: Option<(u64, FieldSource)> = match (
        profile.as_ref().and_then(|b| b.log_sample),
        base.and_then(|f| f.log_sample),
    ) {
        (Some(v), _) => Some((v, FieldSource::Profile)),
        (None, Some(v)) => Some((v, FieldSource::File)),
        (None, None) => None,
    };

    // ---- host: cli > env > profile > file > default
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
    } else if let Some((h, layer)) = fl_host {
        validate_host(
            &h,
            &layer_source(layer, file_path, active_profile.as_deref()),
        )?;
        sources.host = layer;
        h
    } else {
        DEFAULT_HOST.to_string()
    };

    // ---- port: cli > VELQU_PORT > PORT > profile > file > default
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
    } else if let Some((p, layer)) = fl_port {
        sources.port = layer;
        port_src(
            p as u64,
            &layer_source(layer, file_path, active_profile.as_deref()),
        )?
    } else {
        DEFAULT_PORT
    };

    // ---- maxBodyBytes: env > profile > file > default
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
    } else if let Some((n, layer)) = fl_body {
        check_range(
            "maxBodyBytes",
            n,
            1,
            MAX_BODY_BYTES_CEILING as u64,
            &layer_source(layer, file_path, active_profile.as_deref()),
        )?;
        sources.max_body_bytes = layer;
        n as usize
    } else {
        DEFAULT_MAX_BODY_BYTES
    };

    // ---- maxQueue: env > profile > file > default
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
    } else if let Some((n, layer)) = fl_queue {
        check_range(
            "maxQueue",
            n,
            1,
            MAX_QUEUE_CEILING as u64,
            &layer_source(layer, file_path, active_profile.as_deref()),
        )?;
        sources.max_queue = layer;
        n as usize
    } else {
        DEFAULT_MAX_QUEUE
    };

    // ---- log: cli > env > profile > file > default (closed set, canonicalized)
    let log: &'static str = if let Some(v) = &s.cli.log {
        sources.log = FieldSource::Cli;
        parse_log(v, "cli")?
    } else if let Some(v) = (s.env)(ENV_LOG) {
        sources.log = FieldSource::Env;
        parse_log(&v, "env:VELQU_LOG")?
    } else if let Some((v, layer)) = fl_log {
        sources.log = layer;
        parse_log(
            &v,
            &layer_source(layer, file_path, active_profile.as_deref()),
        )?
    } else {
        DEFAULT_LOG
    };

    // ---- logSample: cli > env > profile > file > default
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
    } else if let Some((n, layer)) = fl_sample {
        check_range(
            "logSample",
            n,
            0,
            LOG_SAMPLE_CEILING,
            &layer_source(layer, file_path, active_profile.as_deref()),
        )?;
        sources.log_sample = layer;
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
        active_profile,
        sources,
    })
}

/// BETA-007-D: closed profile-name shape — 1..=32 characters of
/// a-z, 0-9, '-'.
fn valid_profile_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 32
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Error-label for a file/profile-layer value.
fn layer_source(layer: FieldSource, file_path: Option<&str>, profile: Option<&str>) -> String {
    match layer {
        FieldSource::Profile => format!("profile:{}", profile.unwrap_or("?")),
        _ => file_source(file_path),
    }
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
        "activeProfile": r.active_profile,
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

/// BETA-007-C: typed wrapper for secret configuration values —
/// capability credentials such as the database URL. `Debug` and
/// `Display` always render `[redacted]`, so an accidental log, error,
/// or inspect of a holder can never disclose the value. The only read
/// path is the explicit, grep-auditable [`SecretString::expose`].
///
/// Memory zeroization on drop is deliberately NOT claimed: the
/// guarantee here is redaction across all formatting and serialization
/// paths, not memory hygiene. No `Clone`, no `PartialEq`: secret
/// material is neither duplicated nor compared.
pub struct SecretString {
    inner: String,
}

impl SecretString {
    pub fn new(value: String) -> Self {
        SecretString { inner: value }
    }

    /// The only read path — named for audit greppability.
    pub fn expose(&self) -> &str {
        &self.inner
    }

    /// Read an environment variable into a wrapped secret through an
    /// injected lookup (testable without touching the process
    /// environment). The raw value never exists outside the wrapper.
    pub fn from_env(var: &str, lookup: &dyn Fn(&str) -> Option<String>) -> Option<SecretString> {
        lookup(var).map(SecretString::new)
    }
}

impl std::fmt::Debug for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[redacted]")
    }
}

impl std::fmt::Display for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[redacted]")
    }
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
            "activeProfile",
        ];
        let keys: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
        assert_eq!(keys, expected, "config block keys are a fixed allowlist");
        // Default provenance is visible, and no secret-shaped field exists.
        assert_eq!(obj["portSource"], "default");
        assert_eq!(obj["log"], "errors");
    }

    #[test]
    fn secret_debug_and_display_render_redacted() {
        let s = SecretString::new("postgres://bench:s3cret-pw@127.0.0.1:5433/db".to_string());
        assert_eq!(format!("{}", &s), "[redacted]");
        assert_eq!(format!("{:?}", &s), "[redacted]");
        // Holders that Debug-print their contents cannot disclose it.
        let holder = vec![&s];
        let rendered = format!("{:?}", holder);
        assert!(!rendered.contains("s3cret-pw"), "{rendered}");
        assert!(rendered.contains("[redacted]"), "{rendered}");
    }

    #[test]
    fn secret_expose_is_the_only_read_path() {
        let s = SecretString::new("hunter2".to_string());
        assert_eq!(s.expose(), "hunter2");
    }

    #[test]
    fn secret_from_env_wraps_without_disclosure() {
        let lookup = |k: &str| {
            if k == "VELQU_DATABASE_URL" {
                Some("postgres://u:pw@h/db".to_string())
            } else {
                None
            }
        };
        let s = SecretString::from_env("VELQU_DATABASE_URL", &lookup).expect("present");
        assert_eq!(s.expose(), "postgres://u:pw@h/db");
        assert!(!format!("{:?}", s).contains("pw"));
        assert!(SecretString::from_env("VELQU_MISSING", &lookup).is_none());
    }

    #[test]
    fn profile_overrides_file_but_not_env() {
        let read = file_map(&[(
            "/app/velqu.config.json",
            r#"{"configVersion":1,"activeProfile":"production","maxQueue":256,
                "profiles":{"production":{"maxQueue":512,"log":"full"}}}"#,
        )]);
        let cli = CliConfig {
            config: Some("/app/velqu.config.json".into()),
            ..cli_default()
        };
        let r = resolve_with(cli.clone(), &no_env, &read).unwrap();
        assert_eq!(r.active_profile.as_deref(), Some("production"));
        assert_eq!(r.max_queue, 512);
        assert_eq!(r.sources.max_queue, FieldSource::Profile);
        assert_eq!(r.log, "full");
        assert_eq!(r.sources.log, FieldSource::Profile);
        // Base file fields still apply where the profile is silent.
        assert_eq!(r.sources.host, FieldSource::Default);

        // Env still wins above the profile layer.
        let e = env_of(&[("VELQU_MAX_QUEUE", "1024")]);
        let r = resolve_with(cli, &e, &read).unwrap();
        assert_eq!(r.max_queue, 1024);
        assert_eq!(r.sources.max_queue, FieldSource::Env);
    }

    #[test]
    fn velqu_profile_env_selects_and_beats_file_selection() {
        let read = file_map(&[(
            "/app/velqu.config.json",
            r#"{"configVersion":1,"activeProfile":"dev",
                "profiles":{"dev":{"log":"full"},"production":{"log":"errors","maxQueue":4096}}}"#,
        )]);
        let cli = CliConfig {
            config: Some("/app/velqu.config.json".into()),
            ..cli_default()
        };
        let r = resolve_with(cli.clone(), &no_env, &read).unwrap();
        assert_eq!(r.active_profile.as_deref(), Some("dev"));
        assert_eq!(r.log, "full");

        let e = env_of(&[("VELQU_PROFILE", "production")]);
        let r = resolve_with(cli, &e, &read).unwrap();
        assert_eq!(r.active_profile.as_deref(), Some("production"));
        assert_eq!(r.log, "errors");
        assert_eq!(r.max_queue, 4096);
        assert_eq!(r.sources.max_queue, FieldSource::Profile);
    }

    #[test]
    fn unknown_active_profile_fails_closed() {
        let read = file_map(&[(
            "/app/velqu.config.json",
            r#"{"configVersion":1,"activeProfile":"staging","profiles":{"production":{}}}"#,
        )]);
        let cli = CliConfig {
            config: Some("/app/velqu.config.json".into()),
            ..cli_default()
        };
        let err = resolve_with(cli, &no_env, &read).unwrap_err();
        let rendered = err.to_string();
        assert!(
            rendered.contains("staging") && rendered.contains("production"),
            "{rendered}"
        );

        // Same when the undeclared name comes from VELQU_PROFILE.
        let read2 = file_map(&[(
            "/app/v2.json",
            r#"{"configVersion":1,"profiles":{"production":{}}}"#,
        )]);
        let cli = CliConfig {
            config: Some("/app/v2.json".into()),
            ..cli_default()
        };
        let e = env_of(&[("VELQU_PROFILE", "staging")]);
        let err = resolve_with(cli, &e, &read2).unwrap_err();
        assert!(matches!(err, ConfigError::UnknownProfile { .. }), "{err}");

        // Active profile with no profiles map at all.
        let read3 = file_map(&[("/app/v3.json", r#"{"configVersion":1,"activeProfile":"x"}"#)]);
        let cli = CliConfig {
            config: Some("/app/v3.json".into()),
            ..cli_default()
        };
        let err = resolve_with(cli, &no_env, &read3).unwrap_err();
        assert!(err.to_string().contains("declares no profiles"), "{err}");
    }

    #[test]
    fn profile_names_are_validated() {
        // Declared name violating the shape.
        let read = file_map(&[(
            "/app/velqu.config.json",
            r#"{"configVersion":1,"profiles":{"PROD!":{}}}"#,
        )]);
        let cli = CliConfig {
            config: Some("/app/velqu.config.json".into()),
            ..cli_default()
        };
        let err = resolve_with(cli, &no_env, &read).unwrap_err();
        assert!(
            matches!(err, ConfigError::InvalidProfileName { .. }),
            "{err}"
        );

        // Selected name from env violating the shape.
        let read2 = file_map(&[(
            "/app/v2.json",
            r#"{"configVersion":1,"profiles":{"dev":{}}}"#,
        )]);
        let cli = CliConfig {
            config: Some("/app/v2.json".into()),
            ..cli_default()
        };
        let e = env_of(&[("VELQU_PROFILE", "../etc")]);
        let err = resolve_with(cli, &e, &read2).unwrap_err();
        assert!(
            matches!(err, ConfigError::InvalidProfileName { .. }),
            "{err}"
        );
    }

    #[test]
    fn profile_blocks_reject_unknown_fields_and_nesting() {
        // Unknown field inside a block.
        let read = file_map(&[(
            "/app/velqu.config.json",
            r#"{"configVersion":1,"activeProfile":"p","profiles":{"p":{"maxQeue":1}}}"#,
        )]);
        let cli = CliConfig {
            config: Some("/app/velqu.config.json".into()),
            ..cli_default()
        };
        let err = resolve_with(cli, &no_env, &read).unwrap_err();
        assert!(matches!(err, ConfigError::FileSchema { .. }), "{err}");

        // Nesting profiles inside a profile is structurally rejected.
        let read2 = file_map(&[(
            "/app/v2.json",
            r#"{"configVersion":1,"activeProfile":"p","profiles":{"p":{"profiles":{}}}}"#,
        )]);
        let cli = CliConfig {
            config: Some("/app/v2.json".into()),
            ..cli_default()
        };
        let err = resolve_with(cli, &no_env, &read2).unwrap_err();
        assert!(matches!(err, ConfigError::FileSchema { .. }), "{err}");
    }

    #[test]
    fn active_profile_reported_in_startup_config() {
        let read = file_map(&[(
            "/app/velqu.config.json",
            r#"{"configVersion":1,"activeProfile":"production","profiles":{"production":{"log":"full"}}}"#,
        )]);
        let cli = CliConfig {
            config: Some("/app/velqu.config.json".into()),
            ..cli_default()
        };
        let r = resolve_with(cli, &no_env, &read).unwrap();
        let v = startup_config_json(&r);
        assert_eq!(v["activeProfile"], "production");
        assert_eq!(v["logSource"], "profile");

        // No profile: the key is present but null.
        let r = resolve_with(cli_default(), &no_env, &no_file).unwrap();
        let v = startup_config_json(&r);
        assert!(v["activeProfile"].is_null());
    }
}
