use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum ReferenceFormulation {
    XPath,
    JSONPath,
    None,
}

impl fmt::Display for ReferenceFormulation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ReferenceFormulation::XPath => write!(f, "XPath"),
            ReferenceFormulation::JSONPath => write!(f, "JSONPath"),
            ReferenceFormulation::None => write!(f, ""),
        }
    }
}