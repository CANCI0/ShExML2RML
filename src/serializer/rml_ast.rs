#[derive(Debug)]
struct TriplesMap {
    logical_source: LogicalSource,
    predicate_object_maps: Vec<PredicateObjectMap>,
    subject_maps: SubjectMap,
}

#[derive(Debug)]
struct LogicalSource {
    iterator: String,
    reference_formulation: ReferenceFormulation,
    source: String,
}

#[derive(Debug)]
enum ReferenceFormulation {
    XPath,
    JSONPath,
}

#[derive(Debug)]
struct SubjectMap {
    template: String
}

#[derive(Debug)]
struct PredicateObjectMap {
    object_map: ObjectMap,
    predicate_map: PredicateMap,
}

#[derive(Debug)]
struct ObjectMap {
    template: Option<String>,
    term_type: Option<TermType>,
    parent_triples_map: Option<TriplesMap>
}

#[derive(Debug)]
enum TermType {
    IRI,
    Literal
}

#[derive(Debug)]
struct PredicateMap {
    constant: String,
}
