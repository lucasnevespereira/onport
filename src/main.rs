mod cli;
mod output;
mod port;
mod process;

use clap::Parser;
use cli::Args;
use port::{inspect, inspect_all};

fn main() {
    let args = Args::parse();

    let result = match args.port {
        Some(port) => inspect(port),
        None => inspect_all(),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
