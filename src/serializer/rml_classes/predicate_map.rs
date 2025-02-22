use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct PredicateMap {
    constant: String,
}

impl PredicateMap {
    pub fn new() -> PredicateMap {
        PredicateMap {
            constant: String::from(":year"),
        }
    }
}

impl fmt::Display for PredicateMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "rr:predicateMap  [ a            rr:predicateMap ;");
        writeln!(f, "                   rr:constant  {}", self.constant);
        write!(f, "                 ]")
    }
}
