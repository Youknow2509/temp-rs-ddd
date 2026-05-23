//! Filesystem path constants.
//!
//! Relative paths are resolved against the process current working directory
//! at runtime. Compile-time `include_str!` paths must stay inline at the
//! call site (they resolve relative to the source file, not via constants).

// ===
// Config files
// ===

pub const CONFIG_DIR: &str = "config";

// Full paths (with extension) — for direct `std::fs` reads.
pub const CONFIG_FILE_DEFAULT: &str = "config/default.toml";
pub const CONFIG_FILE_DEVELOPMENT: &str = "config/development.toml";
pub const CONFIG_FILE_PRODUCTION: &str = "config/production.toml";

// Base names (no extension) — for the `config` crate `File::with_name`
// which auto-detects the format. Mode-specific paths are computed at
// runtime as `format!("{CONFIG_DIR}/{run_mode}")`.
pub const CONFIG_BASE_DEFAULT: &str = "config/default";
pub const CONFIG_BASE_LOCAL: &str = "config/local";

// Run mode values (must match the basename of `config/<mode>.toml`).
pub const RUN_MODE_DEVELOPMENT: &str = "development";
pub const RUN_MODE_PRODUCTION: &str = "production";

// ===
// Certs / secrets directories
// ===

pub const CERTS_DIR: &str = "certs";
pub const SECRETS_DIR: &str = "secrets";
