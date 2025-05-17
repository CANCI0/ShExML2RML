use std::env;
mod cli;
mod api;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "--api") {
        // Modo API
        let _ = api::runner::main();
    } else {
        // Modo CLI
        let _ = cli::runner::run();
    }
}