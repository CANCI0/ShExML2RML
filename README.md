El proyecto se organiza de la siguiente manera:
```
shexml-transpiler/
├── src/
│   ├── main.rs              # Punto de entrada principal
│   ├── lib.rs               # Define los módulos principales
│   ├── parser/              # Lógica para el análisis y parsing de SheXML
│   │   ├── mod.rs           # Define el módulo `parser`
│   │   ├── lexer.rs         # Analizador léxico (tokenización)
│   │   ├── syntax.rs        # Analizador sintáctico
│   │   ├── grammar.rs       # Gramática y validación
│   ├── sources/             # Procesamiento de fuentes de datos (JSON, XML, etc.)
│   │   ├── mod.rs           # Define el módulo `sources`
│   │   ├── json.rs          # Procesamiento de JSON
│   │   ├── xml.rs           # Procesamiento de XML
│   │   ├── source_loader.rs # Descarga y manejo de datos desde fuentes remotas
│   ├── transpilation/       # Módulo de conversión de SheXML a RML
│   │   ├── mod.rs           # Define el módulo `transpilation`
│   │   ├── shexml_to_rml.rs # Lógica de conversión de SheXML a RML
│   │   ├── templates.rs     # Plantillas y generación de estructuras RML
│   ├── output/              # Generación del código de salida en RML
│   │   ├── mod.rs           # Define el módulo `output`
│   │   ├── rml_writer.rs    # Escritura del resultado en RML
│   │   ├── format.rs        # Formateo y validación del output
│   ├── utils/               # Funciones auxiliares
│   │   ├── mod.rs           # Define el módulo `utils`
│   │   ├── config.rs        # Configuración del transpilador
│   │   ├── error.rs         # Manejo de errores personalizados
│   ├── tests/               # Pruebas del proyecto
│   │   ├── integration.rs   # Pruebas de integración
│   │   ├── unit/            # Pruebas unitarias
│   │       ├── parser_tests.rs
│   │       ├── source_tests.rs
│   │       ├── transpilation_tests.rs
│   │       ├── output_tests.rs
├── resources/               # Archivos de prueba en SheXML
│   ├── ...                  # Archivos de prueba (.shexml)
├── Cargo.toml               # Declaración del proyecto y dependencias
├── README.md                # Documentación del proyecto
├── LICENSE                  # Licencia del proyecto
```