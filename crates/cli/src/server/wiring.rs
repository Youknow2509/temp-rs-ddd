//! Phase 2: wire the DDD layers on top of the bootstrap primitives.
//!
//! Direction of dependency (outer -> inner):
//!   Connections ──► Repositories ──► Services ──► UseCases
//!
//! Only `UseCases` (plus the raw config) are handed off to the `run` phase.

pub mod repositories;
pub mod services;
pub mod use_cases;

use anyhow::Result;

use super::bootstrap::Bootstrap;

use self::{repositories::Repositories, services::Services, use_cases::UseCases};

/// Fully-wired application: bootstrap primitives + the DDD layer stack.
#[derive(Debug)]
#[allow(dead_code)] // fields consumed once real adapters land
pub struct Wired {
    pub bootstrap: Bootstrap,
    pub repositories: Repositories,
    pub services: Services,
    pub use_cases: UseCases,
}

pub fn wire(bootstrap: Bootstrap) -> Result<Wired> {
    let repositories = repositories::build(&bootstrap.connections)?;
    let services = services::build(&repositories)?;
    let use_cases = use_cases::build(&services)?;

    Ok(Wired {
        bootstrap,
        repositories,
        services,
        use_cases,
    })
}
