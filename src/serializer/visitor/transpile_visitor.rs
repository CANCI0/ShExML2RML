use crate::parser::shexml_actions::*;
use crate::serializer::visitor::abstract_visitor::Visitor;
use crate::serializer::rml_classes::*;
use std::any::Any;
use std::collections::HashMap;
use crate::serializer::rml_classes::PredicateObjectMap;
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

    fn visit_expression(&mut self, n: &Expression, o: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        for a in n.paths.iter() {
            self.visit_iterator_file_relation(a, o);
        }

        None
    }


    fn visit_iterator(&mut self, n: &Iterator_, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        self.iterators.insert(String::from(&n.identifier), n.clone());

        None
    }

    fn visit_shape(&mut self, n: &Shape, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        let subject_identifier = &n.subject.subject_identifier;
        let mut prefix = None;
        if let Some(_prefix) = &n.subject.subject_identifier.prefix {
            prefix = Some(self.prefixes
                .get(&String::from(_prefix))
                .expect(format!("Unexpected namespace: {}", _prefix).as_str()));
        }

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

                let mut triples_map = TriplesMap::new(
                    logical_source.clone(),
                    subject_map,
                    vec![]
                );

                if let Some(predicate_objects) = &n.predicate_objects {
                    for predicate_object in predicate_objects {
                        let po_box = self.visit_predicate_object(predicate_object, &Some(Box::new(iterator) as Box<dyn Any>));

                        if let Some(po_box) = po_box {
                            if let Ok(po) = po_box.downcast::<PredicateObject>() {
                                triples_map.predicate_object_maps.push(po.clone());
                            } else {
                                panic!("Expected PredicateObject but got a different type");
                            }
                        }
                    }
                }

            }
        }

        None
    }

    fn visit_predicate_object(&mut self, p: &PredicateObject, o: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        if let Some(o) = o {
            if let Some(iterator) = o.downcast_ref::<Iterator_>() {
                let object;
                match &p.object {
                    Object::DataValue(data_value) => {
                        let mut prefix = None;
                        if let Some(prefix_) = &data_value.namespace {
                            prefix = Some(self.prefixes
                                .get(&String::from(prefix_))
                                .expect(format!("Unexpected namespace: {}", prefix_).as_str()));
                        }

                        let path = data_value.shape_path.split(".").map(|s| s.to_string()).collect::<Vec<String>>();
                        let field_id = path.get(1).unwrap();

                        if let Some(_field) = iterator.fields.iter().find(|f| f.identifier == *field_id) {
                            object = ObjectMap::new(
                                Some(format!("{}{}", prefix.unwrap(), _field.path)),
                                Some(TermType::IRI),
                                None
                            );
                        } else {
                            panic!("Field with identifier '{}' not found for iterator '{}'", field_id, iterator.identifier.as_str());
                        }
                    }
                    //TODO: Implementar objetos referencia
                    // Object::Reference(reference) => {
                    //     println!("Reference: {:?}", reference);
                    // }
                    _ => {}
                }

                let predicate = PredicateMap::new(format!("{}{}",  p.predicate.namespace, p.predicate.identifier));
                let predicate_object_map = PredicateObjectMap::new(
                    object?,
                    predicate,
                );

                Some(Box::new(predicate_object_map));
            } else {
                panic!("Expected a TriplesMap but got a different type");
            }
        }

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

        self.ids_for_logical_sources.insert(String::from(&iterator.identifier), logical_source);

        None
    }
}