#[cfg(test)]
mod tests {
    use rust_shexml_interpreter::parser::shexml::ShexmlParser;
    use std::fs;
    use std::iter::Iterator;
    use std::path::Path;
    use rustemo::Parser;
    use rust_shexml_interpreter::parser::shexml_actions::{Prefix1, Source1};

    fn read_test_file(file_name: &str) -> String {
        let path = Path::new("resources").join(file_name);
        fs::read_to_string(path).expect(&format!("Failed to read {}", file_name))
    }

    #[test]
    fn test_valid_prefixes() {
        let parser = ShexmlParser::new();
        let input = read_test_file("valid_prefix.shexml");

        let result = parser.parse(&input);

        assert!(result.is_ok(), "The parser failed to parse valid input.");

        let ast = result.unwrap();

        let prefix_declarations: Prefix1 = ast.prefixes.unwrap_or(vec![]);

        assert_eq!(prefix_declarations.len(), 3, "Expected 3 PREFIX declarations");

        let expected_prefixes = vec![
            (":", "http://example.com/"),
            ("dbr:", "http://dbpedia.org/resource/"),
            ("schema:", "http://schema.org/"),
        ];

        for (i, (expected_id, expected_uri)) in expected_prefixes.iter().enumerate() {
            let prefix = prefix_declarations.get(i).unwrap();
            assert_eq!(&prefix.identifier, expected_id, "Mismatch in prefix identifier");
            assert_eq!(&prefix.uri, expected_uri, "Mismatch in prefix URI");
        }
    }


    #[test]
    fn test_invalid_prefix() {
        let parser = ShexmlParser::new();
        let input = read_test_file("invalid_prefix.shexml");

        let result = parser.parse(&input);

        assert!(result.is_err(), "The parser should fail on invalid input.");
    }

    #[test]
    fn test_valid_source() {
        let parser = ShexmlParser::new();
        let input = read_test_file("valid_source.shexml");

        let result = parser.parse(&input);

        assert!(result.is_ok(), "The parser failed to parse valid input.");

        let ast = result.unwrap();

        let sources: Source1 = ast.sources.unwrap_or(vec![]);

        assert_eq!(sources.len(), 3, "Expected 3 PREFIX declarations");

        let expected_prefixes = vec![
            ("json_file", "file:///example/path/to/file/file.json"),
            ("json_relative_path", "file.json"),
            ("xml_file", "https://example.com/file.xml"),
        ];

        for (i, (expected_id, expected_uri)) in expected_prefixes.iter().enumerate() {
            let source = sources.get(i).unwrap();
            assert_eq!(&source.identifier, expected_id, "Mismatch in source identifier");
            assert_eq!(&source.path, expected_uri, "Mismatch in source path");
        }
    }

    #[test]
    fn test_invalid_source() {
        let parser = ShexmlParser::new();
        let input = read_test_file("invalid_source.shexml");

        let result = parser.parse(&input);

        assert!(result.is_err(), "The parser should fail on invalid input.");
    }
}