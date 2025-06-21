use futures::{stream, StreamExt};

use crate::{
    endpoint::{self, Endpoint}, fmt::Link, verifier::Verifier
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
}

pub struct MutationChain<V: Verifier> {
    pub verifier: V,
    pub first: MutationChainNode,
}

impl<V: Verifier> MutationChain<V> {
    pub fn new(verifier: V, first: MutationChainNode) -> Self {
        MutationChain {
            verifier,
            first,
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

        let results = stream::iter(endpoints.clone())
            .map(|ep| {
                let verifier = &self.verifier;
                async move {
                    let is_valid = verifier.verify(ep.clone()).await;

                    if is_valid {
                        println!("Found {}!", Link::new(&ep.fqdn()));
                    }

                    (ep, is_valid)
                }
            })
            .buffer_unordered(usize::MAX)
            .collect::<Vec<_>>()
            .await;

        results
            .into_iter()
            .filter_map(|(ep, is_valid)| if is_valid { Some(ep) } else { None })
            .collect()
    }

    fn mutate(&self, endpoint: &Endpoint) -> Vec<Endpoint> {
        let mut current_node = Some(&self.first);
        let mut endpoints = Vec::<Endpoint>::new();

        while let Some(node) = current_node {
            let mut mutated_endpoints = (node.mutator)(endpoint.clone());

            endpoints.append(&mut mutated_endpoints);
            current_node = node.next();
        }

        endpoints
    }
}
