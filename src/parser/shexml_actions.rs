/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
use crate::parser::ast::*;
use super::shexml::{TokenKind, Context};
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;

pub fn namespace(_ctx: &Ctx, token: Token) -> Namespace {
    token.value.into()
}

pub fn identifier(_ctx: &Ctx, token: Token) -> Identifier {
    token.value.into()
}

pub fn path_literal(_ctx: &Ctx, token: Token) -> PathLiteral {
    token.value.into()
}

pub fn path(_ctx: &Ctx, token: Token) -> Path {
    token.value.into()
}

pub fn shape_path(_ctx: &Ctx, token: Token) -> ShapePath {
    token.value.into()
}

pub fn uri(_ctx: &Ctx, token: Token) -> Uri {
    token.value.into()
}

pub fn shexml_c1(_ctx: &Ctx, declarations: Declaration0, shapes: Shape0) -> Shexml {
    Shexml { declarations, shapes }
}

pub fn declaration1_c1(
    _ctx: &Ctx,
    mut declaration1: Declaration1,
    declaration: Declaration,
) -> Declaration1 {
    declaration1.push(declaration);
    declaration1
}
pub fn declaration1_declaration(_ctx: &Ctx, declaration: Declaration) -> Declaration1 {
    vec![declaration]
}

pub fn declaration0_declaration1(
    _ctx: &Ctx,
    declaration1: Declaration1,
) -> Declaration0 {
    Some(declaration1)
}
pub fn declaration0_empty(_ctx: &Ctx) -> Declaration0 {
    None
}

pub fn shape1_c1(_ctx: &Ctx, mut shape1: Shape1, shape: Shape) -> Shape1 {
    shape1.push(shape);
    shape1
}
pub fn shape1_shape(_ctx: &Ctx, shape: Shape) -> Shape1 {
    vec![shape]
}

pub fn shape0_shape1(_ctx: &Ctx, shape1: Shape1) -> Shape0 {
    Some(shape1)
}
pub fn shape0_empty(_ctx: &Ctx) -> Shape0 {
    None
}

pub fn declaration_prefix(_ctx: &Ctx, prefix: Prefix) -> Declaration {
    Declaration::Prefix(prefix)
}
pub fn declaration_source(_ctx: &Ctx, source: Source) -> Declaration {
    Declaration::Source(source)
}
pub fn declaration_expression(_ctx: &Ctx, expression: Expression) -> Declaration {
    Declaration::Expression(expression)
}
pub fn declaration_iterator(_ctx: &Ctx, iterator: Iterator) -> Declaration {
    Declaration::Iterator(iterator)
}

pub fn prefix_c1(_ctx: &Ctx, identifier: Namespace, uri: Uri) -> Prefix {
    Prefix { identifier, uri }
}

pub fn source_c1(_ctx: &Ctx, identifier: Identifier, path: Uri) -> Source {
    Source { identifier, path }
}

pub fn expression_c1(_ctx: &Ctx, identifier: Identifier, paths: Path1) -> Expression {
    Expression { identifier, paths }
}

pub fn path1_c1(_ctx: &Ctx, mut path1: Path1, path: Path) -> Path1 {
    path1.push(path);
    path1
}
pub fn path1_path(_ctx: &Ctx, path: Path) -> Path1 {
    vec![path]
}

pub fn iterator_c1(
    _ctx: &Ctx,
    identifier: Identifier,
    path_type: PathLiteral,
    path: Path,
    fields: Attribute1,
    iterators: Nestedterator0,
) -> Iterator {
    Iterator {
        identifier,
        path_type,
        path,
        fields,
        iterators,
    }
}

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

pub fn nestedterator1_c1(
    _ctx: &Ctx,
    mut nestedterator1: Nestedterator1,
    nestedterator: Nestedterator,
) -> Nestedterator1 {
    nestedterator1.push(nestedterator);
    nestedterator1
}
pub fn nestedterator1_nestedterator(
    _ctx: &Ctx,
    nestedterator: Nestedterator,
) -> Nestedterator1 {
    vec![nestedterator]
}

pub fn nestedterator0_nestedterator1(
    _ctx: &Ctx,
    nestedterator1: Nestedterator1,
) -> Nestedterator0 {
    Some(nestedterator1)
}
pub fn nestedterator0_empty(_ctx: &Ctx) -> Nestedterator0 {
    None
}

pub fn nestedterator_c1(
    _ctx: &Ctx,
    identifier: Identifier,
    path: Path,
    fields: Attribute1,
    iterators: Iterator0,
) -> Nestedterator {
    Nestedterator {
        identifier,
        path,
        fields,
        iterators,
    }
}

pub fn iterator1_c1(
    _ctx: &Ctx,
    mut iterator1: Iterator1,
    iterator: Iterator,
) -> Iterator1 {
    iterator1.push(Box::new(iterator));
    iterator1
}
pub fn iterator1_iterator(_ctx: &Ctx, iterator: Iterator) -> Iterator1 {
    vec![Box::new(iterator)]
}

pub fn iterator0_iterator1(_ctx: &Ctx, iterator1: Iterator1) -> Iterator0 {
    Some(iterator1)
}
pub fn iterator0_empty(_ctx: &Ctx) -> Iterator0 {
    None
}

pub fn attribute_c1(_ctx: &Ctx, identifier: Identifier, path: Path) -> Attribute {
    Attribute { identifier, path }
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

pub fn predicate_object0_predicate_object1(
    _ctx: &Ctx,
    predicate_object1: PredicateObject1,
) -> PredicateObject0 {
    Some(predicate_object1)
}
pub fn predicate_object0_empty(_ctx: &Ctx) -> PredicateObject0 {
    None
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

pub fn class_c1(_ctx: &Ctx, namespace: Namespace, identifier: Identifier) -> Class {
    Class { namespace, identifier }
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

pub fn namespace_opt_namespace(_ctx: &Ctx, namespace: Namespace) -> NamespaceOpt {
    Some(namespace)
}
pub fn namespace_opt_empty(_ctx: &Ctx) -> NamespaceOpt {
    None
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

pub fn predicate_c1(
    _ctx: &Ctx,
    namespace: Namespace,
    identifier: Identifier,
) -> Predicate {
    Predicate { namespace, identifier }
}

pub fn object_data_value(_ctx: &Ctx, data_value: DataValue) -> Object {
    Object::DataValue(data_value)
}
pub fn object_reference(_ctx: &Ctx, reference: Reference) -> Object {
    Object::Reference(reference)
}

pub fn data_value_c1(
    _ctx: &Ctx,
    namespace: NamespaceOpt,
    shape_path: ShapePath,
) -> DataValue {
    DataValue { namespace, shape_path }
}

pub fn reference_c1(_ctx: &Ctx, identifier: Identifier) -> Reference {
    Reference { identifier }
}
