use std::fmt;
use std::fmt::Formatter;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::serializer::rml_classes::{LogicalSource, PredicateObjectMap, ReferenceFormulation, SubjectMap};
use indoc::indoc;
use once_cell::sync::Lazy;

static COUNTER: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(1));

#[derive(Debug, Clone)]
pub struct TriplesMap {
    pub id: String,
    pub logical_source: LogicalSource,
    pub predicate_object_maps: Vec<PredicateObjectMap>,
    pub subject_map: SubjectMap,
}

impl TriplesMap {
    pub fn new(logical_source: LogicalSource, subject_map: SubjectMap, predicate_object_maps: Vec<PredicateObjectMap>) -> TriplesMap {
        let id_number = COUNTER.fetch_add(1, Ordering::Relaxed);
        let id = format!("s_{}", id_number);

        TriplesMap {
            id,
            logical_source,
            subject_map,
            predicate_object_maps,
        }
    }
}

impl fmt::Display for TriplesMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        let maps = self.predicate_object_maps.iter()
            .map(|po| format!("map:{} ", po.id))
            .collect::<Vec<_>>()
            .join(", ");

        result.push_str(&format!("map:{}  a                     rr:TriplesMap ;\n", self.id));
        result.push_str(&format!("        rml:logicalSource      map:{} ;\n", self.logical_source.id));
        result.push_str(&format!("        rr:predicateObjectMap  {};\n", maps));
        result.push_str(&format!("        rr:subjectMap          map:{} .", self.subject_map.id));

        writeln!(f, "{}", result)
    }
}