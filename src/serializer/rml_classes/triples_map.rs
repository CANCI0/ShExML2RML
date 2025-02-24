use std::fmt;
use std::fmt::Formatter;
use crate::serializer::rml_classes::{LogicalSource, PredicateObjectMap, SubjectMap};

#[derive(Debug, Clone)]
pub struct TriplesMap {
    pub id: String,
    pub logical_source: LogicalSource,
    pub predicate_object_maps: Vec<PredicateObjectMap>,
    pub subject_map: SubjectMap,
}

impl TriplesMap {
    pub fn new() -> TriplesMap {
        TriplesMap{
            id: String::from("m_2"),
            logical_source: LogicalSource::new(),
            predicate_object_maps: vec![],
            subject_map: SubjectMap::new(),
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