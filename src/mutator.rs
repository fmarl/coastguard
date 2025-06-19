use crate::{endpoint::Endpoint, verifier::Verifier};

mod levenshtein;
mod homoglyph;
mod pre_in_suffix;

pub trait Mutator {
    fn mutate<T: Verifier>(source: Endpoint, verifier: T) -> Vec<Box<Future<(),()> + Send>>;
}
