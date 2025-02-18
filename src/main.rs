use std::{fs};
use rust_shexml_interpreter::parser::shexml::ShexmlParser;
use rustemo::Parser;

fn main() {
    let input = fs::read_to_string("resources/test.shexml").unwrap();
    let result = ShexmlParser::new().parse(&input);

    println!("{:#?}", result);
}