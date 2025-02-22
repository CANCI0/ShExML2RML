use crate::serializer::rml_classes::{LogicalSource, PredicateObjectMap, SubjectMap};

#[derive(Debug)]
pub struct TriplesMap {
    pub id: String,
    pub logical_source: LogicalSource,
    pub predicate_object_maps: Vec<PredicateObjectMap>,
    pub subject_maps: SubjectMap,
}

