use std::fmt;
use crate::serializer::rml_classes::*;

#[derive(Debug, Clone)]
pub enum RmlNode {
    TriplesMap(TriplesMap),
    NamespacePrefix(NamespacePrefix)
}

impl fmt::Display for RmlNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RmlNode::TriplesMap(tm) => write!(f, "{}", tm),
            RmlNode::NamespacePrefix(np) => write!(f, "{}", np),
        }
    }
}