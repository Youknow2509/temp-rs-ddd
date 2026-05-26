use crate::connection::postgres_conn::PgPool;
use anyhow::Context;
use domain::repository::healthy_repo::HealthyRepository;

#[derive(Debug)]
pub struct PgHealthyRepo<'a> {
    pool: &'a PgPool,
}

impl<'a> PgHealthyRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
}

impl<'a> HealthyRepository<'a> for PgHealthyRepo<'a> {
    type Error = anyhow::Error;

    fn is_healthy(&self) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send + 'a {
        let pool = self.pool.clone();
        async move {
            let client = pool.get().await.context("acquiring connection").inspect_err(|e| {
                tracing::error!(error = %e, "postgres health check: failed to acquire connection");
            })?;
            client
                .simple_query("SELECT 1")
                .await
                .context("executing health check query")
                .inspect_err(|e| {
                    tracing::error!(error = %e, "postgres health check: query failed");
                })?;
            Ok(())
        }
    }
}
