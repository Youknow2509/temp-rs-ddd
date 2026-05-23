//! Application use cases — orchestrate domain services to serve a request.
//! These are what the inbound interfaces call into.

use anyhow::Result;

use super::services::Services;

#[derive(Debug, Default)]
pub struct UseCases {
    // TODO: CreateUser, IssueInvoice, SendNotification, ...
}

pub fn build(_services: &Services) -> Result<UseCases> {
    Ok(UseCases::default())
}
