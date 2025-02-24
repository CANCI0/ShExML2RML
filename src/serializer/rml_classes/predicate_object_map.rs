use std::fmt;
use std::fmt::Formatter;
use crate::serializer::rml_classes::{ObjectMap, PredicateMap};

#[derive(Debug)]
pub struct PredicateObjectMap {
    pub id: String,
    object_map: ObjectMap,
    predicate_map: PredicateMap,
}

impl PredicateObjectMap {
    pub fn new() -> Self {
        Self{
            id: String::from("po_4"),
            object_map: ObjectMap::new(),
            predicate_map: PredicateMap::new(),
        }
    }
}

impl fmt::Display for PredicateObjectMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        result.push_str(&format!("map:{}  a              rr:PredicateObjectMap ;\n", self.id));
        result.push_str(&format!("        rr:objectMap     map:{} ;\n", self.object_map.id));
        result.push_str(&format!("        rr:predicateMap  map:{} .", self.predicate_map.id));

        writeln!(f, "{}", result)
    }
}
