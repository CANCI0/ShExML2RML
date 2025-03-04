use clap::Parser;
use crate::cli::args::CliArgs;
use crate::cli::commands::transpile_file;

pub fn run() {
    let args = CliArgs::parse();

    if args.version {
        println!("shexml2rml v0.1.0");
        return;
    }

    transpile_file(&args.input, args.output.as_deref());
}
