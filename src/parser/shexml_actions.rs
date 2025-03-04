/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
use super::shexml::{TokenKind, Context};
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type Namespace = String;
pub fn namespace(_ctx: &Ctx, token: Token) -> Namespace {
    token.value.into()
}
pub type Identifier = String;
pub fn identifier(_ctx: &Ctx, token: Token) -> Identifier {
    token.value.into()
}
pub type PathLiteral = String;
pub fn path_literal(_ctx: &Ctx, token: Token) -> PathLiteral {
    token.value.into()
}
pub type Path = String;
pub fn path(_ctx: &Ctx, token: Token) -> Path {
    token.value.into()
}
pub type ShapePath = String;
pub fn shape_path(_ctx: &Ctx, token: Token) -> ShapePath {
    token.value.into()
}
pub type Uri = String;
pub fn uri(_ctx: &Ctx, token: Token) -> Uri {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct Shexml {
    pub prefixes: Prefix0,
    pub sources: Source0,
    pub iterators: Iterator0,
    pub expressions: Expression0,
    pub shapes: Shape0,
}
pub fn shexml_c1(
    _ctx: &Ctx,
    prefixes: Prefix0,
    sources: Source0,
    iterators: Iterator0,
    expressions: Expression0,
    shapes: Shape0,
) -> Shexml {
    Shexml {
        prefixes,
        sources,
        iterators,
        expressions,
        shapes,
    }
}
pub type Prefix1 = Vec<Prefix>;
pub fn prefix1_c1(_ctx: &Ctx, mut prefix1: Prefix1, prefix: Prefix) -> Prefix1 {
    prefix1.push(prefix);
    prefix1
}
pub fn prefix1_prefix(_ctx: &Ctx, prefix: Prefix) -> Prefix1 {
    vec![prefix]
}
pub type Prefix0 = Option<Prefix1>;
pub fn prefix0_prefix1(_ctx: &Ctx, prefix1: Prefix1) -> Prefix0 {
    Some(prefix1)
}
pub fn prefix0_empty(_ctx: &Ctx) -> Prefix0 {
    None
}
pub type Source1 = Vec<Source>;
pub fn source1_c1(_ctx: &Ctx, mut source1: Source1, source: Source) -> Source1 {
    source1.push(source);
    source1
}
pub fn source1_source(_ctx: &Ctx, source: Source) -> Source1 {
    vec![source]
}
pub type Source0 = Option<Source1>;
pub fn source0_source1(_ctx: &Ctx, source1: Source1) -> Source0 {
    Some(source1)
}
pub fn source0_empty(_ctx: &Ctx) -> Source0 {
    None
}
pub type Iterator1 = Vec<Iterator>;
pub fn iterator1_c1(
    _ctx: &Ctx,
    mut iterator1: Iterator1,
    iterator: Iterator,
) -> Iterator1 {
    iterator1.push(iterator);
    iterator1
}
pub fn iterator1_iterator(_ctx: &Ctx, iterator: Iterator) -> Iterator1 {
    vec![iterator]
}
pub type Iterator0 = Option<Iterator1>;
pub fn iterator0_iterator1(_ctx: &Ctx, iterator1: Iterator1) -> Iterator0 {
    Some(iterator1)
}
pub fn iterator0_empty(_ctx: &Ctx) -> Iterator0 {
    None
}
pub type Expression1 = Vec<Expression>;
pub fn expression1_c1(
    _ctx: &Ctx,
    mut expression1: Expression1,
    expression: Expression,
) -> Expression1 {
    expression1.push(expression);
    expression1
}
pub fn expression1_expression(_ctx: &Ctx, expression: Expression) -> Expression1 {
    vec![expression]
}
pub type Expression0 = Option<Expression1>;
pub fn expression0_expression1(_ctx: &Ctx, expression1: Expression1) -> Expression0 {
    Some(expression1)
}
pub fn expression0_empty(_ctx: &Ctx) -> Expression0 {
    None
}
pub type Shape1 = Vec<Shape>;
pub fn shape1_c1(_ctx: &Ctx, mut shape1: Shape1, shape: Shape) -> Shape1 {
    shape1.push(shape);
    shape1
}
pub fn shape1_shape(_ctx: &Ctx, shape: Shape) -> Shape1 {
    vec![shape]
}
pub type Shape0 = Option<Shape1>;
pub fn shape0_shape1(_ctx: &Ctx, shape1: Shape1) -> Shape0 {
    Some(shape1)
}
pub fn shape0_empty(_ctx: &Ctx) -> Shape0 {
    None
}
#[derive(Debug, Clone)]
pub struct Prefix {
    pub identifier: Namespace,
    pub uri: Uri,
}
pub fn prefix_c1(_ctx: &Ctx, identifier: Namespace, uri: Uri) -> Prefix {
    Prefix { identifier, uri }
}
#[derive(Debug, Clone)]
pub struct Source {
    pub identifier: Identifier,
    pub path: Uri,
}
pub fn source_c1(_ctx: &Ctx, identifier: Identifier, path: Uri) -> Source {
    Source { identifier, path }
}
#[derive(Debug, Clone)]
pub struct Expression {
    pub identifier: Identifier,
    pub paths: IteratorFileRelation1,
}
pub fn expression_c1(
    _ctx: &Ctx,
    identifier: Identifier,
    paths: IteratorFileRelation1,
) -> Expression {
    Expression { identifier, paths }
}
pub type IteratorFileRelation1 = Vec<IteratorFileRelation>;
pub fn iterator_file_relation1_c1(
    _ctx: &Ctx,
    mut iterator_file_relation1: IteratorFileRelation1,
    iterator_file_relation: IteratorFileRelation,
) -> IteratorFileRelation1 {
    iterator_file_relation1.push(iterator_file_relation);
    iterator_file_relation1
}
pub fn iterator_file_relation1_iterator_file_relation(
    _ctx: &Ctx,
    iterator_file_relation: IteratorFileRelation,
) -> IteratorFileRelation1 {
    vec![iterator_file_relation]
}
#[derive(Debug, Clone)]
pub struct IteratorFileRelation {
    pub file: Identifier,
    pub iterator: Identifier,
}
pub fn iterator_file_relation_c1(
    _ctx: &Ctx,
    file: Identifier,
    iterator: Identifier,
) -> IteratorFileRelation {
    IteratorFileRelation {
        file,
        iterator,
    }
}
#[derive(Debug, Clone)]
pub struct Iterator {
    pub identifier: Identifier,
    pub path_type: PathLiteral,
    pub path: Path,
    pub fields: Attribute1,
    pub iterators: NestedIterator0,
}
pub fn iterator_c1(
    _ctx: &Ctx,
    identifier: Identifier,
    path_type: PathLiteral,
    path: Path,
    fields: Attribute1,
    iterators: NestedIterator0,
) -> Iterator {
    Iterator {
        identifier,
        path_type,
        path,
        fields,
        iterators,
    }
}
pub type Attribute1 = Vec<Attribute>;
pub fn attribute1_c1(
    _ctx: &Ctx,
    mut attribute1: Attribute1,
    attribute: Attribute,
) -> Attribute1 {
    attribute1.push(attribute);
    attribute1
}
pub fn attribute1_attribute(_ctx: &Ctx, attribute: Attribute) -> Attribute1 {
    vec![attribute]
}
pub type NestedIterator1 = Vec<NestedIterator>;
pub fn nested_iterator1_c1(
    _ctx: &Ctx,
    mut nested_iterator1: NestedIterator1,
    nested_iterator: NestedIterator,
) -> NestedIterator1 {
    nested_iterator1.push(nested_iterator);
    nested_iterator1
}
pub fn nested_iterator1_nested_iterator(
    _ctx: &Ctx,
    nested_iterator: NestedIterator,
) -> NestedIterator1 {
    vec![nested_iterator]
}
pub type NestedIterator0 = Option<NestedIterator1>;
pub fn nested_iterator0_nested_iterator1(
    _ctx: &Ctx,
    nested_iterator1: NestedIterator1,
) -> NestedIterator0 {
    Some(nested_iterator1)
}
pub fn nested_iterator0_empty(_ctx: &Ctx) -> NestedIterator0 {
    None
}
#[derive(Debug, Clone)]
pub struct NestedIterator {
    pub identifier: Identifier,
    pub path: Path,
    pub fields: Attribute1,
    pub iterators: Box<NestedIterator0>,
}
pub fn nested_iterator_c1(
    _ctx: &Ctx,
    identifier: Identifier,
    path: Path,
    fields: Attribute1,
    iterators: NestedIterator0,
) -> NestedIterator {
    NestedIterator {
        identifier,
        path,
        fields,
        iterators: Box::new(iterators),
    }
}
#[derive(Debug, Clone)]
pub struct Attribute {
    pub identifier: Identifier,
    pub path: Path,
}
pub fn attribute_c1(_ctx: &Ctx, identifier: Identifier, path: Path) -> Attribute {
    Attribute { identifier, path }
}
#[derive(Debug, Clone)]
pub struct Shape {
    pub subject: Subject,
    pub predicate_objects: PredicateObject0,
}
pub fn shape_c1(
    _ctx: &Ctx,
    subject: Subject,
    predicate_objects: PredicateObject0,
) -> Shape {
    Shape {
        subject,
        predicate_objects,
    }
}
pub type PredicateObject1 = Vec<PredicateObject>;
pub fn predicate_object1_c1(
    _ctx: &Ctx,
    mut predicate_object1: PredicateObject1,
    predicate_object: PredicateObject,
) -> PredicateObject1 {
    predicate_object1.push(predicate_object);
    predicate_object1
}
pub fn predicate_object1_predicate_object(
    _ctx: &Ctx,
    predicate_object: PredicateObject,
) -> PredicateObject1 {
    vec![predicate_object]
}
pub type PredicateObject0 = Option<PredicateObject1>;
pub fn predicate_object0_predicate_object1(
    _ctx: &Ctx,
    predicate_object1: PredicateObject1,
) -> PredicateObject0 {
    Some(predicate_object1)
}
pub fn predicate_object0_empty(_ctx: &Ctx) -> PredicateObject0 {
    None
}
#[derive(Debug, Clone)]
pub struct Subject {
    pub class: Class,
    pub subject_identifier: SubjectIdentifier,
}
pub fn subject_c1(
    _ctx: &Ctx,
    class: Class,
    subject_identifier: SubjectIdentifier,
) -> Subject {
    Subject {
        class,
        subject_identifier,
    }
}
#[derive(Debug, Clone)]
pub struct Class {
    pub namespace: Namespace,
    pub identifier: Identifier,
}
pub fn class_c1(_ctx: &Ctx, namespace: Namespace, identifier: Identifier) -> Class {
    Class { namespace, identifier }
}
#[derive(Debug, Clone)]
pub struct SubjectIdentifier {
    pub prefix: NamespaceOpt,
    pub subject_generator: ShapePath,
}
pub fn subject_identifier_c1(
    _ctx: &Ctx,
    prefix: NamespaceOpt,
    subject_generator: ShapePath,
) -> SubjectIdentifier {
    SubjectIdentifier {
        prefix,
        subject_generator,
    }
}
pub type NamespaceOpt = Option<Namespace>;
pub fn namespace_opt_namespace(_ctx: &Ctx, namespace: Namespace) -> NamespaceOpt {
    Some(namespace)
}
pub fn namespace_opt_empty(_ctx: &Ctx) -> NamespaceOpt {
    None
}
#[derive(Debug, Clone)]
pub struct PredicateObject {
    pub predicate: Predicate,
    pub object: Object,
}
pub fn predicate_object_c1(
    _ctx: &Ctx,
    predicate: Predicate,
    object: Object,
) -> PredicateObject {
    PredicateObject {
        predicate,
        object,
    }
}
#[derive(Debug, Clone)]
pub struct Predicate {
    pub namespace: Namespace,
    pub identifier: Identifier,
}
pub fn predicate_c1(
    _ctx: &Ctx,
    namespace: Namespace,
    identifier: Identifier,
) -> Predicate {
    Predicate { namespace, identifier }
}
#[derive(Debug, Clone)]
pub enum Object {
    DataValue(DataValue),
    Reference(Reference),
}
pub fn object_data_value(_ctx: &Ctx, data_value: DataValue) -> Object {
    Object::DataValue(data_value)
}
pub fn object_reference(_ctx: &Ctx, reference: Reference) -> Object {
    Object::Reference(reference)
}
#[derive(Debug, Clone)]
pub struct DataValue {
    pub namespace: NamespaceOpt,
    pub shape_path: ShapePath,
}
pub fn data_value_c1(
    _ctx: &Ctx,
    namespace: NamespaceOpt,
    shape_path: ShapePath,
) -> DataValue {
    DataValue { namespace, shape_path }
}
#[derive(Debug, Clone)]
pub struct Reference {
    pub namespace: Namespace,
    pub identifier: Identifier,
}
pub fn reference_c1(
    _ctx: &Ctx,
    namespace: Namespace,
    identifier: Identifier,
) -> Reference {
    Reference { namespace, identifier }
}
