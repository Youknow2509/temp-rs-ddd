mod fields;
mod fmt;

use anyhow::Result;
use tracing::Subscriber;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, Layer, registry::LookupSpan};

use domain::config::LoggerSetting;

pub fn build_env_filter(level: &str) -> EnvFilter {
    EnvFilter::try_new(level).unwrap_or_else(|_| EnvFilter::new("info"))
}

pub fn build_layers<S>(
    cfg: &LoggerSetting,
    extra_fields: &[(&str, &str)],
) -> Result<(
    Vec<Box<dyn Layer<S> + Send + Sync + 'static>>,
    Option<WorkerGuard>,
)>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    if !cfg.stacktrace_level.is_empty() {
        tracing::warn!(
            stacktrace_level = %cfg.stacktrace_level,
            "stacktrace_level is not supported by tracing-subscriber; ignored"
        );
    }

    let mut layers: Vec<Box<dyn Layer<S> + Send + Sync + 'static>> = Vec::new();
    let mut worker_guard: Option<WorkerGuard> = None;

    for output in &cfg.output {
        match output.as_str() {
            crate::constants::LOG_OUTPUT_STDOUT => {
                layers.push(fmt::make_fmt_layer::<S, _>(
                    false,
                    cfg.caller,
                    true,
                    std::io::stdout,
                    extra_fields,
                ));
            }
            crate::constants::LOG_OUTPUT_STDERR => {
                layers.push(fmt::make_fmt_layer::<S, _>(
                    false,
                    cfg.caller,
                    true,
                    std::io::stderr,
                    extra_fields,
                ));
            }
            crate::constants::LOG_OUTPUT_FILE => {
                if !cfg.file.enabled {
                    continue;
                }
                if cfg.file.max_size_mb > 0
                    || cfg.file.max_backups > 0
                    || cfg.file.max_age_days > 0
                    || cfg.file.compress
                {
                    tracing::warn!(
                        max_size_mb = cfg.file.max_size_mb,
                        max_backups = cfg.file.max_backups,
                        max_age_days = cfg.file.max_age_days,
                        compress = cfg.file.compress,
                        "tracing-appender supports only daily/hourly rotation; max_size_mb, max_backups, max_age_days, and compress are ignored"
                    );
                }
                let appender =
                    tracing_appender::rolling::daily(&cfg.file.folder, &cfg.file.filename);
                let (non_blocking, guard) = tracing_appender::non_blocking(appender);
                worker_guard = Some(guard);
                layers.push(fmt::make_fmt_layer::<S, _>(
                    true,
                    cfg.caller,
                    false,
                    non_blocking,
                    extra_fields,
                ));
            }
            other => {
                tracing::warn!(output = other, "unknown log output; skipping");
            }
        }
    }

    Ok((layers, worker_guard))
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::config::{LoggerFileSetting, LoggerSetting};

    fn file_cfg(enabled: bool) -> LoggerFileSetting {
        LoggerFileSetting {
            enabled,
            folder: "/tmp".into(),
            filename: "test.log".into(),
            max_size_mb: 10,
            max_backups: 3,
            max_age_days: 7,
            compress: false,
        }
    }

    fn logger_cfg(output: &str, file_enabled: bool) -> LoggerSetting {
        LoggerSetting {
            level: "info".into(),
            output: vec![output.into()],
            caller: false,
            stacktrace_level: "error".into(),
            file: file_cfg(file_enabled),
        }
    }

    #[test]
    fn env_filter_parses_level() {
        let f = build_env_filter("debug");
        drop(f);
    }

    #[test]
    fn stdout_layer_builds() {
        let cfg = logger_cfg(crate::constants::LOG_OUTPUT_STDOUT, false);
        let result = build_layers::<tracing_subscriber::Registry>(&cfg, &[]);
        assert!(result.is_ok());
        let (layers, guard) = result.unwrap();
        assert!(!layers.is_empty());
        assert!(guard.is_none());
    }

    #[test]
    fn file_output_returns_guard() {
        let cfg = logger_cfg(crate::constants::LOG_OUTPUT_FILE, true);
        let result = build_layers::<tracing_subscriber::Registry>(&cfg, &[]);
        assert!(result.is_ok());
        let (layers, guard) = result.unwrap();
        assert!(!layers.is_empty());
        assert!(guard.is_some());
    }

    #[test]
    fn stderr_output_builds() {
        let cfg = logger_cfg(crate::constants::LOG_OUTPUT_STDERR, false);
        let (layers, guard) = build_layers::<tracing_subscriber::Registry>(&cfg, &[]).unwrap();
        assert_eq!(layers.len(), 1);
        assert!(guard.is_none());
    }

    #[test]
    fn file_output_disabled_flag_produces_no_layer() {
        // output list contains "file" but file.enabled = false → skip silently
        let cfg = logger_cfg(crate::constants::LOG_OUTPUT_FILE, false);
        let (layers, guard) = build_layers::<tracing_subscriber::Registry>(&cfg, &[]).unwrap();
        assert!(layers.is_empty());
        assert!(guard.is_none());
    }

    #[test]
    fn unknown_output_produces_no_layer() {
        let cfg = logger_cfg("syslog", false);
        let (layers, _) = build_layers::<tracing_subscriber::Registry>(&cfg, &[]).unwrap();
        assert!(layers.is_empty());
    }

    #[test]
    fn invalid_level_falls_back_without_panic() {
        let f = build_env_filter("not_a_real_level_!!!");
        drop(f);
    }

    #[test]
    fn multiple_outputs_build_multiple_layers() {
        let cfg = LoggerSetting {
            level: "info".into(),
            output: vec![
                crate::constants::LOG_OUTPUT_STDOUT.into(),
                crate::constants::LOG_OUTPUT_STDERR.into(),
            ],
            caller: false,
            stacktrace_level: String::new(),
            file: file_cfg(false),
        };
        let (layers, guard) = build_layers::<tracing_subscriber::Registry>(&cfg, &[]).unwrap();
        assert_eq!(layers.len(), 2);
        assert!(guard.is_none());
    }
}
