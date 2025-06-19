use crate::{endpoint::Endpoint, verifier::Verifier};
use async_trait::async_trait;

mod levenshtein;
mod homoglyph;
mod pre_in_suffix;

#[async_trait]
pub trait Mutator {
    async fn mutate<T: Verifier + 'static>(&self, source: Endpoint, verifier: T) -> Vec<Endpoint>;
}
