use clap::Parser;
use shexml2rml::core::core::transpile_file;
use crate::cli::args::CliArgs;

pub fn run() {
    let args = CliArgs::parse();

    if args.version {
        println!("shexml2rml v0.1.0");
        return;
    }

    transpile_file(&args.input, args.output.as_deref());
}
