use crate::{endpoint::Endpoint, resources::TLDS};

pub fn mutate_tlds(source: Endpoint) -> Vec<Endpoint> {
    TLDS.iter().map(|tld| {
        let mut result = source.clone();

        result.tld = tld.to_string();
        result
    }).collect()
}