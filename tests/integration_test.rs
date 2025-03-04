
#[cfg(test)]
mod integration_test {
    use std::fs;
    use std::path::Path;
    use rustemo::Parser;
    use shexml2rml::parser::shexml::ShexmlParser;
    use shexml2rml::serializer::visitor::{TranspileVisitor, Visitor};

    fn read_test_file(file_name: &str) -> String {
        let path = Path::new("./resources/integration/").join(file_name);
        fs::read_to_string(path).expect(&format!("Failed to read path: {}", file_name))
    }

    #[test]
    fn test_iterators_single_expression() {
        let parser = ShexmlParser::new();
        let input = read_test_file("single_iterator_single_expression.shexml");
        let expected = fs::read_to_string("./outputs/single_iterator_single_expression_expected.ttl")
            .expect("Failed to read path")
            .replace("\r", "");

        let result = parser.parse(&input);

        assert!(result.is_ok(), "The parser failed to parse valid input.");

        let ast = result.unwrap();

        let mut visitor = TranspileVisitor::new();
        visitor.visit_shexml(&ast, &None);

        let result_str = visitor.rml_code
            .iter()
            .map(|node| node.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        println!("{}", result_str);

        assert_eq!(result_str, expected, "The generated RML does not match the expected output.");
    }

    #[test]
    fn test_iterators_single_expression_nested() {
        let parser = ShexmlParser::new();
        let input = read_test_file("iterators_single_expression_nested.shexml");

        let result = parser.parse(&input);

        assert!(result.is_ok(), "The parser failed to parse valid input.");

        let ast = result.unwrap();

        let mut visitor = TranspileVisitor::new();
        visitor.visit_shexml(&ast, &None);
        let result = visitor.rml_code;

        for node in result {
            println!("{}", node);
        }
    }
}