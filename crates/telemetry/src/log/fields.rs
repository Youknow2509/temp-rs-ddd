use serde_json::{Map, Value};
use std::fmt;
use tracing::{Event, Subscriber};
use tracing_subscriber::{
    fmt::{FmtContext, FormatEvent, FormatFields, format::Writer},
    registry::LookupSpan,
};

pub(super) struct JsonWithStaticFields {
    extra: Map<String, Value>,
    caller: bool,
}

impl JsonWithStaticFields {
    pub(super) fn new(fields: &[(&str, &str)], caller: bool) -> Self {
        Self {
            extra: fields
                .iter()
                .map(|(k, v)| ((*k).to_owned(), Value::String((*v).to_owned())))
                .collect(),
            caller,
        }
    }
}

impl<S, N> FormatEvent<S, N> for JsonWithStaticFields
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let mut map = Map::new();

        use crate::constants;

        map.insert(
            constants::JSON_FIELD_TIMESTAMP.into(),
            Value::String(chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)),
        );

        map.insert(
            constants::JSON_FIELD_LEVEL.into(),
            Value::String(event.metadata().level().to_string()),
        );
        map.insert(
            constants::JSON_FIELD_TARGET.into(),
            Value::String(event.metadata().target().to_owned()),
        );

        if self.caller {
            if let Some(file) = event.metadata().file() {
                map.insert(
                    constants::JSON_FIELD_FILE.into(),
                    Value::String(file.to_owned()),
                );
            }
            if let Some(line) = event.metadata().line() {
                map.insert(constants::JSON_FIELD_LINE.into(), serde_json::json!(line));
            }
        }

        let mut v = FieldVisitor::default();
        event.record(&mut v);
        map.extend(v.0);

        if let Some(span) = ctx.lookup_current() {
            map.insert(
                constants::JSON_FIELD_SPAN.into(),
                Value::String(span.name().to_owned()),
            );
        }

        // Static extra fields — override event fields if key conflicts.
        map.extend(self.extra.clone());

        let json = serde_json::to_string(&map).map_err(|_| fmt::Error)?;
        write!(writer, "{}", json)?;
        writeln!(writer)
    }
}

#[derive(Default)]
pub(super) struct FieldVisitor(pub Map<String, Value>);

impl tracing::field::Visit for FieldVisitor {
    fn record_str(&mut self, f: &tracing::field::Field, v: &str) {
        self.0
            .insert(f.name().to_owned(), Value::String(v.to_owned()));
    }
    fn record_debug(&mut self, f: &tracing::field::Field, v: &dyn fmt::Debug) {
        self.0
            .insert(f.name().to_owned(), Value::String(format!("{v:?}")));
    }
    fn record_bool(&mut self, f: &tracing::field::Field, v: bool) {
        self.0.insert(f.name().to_owned(), Value::Bool(v));
    }
    fn record_i64(&mut self, f: &tracing::field::Field, v: i64) {
        self.0.insert(f.name().to_owned(), serde_json::json!(v));
    }
    fn record_u64(&mut self, f: &tracing::field::Field, v: u64) {
        self.0.insert(f.name().to_owned(), serde_json::json!(v));
    }
    fn record_f64(&mut self, f: &tracing::field::Field, v: f64) {
        self.0.insert(f.name().to_owned(), serde_json::json!(v));
    }
    fn record_error(&mut self, f: &tracing::field::Field, v: &(dyn std::error::Error + 'static)) {
        self.0
            .insert(f.name().to_owned(), Value::String(v.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_with_static_fields_constructs() {
        let formatter = JsonWithStaticFields::new(&[("env", "test"), ("version", "1.0")], false);
        assert_eq!(formatter.extra.len(), 2);
        assert_eq!(formatter.extra["env"], Value::String("test".into()));
        assert_eq!(formatter.extra["version"], Value::String("1.0".into()));
    }

    #[test]
    fn json_with_empty_fields_constructs() {
        let formatter = JsonWithStaticFields::new(&[], false);
        assert!(formatter.extra.is_empty());
    }
}
