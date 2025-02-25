use std::fs;
use std::path::Path;
use std::time::Instant;
use rustemo::Parser;
use rust_shexml_interpreter::parser::shexml::ShexmlParser;
use rust_shexml_interpreter::serializer::visitor::{TranspileVisitor, Visitor};

fn main() {
    let start = Instant::now();

    let parser = ShexmlParser::new();
    let input = read_test_file("test.shexml");

    let shexml = parser.parse(&input).unwrap();

    println!("{:#?}", shexml);

    let mut visitor = TranspileVisitor::new();
    visitor.visit_shexml(&shexml, &None);

    for node in &visitor.rml_code {
        println!("{}", node);
    }

    let duration = start.elapsed();

    println!("La función tardó: {:?}", duration);
}
fn read_test_file(file_name: &str) -> String {
    let path = Path::new("resources").join(file_name);
    fs::read_to_string(path).expect(&format!("Failed to read {}", file_name))
}
