use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct PredicateMap {
    pub(crate) id: String,
    constant: String,
}

impl PredicateMap {
    pub fn new() -> PredicateMap {
        PredicateMap {
            id: String::from("p_3"),
            constant: String::from(":year"),
        }
    }
}

impl fmt::Display for PredicateMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        result.push_str(&format!("map:{}  a           rr:predicateMap ;\n", self.id));
        result.push_str(&format!("        rr:constant  {} .", self.constant));

        writeln!(f, "{}", result)
    }
}
