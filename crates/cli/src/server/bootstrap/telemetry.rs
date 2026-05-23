use anyhow::Result;
use domain::config::SystemConfig;

pub use telemetry::TelemetryGuard;

pub fn init(config: &SystemConfig) -> Result<TelemetryGuard> {
    telemetry::init(
        config.telemetry(),
        &[
            ("service", env!("CARGO_PKG_NAME")),
            ("version", env!("CARGO_PKG_VERSION")),
        ],
    )
}
