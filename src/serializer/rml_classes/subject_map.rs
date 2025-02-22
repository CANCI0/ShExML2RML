#[derive(Debug)]
pub struct SubjectMap {
    template: String
}

impl SubjectMap {
    pub fn new() -> SubjectMap {
        SubjectMap {
            template: String::from("http://example.com/{@id}"),
        }
    }
}