use std::fmt;
use std::fmt::Formatter;
use crate::serializer::rml_classes::{LogicalSource, PredicateObjectMap, SubjectMap};
use std::sync::atomic::{AtomicUsize, Ordering};
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
        let id = format!("m_{}", id_number);

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
        let maps = self.predicate_object_maps.iter()
            .map(|po| format!("map:{} ", po.id))
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(f, "")?;
        writeln!(f, "map:{}  a rr:TriplesMap ;", self.id)?;
        writeln!(f, "    rml:logicalSource      map:{} ;", self.logical_source.id)?;
        writeln!(f, "    rr:predicateObjectMap  {};", maps)?;
        writeln!(f, "    rr:subjectMap          map:{} .", self.subject_map.id)?;
        writeln!(f, "")?;
        writeln!(f, "{}", self.logical_source)?;
        writeln!(f, "{}", self.subject_map)?;
        for map in &self.predicate_object_maps {
            write!(f, "{}", map)?;
        }

        write!(f, "")
    }
}