use std::{fs};
use std::path::Path;
use rust_shexml_interpreter::parser::syntax::parse_input;

fn main() {
    let unparsed_file = read_test_file("test.shexml");
    let file = parse_input(&unparsed_file);

    println!("{:#?}", file);
}

fn read_test_file(file_name: &str) -> String {
    let path = Path::new("resources").join(file_name);
    fs::read_to_string(path).expect(&format!("Failed to read {}", file_name))
}