use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct SubjectMap {
    pub id: String,
    pub template: String
}

impl SubjectMap {
    pub fn new() -> SubjectMap {
        SubjectMap {
            id: String::from("s_2"),
            template: String::from("http://example.com/{id}"),
        }
    }
}

impl fmt::Display for SubjectMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        result.push_str(&format!("map:{}  a           rr:SubjectMap ;\n", self.id));
        result.push_str(&format!("        rr:template  \"{}\" .", self.template));

        writeln!(f, "{}", result)
    }
}