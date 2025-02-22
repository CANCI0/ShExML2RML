use crate::serializer::rml_classes::{ObjectMap, PredicateMap};

#[derive(Debug)]
pub struct PredicateObjectMap {
    object_map: ObjectMap,
    predicate_map: PredicateMap,
}
