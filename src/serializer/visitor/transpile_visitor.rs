use crate::parser::shexml_actions::*;
use crate::serializer::visitor::abstract_visitor::Visitor;
use crate::serializer::rml_classes::{RmlNode, NamespacePrefix};

pub struct TranspileVisitor {
    pub rml_code: Vec<RmlNode>,
}

impl TranspileVisitor {
    pub fn new() -> TranspileVisitor {
        let mut v = TranspileVisitor { rml_code: vec![] };

        v.rml_code.push(RmlNode::NamespacePrefix {
            0: NamespacePrefix {
                identifier: String::from("rml:"),
                uri: String::from("http://semweb.mmlab.be/ns/rml#"),
            },
        });

        v.rml_code.push(RmlNode::NamespacePrefix {
            0: NamespacePrefix {
                identifier: String::from("rr:"),
                uri: String::from("http://www.w3.org/ns/r2rml#"),
            },
        });

        v.rml_code.push(RmlNode::NamespacePrefix {
            0: NamespacePrefix {
                identifier: String::from("d2rq:"),
                uri: String::from("http://www.wiwiss.fu-berlin.de/suhl/bizer/D2RQ/0.1#"),
            },
        });

        v.rml_code.push(RmlNode::NamespacePrefix {
            0: NamespacePrefix {
                identifier: String::from("ql:"),
                uri: String::from("http://semweb.mmlab.be/ns/ql#"),
            },
        });

        v.rml_code.push(RmlNode::NamespacePrefix {
            0: NamespacePrefix {
                identifier: String::from("map:"),
                uri: String::from("http://mapping.example.com/"),
            },
        });

        v
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