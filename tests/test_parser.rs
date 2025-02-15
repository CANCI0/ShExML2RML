#[cfg(test)]
mod tests {
    use rust_shexml_interpreter::parser::syntax::parse_input;
    use std::fs;
    use std::path::Path;
    use rust_shexml_interpreter::parser::ast::*; 

    fn read_test_file(file_name: &str) -> String {
        let path = Path::new("resources").join(file_name);
        fs::read_to_string(path).expect(&format!("Failed to read {}", file_name))
    }

    #[test]
    fn test_valid_prefixes() {
        let input = read_test_file("valid_prefix.shexml");

        let result = parse_input(&input);

        assert!(result.is_ok(), "The parser failed to parse valid input.");

        let ast = result.unwrap();
        println!("Jarto: {:?}", ast.declarations);

        // Verificar que hay exactamente 3 declaraciones de prefijo
        let prefix_declarations: Vec<&Prefix> = ast
            .declarations
            .iter()
            .filter_map(|decl| {
                if let Declaration::Prefix(prefix) = decl {
                    Some(prefix)
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(prefix_declarations.len(), 3, "Expected 3 PREFIX declarations");

        let expected_prefixes = vec![
            ("", "http://example.com/"),
            ("dbr", "http://dbpedia.org/resource/"),
            ("schema", "http://schema.org/"),
        ];

        for (i, (expected_id, expected_uri)) in expected_prefixes.iter().enumerate() {
            let prefix = prefix_declarations.get(i).unwrap();
            assert_eq!(&prefix.identifier, expected_id, "Mismatch in prefix identifier");
            assert_eq!(&prefix.uri, expected_uri, "Mismatch in prefix URI");
        }
    }


    #[test]
    fn test_invalid_prefix() {
        let input = read_test_file("invalid_prefix.shexml");

        let result = parse_input(&input);

        assert!(result.is_err(), "The parser should fail on invalid input.");
    }

    #[test]
    fn test_valid_source() {
        let input = read_test_file("valid_source.shexml");

        let result = parse_input(&input);

        assert!(result.is_ok(), "The parser failed to parse valid input.");
    }

    #[test]
    fn test_invalid_source() {
        let input = read_test_file("invalid_source.shexml");

        let result = parse_input(&input);

        assert!(result.is_err(), "The parser should fail on invalid input.");
    }
}