use crate::parser::ast::{Shexml, Declaration, Prefix, Source, Iterator_, Expression, Field, Shape, IteratorQuery, Predicate, PredicateObject, ShapeLink};
use std::iter::Iterator;
use pest::Parser;
use pest_derive::Parser;
use crate::parser::ast::*;

#[derive(Parser)]
#[grammar = "./parser/grammar.pest"]
pub struct ShexmlParser;

pub fn parse_input(input: &str) -> Result<Shexml, String> {
    let parsed = ShexmlParser::parse(Rule::SHEXML, input);
    match parsed {
        Ok(pairs) => {
            let pair = pairs.into_iter().next().unwrap();
            let ast = parse_shexml(pair);
            Ok(ast)
            }
        Err(err) => Err(format!("Error: {:?}", err)),
    }
}

fn parse_shexml(pair: pest::iterators::Pair<Rule>) -> Shexml {
    let inner_pairs = pair.into_inner();
    let mut declarations = Vec::new();
    let mut shapes = Vec::new();

    for declaration_pair in inner_pairs {
        // Imprimir cada par de regla
        println!("Found pair: {:?}", declaration_pair.as_rule());
        println!("Pair content: {}", declaration_pair.as_str());

        match declaration_pair.as_rule() {
            Rule::DECLARATION => {
                println!("Found DECLARATION");
                // Llamada a la función para parsear declaración
                declarations.push(parse_declaration(declaration_pair))
            }
            Rule::SHAPE => {
                println!("Found SHAPE");
                // Llamada a la función para parsear shape
                shapes.push(parse_shape(declaration_pair))
            }
            other_rule => {
                // Si es una regla inesperada, imprimirla
                println!("Unexpected rule: {:?}", other_rule);
            }
        }
    }

    // Imprimir el resultado final para debug
    println!("Parsed Shexml - Declarations: {:?}", declarations);
    println!("Parsed Shexml - Shapes: {:?}", shapes);

    Shexml { declarations, shapes }
}

fn parse_declaration(pair: pest::iterators::Pair<Rule>) -> Declaration {
    let inner_pair = pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::PREFIX => Declaration::Prefix(parse_prefix(inner_pair)),
        Rule::SOURCE => Declaration::Source(parse_source(inner_pair)),
        Rule::ITERATOR => Declaration::Iterator_(parse_iterator(inner_pair)),
        Rule::EXPRESSION => Declaration::Expression(parse_expression(inner_pair)),
        _ => panic!("Unexpected rule in DECLARATION"),
    }
}

fn parse_prefix(pair: pest::iterators::Pair<Rule>) -> Prefix {
    let mut inner_pairs = pair.into_inner();
    let identifier = inner_pairs.next().unwrap().as_str().to_string();
    let uri = inner_pairs.next().unwrap().as_str().to_string();
    Prefix { identifier, uri }
}

fn parse_source(pair: pest::iterators::Pair<Rule>) -> Source {
    let mut inner_pairs = pair.into_inner();
    let identifier = inner_pairs.next().unwrap().as_str().to_string();
    let uri = inner_pairs.next().unwrap().as_str().to_string();
    Source { identifier, uri }
}

fn parse_iterator(pair: pest::iterators::Pair<Rule>) -> Iterator_ {
    let mut inner_pairs = pair.into_inner();
    let identifier = inner_pairs.next().unwrap().as_str().to_string();
    let query = inner_pairs.next().unwrap().as_str().to_string();
    let mut fields = Vec::new();
    let mut nested_iterators = Vec::new();

    for field_or_iterator_pair in inner_pairs {
        match field_or_iterator_pair.as_rule() {
            Rule::FIELD => fields.push(parse_field(field_or_iterator_pair)),
            Rule::ITERATOR => nested_iterators.push(parse_iterator(field_or_iterator_pair)),
            _ => panic!("Unexpected rule in ITERATOR"),
        }
    }

    Iterator_ { identifier, query, fields, nested_iterators }
}

fn parse_expression(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut inner_pairs = pair.into_inner();
    let identifier = inner_pairs.next().unwrap().as_str().to_string();
    let mut union = Vec::new();

    for iterator_query_pair in inner_pairs {
        union.push(parse_iterator_query(iterator_query_pair));
    }

    Expression { identifier, union }
}

fn parse_field(pair: pest::iterators::Pair<Rule>) -> Field {
    let mut inner_pairs = pair.into_inner();
    let identifier = inner_pairs.next().unwrap().as_str().to_string();
    let query = inner_pairs.next().unwrap().as_str().to_string();
    Field { identifier, query }
}

fn parse_iterator_query(pair: pest::iterators::Pair<Rule>) -> IteratorQuery {
    let mut path = Vec::new();
    for path_part_pair in pair.into_inner() {
        path.push(path_part_pair.as_str().to_string());
    }
    IteratorQuery { path }
}

fn parse_shape(pair: pest::iterators::Pair<Rule>) -> Shape {
    let mut inner_pairs = pair.into_inner();
    let shape_type = inner_pairs.next().unwrap().as_str().to_string();
    let prefix_identifier = inner_pairs.next().unwrap().as_str().to_string();
    let iterator_query = parse_iterator_query(inner_pairs.next().unwrap());
    let mut predicate_objects = Vec::new();

    for predicate_object_pair in inner_pairs {
        predicate_objects.push(parse_predicate_object(predicate_object_pair));
    }

    Shape { shape_type, prefix_identifier, iterator_query, predicate_objects }
}

fn parse_predicate_object(pair: pest::iterators::Pair<Rule>) -> PredicateObject {
    let inner_pair = pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::PREDICATE => PredicateObject::Predicate(parse_predicate(inner_pair)),
        Rule::SHAPE_LINK => PredicateObject::ShapeLink(parse_shape_link(inner_pair)),
        _ => panic!("Unexpected rule in PREDICATE_OBJECT"),
    }
}

fn parse_predicate(pair: pest::iterators::Pair<Rule>) -> Predicate {
    let mut inner_pairs = pair.into_inner();
    let prefix_identifier = inner_pairs.next().map(|p| p.as_str().to_string());
    let iterator_query = parse_iterator_query(inner_pairs.next().unwrap());
    Predicate { prefix_identifier, iterator_query }
}

fn parse_shape_link(pair: pest::iterators::Pair<Rule>) -> ShapeLink {
    let mut inner_pairs = pair.into_inner();
    let prefix_identifier = inner_pairs.next().unwrap().as_str().to_string();
    let identifier = inner_pairs.next().unwrap().as_str().to_string();
    ShapeLink { prefix_identifier, identifier }
}
