use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
    pub port: Option<u16>,
}

#[derive(Subcommand)]
pub enum Commands {
    Kill { port: u16 },
}
