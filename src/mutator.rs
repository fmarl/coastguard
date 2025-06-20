use futures::future::join_all;

use crate::{
    endpoint::{self, Endpoint},
    verifier::Verifier,
};

pub(crate) mod homoglyph;
pub(crate) mod levenshtein;
pub(crate) mod pre_in_suffix;
pub(crate) mod tlds;

pub struct MutationChainNode {
    pub mutator: fn(Endpoint) -> Vec<Endpoint>,
    pub next: Option<Box<MutationChainNode>>,
}

impl MutationChainNode {
    pub fn new(mutator: fn(Endpoint) -> Vec<Endpoint>) -> Self {
        MutationChainNode {
            mutator,
            next: None,
        }
    }

    pub fn next(&self) -> Option<&MutationChainNode> {
        self.next.as_deref()
    }

    pub fn next_mut(&mut self) -> Option<&mut MutationChainNode> {
        self.next.as_deref_mut()
    }
}

pub struct MutationChain<V: Verifier> {
    pub verifier: V,
    pub first: MutationChainNode,
    pub endpoints: Vec<Endpoint>,
}

impl<V: Verifier> MutationChain<V> {
    pub fn new(verifier: V, first: MutationChainNode) -> Self {
        MutationChain {
            verifier,
            first,
            endpoints: Vec::new(),
        }
    }

    pub fn add(&mut self, node: MutationChainNode) {
        let mut current = &mut self.first;

        loop {
            match current.next {
                Some(ref mut next) => {
                    current = next;
                }
                None => {
                    current.next = Some(Box::new(node));
                    break;
                }
            }
        }
    }

    pub async fn run(&self, endpoint: &Endpoint) -> Vec<Endpoint> {
        let endpoints = self.mutate(endpoint);

        let futures = endpoints.iter().map(|ep| self.verifier.verify(ep.clone()));
        let results = join_all(futures).await;

        endpoints
            .into_iter()
            .zip(results)
            .filter_map(|(ep, is_valid)| if is_valid { Some(ep) } else { None })
            .collect()
    }

    fn mutate(&self, endpoint: &Endpoint) -> Vec<Endpoint> {
        let mut current_node = Some(&self.first);
        let mut endpoints = vec![endpoint.clone()];

        while let Some(node) = current_node {
            let mut next_endpoints = Vec::new();

            for ep in &endpoints {
                let mutated = (node.mutator)(ep.clone());
                next_endpoints.extend(mutated);
            }

            endpoints = next_endpoints;
            current_node = node.next();
        }

        endpoints
    }
}
