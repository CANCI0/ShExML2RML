use std::fmt;
use crate::serializer::rml_classes::{TermType, TriplesMap};
use std::sync::atomic::{AtomicUsize, Ordering};
use once_cell::sync::Lazy;

static COUNTER: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(1));

#[derive(Debug, Clone)]
pub struct ObjectMap {
    pub id: String,
    pub template: Option<String>,
    pub term_type: Option<TermType>,
    pub parent_triples_map: Option<TriplesMap>
}

impl ObjectMap {
    /// Creates a new ObjectMap with an optional ID
    ///
    /// # Arguments
    ///
    /// * `template` - Optional template string
    /// * `term_type` - Optional term type
    /// * `parent_triples_map` - Optional parent triples map
    /// * `id` - Optional ID, will be auto-generated if None
    pub fn new(
        template: Option<String>, 
        term_type: Option<TermType>, 
        parent_triples_map: Option<TriplesMap>,
    ) -> ObjectMap {
        let id_number = COUNTER.fetch_add(1, Ordering::Relaxed);
        let id = format!("o_{}", id_number);

        ObjectMap{
            id,
            template,
            term_type,
            parent_triples_map
        }
    }

    pub fn with_id(
        id: String,
        template: Option<String>, 
        term_type: Option<TermType>, 
        parent_triples_map: Option<TriplesMap>,
    ) -> ObjectMap {
        ObjectMap{
            id,
            template,
            term_type,
            parent_triples_map
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