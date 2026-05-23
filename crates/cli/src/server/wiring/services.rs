//! Domain services — pure logic that depends only on repository ports.

use anyhow::Result;

use super::repositories::Repositories;

#[derive(Debug, Default)]
pub struct Services {
    // TODO: UserService, BillingService, NotificationService, ...
}

pub fn build(_repositories: &Repositories) -> Result<Services> {
    Ok(Services::default())
}
