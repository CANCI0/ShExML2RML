#[cfg(test)]
mod tests {
    use rust_shexml_interpreter::serializer::rml_classes::{ObjectMap, TermType, TriplesMap};

    #[test]
    fn test_object_map_display_template() {
        let input = ObjectMap {
            template: Some("http://dbpedia.org/resource/{year}".parse().unwrap()),
            term_type: Option::from(TermType::IRI),
            parent_triples_map: None,
        };

        let expected_output = r#"
rr:objectMap     [ a                    rr:ObjectMap ;
                   rr:template          "http://dbpedia.org/resource/{year}" ;
                   rr:termType          rr:IRI ;
                 ] ;
"#.trim();

        assert_eq!(format!("{}", input), expected_output);
    }

    #[test]
    fn test_object_map_display_parent_triples_map() {
        let input = ObjectMap {
            template: None,
            term_type: None,
            parent_triples_map: Option::from(TriplesMap::new()),
        };

        let expected_output = r#"
rr:objectMap     [ a                    rr:ObjectMap ;
                   rr:parentTriplesMap  map:m_2 ;
                 ] ;
"#.trim();

        assert_eq!(format!("{}", input), expected_output);
    }
}

