use std::env;
mod cli;
mod api;
mod core;
mod parser;
mod serializer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "--api") {
        let _ = api::runner::main();
    } else {
        let _ = cli::runner::run();
    }
}