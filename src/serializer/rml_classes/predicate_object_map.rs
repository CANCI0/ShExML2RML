use std::fmt;
use std::fmt::Formatter;
use crate::serializer::rml_classes::{ObjectMap, PredicateMap};
use std::sync::atomic::{AtomicUsize, Ordering};
use once_cell::sync::Lazy;

static COUNTER: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(1));

#[derive(Debug, Clone)]
pub struct PredicateObjectMap {
    pub id: String,
    object_map: ObjectMap,
    predicate_map: PredicateMap,
}

impl PredicateObjectMap {
    pub fn new(object_map: ObjectMap, predicate_map: PredicateMap) -> Self {
        let id_number = COUNTER.fetch_add(1, Ordering::Relaxed);
        let id = format!("s_{}", id_number);

        PredicateObjectMap{
            id,
            object_map,
            predicate_map,
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
