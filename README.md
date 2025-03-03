# ShExML2RML

## 🌟About

ShExML2RML is a Rust-based transpiler that converts ShExML (Shape Expressions Mapping Language) to RML (RDF Mapping Language), enabling seamless generation of RDF triples from ShExML data.

## 🚀Features

✅ Transpile ShExML to RML effortlessly.\
✅ Command-line interface (CLI) for easy usage.\
✅ Supports basic ShExML structures.\
✅ Error handling for invalid input files.\
✅ Outputs RML in a structured format for interoperability.

## 🔧Installation

To build and install the transpiler, ensure you have Rust installed:

```sh
cargo build --release
```

Or install it directly with:

```sh
cargo install --path .
```

## 📋Usage

Run the CLI tool to convert a `.shexml` file into an `.rml` file:

```sh
shexml2rml input.shexml -m output.rml
```

### 🛠️CLI Options

```sh
shexml2rml --help
```

| Option           | Description                 |
| ---------------- | --------------------------- |
| `<input.shexml>` | Input file in ShExML format |
| `-m, --mapping`  | Specify output file for RML |
| `-v, --version`  | Display version info        |
| `-h, --help`     | Show help message           |

## 👨‍💻Example

Input ShExML:

```shexml
PREFIX : <http://example.com/>
PREFIX schema: <http://schema.org/>
SOURCE films_xml_file <http://shexml.herminiogarcia.com/files/films.xml>

ITERATOR film_xml <xpath: //film> {
    FIELD id <@id>
    FIELD name <name>
}

EXPRESSION films <films_xml_file.film_xml>

:Film :[films.id] {
    schema:name [films.name] ;
}
```

Transpiled Output (RML):

```ttl
@prefix rml: <http://semweb.mmlab.be/ns/rml#> .
@prefix rr: <http://www.w3.org/ns/r2rml#> .
@prefix d2rq: <http://www.wiwiss.fu-berlin.de/suhl/bizer/D2RQ/0.1#> .
@prefix ql: <http://semweb.mmlab.be/ns/ql#> .
@prefix map: <http://mapping.example.com/> .
@prefix : <http://example.com/> .
@prefix schema: <http://schema.org/> .

map:m_1  a rr:TriplesMap ;
    rml:logicalSource      map:ls_1 ;
    rr:predicateObjectMap  map:po_1 ;
    rr:subjectMap          map:s_1 .

map:ls_1  a                rml:LogicalSource ;
        rml:iterator              "//film" ;
        rml:referenceFormulation  ql:XPath ;
        rml:source                "http://shexml.herminiogarcia.com/files/films.xml" .

map:s_1  a           rr:SubjectMap ;
        rr:template  "http://example.com/{@id}" .

map:po_1  a              rr:PredicateObjectMap ;
        rr:objectMap     map:o_1 ;
        rr:predicateMap  map:p_1 .

map:o_1  a          rr:ObjectMap ;
        rr:template  "{name}" ;
        rr:termType  rr:Literal .

map:p_1  a           rr:predicateMap ;
        rr:constant  schema:name .

```

## 🛠️Development

Clone the repository and start developing:

```sh
git clone https://github.com/yourusername/shexml2rml.git
cd shexml2rml
cargo run -- input.shexml -m output.rml
```

## 🚧Limitations

While ShExML2RML project provides a functional ShExML to RML transformation, several aspects of the ShExML specification have yet to be fully implemented. These limitations include:

- [**Nested Iterators**](https://shexml.herminiogarcia.com/spec/#nested-iterator)
- [**Queries**](https://shexml.herminiogarcia.com/spec/#query)
- [**Pushed and Popped Fields**](https://shexml.herminiogarcia.com/spec/#pushed-and-popped-fields)
- [**Data Types**](https://shexml.herminiogarcia.com/spec/#data-types-static-version)
- [**Lang Tags**](https://shexml.herminiogarcia.com/spec/#lang-tags-static-version)
- [**Graphs**](https://shexml.herminiogarcia.com/spec/#graphs)

The project currently only supports [**basic**](https://shexml.herminiogarcia.com/spec/#basic-expression) and [**union**](https://shexml.herminiogarcia.com/spec/#union) expressions in ShExML. More advanced expression types and their combinations are not yet supported.

---

These limitations highlight areas where the ShExML2RML is still evolving, and where future development is focused on expanding functionality and improving robustness.



## 🤝🏼Contributing

Contributions are welcome! Please open an issue or a pull request.

