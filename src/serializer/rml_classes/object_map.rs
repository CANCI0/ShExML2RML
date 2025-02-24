use std::fmt;
use crate::serializer::rml_classes::{TermType, TriplesMap};

#[derive(Debug, Clone)]
pub struct ObjectMap {
    pub id: String,
    pub template: Option<String>,
    pub term_type: Option<TermType>,
    pub parent_triples_map: Option<TriplesMap>
}

impl ObjectMap {
    pub fn new() -> ObjectMap {
        ObjectMap{
            id: String::from("o_11"),
            template: Some(String::from("http://dbpedia.org/resource/{year}")),
            term_type: Some(TermType::IRI),
            parent_triples_map: None,
        }
    }
}

impl fmt::Display for ObjectMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        result.push_str(&format!("map:{}  a          rr:ObjectMap ;\n", self.id));

        if let Some(template) = &self.template {
            result.push_str(&format!("        rr:template  \"{}\" ;\n", template));
        }

        if let Some(term_type) = &self.term_type {
            result.push_str(&format!("        rr:termType  rr:{} ;\n", term_type));
        }

        if let Some(parent_map) = &self.parent_triples_map {
            result.push_str(&format!("        rr:parentTriplesMap  map:{} ;\n", parent_map.id));
        }

        result.pop();
        result.pop();
        result.push('.');

        writeln!(f, "{}", result)
    }
}