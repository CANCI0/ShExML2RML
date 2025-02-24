#[cfg(test)]
mod tests {
    use indoc::indoc;
    use rust_shexml_interpreter::serializer::rml_classes::{LogicalSource, ObjectMap, PredicateMap, PredicateObjectMap, SubjectMap, TriplesMap};

    #[test]
    fn test_logical_source_display() {
        let input = LogicalSource::new();

        let expected_output = indoc!{"
            map:l_260963552  a                rml:LogicalSource ;
                    rml:iterator              \"//film/cast/actress\" ;
                    rml:referenceFormulation  ql:XPath ;
                    rml:source                \"https://shexml.herminiogarcia.com/files/films.xml\" .
        "};

        assert_eq!(format!("{}", input), expected_output);
    }

    #[test]
    fn test_object_map_display_template() {
        let input = ObjectMap::new();

        let expected_output = indoc!{"
            map:o_11  a          rr:ObjectMap ;
                    rr:template  \"http://dbpedia.org/resource/{year}\" ;
                    rr:termType  rr:IRI .
        "};

        assert_eq!(format!("{}", input), expected_output);
    }

    #[test]
    fn test_predicate_map_display() {
        let input = PredicateMap::new();

        let expected_output = indoc!{"
            map:p_3  a           rr:predicateMap ;
                    rr:constant  :year .
        "};

        assert_eq!(format!("{}", input), expected_output);
    }

    #[test]
    fn test_predicate_object_map_display() {
        let input = PredicateObjectMap::new();

        let expected_output = indoc!{"
            map:po_4  a              rr:PredicateObjectMap ;
                    rr:objectMap     map:o_11 ;
                    rr:predicateMap  map:p_3 .
        "};

        assert_eq!(format!("{}", input), expected_output);
    }

    #[test]
    fn test_object_map_display_parent_triples_map() {
        let input = ObjectMap {
            id: String::from("o_22"),
            template: None,
            term_type: None,
            parent_triples_map: Option::from(TriplesMap::new()),
        };

        let expected_output = indoc!{"
            map:o_22  a          rr:ObjectMap ;
                    rr:parentTriplesMap  map:m_2 .
        "};

        assert_eq!(format!("{}", input), expected_output);
    }

    #[test]
    fn test_subject_map_display() {
        let input = SubjectMap::new();

        let expected_output = indoc!{"
            map:s_2  a           rr:SubjectMap ;
                    rr:template  \"http://example.com/{id}\" .
        "};

        assert_eq!(format!("{}", input), expected_output);
    }

    #[test]
    fn test_triples_map_display() {
        let mut input = TriplesMap::new();
        input.id = String::from("m_3");
        input.logical_source.id = String::from("l_260963552");

        let mut pom1 = PredicateObjectMap::new();
        pom1.id = String::from("po_21");
        let mut pom2 = PredicateObjectMap::new();
        pom2.id = String::from("po_20");

        input.predicate_object_maps.push(pom1);
        input.predicate_object_maps.push(pom2);

        let expected_output = indoc!{"
            map:m_3  a                     rr:TriplesMap ;
                    rml:logicalSource      map:l_260963552 ;
                    rr:predicateObjectMap  map:po_21 , map:po_20 ;
                    rr:subjectMap          map:s_2 .
        "};

        assert_eq!(format!("{}", input), expected_output);
    }
}

