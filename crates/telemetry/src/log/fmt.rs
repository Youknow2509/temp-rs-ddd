use tracing::Subscriber;
use tracing_subscriber::{
    EnvFilter, Layer,
    fmt::{self, MakeWriter},
    registry::LookupSpan,
};

use super::fields::JsonWithStaticFields;

pub(super) fn make_fmt_layer<S, W>(
    use_json: bool,
    caller: bool,
    ansi: bool,
    writer: W,
    extra_fields: &[(&str, &str)],
    level_filter: EnvFilter,
) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    W: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    if use_json && !extra_fields.is_empty() {
        fmt::layer()
            .event_format(JsonWithStaticFields::new(extra_fields, caller))
            .with_ansi(false)
            .with_writer(writer)
            .with_filter(level_filter)
            .boxed()
    } else if use_json {
        fmt::layer()
            .json()
            .with_ansi(false)
            .with_writer(writer)
            .with_file(caller)
            .with_line_number(caller)
            .with_target(true)
            .with_filter(level_filter)
            .boxed()
    } else {
        fmt::layer()
            .pretty()
            .with_ansi(ansi)
            .with_writer(writer)
            .with_file(caller)
            .with_line_number(caller)
            .with_target(true)
            .with_filter(level_filter)
            .boxed()
    }
}
