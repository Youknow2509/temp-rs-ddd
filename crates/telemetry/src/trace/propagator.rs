use opentelemetry::{
    global,
    propagation::{TextMapCompositePropagator, TextMapPropagator},
};

pub(super) fn install_propagators(propagation: &[String]) {
    let mut propagators: Vec<Box<dyn TextMapPropagator + Send + Sync>> = Vec::new();

    for p in propagation {
        match p.as_str() {
            crate::constants::PROPAGATOR_TRACECONTEXT => {
                propagators.push(Box::new(
                    opentelemetry_sdk::propagation::TraceContextPropagator::new(),
                ));
            }
            crate::constants::PROPAGATOR_BAGGAGE => {
                propagators.push(Box::new(
                    opentelemetry_sdk::propagation::BaggagePropagator::new(),
                ));
            }
            crate::constants::PROPAGATOR_JAEGER => {
                #[allow(deprecated)]
                propagators.push(Box::new(opentelemetry_jaeger_propagator::Propagator::new()));
            }
            other => {
                tracing::warn!(propagator = other, "unknown propagator; skipping");
            }
        }
    }

    if !propagators.is_empty() {
        global::set_text_map_propagator(TextMapCompositePropagator::new(propagators));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_list_is_noop() {
        install_propagators(&[]);
    }

    #[test]
    fn known_propagators_install_without_panic() {
        install_propagators(&[
            crate::constants::PROPAGATOR_TRACECONTEXT.into(),
            crate::constants::PROPAGATOR_BAGGAGE.into(),
            crate::constants::PROPAGATOR_JAEGER.into(),
        ]);
    }

    #[test]
    fn unknown_propagator_is_skipped_without_panic() {
        install_propagators(&["zipkin".into()]);
    }
}
