use std::fmt;
use std::fmt::Formatter;
use crate::serializer::rml_classes::{ObjectMap, PredicateMap};

#[derive(Debug)]
pub struct PredicateObjectMap {
    object_map: ObjectMap,
    predicate_map: PredicateMap,
}

impl PredicateObjectMap {
    pub fn new() -> Self {
        Self{
            object_map: ObjectMap::new(),
            predicate_map: PredicateMap::new(),
        }
    }
}

impl fmt::Display for PredicateObjectMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "rr:predicateObjectMap  [ a                rr:PredicateObjectMap ;")?;
        writeln!(f, "                       {}", self.object_map)?;
        write!(f, "                       {}", self.predicate_map)
    }
}
