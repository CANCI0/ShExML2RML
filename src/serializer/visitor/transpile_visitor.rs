use crate::parser::shexml_actions::*;
use crate::serializer::visitor::abstract_visitor::Visitor;
use crate::serializer::rml_classes::{RmlNode, NamespacePrefix, ReferenceFormulation};
use std::any::Any;
use std::collections::HashMap;
use crate::parser::shexml_actions::Prefix;
use crate::serializer::rml_classes::LogicalSource;

pub struct TranspileVisitor {
    pub rml_code: Vec<RmlNode>,
    pub prefixes: HashMap<String, Prefix>,
    pub sources: HashMap<String, Source>,
    pub iterators: HashMap<String, Iterator>,
    pub iterators_for_expression: HashMap<String, Iterator>,
}

impl TranspileVisitor {
    pub fn new() -> TranspileVisitor {
        let mut v = TranspileVisitor {
            rml_code: vec![],
            prefixes: Default::default(),
            sources: Default::default(),
            iterators: Default::default(),
            iterators_for_expression: Default::default(),
        };

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

impl Visitor<Option<Box<dyn Any>>> for TranspileVisitor {
    
    fn visit_prefix(&mut self, n: &Prefix, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        let prefix = RmlNode::NamespacePrefix {
            0: NamespacePrefix {
                identifier: String::from(&n.identifier),
                uri: String::from(&n.uri),
            },
        };

        self.prefixes.insert(String::from(&n.identifier), n.clone());

        self.rml_code.push(prefix);

        None
    }

    fn visit_source(&mut self, n: &Source, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        self.sources.insert(String::from(&n.identifier), n.clone());

        None
    }

    fn visit_expression(&mut self, n: &Expression, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        for a in n.paths.iter() {
            self.visit_iterator_file_relation(a, &Some(Box::new(String::from(&n.identifier)) as Box<dyn Any>));
        }

        None
    }

    fn visit_iterator(&mut self, n: &Iterator, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        self.iterators.insert(String::from(&n.identifier), n.clone());

        None
    }

    fn visit_iterator_file_relation(&mut self, n: &IteratorFileRelation, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        let iterator = self.iterators
            .get(&n.iterator)
            .expect(&format!("Iterator '{}' not found", n.iterator));

        let file = self.sources
            .get(&n.file)
            .expect(&format!("File '{}' not found", n.file));

        let logical_source = LogicalSource::new(
            String::from(&iterator.path),
            match iterator.path_type.as_str() {
                "xpath:" => ReferenceFormulation::XPath,
                "jsonpath:" => ReferenceFormulation::JSONPath,
                _ => panic!("Unknown path type: {}", iterator.path_type),
            },
            String::from(&file.path),
        );

        self.rml_code.push(RmlNode::LogicalSource(logical_source));

        None
    }

}
