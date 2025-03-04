use clap::{Parser};

#[derive(Parser)]
#[command(name = "shexml2rml")]
#[command(about = "A CLI tool to transpile ShExML to RML", long_about = None)]
pub struct CliArgs {
    /// Input .shexml file
    pub input: String,

    /// Output RML file
    #[arg(short = 'm', long = "mapping")]
    pub output: Option<String>,

    /// Display version
    #[arg(short = 'v', long = "version")]
    pub version: bool,
}
