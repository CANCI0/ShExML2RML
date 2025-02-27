use std::fmt;
use std::fmt::Formatter;
use std::sync::atomic::{AtomicUsize, Ordering};
use once_cell::sync::Lazy;

static COUNTER: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(1));

#[derive(Debug, Clone)]
pub struct SubjectMap {
    pub id: String,
    pub template: String
}

impl SubjectMap {
    pub(crate) fn new(template: String) -> SubjectMap {
        let id_number = COUNTER.fetch_add(1, Ordering::Relaxed);
        let id = format!("s_{}", id_number);

        SubjectMap{
            id,
            template,
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