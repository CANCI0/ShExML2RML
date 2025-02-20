use crate::parser::ast::*;

pub trait Visitor<T> {
    fn visit_shexml(&mut self, n: &Shexml) -> T;
    fn visit_declaration(&mut self, n: &Declaration) -> T;
    fn visit_prefix(&mut self, n: &Prefix) -> T;
    fn visit_source(&mut self, n: &Source) -> T;
    fn visit_expression(&mut self, n: &Expression) -> T;
    fn visit_iterator(&mut self, n: &Iterator) -> T;
    fn visit_nested_iterator(&mut self, n: &NestedIterator) -> T;
    fn visit_attribute(&mut self, n: &Attribute) -> T;
    fn visit_shape(&mut self, n: &Shape) -> T;
    fn visit_subject(&mut self, n: &Subject) -> T;
    fn visit_class(&mut self, n: &Class) -> T;
    fn visit_subject_identifier(&mut self, n: &SubjectIdentifier) -> T;
    fn visit_predicate_object(&mut self, n: &PredicateObject) -> T;
    fn visit_predicate(&mut self, n: &Predicate) -> T;
    fn visit_object(&mut self, n: &Object) -> T;
    fn visit_data_value(&mut self, n: &DataValue) -> T;
    fn visit_reference(&mut self, n: &Reference) -> T;
}

pub struct RmlSerializer {
    pub output: String,
}

impl RmlSerializer {
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }
}

impl Visitor<String> for RmlSerializer {
    fn visit_shexml(&mut self, n: &Shexml) -> String {
        let mut result = String::new();
        let declarations = &n.declarations.clone().unwrap();
        let shapes = &n.shapes.clone().unwrap();

        for decl in declarations {
            result.push_str(&self.visit_declaration(decl));
        }

        for shape in shapes {
            result.push_str(&self.visit_shape(shape));
        }
        result
    }

    fn visit_declaration(&mut self, n: &Declaration) -> String {
        match n {
            Declaration::Prefix(p) => self.visit_prefix(p),
            Declaration::Source(s) => self.visit_source(s),
            Declaration::Expression(e) => self.visit_expression(e),
            Declaration::Iterator(i) => self.visit_iterator(i),
        }
    }

    fn visit_prefix(&mut self, n: &Prefix) -> String {
        format!("@prefix {}: <{}> .\n", n.identifier, n.uri)
    }

    fn visit_source(&mut self, n: &Source) -> String {
        format!("<{}> a rml:Source ; rml:source \"{}\" .\n", n.identifier, n.path)
    }

    fn visit_expression(&mut self, n: &Expression) -> String {
        format!("<{}> a rml:ExpressionMap ; rml:referenceFormulation \"{}\" .\n", n.identifier, n.paths.join(","))
    }

    fn visit_iterator(&mut self, n: &Iterator) -> String {
        let mut result = format!("<{}> a rml:LogicalSource ; rml:iterator \"{}\" .\n", n.identifier, n.path);
        let nested_iterators = &n.iterators.clone().unwrap();

        for field in &n.fields {
            result.push_str(&self.visit_attribute(field));
        }
        for nested in nested_iterators {
            result.push_str(&self.visit_nested_iterator(nested));
        }
        result
    }

    fn visit_nested_iterator(&mut self, n: &NestedIterator) -> String {
        format!("Nested iterator: {} -> {}\n", n.identifier, n.path)
    }

    fn visit_attribute(&mut self, n: &Attribute) -> String {
        format!("Attribute: {} -> {}\n", n.identifier, n.path)
    }

    fn visit_shape(&mut self, n: &Shape) -> String {
        let mut i = 1;
        let mut result = format!(
            "map:m_{i}                     rr:TriplesMap ;
                    rml:logicalSource      map:l_260963552 ;
                    rr:predicateObjectMap  map:po_21 , map:po_20 ;
                    rr:subjectMap          map:s_5 .", );
        let predicate_objects = &n.predicate_objects.clone().unwrap();

        for po in predicate_objects {
            result.push_str(&self.visit_predicate_object(po));
        }
        result.push_str(".\n");
        result
    }

    fn visit_subject(&mut self, n: &Subject) -> String {
        self.visit_class(&n.class)
    }

    fn visit_class(&mut self, n: &Class) -> String {
        format!("{}:{}", n.namespace, n.identifier)
    }

    fn visit_subject_identifier(&mut self, n: &SubjectIdentifier) -> String {
        match &n.prefix {
            Some(prefix) => format!("{}:{}", prefix, n.subject_generator),
            None => n.subject_generator.clone(),
        }
    }

    fn visit_predicate_object(&mut self, n: &PredicateObject) -> String {
        let mut result = format!("  rml:predicateObjectMap [ rml:predicate {}:{} ] ;\n", n.predicate.namespace, n.predicate.identifier);
        result.push_str(&self.visit_object(&n.object));
        result
    }

    fn visit_predicate(&mut self, n: &Predicate) -> String {
        format!("{}:{}", n.namespace, n.identifier)
    }

    fn visit_object(&mut self, n: &Object) -> String {
        match n {
            Object::DataValue(dv) => self.visit_data_value(dv),
            Object::Reference(r) => self.visit_reference(r),
        }
    }

    fn visit_data_value(&mut self, n: &DataValue) -> String {
        format!("  rml:objectMap [ rml:template \"{}\" ] ;\n", n.shape_path)
    }

    fn visit_reference(&mut self, n: &Reference) -> String {
        format!("  rml:objectMap [ rml:parentTriplesMap \"{}\" ] ;\n", n.identifier)
    }
}