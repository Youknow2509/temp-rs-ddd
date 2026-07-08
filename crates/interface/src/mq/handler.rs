pub(super) mod topic01_handler;
pub(super) mod topic02_handler;
use crate::mq::message::MQMessage;
use anyhow::Result;
use infrastructure::state::AppState;
use once_cell::sync::Lazy;
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

/// Handler function type: takes an `MQMessage` and returns a boxed future
/// that resolves to `()`.
pub type HandlerFn = Arc<
    dyn Fn(Arc<AppState>, MQMessage) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>
        + Send
        + Sync,
>;

static HANDLER_MAP: Lazy<HashMap<&'static str, HandlerFn>> = Lazy::new(|| {
    let mut m: HashMap<&'static str, HandlerFn> = HashMap::new();
    m.insert(
        "topic01_handler",
        Arc::new(|app_state: Arc<AppState>, msg: MQMessage| {
            Box::pin(topic01_handler::handle(app_state, msg))
        }),
    );
    m.insert(
        "topic02_handler",
        Arc::new(|app_state: Arc<AppState>, msg: MQMessage| {
            Box::pin(topic02_handler::handle(app_state, msg))
        }),
    );
    m
});

/// Get a handler by name. Returns `None` if the handler is unknown.
pub fn get_handler(name: &str) -> Option<HandlerFn> {
    HANDLER_MAP.get(name).cloned()
}

/// Convenience: dispatch message to a named handler. Logs a warning if
/// handler is not found.
pub async fn dispatch(name: &str, app_state: Arc<AppState>, msg: MQMessage) -> Result<()> {
    if let Some(h) = get_handler(name) {
        (h)(app_state, msg).await
    } else {
        tracing::warn!("No handler registered for '{}', dropping message", name);
        Ok(())
    }
}
