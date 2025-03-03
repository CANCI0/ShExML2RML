use std::fmt;

#[derive(Debug, Clone)]
pub enum TermType {
    IRI,
    Literal
}

impl fmt::Display for TermType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TermType::IRI => write!(f, "IRI"),
            TermType::Literal => write!(f, "Literal"),
        }
    }
}