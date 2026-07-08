mod recorder;

use anyhow::Result;
use domain::config::MetricsSetting;
use tokio::task::JoinHandle;

pub fn build(cfg: &MetricsSetting) -> Result<Option<JoinHandle<()>>> {
    recorder::install(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::config::MetricsSetting;
    use std::net::IpAddr;
    use std::str::FromStr;

    fn disabled_cfg() -> MetricsSetting {
        MetricsSetting {
            enabled: false,
            path: "/metrics".into(),
            port: 19091,
            host: IpAddr::from_str("127.0.0.1").unwrap(),
            namespace: "test".into(),
            collect_interval_secs: 15,
        }
    }

    fn enabled_cfg(port: u16) -> MetricsSetting {
        MetricsSetting {
            enabled: true,
            path: "/metrics".into(),
            port,
            host: IpAddr::from_str("127.0.0.1").unwrap(),
            namespace: "test_ns".into(),
            collect_interval_secs: 15,
        }
    }

    #[test]
    fn disabled_returns_none() {
        let result = build(&disabled_cfg());
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn enabled_returns_handle() {
        let result = build(&enabled_cfg(19091));
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }
}
