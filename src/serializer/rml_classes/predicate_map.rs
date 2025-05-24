use std::fmt;
use std::fmt::Formatter;
use std::sync::atomic::{AtomicUsize, Ordering};
use once_cell::sync::Lazy;

static COUNTER: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(1));

#[derive(Debug, Clone)]
pub struct PredicateMap {
    pub id: String,
    pub constant: String,
}

impl PredicateMap {
    pub fn new(constant: String) -> PredicateMap {
        let id_number = COUNTER.fetch_add(1, Ordering::Relaxed);
        let id = format!("p_{}", id_number);

        PredicateMap {
            id,
            constant
        }
    }
    
}

impl fmt::Display for PredicateMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        result.push_str(&format!("map:{}  a           rr:predicateMap ;\n", self.id));
        result.push_str(&format!("        rr:constant  {} .", self.constant));

        write!(f, "{}", result)
    }
}
