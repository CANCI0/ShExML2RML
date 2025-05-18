use rustemo::Parser;
use crate::parser::shexml::ShexmlParser;
use crate::serializer::visitor::{TranspileVisitor, Visitor};

pub fn transpile_content(input_content: &str) -> Result<String, String> {
    let parser = ShexmlParser::new();

    let ast = parser.parse(input_content)
        .map_err(|e| format!("Parsing error: {:?}", e))?;

    let mut visitor = TranspileVisitor::new();
    visitor.visit_shexml(&ast, &None);

    let prefixes = visitor.result_prefixes
        .iter()
        .map(|node| node.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    let triple_maps = visitor.result_triple_maps
        .iter()
        .map(|node| node.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    Ok(format!("{}\n{}", prefixes, triple_maps))
}