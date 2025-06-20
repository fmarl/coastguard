use crate::endpoint::Endpoint;
use async_trait::async_trait;

pub(crate) mod dns;

#[async_trait]
pub trait Verifier: Send + Sync {
    async fn verify(&self, target: Endpoint) -> bool;
}