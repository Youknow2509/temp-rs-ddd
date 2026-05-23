use super::{
    ClientSystemSetting, InterfacesSystemSetting, RepositorySystemSetting, SystemSetting,
    TelemetrySystemSetting,
};
use serde::Deserialize;

/// Config struct for system
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct SystemConfig {
    system: SystemSetting,
    interfaces: InterfacesSystemSetting,
    repository: RepositorySystemSetting,
    clients: ClientSystemSetting,
    telemetry: TelemetrySystemSetting,
}

impl SystemConfig {
    pub fn telemetry(&self) -> &TelemetrySystemSetting {
        &self.telemetry
    }
}
