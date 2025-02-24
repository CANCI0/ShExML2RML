use crate::parser::ast::Prefix;
use crate::serializer::visitor::abstract_visitor::Visitor;
use crate::serializer::rml_classes::{RmlNode, NamespacePrefix};

pub struct TranspileVisitor {
    pub rml_code: Vec<RmlNode>,
}

impl TranspileVisitor {
    pub fn new() -> TranspileVisitor {
        TranspileVisitor { rml_code: vec![] }
    }
}

impl Visitor<()> for TranspileVisitor {
    fn visit_prefix(&mut self, n: &Prefix) {
        let prefix = RmlNode::NamespacePrefix {
            0: NamespacePrefix {
                identifier: n.identifier.clone(),
                uri: n.uri.clone(),   },
        };

        self.rml_code.push(prefix);
    }
}