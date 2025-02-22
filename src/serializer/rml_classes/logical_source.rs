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