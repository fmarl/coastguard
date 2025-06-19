use crate::endpoint::Endpoint;

pub trait Verifier {
    fn verify(target: Endpoint) -> bool;
}