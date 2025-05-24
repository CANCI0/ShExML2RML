use crate::parser::shexml_actions::{Iterator as Iterator_, *};
use crate::serializer::rml_classes::*;
use crate::serializer::visitor::abstract_visitor::Visitor;
use std::any::Any;
use std::collections::HashMap;
use std::iter::Iterator;

pub struct TranspileVisitor {
    pub result_prefixes: Vec<NamespacePrefix>,
    pub result_triple_maps: Vec<TriplesMap>,
    pub prefixes: HashMap<String, Prefix>,
    pub sources: HashMap<String, Source>,
    pub iterators: HashMap<String, Iterator_>,
    pub shapes: HashMap<String, Shape>,
    pub iterators_for_expression: HashMap<String, Vec<Iterator_>>,
    pub ids_for_logical_sources: HashMap<String, LogicalSource>,
}

impl TranspileVisitor {
    pub fn new() -> Self {
        let mut v = Self {
            result_prefixes: vec![],
            result_triple_maps: vec![],
            prefixes: HashMap::new(),
            sources: HashMap::new(),
            iterators: HashMap::new(),
            shapes: Default::default(),
            iterators_for_expression: HashMap::new(),
            ids_for_logical_sources: HashMap::new(),
        };

        let namespaces = vec![
            ("rml:", "http://semweb.mmlab.be/ns/rml#"),
            ("rr:", "http://www.w3.org/ns/r2rml#"),
            (
                "d2rq:",
                "http://www.wiwiss.fu-berlin.de/suhl/bizer/D2RQ/0.1#",
            ),
            ("ql:", "http://semweb.mmlab.be/ns/ql#"),
            ("map:", "http://mapping.example.com/"),
        ];

        for (id, uri) in namespaces {
            v.result_prefixes.push(NamespacePrefix {
                identifier: String::from(id),
                uri: String::from(uri),
            });
        }
        v
    }
}

impl Visitor<Option<Box<dyn Any>>> for TranspileVisitor {
    fn visit_shexml(&mut self, n: &Shexml, o: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        if let Some(prefixes) = &n.prefixes {
            for decl in prefixes {
                self.visit_prefix(decl, &o);
            }
        }
        if let Some(sources) = &n.sources {
            for decl in sources {
                self.visit_source(decl, &o);
            }
        }
        if let Some(iterators) = &n.iterators {
            for decl in iterators {
                self.visit_iterator(decl, &o);
            }
        }
        if let Some(expressions) = &n.expressions {
            for decl in expressions {
                self.visit_expression(decl, &o);
            }
        }
        if let Some(shapes) = &n.shapes {
            for shape in shapes {
                self.shapes.insert(
                    format!(
                        "{}{}",
                        shape.subject.class.namespace, shape.subject.class.identifier
                    ),
                    shape.clone(),
                );
            }

            for decl in shapes {
                self.visit_shape(decl, &o);
            }
        }

        let mut parent_triples_maps: Vec<&mut TriplesMap> = Vec::new();
        let mut child_triples_maps: Vec<&TriplesMap> = Vec::new();

        for triples_map_node in self.result_triple_maps.iter_mut() {
            let is_child = triples_map_node
                .predicate_object_maps
                .iter()
                .all(|po_map| po_map.object_map.parent_triples_map.is_none());

            if is_child {
                child_triples_maps.push(triples_map_node);
            } else {
                parent_triples_maps.push(triples_map_node);
            }
        }
        for parent_triples_map in parent_triples_maps.iter_mut() {
            parent_triples_map
                .predicate_object_maps
                .retain_mut(|po_map| {
                    if po_map.object_map.parent_triples_map.is_some() {
                        if let Some(index) = child_triples_maps.iter().position(|child| {
                            child.logical_source.source == parent_triples_map.logical_source.source
                                && child
                                    .logical_source
                                    .iterator
                                    .starts_with(&parent_triples_map.logical_source.iterator)
                        }) {
                            let child = child_triples_maps[index];
                            po_map.object_map.parent_triples_map = Some(child.clone());
                            child_triples_maps.remove(index);
                            true
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                });
        }

        None
    }

    fn visit_prefix(&mut self, n: &Prefix, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        self.prefixes.insert(n.identifier.clone(), n.clone());
        self.result_prefixes.push(NamespacePrefix {
            identifier: String::from(&n.identifier),
            uri: String::from(&n.uri),
        });
        None
    }

    fn visit_source(&mut self, n: &Source, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        self.sources.insert(n.identifier.clone(), n.clone());
        None
    }

    fn visit_expression(
        &mut self,
        n: &Expression,
        _: &Option<Box<dyn Any>>,
    ) -> Option<Box<dyn Any>> {
        let identifier = Some(Box::new(n.identifier.clone()) as Box<dyn Any>);

        for a in &n.paths {
            self.visit_iterator_file_relation(a, &identifier);
        }

        None
    }

    fn visit_iterator(&mut self, n: &Iterator_, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        self.iterators.insert(n.identifier.clone(), n.clone());

        None
    }

    fn visit_shape(&mut self, n: &Shape, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        let subject_generator = &n.subject.subject_identifier.subject_generator.as_str();

        let mut path = "";
        let mut attr = "";

        if let Some(pos) = subject_generator.rfind('.') {
            path = &subject_generator[..pos];
            attr = &subject_generator[pos + 1..];
        }

        let iterators = match self.iterators_for_expression.get(path) {
            Some(it) => it.clone(),
            None => return None,
        };

        for iterator in iterators {
            if let Some(logical_source) = self.ids_for_logical_sources.get(&iterator.path) {
                let prefix = n.subject.subject_identifier.prefix.as_deref().unwrap_or("");
                let prefix_uri = self.prefixes.get(prefix).map(|p| &p.uri).map_or("", |v| v);

                let field = iterator.fields.iter().find(|f| f.identifier == attr)?;

                let subject_map = SubjectMap::new(format!("{}{{{}}}", prefix_uri, field.path));
                let mut triples_map = TriplesMap::new(logical_source.clone(), subject_map, vec![]);

                if let Some(predicate_objects) = &n.predicate_objects {
                    for predicate_object in predicate_objects {
                        if let Some(po_box) = self.visit_predicate_object(
                            predicate_object,
                            &Some(Box::new(iterator.clone()) as Box<dyn Any>),
                        ) {
                            if let Ok(po) = po_box.downcast::<PredicateObjectMap>() {
                                triples_map.predicate_object_maps.push(*po);
                            }
                        }
                    }
                }
                self.result_triple_maps.push(triples_map);
            }
        }
        None
    }

    fn visit_predicate_object(
        &mut self,
        p: &PredicateObject,
        o: &Option<Box<dyn Any>>,
    ) -> Option<Box<dyn Any>> {
        let Some(iterator) = o.as_ref().and_then(|o| o.downcast_ref::<Iterator_>()) else {
            return None;
        };

        let object = if let Object::DataValue(data_value) = &p.object {
            let prefix_uri = data_value
                .namespace
                .as_ref()
                .and_then(|prefix| self.prefixes.get(prefix))
                .map(|p| &p.uri)
                .map_or("", |v| v);

            let mut path = "";

            if let Some(pos) = data_value.shape_path.rfind('.') {
                path = &data_value.shape_path[pos + 1..];
            }

            let term_type = match prefix_uri {
                "" => TermType::Literal,
                _ => TermType::IRI,
            };
            let field = iterator.fields.iter().find(|f| f.identifier == path)?;
            ObjectMap::new(
                Some(format!("{}{{{}}}", prefix_uri, field.path)),
                Some(term_type),
                None,
            )
        } else if let Object::Reference(reference) = &p.object {
            // Create ObjectMap with parent triples map reference
            let logical_source = LogicalSource::new(
                "placeholder".to_string(),
                ReferenceFormulation::None,
                "placeholder".to_string(),
            );
            let subject_map =
                SubjectMap::new(format!("{}{}", reference.namespace, reference.identifier));
            let mut parent_triples_map = TriplesMap::new(logical_source, subject_map, vec![]);
            parent_triples_map.id = format!("ref_{}", reference.identifier);

            ObjectMap::new(None, None, Some(parent_triples_map))
        } else {
            panic!()
        };

        let predicate = PredicateMap::new(format!(
            "{}{}",
            p.predicate.namespace, p.predicate.identifier
        ));
        Some(Box::new(PredicateObjectMap::new(object, predicate)))
    }

    fn visit_iterator_file_relation(
        &mut self,
        n: &IteratorFileRelation,
        o: &Option<Box<dyn Any>>,
    ) -> Option<Box<dyn Any>> {
        let iterator = self.iterators.get(&n.iterator)?;
        let file = self.sources.get(&n.file)?;

        let logical_source = LogicalSource::new(
            iterator.path.clone(),
            match iterator.path_type.clone()?.as_str() {
                "xpath:" => ReferenceFormulation::XPath,
                "jsonpath:" => ReferenceFormulation::JSONPath,
                _ => return None,
            },
            file.path.clone(),
        );

        self.ids_for_logical_sources
            .insert(iterator.path.clone(), logical_source.clone());

        if let Some(nested_iterators) = &iterator.iterators.as_ref() {
            for nested_iterator in nested_iterators {
                let separator = match iterator.path_type.clone()?.as_str() {
                    "xpath:" => "/",
                    "jsonpath:" => ".",
                    _ => "",
                };
                let nested_path = format!("{}{}{}", iterator.path, separator, nested_iterator.path);

                let nested_ls = LogicalSource::new(
                    nested_path.clone(),
                    logical_source.reference_formulation.clone(),
                    file.path.clone(),
                );

                self.ids_for_logical_sources
                    .insert(nested_iterator.path.clone(), nested_ls);
            }
        }

        if let Some(identifier) = o.as_ref().and_then(|b| b.downcast_ref::<String>()) {
            self.iterators_for_expression
                .entry(identifier.clone())
                .or_insert_with(Vec::new)
                .push(iterator.clone());

            if let Some(nested_iterators) = &iterator.iterators.as_ref() {
                for nested_iterator in nested_iterators {
                    let nested_key = format!("{}.{}", identifier, nested_iterator.identifier);

                    self.iterators_for_expression
                        .entry(nested_key)
                        .or_insert_with(Vec::new)
                        .push(nested_iterator.clone());
                }
            }
        }

        None
    }
}
