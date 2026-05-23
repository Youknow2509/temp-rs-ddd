//! Setting common for setting.

pub mod setting;
pub mod setting_client;
pub mod setting_interface;
pub mod setting_mq;
pub mod setting_repository;
pub mod setting_telemetry;
pub mod system_config;

// re-exporting
pub use self::{
    setting::*, setting_client::*, setting_interface::*, setting_mq::*, setting_repository::*,
    setting_telemetry::*, system_config::*,
};
