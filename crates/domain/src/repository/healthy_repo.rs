use std::future::Future;

/// Healthy repository - Trait template for repositories
/// For template, define the trait in domain and implement it in infrastructure
pub trait HealthyRepository<'a> {
    type Error: std::fmt::Debug + std::fmt::Display + Send + Sync + 'a;
    fn is_healthy(&self) -> impl Future<Output = Result<(), Self::Error>> + Send + 'a;
}
