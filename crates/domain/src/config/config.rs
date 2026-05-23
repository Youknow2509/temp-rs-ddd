use super::{ClientSystemSetting, InterfacesSystemSetting, RepositorySystemSetting, SystemSetting};
use serde::Deserialize;

/// Config struct for system
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct SystemConfig {
    system: SystemSetting,
    interfaces: InterfacesSystemSetting,
    repository: RepositorySystemSetting,
    clients: ClientSystemSetting,
}
