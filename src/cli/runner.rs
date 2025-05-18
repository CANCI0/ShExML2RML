use std::fs;
use clap::Parser;
use crate::core::core::transpile_content;
use crate::cli::args::CliArgs;

pub fn run() -> Result<(), String> {
    let args = CliArgs::parse();

    if args.version {
        println!("shexml2rml v0.1.0");
        return Ok(());
    }

    let input_content = fs::read_to_string(&args.input)
        .map_err(|e| format!("Error reading input file '{}': {}", args.input, e))?;

    let result = transpile_content(&input_content)
        .map_err(|e| format!("Transpilation error: {}", e))?;

    match &args.output {
        Some(output_path) => {
            fs::write(output_path, &result)
                .map_err(|e| format!("Error writing to '{}': {}", output_path, e))?;
            println!("✅ Transpilation successful! Output saved to: {}", output_path);
        }
        None => println!("{}", result),
    }

    Ok(())
}