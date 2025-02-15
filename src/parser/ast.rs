
#[derive(Debug, Clone)]
pub struct Shexml {
    pub declarations: Vec<Declaration>,
    pub shapes: Vec<Shape>,
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Prefix(Prefix),
    Source(Source),
    Iterator_(Iterator_),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub struct Prefix {
    pub identifier: String,
    pub uri: String,
}

#[derive(Debug, Clone)]
pub struct Source {
    pub identifier: String,
    pub uri: String,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub identifier: String,
    pub union: Vec<IteratorQuery>,
}

#[derive(Debug, Clone)]
pub struct Iterator_ {
    pub identifier: String,
    pub query: String,
    pub fields: Vec<Field>,
    pub nested_iterators: Vec<Iterator_>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub identifier: String,
    pub query: String,
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub shape_type: String,
    pub prefix_identifier: String,
    pub iterator_query: IteratorQuery,
    pub predicate_objects: Vec<PredicateObject>,
}

#[derive(Debug, Clone)]
pub struct IteratorQuery {
    pub path: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum PredicateObject {
    Predicate(Predicate),
    ShapeLink(ShapeLink),
}

#[derive(Debug, Clone)]
pub struct Predicate {
    pub prefix_identifier: Option<String>,
    pub iterator_query: IteratorQuery,
}

#[derive(Debug, Clone)]
pub struct ShapeLink {
    pub prefix_identifier: String,
    pub identifier: String,
}
