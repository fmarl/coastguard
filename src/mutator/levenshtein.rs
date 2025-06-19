use async_trait::async_trait;

use crate::{endpoint::Endpoint, mutator::Mutator, verifier::Verifier};

pub struct LevenshteinMutator {
    len: u16
}

impl LevenshteinMutator {
    pub fn new(len: u16) -> Self {
        LevenshteinMutator { len }
    }
}

#[async_trait]
impl Mutator for LevenshteinMutator {
    async fn mutate<T: Verifier + 'static>(&self, source: Endpoint, verifier: T) -> Vec<Endpoint> {
        todo!()
    }
}