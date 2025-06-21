#[derive(Clone)]
pub struct Domain {
    pub name: String,
    pub sub_domain: Option<Box<Domain>>,
}

#[derive(Clone)]
pub struct Endpoint {
    pub tld: String,
    pub domain: Box<Domain>,
}

impl Domain {
    pub fn new(name: String, sub_domain: Option<Box<Domain>>) -> Box<Self> {
        Box::new(Domain { name, sub_domain })
    }

    pub fn next(&self) -> Option<&Box<Domain>> {
        return self.sub_domain.as_ref();
    }
}

impl Endpoint {
    pub fn new(fqdn: &String) -> Self {
        let labels: Vec<&str> = fqdn.split('.').collect();
        let root = Self::build_domain_chain(&labels);

        Endpoint {
            tld: root.name,
            domain: root
                .sub_domain
                .expect("Endpoint doesn't seem to be a FQDN."),
        }
    }

    pub fn lower_path(&self) -> Vec<&String> {
        let mut path: Vec<&String> = Vec::new();
        let mut current = Some(&self.domain);

        while let Some(domain) = current {
            path.push(&domain.name);
            current = domain.next();
        }

        path
    }

    pub fn fqdn(&self) -> String {
        let mut result = format!("{}.{}", self.domain.name, self.tld);
        let mut current = self.domain.next();

        while let Some(domain) = current {
            result = format!("{}.{}", domain.name, result);

            current = domain.next();
        }

        return result;
    }

    fn build_domain_chain(parts: &[&str]) -> Box<Domain> {
        if let Some((head, tail)) = parts.split_last() {
            let sub = if tail.is_empty() {
                None
            } else {
                Some(Self::build_domain_chain(tail))
            };

            Domain::new(head.to_string(), sub)
        } else {
            panic!("FQDN must contain at least one part");
        }
    }
}
