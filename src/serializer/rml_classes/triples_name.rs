#[derive(Debug)]
struct TriplesMap {
    logical_source: LogicalSource,
    predicate_object_maps: Vec<PredicateObjectMap>,
    subject_maps: SubjectMap,
}