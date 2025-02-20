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
    Iterator(Iterator),
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

pub type Attribute1 = Vec<Attribute>;

pub type Declaration0 = Option<Declaration1>;

pub type Declaration1 = Vec<Declaration>;

pub type Identifier = String;

pub type Iterator0 = Option<Iterator1>;

pub type Iterator1 = Vec<Box<Iterator>>;

pub type Namespace = String;

pub type NamespaceOpt = Option<Namespace>;

pub type Nestedterator0 = Option<Nestedterator1>;

pub type Nestedterator1 = Vec<Nestedterator>;

pub type Path = String;

pub type Path1 = Vec<Path>;

pub type PathLiteral = String;

pub type PredicateObject0 = Option<PredicateObject1>;

pub type PredicateObject1 = Vec<PredicateObject>;

pub type Shape0 = Option<Shape1>;

pub type Shape1 = Vec<Shape>;

pub type ShapePath = String;

pub type Uri = String;