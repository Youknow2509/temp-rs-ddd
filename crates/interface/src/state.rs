use domain::config::SystemConfig;
use infrastructure::connection::Connections;

/// Shared application state. Wrap in `Arc<AppState>` for sharing across interfaces.
/// Arc<AppState> clone = 1 atomic bump — cheaper than Arc-per-field.
pub struct AppState {
    pub connections: Connections,
    pub config: SystemConfig,
}

impl AppState {
    pub fn new(connections: Connections, config: SystemConfig) -> Self {
        Self {
            connections,
            config,
        }
    }
}
