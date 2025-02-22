#[derive(Debug)]
struct ObjectMap {
    template: Option<String>,
    term_type: Option<TermType>,
    parent_triples_map: Option<TriplesMap>
}