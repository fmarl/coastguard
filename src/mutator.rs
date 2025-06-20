use crate::{endpoint::Endpoint, verifier::Verifier};

mod levenshtein;
mod homoglyph;
mod pre_in_suffix;

pub struct MutationChainNode<'a> {
    mutator: fn(source: Endpoint) -> Vec<Endpoint>,
    next: Option<Box<&'a MutationChainNode<'a>>>
}

pub struct MutationChain<'a, V: Verifier> {
    verifier: V,
    first: MutationChainNode<'a>,
    endpoints: Vec<Endpoint>
}

impl<'a> MutationChainNode<'a> {
    pub fn new(mutator: fn(source: Endpoint) -> Vec<Endpoint>) -> Self {
        MutationChainNode { mutator: mutator, next: None }
    }

    pub fn add(&self, node: &MutationChainNode) {
        self.next = Some(Box::new(node))
    }

    pub fn next(&self) -> Option<Box<MutationChainNode>> {
        self.next
    }
}

impl<V: Verifier> MutationChain<V> {
    pub fn new(verifier: V, first: &MutationChainNode) -> Self {
        MutationChain { verifier: verifier, first: first, endpoints: Vec::new() }
    }
}