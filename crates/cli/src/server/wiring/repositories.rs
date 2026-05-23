//! Concrete repository implementations bound to the connection pools.
//!
//! These are the outer adapters that implement the repository *ports*
//! defined by the `domain` crate.

use anyhow::Result;

use crate::server::bootstrap::connections::Connections;

#[derive(Debug, Default)]
pub struct Repositories {
    // TODO: pg_user_repo, redis_session_repo, scylla_event_repo, s3_blob_repo, ...
}

pub fn build(_connections: &Connections) -> Result<Repositories> {
    Ok(Repositories::default())
}
