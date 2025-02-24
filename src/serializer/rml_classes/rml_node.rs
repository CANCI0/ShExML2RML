use std::fmt;
use crate::serializer::rml_classes::*;

#[derive(Debug, Clone)]
pub enum RmlNode {
    LogicalSource(LogicalSource),
    ObjectMap(ObjectMap),
    PredicateMap(PredicateMap),
    PredicatObjectMap(PredicateObjectMap),
    ReferenceFormulation(ReferenceFormulation),
    SubjectMap(SubjectMap),
    TermType(TermType),
    TriplesMap(TriplesMap),
    NamespacePrefix(NamespacePrefix)
}

impl fmt::Display for RmlNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RmlNode::LogicalSource(ls) => write!(f, "{}", ls),
            RmlNode::ObjectMap(om) => write!(f, "{}", om),
            RmlNode::PredicateMap(pm) => write!(f, "{}", pm),
            RmlNode::PredicatObjectMap(pom) => write!(f, "{}", pom),
            RmlNode::ReferenceFormulation(rf) => write!(f, "{}", rf),
            RmlNode::SubjectMap(sm) => write!(f, "{}", sm),
            RmlNode::TermType(tt) => write!(f, "{}", tt),
            RmlNode::TriplesMap(tm) => write!(f, "{}", tm),
            RmlNode::NamespacePrefix(np) => write!(f, "{}", np),
        }
    }
}