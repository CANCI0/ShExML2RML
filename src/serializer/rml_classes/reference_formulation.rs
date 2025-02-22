use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ReferenceFormulation {
    XPath,
    JSONPath,
}

impl fmt::Display for ReferenceFormulation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ReferenceFormulation::XPath => write!(f, "XPath"),
            ReferenceFormulation::JSONPath => write!(f, "JSONPath"),
        }
    }
}