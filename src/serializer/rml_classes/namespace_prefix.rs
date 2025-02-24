use std::fmt;

#[derive(Debug, Clone)]
pub struct NamespacePrefix {
    pub identifier: String,
    pub uri: String,
}

impl NamespacePrefix {
    pub fn new() -> NamespacePrefix {
        NamespacePrefix{
            identifier: String::from("schema:"),
            uri: String::from("http://schema.org/"),
        }
    }
}

impl fmt::Display for NamespacePrefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@prefix {} <{}> .", self.identifier, self.uri)
    }
}