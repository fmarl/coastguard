use crate::{
    endpoint::{Domain, Endpoint},
    resources::homoglyphs,
};

fn replace(name: &String, i: usize, homoglyph: &char) -> String {
    let mut chars: Vec<char> = name.chars().collect();
    chars[i] = *homoglyph;

    chars.into_iter().collect()
}

pub fn mutate_homoglyphs(source: Endpoint) -> Vec<Endpoint> {
    let mut result = Vec::new();
    let path = source.lower_path();

    for (depth, domain_name) in path.iter().enumerate() {
        for (i, c) in domain_name.char_indices() {
            if let Some(homoglyphs) = homoglyphs().get(&c) {
                for h in homoglyphs {
                    let mutated_name: String = replace(domain_name, i, h);
                    let mut new_chain = None;

                    for (j, d) in path.iter().enumerate().rev() {
                        let name = if j == depth {
                            mutated_name.clone()
                        } else {
                            d.name.clone()
                        };

                        new_chain = Some(Box::new(Domain {
                            name,
                            sub_domain: new_chain,
                        }));
                    }

                    result.push(Endpoint {
                        tld: source.tld.clone(),
                        domain: new_chain.expect("Domain chain should not be empty"),
                    });
                }
            }
        }
    }

    result
}
