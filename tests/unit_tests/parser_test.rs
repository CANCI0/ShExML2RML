#[cfg(test)]
mod unit_tests {
    use shexml2rml::parser::shexml::ShexmlParser;
    use shexml2rml::parser::shexml_actions::Object::{DataValue, Reference};
    use shexml2rml::parser::shexml_actions::{
        Attribute, Attribute1, Class, DataValue as OtherDataValue, Expression, Iterator1,
        IteratorFileRelation, Iterator_, NestedIterator, Predicate, PredicateObject, Prefix1,
        Reference as OtherReference, Shape, Shape1, Source1, Subject, SubjectIdentifier,
    };
    use rustemo::Parser;
    use std::fs;
    use std::iter::Iterator;
    use std::path::Path;

    fn read_test_file(file_name: &str) -> String {
        let path = Path::new("./resources/").join(file_name);
        fs::read_to_string(path).expect(&format!("Failed to read path: {}", file_name))
    }

    #[test]
    fn test_valid_prefixes() {
        let parser = ShexmlParser::new();
        let input = read_test_file("valid_prefix.shexml");

        let result = parser.parse(&input);

        assert!(result.is_ok(), "The parser failed to parse valid input.");

        let ast = result.unwrap();

        let prefix_declarations: Prefix1 = ast.prefixes.unwrap_or(vec![]);

        assert_eq!(
            prefix_declarations.len(),
            3,
            "Expected 3 PREFIX declarations"
        );

        let expected_prefixes = vec![
            (":", "http://example.com/"),
            ("dbr:", "http://dbpedia.org/resource/"),
            ("schema:", "http://schema.org/"),
        ];

        for (i, (expected_id, expected_uri)) in expected_prefixes.iter().enumerate() {
            let prefix = prefix_declarations.get(i).unwrap();
            assert_eq!(
                &prefix.identifier, expected_id,
                "Mismatch in prefix identifier"
            );
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

        assert_eq!(sources.len(), 2, "Expected 2 PREFIX declarations");

        let expected_prefixes = vec![
            ("json_file", "http://example.com/file.json"),
            ("xml_file", "https://example.com/file.xml"),
        ];

        for (i, (expected_id, expected_uri)) in expected_prefixes.iter().enumerate() {
            let source = sources.get(i).unwrap();
            assert_eq!(
                &source.identifier, expected_id,
                "Mismatch in source identifier"
            );
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

    #[test]
    fn test_valid_iterators() {
        let parser = ShexmlParser::new();
        let input = read_test_file("valid_iterators.shexml");

        let result = parser.parse(&input);

        assert!(result.is_ok(), "The parser failed to parse valid input.");

        let ast = result.unwrap();

        let iterators: Iterator1 = ast.iterators.unwrap_or(vec![]);

        assert_eq!(iterators.len(), 2, "Expected 2 ITERATORS declarations");

        let expected_iterators = vec![
            Iterator_ {
                identifier: String::from("film_xml"),
                path_type: String::from("xpath:"),
                path: String::from("//film"),
                fields: Attribute1::from([
                    Attribute {
                        identifier: String::from("id"),
                        path: String::from("@id"),
                    },
                    Attribute {
                        identifier: String::from("name"),
                        path: String::from("name"),
                    },
                    Attribute {
                        identifier: String::from("year"),
                        path: String::from("year"),
                    },
                    Attribute {
                        identifier: String::from("country"),
                        path: String::from("country"),
                    },
                    Attribute {
                        identifier: String::from("directors"),
                        path: String::from("crew/directors/director"),
                    },
                    Attribute {
                        identifier: String::from("screenwritters"),
                        path: String::from("crew//screenwritter"),
                    },
                    Attribute {
                        identifier: String::from("music"),
                        path: String::from("crew/music"),
                    },
                    Attribute {
                        identifier: String::from("photography"),
                        path: String::from("crew/photography"),
                    },
                ]),
                iterators: Some(vec![
                    NestedIterator {
                        identifier: String::from("actors"),
                        path: String::from("cast/actor"),
                        fields: Attribute1::from([
                            Attribute {
                                identifier: String::from("name"),
                                path: String::from("name"),
                            },
                            Attribute {
                                identifier: String::from("role"),
                                path: String::from("role"),
                            },
                            Attribute {
                                identifier: String::from("film"),
                                path: String::from("../../@id"),
                            },
                        ]),
                        iterators: Box::new(None),
                    },
                    NestedIterator {
                        identifier: String::from("actresses"),
                        path: String::from("cast/actress"),
                        fields: Attribute1::from([
                            Attribute {
                                identifier: String::from("name"),
                                path: String::from("name"),
                            },
                            Attribute {
                                identifier: String::from("role"),
                                path: String::from("role"),
                            },
                            Attribute {
                                identifier: String::from("film"),
                                path: String::from("../../@id"),
                            },
                        ]),
                        iterators: Box::new(None),
                    },
                ]),
            },
            Iterator_ {
                identifier: String::from("film_json"),
                path_type: String::from("jsonpath:"),
                path: String::from("$.films[*]"),
                fields: Attribute1::from([
                    Attribute {
                        identifier: String::from("id"),
                        path: String::from("id"),
                    },
                    Attribute {
                        identifier: String::from("name"),
                        path: String::from("name"),
                    },
                    Attribute {
                        identifier: String::from("year"),
                        path: String::from("year"),
                    },
                    Attribute {
                        identifier: String::from("country"),
                        path: String::from("country"),
                    },
                    Attribute {
                        identifier: String::from("directors"),
                        path: String::from("crew.director"),
                    },
                    Attribute {
                        identifier: String::from("screenwritters"),
                        path: String::from("crew.screenwritter"),
                    },
                    Attribute {
                        identifier: String::from("music"),
                        path: String::from("crew.music"),
                    },
                    Attribute {
                        identifier: String::from("photography"),
                        path: String::from("crew.cinematography"),
                    },
                ]),
                iterators: Some(vec![NestedIterator {
                    identifier: String::from("actors"),
                    path: String::from("cast[*]"),
                    fields: Attribute1::from([
                        Attribute {
                            identifier: String::from("name"),
                            path: String::from("name"),
                        },
                        Attribute {
                            identifier: String::from("role"),
                            path: String::from("role"),
                        },
                    ]),
                    iterators: Box::new(None),
                }]),
            },
        ];

        for (iterator, expected) in iterators.iter().zip(expected_iterators.iter()) {
            assert_eq!(expected, iterator);
        }
    }

    #[test]
    fn test_valid_expressions() {
        let parser = ShexmlParser::new();
        let input = read_test_file("valid_expressions.shexml");

        let result = parser.parse(&input);
        assert!(result.is_ok(), "The parser failed to parse valid input.");

        let ast = result.unwrap();
        let expressions: Vec<Expression> = ast.expressions.unwrap_or_default();

        assert_eq!(expressions.len(), 3, "Expected 3 EXPRESSION declarations");

        let expected_expressions = vec![
            Expression {
                identifier: String::from("exp1"),
                paths: vec![IteratorFileRelation {
                    file: String::from("file"),
                    iterator: String::from("example"),
                }],
            },
            Expression {
                identifier: String::from("exp2"),
                paths: vec![
                    IteratorFileRelation {
                        file: String::from("file"),
                        iterator: String::from("it1"),
                    },
                    IteratorFileRelation {
                        file: String::from("file"),
                        iterator: String::from("it2"),
                    },
                ],
            },
            Expression {
                identifier: String::from("exp3"),
                paths: vec![
                    IteratorFileRelation {
                        file: String::from("file"),
                        iterator: String::from("it1"),
                    },
                    IteratorFileRelation {
                        file: String::from("file"),
                        iterator: String::from("it2"),
                    },
                    IteratorFileRelation {
                        file: String::from("file"),
                        iterator: String::from("it3"),
                    },
                ],
            },
        ];

        for (i, expected) in expected_expressions.iter().enumerate() {
            let actual = expressions
                .get(i)
                .expect("Missing expression in parsed output");

            assert_eq!(
                actual.identifier, expected.identifier,
                "Mismatch in expression identifier"
            );

            assert_eq!(
                actual.paths.len(),
                expected.paths.len(),
                "Mismatch in number of paths for expression {}",
                expected.identifier
            );

            for (j, expected_path) in expected.paths.iter().enumerate() {
                let actual_path = actual
                    .paths
                    .get(j)
                    .expect("Missing path in parsed expression");
                assert_eq!(
                    actual_path.file, expected_path.file,
                    "Mismatch in file name for path {} in expression {}",
                    j, expected.identifier
                );
                assert_eq!(
                    actual_path.iterator, expected_path.iterator,
                    "Mismatch in iterator for path {} in expression {}",
                    j, expected.identifier
                );
            }
        }
    }

    #[test]
    fn test_valid_shapes() {
        let parser = ShexmlParser::new();
        let input = read_test_file("valid_shapes.shexml");

        let result = parser.parse(&input);

        assert!(result.is_ok(), "The parser failed to parse valid input.");

        let ast = result.unwrap();

        let shapes: Shape1 = ast.shapes.unwrap_or(vec![]);

        assert_eq!(shapes.len(), 3, "Expected 3 SHAPES declarations");

        let expected_shapes = vec![
            Shape {
                subject: Subject {
                    class: Class {
                        namespace: String::from(":"),
                        identifier: String::from("Films"),
                    },
                    subject_identifier: SubjectIdentifier {
                        prefix: Some(String::from(":")),
                        subject_generator: String::from("films.id"),
                    },
                },
                predicate_objects: Some(vec![
                    PredicateObject {
                        predicate: Predicate {
                            namespace: String::from("schema:"),
                            identifier: String::from("name"),
                        },
                        object: DataValue(OtherDataValue {
                            namespace: None,
                            shape_path: String::from("films.name"),
                        }),
                    },
                    PredicateObject {
                        predicate: Predicate {
                            namespace: String::from(":"),
                            identifier: String::from("year"),
                        },
                        object: DataValue(OtherDataValue {
                            namespace: Some(String::from("dbr:")),
                            shape_path: String::from("films.year"),
                        }),
                    },
                    PredicateObject {
                        predicate: Predicate {
                            namespace: String::from("schema:"),
                            identifier: String::from("countryOfOrigin"),
                        },
                        object: DataValue(OtherDataValue {
                            namespace: Some(String::from("dbr:")),
                            shape_path: String::from("films.country"),
                        }),
                    },
                    PredicateObject {
                        predicate: Predicate {
                            namespace: String::from("schema:"),
                            identifier: String::from("director"),
                        },
                        object: DataValue(OtherDataValue {
                            namespace: Some(String::from("dbr:")),
                            shape_path: String::from("films.directors"),
                        }),
                    },
                    PredicateObject {
                        predicate: Predicate {
                            namespace: String::from(":"),
                            identifier: String::from("screenwritter"),
                        },
                        object: DataValue(OtherDataValue {
                            namespace: Some(String::from("dbr:")),
                            shape_path: String::from("films.screenwritters"),
                        }),
                    },
                    PredicateObject {
                        predicate: Predicate {
                            namespace: String::from("schema:"),
                            identifier: String::from("musicBy"),
                        },
                        object: DataValue(OtherDataValue {
                            namespace: Some(String::from("dbr:")),
                            shape_path: String::from("films.music"),
                        }),
                    },
                    PredicateObject {
                        predicate: Predicate {
                            namespace: String::from(":"),
                            identifier: String::from("cinematographer"),
                        },
                        object: DataValue(OtherDataValue {
                            namespace: Some(String::from("dbr:")),
                            shape_path: String::from("films.photography"),
                        }),
                    },
                    PredicateObject {
                        predicate: Predicate {
                            namespace: String::from("schema:"),
                            identifier: String::from("actor"),
                        },
                        object: Reference(OtherReference {
                            identifier: String::from("Actor"),
                        }),
                    },
                    PredicateObject {
                        predicate: Predicate {
                            namespace: String::from("schema:"),
                            identifier: String::from("actor"),
                        },
                        object: Reference(OtherReference {
                            identifier: String::from("Actress"),
                        }),
                    },
                ]),
            },
            Shape {
                subject: Subject {
                    class: Class {
                        namespace: String::from(":"),
                        identifier: String::from("Actor"),
                    },
                    subject_identifier: SubjectIdentifier {
                        prefix: Some(String::from("dbr:")),
                        subject_generator: String::from("films.actors.name"),
                    },
                },
                predicate_objects: Some(vec![
                    PredicateObject {
                        predicate: Predicate {
                            namespace: String::from(":"),
                            identifier: String::from("name"),
                        },
                        object: DataValue(OtherDataValue {
                            namespace: None,
                            shape_path: String::from("films.actors.name"),
                        }),
                    },
                    PredicateObject {
                        predicate: Predicate {
                            namespace: String::from(":"),
                            identifier: String::from("appear_on"),
                        },
                        object: DataValue(OtherDataValue {
                            namespace: Some(String::from(":")),
                            shape_path: String::from("films.actors.film"),
                        }),
                    },
                ]),
            },
            Shape {
                subject: Subject {
                    class: Class {
                        namespace: String::from(":"),
                        identifier: String::from("Actress"),
                    },
                    subject_identifier: SubjectIdentifier {
                        prefix: Some(String::from("dbr:")),
                        subject_generator: String::from("films.actresses.name"),
                    },
                },
                predicate_objects: Some(vec![
                    PredicateObject {
                        predicate: Predicate {
                            namespace: String::from(":"),
                            identifier: String::from("name"),
                        },
                        object: DataValue(OtherDataValue {
                            namespace: None,
                            shape_path: String::from("films.actresses.name"),
                        }),
                    },
                    PredicateObject {
                        predicate: Predicate {
                            namespace: String::from(":"),
                            identifier: String::from("appear_on"),
                        },
                        object: DataValue(OtherDataValue {
                            namespace: Some(String::from(":")),
                            shape_path: String::from("films.actresses.film"),
                        }),
                    },
                ]),
            },
        ];

        for (shape, expected) in shapes.iter().zip(expected_shapes.iter()) {
            assert_eq!(expected, shape);
        }
    }
}
