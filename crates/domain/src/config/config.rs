use super::{
    ClientSystemSetting, InterfacesSystemSetting, RepositorySystemSetting, SystemSetting,
    TelemetrySystemSetting,
};
use serde::Deserialize;

/// Config struct for system
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct SystemConfig {
    pub system: SystemSetting,
    pub interfaces: InterfacesSystemSetting,
    pub repository: RepositorySystemSetting,
    pub clients: ClientSystemSetting,
    pub telemetry: TelemetrySystemSetting,
}

impl SystemConfig {
    pub fn telemetry(&self) -> &TelemetrySystemSetting {
        &self.telemetry
    }
}
