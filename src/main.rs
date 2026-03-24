mod cli;
mod port;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    match args.port {
        Some(port) => println!("port {} asked", port),
        None => {
            port::list_all();
        }
    }
}
