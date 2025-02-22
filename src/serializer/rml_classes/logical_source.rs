use std::fmt;
use std::fmt::Formatter;
use crate::serializer::rml_classes::ReferenceFormulation;

#[derive(Debug)]
pub struct LogicalSource {
    iterator: String,
    reference_formulation: ReferenceFormulation,
    source: String,
}

impl LogicalSource {
    pub fn new() -> LogicalSource {
        LogicalSource{
            iterator: String::from("//film"),
            reference_formulation: ReferenceFormulation::XPath,
            source: String::from("https://shexml.herminiogarcia.com/files/films.xml"),
        }
    }
}

impl fmt::Display for LogicalSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "rml:logicalSource      [ a                         rml:LogicalSource ;");
        writeln!(f, "                         rml:iterator              \"{}\" ;", self.iterator);
        writeln!(f, "                         rml:referenceFormulation  ql:{} ;", self.reference_formulation);
        writeln!(f, "                         rml:source                \"{}\"", self.source);
        write!(f, "                       ] ;")
    }
}