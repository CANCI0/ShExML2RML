use crate::parser::shexml_actions::{Attribute1, Declaration0, Identifier, Iterator0, Namespace, NamespaceOpt, Nestedterator0, Path, Path1, PathLiteral, PredicateObject0, Shape0, ShapePath, Uri};

#[derive(Debug, Clone)]
pub struct Shexml {
    pub declarations: Declaration0,
    pub shapes: Shape0,
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Prefix(Prefix),
    Source(Source),
    Expression(Expression),
    Iterator(self::Iterator),
}

#[derive(Debug, Clone)]
pub struct Prefix {
    pub identifier: Namespace,
    pub uri: Uri,
}

#[derive(Debug, Clone)]
pub struct Source {
    pub identifier: Identifier,
    pub path: Uri,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub identifier: Identifier,
    pub paths: Path1,
}

#[derive(Debug, Clone)]
pub struct Iterator {
    pub identifier: Identifier,
    pub path_type: PathLiteral,
    pub path: Path,
    pub fields: Attribute1,
    pub iterators: Nestedterator0,
}

#[derive(Debug, Clone)]
pub struct Nestedterator {
    pub identifier: Identifier,
    pub path: Path,
    pub fields: Attribute1,
    pub iterators: Iterator0,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub identifier: Identifier,
    pub path: Path,
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub subject: Subject,
    pub predicate_objects: PredicateObject0,
}

#[derive(Debug, Clone)]
pub struct Subject {
    pub class: Class,
    pub subject_identifier: SubjectIdentifier,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub namespace: Namespace,
    pub identifier: Identifier,
}

#[derive(Debug, Clone)]
pub struct SubjectIdentifier {
    pub prefix: NamespaceOpt,
    pub subject_generator: ShapePath,
}

#[derive(Debug, Clone)]
pub struct PredicateObject {
    pub predicate: Predicate,
    pub object: Object,
}

#[derive(Debug, Clone)]
pub struct Predicate {
    pub namespace: Namespace,
    pub identifier: Identifier,
}

#[derive(Debug, Clone)]
pub enum Object {
    DataValue(DataValue),
    Reference(Reference),
}

#[derive(Debug, Clone)]
pub struct DataValue {
    pub namespace: NamespaceOpt,
    pub shape_path: ShapePath,
}

#[derive(Debug, Clone)]
pub struct Reference {
    pub identifier: Identifier,
}