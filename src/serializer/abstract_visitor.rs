use crate::parser::ast::*;

pub trait Visitor<T> where T: Default{
    fn visit_shexml(&mut self, n: &Shexml) -> T {
        if let Some(declarations) = &n.declarations {
            for decl in declarations {
                self.visit_declaration(decl);
            }
        }
        if let Some(shapes) = &n.shapes {
            for shape in shapes {
                self.visit_shape(shape);
            }
        }
        T::default()
    }

    fn visit_declaration(&mut self, n: &Declaration) -> T {
        match n {
            Declaration::Prefix(p) => self.visit_prefix(p),
            Declaration::Source(s) => self.visit_source(s),
            Declaration::Expression(e) => self.visit_expression(e),
            Declaration::Iterator(i) => self.visit_iterator(i),
        }
    }

    fn visit_prefix(&mut self, _: &Prefix) -> T {
        T::default()
    }

    fn visit_source(&mut self, _: &Source) -> T {
        T::default()
    }

    fn visit_expression(&mut self, _: &Expression) -> T {
        T::default()
    }

    fn visit_iterator(&mut self, n: &Iterator) -> T {
        for field in &n.fields {
            self.visit_attribute(field);
        }
        if let Some(nested) = &n.iterators {
            for nested_iterator in nested {
                self.visit_nested_iterator(nested_iterator);
            }
        }
        T::default()
    }

    fn visit_nested_iterator(&mut self, n: &NestedIterator) -> T {
        for field in &n.fields {
            self.visit_attribute(field);
        }
        if let Some(iterators) = &n.iterators {
            for it in iterators {
                self.visit_iterator(it);
            }
        }
        T::default()
    }

    fn visit_attribute(&mut self, _: &Attribute) -> T {
        T::default()
    }

    fn visit_shape(&mut self, n: &Shape) -> T {
        self.visit_subject(&n.subject);
        if let Some(predicate_objects) = &n.predicate_objects {
            for po in predicate_objects {
                self.visit_predicate_object(po);
            }
        }
        T::default()
    }

    fn visit_subject(&mut self, n: &Subject) -> T {
        self.visit_class(&n.class);
        self.visit_subject_identifier(&n.subject_identifier);
        T::default()
    }

    fn visit_class(&mut self, _: &Class) -> T {
        T::default()
    }

    fn visit_subject_identifier(&mut self, _: &SubjectIdentifier) -> T {
        T::default()
    }

    fn visit_predicate_object(&mut self, n: &PredicateObject) -> T {
        self.visit_predicate(&n.predicate);
        self.visit_object(&n.object);
        T::default()
    }

    fn visit_predicate(&mut self, _: &Predicate) -> T {
        T::default()
    }

    fn visit_object(&mut self, n: &Object) -> T {
        match n {
            Object::DataValue(dv) => self.visit_data_value(dv),
            Object::Reference(r) => self.visit_reference(r),
        }
    }

    fn visit_data_value(&mut self, _: &DataValue) -> T {
        T::default()
    }

    fn visit_reference(&mut self, _: &Reference) -> T {
        T::default()
    }
}