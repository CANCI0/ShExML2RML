use crate::parser::shexml_actions::*;

pub trait Visitor<T>
where
    T: Default
{
    fn visit_shexml(&mut self, n: &Shexml, obj: &T) -> T {
        if let Some(prefixes) = &n.prefixes {
            for decl in prefixes {
                self.visit_prefix(decl, &obj);
            }
        }
        if let Some(sources) = &n.sources {
            for decl in sources {
                self.visit_source(decl, &obj);
            }
        }
        if let Some(iterators) = &n.iterators {
            for decl in iterators {
                self.visit_iterator(decl, &obj);
            }
        }
        if let Some(expressions) = &n.expressions {
            for decl in expressions {
                self.visit_expression(decl, &obj);
            }
        }
        if let Some(shapes) = &n.shapes {
            for decl in shapes {
                self.visit_shape(decl, &obj);
            }
        }

        T::default()
    }

    fn visit_prefix(&mut self, _: &Prefix, _: &T) -> T {
        T::default()
    }

    fn visit_source(&mut self, _: &Source, _: &T) -> T {
        T::default()
    }

    fn visit_expression(&mut self, n: &Expression, obj: &T) -> T {
        for a in &n.paths {
            self.visit_iterator_file_relation(a, &obj);
        }
        T::default()
    }

    fn visit_iterator(&mut self, n: &Iterator, obj: &T) -> T {
        for field in &n.fields {
            self.visit_attribute(field, &obj);
        }
        if let Some(nested) = &n.iterators {
            for nested_iterator in nested {
                self.visit_nested_iterator(nested_iterator, &obj);
            }
        }
        T::default()
    }

    fn visit_nested_iterator(&mut self, n: &NestedIterator, obj: &T) -> T {
        for field in &n.fields {
            self.visit_attribute(field, &obj);
        }
        if let Some(nested) = n.iterators.as_ref().as_ref() {
            for nested_iterator in nested {
                self.visit_nested_iterator(nested_iterator, &obj);
            }
        }
        T::default()
    }

    fn visit_attribute(&mut self, _: &Attribute, _: &T) -> T {
        T::default()
    }

    fn visit_shape(&mut self, n: &Shape, obj: &T) -> T {
        let mut result = self.visit_subject(&n.subject, obj);
        if let Some(predicate_objects) = &n.predicate_objects {
            for po in predicate_objects {
                result = self.visit_predicate_object(po, &result);
            }
        }
        T::default()
    }

    fn visit_subject(&mut self, n: &Subject, obj: &T) -> T {
        self.visit_class(&n.class, obj);
        self.visit_subject_identifier(&n.subject_identifier, &obj);
        T::default()
    }

    fn visit_class(&mut self, _: &Class, _: &T) -> T {
        T::default()
    }

    fn visit_subject_identifier(&mut self, _: &SubjectIdentifier, _: &T) -> T {
        T::default()
    }

    fn visit_predicate_object(&mut self, n: &PredicateObject, obj: &T) -> T {
        self.visit_predicate(&n.predicate, obj);
        self.visit_object(&n.object, &obj);
        T::default()
    }

    fn visit_predicate(&mut self, _: &Predicate, _: &T) -> T {
        T::default()
    }

    fn visit_object(&mut self, n: &Object, obj: &T) -> T {
        match n {
            Object::DataValue(dv) => self.visit_data_value(dv, obj),
            Object::Reference(r) => self.visit_reference(r, obj),
        };
        T::default()
    }

    fn visit_data_value(&mut self, _: &DataValue, _: &T) -> T {
        T::default()
    }

    fn visit_reference(&mut self, _: &Reference, _: &T) -> T {
        T::default()
    }

    fn visit_iterator_file_relation(&mut self, _: &IteratorFileRelation, _: &T) -> T {
        T::default()
    }
}
