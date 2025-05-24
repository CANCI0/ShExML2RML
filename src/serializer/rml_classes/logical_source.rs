use std::fmt;
use std::fmt::Formatter;
use crate::serializer::rml_classes::ReferenceFormulation;
use std::sync::atomic::{AtomicUsize, Ordering};
use indoc::indoc;
use once_cell::sync::Lazy;

static COUNTER: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(1));

#[derive(Clone, Debug)]
pub struct LogicalSource {
    pub id: String,
    pub iterator: String,
    pub reference_formulation: ReferenceFormulation,
    pub source: String,
}

impl LogicalSource {
    pub fn new(iterator: String, reference_formulation: ReferenceFormulation, source: String) -> Self {
        let id_number = COUNTER.fetch_add(1, Ordering::Relaxed);
        let id = format!("ls_{}", id_number);

        LogicalSource {
            id,
            iterator,
            reference_formulation,
            source,
        }
    }    
    
}


impl fmt::Display for LogicalSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let str = format!(indoc! {"
            map:{}  a                rml:LogicalSource ;
                    rml:iterator              \"{}\" ;
                    rml:referenceFormulation  ql:{} ;
                    rml:source                \"{}\" .
        "}, self.id, self.iterator, self.reference_formulation, self.source);

        let mut result = String::new();

        result.push_str(&format!("map:{}  a                rml:LogicalSource ;\n", self.id));
        result.push_str(&format!("        rml:iterator              \"{}\" ;\n", self.iterator));
        result.push_str(&format!("        rml:referenceFormulation  ql:{} ;\n", self.reference_formulation));
        result.push_str(&format!("        rml:source                \"{}\" .", self.source));

        writeln!(f, "{}", str.trim())
    }
}