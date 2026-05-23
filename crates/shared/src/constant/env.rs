//! Environment variable names read by the application at startup.

/// Run mode selector: "development" | "staging" | "production".
/// Picks which `config/<mode>.toml` overlays on top of `default.toml`.
pub const ENV_RUN_MODE: &str = "APP_MODE";

/// Explicit override path for the config file (wins over `ENV_RUN_MODE`).
pub const ENV_CONFIG_FILE: &str = "APP_CONFIG_FILE";
