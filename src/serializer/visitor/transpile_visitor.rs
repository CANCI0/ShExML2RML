use crate::parser::shexml_actions::*;
use crate::serializer::visitor::abstract_visitor::Visitor;
use crate::serializer::rml_classes::*;
use std::any::Any;
use std::collections::HashMap;

pub struct TranspileVisitor {
    pub rml_code: Vec<RmlNode>,
    pub prefixes: HashMap<String, Prefix>,
    pub sources: HashMap<String, Source>,
    pub iterators: HashMap<String, Iterator_>,
    pub iterators_for_expression: HashMap<String, Vec<Iterator_>>,
    pub sources_for_iterator: HashMap<String, Vec<Source>>,
    pub ids_for_logical_sources: HashMap<String, LogicalSource>,
}

impl TranspileVisitor {
    pub fn new() -> Self {
        let mut visitor = Self {
            rml_code: vec![],
            prefixes: HashMap::new(),
            sources: HashMap::new(),
            iterators: HashMap::new(),
            iterators_for_expression: HashMap::new(),
            sources_for_iterator: Default::default(),
            ids_for_logical_sources: HashMap::new(),
        };

        // Initialize standard RML namespaces
        visitor.add_standard_namespaces();
        visitor
    }
    
    /// Add standard RML namespaces that are required in all RML outputs
    fn add_standard_namespaces(&mut self) {
        let standard_namespaces = vec![
            ("rml:", "http://semweb.mmlab.be/ns/rml#"),
            ("rr:", "http://www.w3.org/ns/r2rml#"),
            ("d2rq:", "http://www.wiwiss.fu-berlin.de/suhl/bizer/D2RQ/0.1#"),
            ("ql:", "http://semweb.mmlab.be/ns/ql#"),
            ("map:", "http://mapping.example.com/"),
        ];

        for (id, uri) in standard_namespaces {
            self.add_namespace_prefix(id, uri);
        }
    }
    
    /// Add a namespace prefix to the RML output
    fn add_namespace_prefix(&mut self, identifier: &str, uri: &str) {
        self.rml_code.push(RmlNode::NamespacePrefix {
            0: NamespacePrefix {
                identifier: identifier.to_string(),
                uri: uri.to_string(),
            },
        });
    }
    
    /// Get a prefix URI for a given namespace prefix
    fn get_prefix_uri(&self, prefix: Option<&String>) -> &str {
        match prefix {
            Some(p) => self.prefixes.get(p).map(|prefix| prefix.uri.as_str()).unwrap_or(""),
            None => ""
        }
    }
    
    /// Find a field in iterator by its identifier
    fn find_field_by_id<'a>(&self, iterator: &'a Iterator_, field_id: &str) -> Option<&'a Attribute> {
        iterator.fields.iter().find(|f| f.identifier == field_id)
    }
    
    /// Create logical source from iterator and file source
    fn create_logical_source(&self, iterator: &Iterator_, file: &Source) -> Option<LogicalSource> {
        let reference_formulation = match iterator.path_type.as_str() {
            "xpath:" => ReferenceFormulation::XPath,
            "jsonpath:" => ReferenceFormulation::JSONPath,
            _ => return None,
        };
        
        Some(LogicalSource::new(
            iterator.path.clone(),
            reference_formulation,
            file.path.clone(),
        ))
    }
    
    /// Create an object map from a data value and iterator
    fn create_object_map(&self, data_value: &DataValue, iterator: &Iterator_) -> Option<ObjectMap> {
        let prefix_uri = self.get_prefix_uri(data_value.namespace.as_ref());
        let path = data_value.shape_path.split('.').nth(1).unwrap_or("");
        
        let term_type = if prefix_uri.is_empty() {
            TermType::Literal
        } else {
            TermType::IRI
        };
        
        let field = self.find_field_by_id(iterator, path)?;
        
        Some(ObjectMap::new(
            Some(format!("{}{{{}}}", prefix_uri, field.path)),
            Some(term_type),
            None
        ))
    }
    
    /// Create a predicate map from predicate data
    fn create_predicate_map(&self, predicate: &Predicate) -> PredicateMap {
        PredicateMap::new(format!("{}{}", predicate.namespace, predicate.identifier))
    }
}

impl Visitor<Option<Box<dyn Any>>> for TranspileVisitor {
    fn visit_prefix(&mut self, n: &Prefix, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        // Store the prefix in our map for later reference
        self.prefixes.insert(n.identifier.clone(), n.clone());
        
        // Add the prefix to the RML output
        self.add_namespace_prefix(&n.identifier, &n.uri);
        
        None
    }

    fn visit_source(&mut self, n: &Source, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        // Store the source in our map for later reference
        self.sources.insert(n.identifier.clone(), n.clone());
        None
    }

    fn visit_expression(&mut self, n: &Expression, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        // Pass the expression identifier to the iterator file relation visitor
        let identifier = Some(Box::new(n.identifier.clone()) as Box<dyn Any>);

        for relation in &n.paths {
            self.visit_iterator_file_relation(relation, &identifier);
        }

        None
    }

    fn visit_iterator(&mut self, n: &Iterator_, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        // Store the iterator in our map for later reference
        self.iterators.insert(n.identifier.clone(), n.clone());
        None
    }

    fn visit_shape(&mut self, n: &Shape, _: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        // Extract the expression path and attribute from the subject generator
        let subject_generator = &n.subject.subject_identifier.subject_generator;
        let path_parts: Vec<&str> = subject_generator.split('.').collect();
        
        let (expr_path, attr) = match path_parts.as_slice() {
            [path, attr, ..] => (*path, *attr),
            _ => return None, // Invalid path format
        };

        // Get iterators for this expression
        let iterators = self.iterators_for_expression.get(expr_path).cloned()?;

        for iterator in iterators {
            if let Some(logical_source) = self.ids_for_logical_sources.get(&iterator.identifier) {
                // Create subject map using the prefix and field path
                let prefix = n.subject.subject_identifier.prefix.as_deref().unwrap_or("");
                let prefix_uri = self.get_prefix_uri(Some(&prefix.to_string()));
                
                let field = self.find_field_by_id(&iterator, attr)?;
                let subject_map = SubjectMap::new(format!("{}{{{}}}", prefix_uri, field.path));
                
                // Create triples map with the logical source and subject map
                let mut triples_map = TriplesMap::new(logical_source.clone(), subject_map, vec![]);
                
                // Process all predicate-object pairs
                if let Some(predicate_objects) = &n.predicate_objects {
                    for predicate_object in predicate_objects {
                        let iterator_box = Box::new(iterator.clone()) as Box<dyn Any>;
                        if let Some(po_box) = self.visit_predicate_object(predicate_object, &Some(iterator_box)) {
                            if let Ok(po) = po_box.downcast::<PredicateObjectMap>() {
                                triples_map.predicate_object_maps.push(*po);
                            }
                        }
                    }
                }
                
                // Add the triples map to the RML code output
                self.rml_code.push(RmlNode::TriplesMap(triples_map));
            }
        }
        
        None
    }


    fn visit_predicate_object(&mut self, p: &PredicateObject, o: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        // Extract iterator from the context object
        let iterator = match o.as_ref().and_then(|o| o.downcast_ref::<Iterator_>()) {
            Some(it) => it,
            None => return None,
        };

        // Handle different types of objects
        let object = match &p.object {
            Object::DataValue(data_value) => self.create_object_map(data_value, iterator)?,
            Object::Reference(_) => return None, // Not implemented yet
        };

        // Create predicate map and predicate-object map
        let predicate = self.create_predicate_map(&p.predicate);
        let predicate_object_map = PredicateObjectMap::new(object, predicate);
        
        Some(Box::new(predicate_object_map))
    }

    fn visit_iterator_file_relation(&mut self, n: &IteratorFileRelation, o: &Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        // Get iterator and file source from their identifiers
        let iterator = self.iterators.get(&n.iterator)?;
        let file = self.sources.get(&n.file)?;

        // Create and store logical source
        let logical_source = self.create_logical_source(iterator, file)?;
        self.ids_for_logical_sources.insert(iterator.identifier.clone(), logical_source);

        // Link iterator to expression if available
        if let Some(identifier) = o.as_ref().and_then(|b| b.downcast_ref::<String>()) {
            self.iterators_for_expression
                .entry(identifier.clone())
                .or_insert_with(Vec::new)
                .push(iterator.clone());
        }

        None
    }
}
