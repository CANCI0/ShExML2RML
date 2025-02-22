use std::fmt;
use crate::serializer::rml_classes::{TermType, TriplesMap};

#[derive(Debug)]
pub struct ObjectMap {
    pub template: Option<String>,
    pub term_type: Option<TermType>,
    pub parent_triples_map: Option<TriplesMap>
}

impl fmt::Display for ObjectMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "rr:objectMap     [ a rr:ObjectMap ;")?;

        if let Some(template) = &self.template {
            writeln!(f, "                      rr:template  \"{}\" ;", template)?;
        }

        if let Some(term_type) = &self.term_type {
            writeln!(f, "                      rr:termType  {} ;", term_type)?;
        }

        if let Some(parent_map) = &self.parent_triples_map {
            writeln!(f, "                      rr:parentTriplesMap  map:{} ;", parent_map.id)?;
        }

        writeln!(f, "                 ] ;")
    }
}