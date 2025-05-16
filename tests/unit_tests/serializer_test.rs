#[cfg(test)]
mod unit_tests {
    use indoc::indoc;
    use shexml2rml::serializer::rml_classes::{
        LogicalSource, ObjectMap, PredicateMap, PredicateObjectMap, ReferenceFormulation,
        SubjectMap, TermType, TriplesMap,
    };

    #[test]
    fn test_logical_source_display() {
        let input = LogicalSource::new(
            String::from("//film/cast/actress"),
            ReferenceFormulation::XPath,
            String::from("https://shexml.herminiogarcia.com/files/films.xml"),
        );

        let expected_output = indoc! {"
            map:ls_1  a                rml:LogicalSource ;
                    rml:iterator              \"//film/cast/actress\" ;
                    rml:referenceFormulation  ql:XPath ;
                    rml:source                \"https://shexml.herminiogarcia.com/files/films.xml\" .
        "};

        assert_eq!(format!("{}", input), expected_output);
    }

    #[test]
    fn test_object_map_display_template() {
        let input = ObjectMap::new(
            Some(String::from("http://dbpedia.org/resource/{year}")),
            Some(TermType::IRI),
            None,
        );

        let expected_output = indoc! {"
            map:o_1  a          rr:ObjectMap ;
                    rr:template  \"http://dbpedia.org/resource/{year}\" ;
                    rr:termType  rr:IRI .
        "};

        assert_eq!(format!("{}", input), expected_output);
    }

    #[test]
    fn test_predicate_map_display() {
        let mut input = PredicateMap::new(String::from(":year"));
        input.set_id(String::from("p_2"));

        let expected_output = indoc! {"
            map:p_2  a           rr:predicateMap ;
                    rr:constant  :year ."};

        assert_eq!(format!("{}", input), expected_output);
    }

    #[test]
    fn test_predicate_object_map_display() {
        let input = PredicateObjectMap::new(
            ObjectMap::new(
                Some(String::from("http://dbpedia.org/resource/{year}")),
                None,
                None,
            ),
            PredicateMap::new(String::from("schema:name")),
        );

        let expected_output = indoc! {"
            map:po_1  a              rr:PredicateObjectMap ;
                    rr:objectMap     map:o_2 ;
                    rr:predicateMap  map:p_2 .

            map:o_1  a          rr:ObjectMap ;
                    rr:template  \"http://dbpedia.org/resource/{year}\" .

            map:p_1  a           rr:predicateMap ;
                    rr:constant  schema:name .
        "};

        assert_eq!(format!("{}", input), expected_output);
    }

    #[test]
    fn test_object_map_display_parent_triples_map() {
        let input = ObjectMap {
            id: String::from("o_22"),
            template: None,
            term_type: None,
            parent_triples_map: Some(TriplesMap {
                id: String::from("m_3"),
                logical_source: LogicalSource {
                    id: String::from("ls_1"),
                    iterator: String::from("//film"),
                    reference_formulation: ReferenceFormulation::XPath,
                    source: String::from("http://shexml.herminiogarcia.com/files/films.xml"),
                },
                predicate_object_maps: vec![PredicateObjectMap {
                    id: String::from("po_1"),
                    object_map: ObjectMap {
                        id: String::from("o_1"),
                        template: Some(String::from("name")),
                        term_type: Some(TermType::IRI),
                        parent_triples_map: None,
                    },
                    predicate_map: PredicateMap {
                        id: String::from("p_1"),
                        constant: String::from("schema:name"),
                    },
                }],
                subject_map: SubjectMap {
                    id: String::from("s_1"),
                    template: String::from(":film_xml"),
                },
            }),
        };

        let expected_output = indoc! {"
            map:o_22  a          rr:ObjectMap ;
                    rr:parentTriplesMap  map:m_3 .
        "};

        assert_eq!(format!("{}", input), expected_output);
    }


    #[test]
    fn test_subject_map_display() {
        let input = SubjectMap::new(String::from("http://example.com/{id}"));

        let expected_output = indoc! {"
            map:s_1  a           rr:SubjectMap ;
                    rr:template  \"http://example.com/{id}\" .
        "};

        assert_eq!(format!("{}", input), expected_output);
    }

    #[test]
    fn test_triples_map_display() {
        let input = TriplesMap {
            id: String::from("m_1"),
            logical_source: LogicalSource {
                id: String::from("ls_1"),
                iterator: String::from("//film"),
                reference_formulation: ReferenceFormulation::XPath,
                source: String::from("http://shexml.herminiogarcia.com/files/films.xml"),
            },
            predicate_object_maps: vec![PredicateObjectMap {
                id: String::from("po_1"),
                object_map: ObjectMap {
                    id: String::from("o_1"),
                    template: Some(String::from("name")),
                    term_type: Some(TermType::IRI),
                    parent_triples_map: None,
                },
                predicate_map: PredicateMap {
                    id: String::from("p_1"),
                    constant: String::from("schema:name"),
                },
            }],
            subject_map: SubjectMap {
                id: String::from("s_1"),
                template: String::from(":film_xml"),
            },
        };

        let expected_output = indoc! {"

            map:m_1  a rr:TriplesMap ;
                rml:logicalSource      map:ls_1 ;
                rr:predicateObjectMap  map:po_1 ;
                rr:subjectMap          map:s_1 .

            map:ls_1  a                rml:LogicalSource ;
                    rml:iterator              \"//film\" ;
                    rml:referenceFormulation  ql:XPath ;
                    rml:source                \"http://shexml.herminiogarcia.com/files/films.xml\" .

            map:s_1  a           rr:SubjectMap ;
                    rr:template  \":film_xml\" .

            map:po_1  a              rr:PredicateObjectMap ;
                    rr:objectMap     map:o_1 ;
                    rr:predicateMap  map:p_1 .

            map:o_1  a          rr:ObjectMap ;
                    rr:template  \"name\" ;
                    rr:termType  rr:IRI .

            map:p_1  a           rr:predicateMap ;
                    rr:constant  schema:name .
        "};

        assert_eq!(format!("{}", input), expected_output);
    }
}
