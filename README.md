# ShExML2RML

## 🌟 About

ShExML2RML is a high-performance Rust-based transpiler that converts ShExML (Shape Expressions Mapping Language) to RML (RDF Mapping Language), enabling seamless generation of RDF triples from ShExML data.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85+-orange.svg)](https://www.rust-lang.org)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://www.docker.com)

## 🚀 Features

✅ **Fast & Reliable**: High-performance Rust implementation\
✅ **Multiple Interfaces**: CLI, Web UI, and REST API\
✅ **Docker Ready**: One-command deployment with Docker Compose\
✅ **Real-time Processing**: Interactive web interface with live transpilation\
✅ **Comprehensive Error Handling**: Clear error messages for invalid inputs\
✅ **Standards Compliant**: Generates valid RML output for interoperability\
✅ **Cross-platform**: Works on Linux, macOS, and Windows

## 🔧Installation

You can install and run ShExML2RML in two ways: using Docker (recommended for easy setup) or building from source with Rust.

### 📦 Using Docker (Recommended)

The easiest way to get started is using Docker. This method includes both the CLI tool and a web interface.

**Prerequisites:**
- Docker and Docker Compose installed on your system

**Build and run with Docker Compose:**

```sh
docker-compose up --build
```

This will:
- Build the Rust CLI tool
- Build the web application 
- Start both services accessible at:
  - **Web Interface**: http://localhost (port 80)
  - **API**: http://localhost:8080

**Using just Docker:**

```sh
docker build -t shexml2rml .
docker run -p 8080:8080 -p 80:80 shexml2rml
```

### 🦀 Building from Source

If you prefer to build from source or want to contribute to development:

**Prerequisites:**
- Rust toolchain installed (https://rustup.rs/)

**Build the CLI tool:**

```sh
cargo build --release
```

**Install locally:**

```sh
cargo install --path .
```

The binary will be available as `shexml2rml` in your system PATH.

## 📋Usage

ShExML2RML can be used in multiple ways depending on your needs:

### 🌐 Web Interface (Easiest)

If you're running with Docker, access the web interface at:
- **http://localhost** (after running `docker-compose up --build`)

The web interface provides an interactive ShExML editor and real-time transpilation to RML

### 💻 Command Line Interface (CLI)

Use the command line tool to convert ShExML files to RML format directly from your terminal.

#### CLI Options

| Option           | Description                 |
| ---------------- | --------------------------- |
| `<input.shexml>` | Input file in ShExML format |
| `-m, --mapping`  | Specify output file for RML |
| `-h, --help`     | Show help message           |

#### Examples:
```sh
# Convert ShExML to RML with specific output file
shexml2rml example.shexml -m result.rml

# Convert ShExML to RML (output will be auto-generated)
shexml2rml example.shexml
```

### 🔗 API Usage

Start the API server:
```sh
shexml2rml --api
```

The API will be available at **http://localhost:8080** with the following endpoints:

- **POST** `/transpile` - Convert ShExML content to RML
- **GET** `/health` - Health check endpoint

>NOTE: If you're running on Docker, this API is started automatically

**Example API call:**
```bash
# Convert ShExML to RML via API
curl -X POST http://localhost:8080/transpile \
  -H "Content-Type: text/plain" \
  -d @your-file.shexml

# Health check
curl http://localhost:8080/health
```

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

## 🛠️ Development

### Setting up the Development Environment

Clone the repository and start developing:

```sh
git clone https://github.com/CANCI0/shexml2rml.git
cd shexml2rml

# Run the CLI tool directly
cargo run -- input.shexml -m output.rml

# Start the API server for development
cargo run -- --api

# Run tests
cargo test
```

### Project Structure

```
shexml2rml/
├── src/                 # Rust source code
├── webapp/             # React web interface
├── Dockerfile          # Multi-stage Docker build
├── docker-compose.yml  # Easy deployment
└── Cargo.toml         # Rust dependencies
```

### Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/awesome-feature`)
3. Commit your changes (`git commit -m 'Add awesome feature'`)
4. Push to the branch (`git push origin feature/awesome-feature`)
5. Open a Pull Request

## 🚧 Limitations

While ShExML2RML provides a functional ShExML to RML transformation, several aspects of the ShExML specification are still under development. Current limitations include:

### Not Yet Supported:
❌ [**Recursive Nested Iterators**](https://shexml.herminiogarcia.com/spec/#nested-iterator) - Complex nested data structures, more that one level deep\
❌ [**Queries**](https://shexml.herminiogarcia.com/spec/#query) - Advanced query capabilities\
❌ [**Pushed and Popped Fields**](https://shexml.herminiogarcia.com/spec/#pushed-and-popped-fields) - Field manipulation operations\
❌ [**Data Types**](https://shexml.herminiogarcia.com/spec/#data-types-static-version) - Explicit data type declarations\
❌ [**Lang Tags**](https://shexml.herminiogarcia.com/spec/#lang-tags-static-version) - Language tag support\
❌ [**Graphs**](https://shexml.herminiogarcia.com/spec/#graphs) - Named graph generation

### Currently Supported:
✅ [**Basic Expressions**](https://shexml.herminiogarcia.com/spec/#basic-expression) - Standard mappings\
✅ [**Nested iterators**](https://shexml.herminiogarcia.com/spec/#basic-expression) - Just one level deep\
✅ [**Union Expressions**](https://shexml.herminiogarcia.com/spec/#union) - Multiple source combinations

More advanced features are planned for future releases. Contributions are welcome!

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Support

- 🐛 **Issues**: [GitHub Issues](https://github.com/CANCI0/shexml2rml/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/CANCI0/shexml2rml/discussions)
