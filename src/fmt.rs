use std::fmt;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Link<'a> {
    pub url: &'a str,
}

impl<'a> Link<'a> {
    /// Create a new link with a name and target url.
    pub fn new(url: &'a str) -> Self {
        Self { url }
    }
}

impl fmt::Display for Link<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
            self.url, self.url
        )
    }
}
