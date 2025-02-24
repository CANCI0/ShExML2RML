use std::fmt;
use std::fmt::Formatter;
use crate::serializer::rml_classes::ReferenceFormulation;
use indoc::indoc;

#[derive(Debug, Clone)]
pub struct LogicalSource {
    pub id: String,
    iterator: String,
    reference_formulation: ReferenceFormulation,
    source: String,
}

impl LogicalSource {
    pub fn new() -> LogicalSource {
        LogicalSource{
            id: String::from("l_260963552"),
            iterator: String::from("//film/cast/actress"),
            reference_formulation: ReferenceFormulation::XPath,
            source: String::from("https://shexml.herminiogarcia.com/files/films.xml"),
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