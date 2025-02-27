use crate::parser::shexml_actions::*;
use crate::serializer::visitor::abstract_visitor::Visitor;
use crate::serializer::rml_classes::{RmlNode, NamespacePrefix, ReferenceFormulation, SubjectMap, LogicalSource, logical_source, TriplesMap};
use std::any::Any;
use std::collections::HashMap;
use crate::parser::shexml_actions::{Prefix};

pub struct TranspileVisitor {
    pub rml_code: Vec<RmlNode>,
    pub prefixes: HashMap<String, Prefix>,
    pub sources: HashMap<String, Source>,
    pub iterators: HashMap<String, Iterator_>,
    pub iterators_for_expression: HashMap<String, Vec<Iterator_>>,
    pub ids_for_logical_sources: HashMap<String, LogicalSource>,
}

impl TranspileVisitor {
    pub fn new() -> TranspileVisitor {
        let mut v = TranspileVisitor {
            rml_code: vec![],
            prefixes: Default::default(),
            sources: Default::default(),
            iterators: Default::default(),
            iterators_for_expression: Default::default(),
            ids_for_logical_sources: Default::default(),
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
            let identifier_ref: &str = &n.identifier;
            let iterator_any = self.visit_iterator_file_relation(a, &Some(Box::new(identifier_ref) as Box<dyn Any>));

            if let Some(iterator_any) = iterator_any {
                if let Ok(iterator) = iterator_any.downcast::<Vec<Iterator_>>() {
                    let v = self.iterators_for_expression.get_mut(&n.identifier).unwrap();
                    v.push(*iterator);
                } else {
                    panic!("Error: No se pudo convertir a Vec<TipoIterador>");
                }
            }
        }

        None
    }


    fn visit_iterator(&mut self, n: &Iterator_, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        self.iterators.insert(String::from(&n.identifier), n.clone());

        None
    }

    fn visit_shape(&mut self, n: &Shape, obj: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        let subject_identifier = &n.subject.subject_identifier;

        let prefix = self.prefixes
            .get(&String::from(&subject_identifier.prefix))
            .expect(format!("Unexpected namespace: {}", &subject_identifier.prefix).as_str());

        let path_sequence = subject_identifier.subject_generator.as_str().split(".").map(|s| s.to_string()).collect::<Vec<String>>();

        let expression_ref = path_sequence.get(0).unwrap();
        let attribute_ref = path_sequence.get(1).unwrap();

        let iterators = self.iterators_for_expression.get(expression_ref);

        if let Some(iterators) = iterators {
            for iterator in iterators {
                let logical_source = self.ids_for_logical_sources.get(iterator.identifier.as_str()).unwrap();
                let subject_map;
                if let Some(field) = iterator.fields.iter().find(|f| f.identifier == *attribute_ref) {
                    subject_map = SubjectMap::new(format!("{}{}", prefix, field.path));
                } else {
                    panic!("Field with identifier '{}' not found for iterator '{}'", attribute_ref, iterator.identifier);
                }

                let triples_map = TriplesMap::new(
                    LogicalSource(logical_source),
                    subject_map,
                    vec![]
                );
            }
        }

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

        self.ids_for_logical_sources.insert(String::from(&iterator.identifier), LogicalSource(logical_source));

        None
    }
}