use std::fmt;

#[derive(Debug, Clone)]
pub struct NamespacePrefix {
    pub identifier: String,
    pub uri: String,
}

impl NamespacePrefix {
}

impl fmt::Display for NamespacePrefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@prefix {} <{}> .", self.identifier, self.uri)
    }
}