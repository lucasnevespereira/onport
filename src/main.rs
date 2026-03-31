mod cli;
mod kill;
mod output;
mod port;
mod process;

use clap::Parser;
use cli::{Args, Commands};
use kill::kill;
use port::{inspect, inspect_all};

fn main() {
    let args = Args::parse();

    let result = match args.command {
        Some(Commands::Kill { port }) => kill(port),
        Some(Commands::Version) => {
            println!("onport {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        None => match args.port {
            Some(port) => inspect(port),
            None => inspect_all(),
        },
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
