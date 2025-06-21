use async_trait::async_trait;

use crate::{endpoint::Endpoint, verifier::Verifier};

pub struct DnsVerifier;

impl DnsVerifier {
    pub fn new() -> Self {
        DnsVerifier {  }
    }
}

#[async_trait]
impl Verifier for DnsVerifier {
    async fn verify(&self, target: Endpoint) -> bool {
        dns_lookup::lookup_host(&target.fqdn()).is_ok()
    }
}
