use std::fs;
use rustemo::Parser;
use shexml2rml::parser::shexml::ShexmlParser;
use shexml2rml::serializer::visitor::{TranspileVisitor, Visitor};

pub fn transpile_file(input: &str, output: Option<&str>) {
    let parser = ShexmlParser::new();

    match fs::read_to_string(input) {
        Ok(content) => {
            let rml = parser.parse(&content);
            let ast = rml.unwrap();
            let mut visitor = TranspileVisitor::new();
            visitor.visit_shexml(&ast, &None);

            let result_str = visitor.rml_code
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join("\n");
            match output {
                Some(out_path) => {
                    if let Err(err) = fs::write(out_path, &result_str) {
                        eprintln!("Error writing output file: {}", err);
                    } else {
                        println!("Transpilation successful! Output saved to {}", out_path);
                    }
                }
                None => println!("{}", result_str),
            }
        }
        Err(err) => eprintln!("Error reading input file: {}", err),
    }
}
