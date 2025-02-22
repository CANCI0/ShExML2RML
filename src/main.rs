
use rustemo::Parser;
use rust_shexml_interpreter::serializer::rml_classes::{ObjectMap, TermType};

fn main() {
    let input = ObjectMap {
        template: Option::Some("http://dbpedia.org/resource/{crew.screenwritter}".parse().unwrap()),
        term_type: Option::from(TermType::IRI),
        parent_triples_map: Option::None
    };

    println!("{}", input);
}