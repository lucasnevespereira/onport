use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    version,
    about = "Find and stop processes listening on local TCP ports"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
    /// Show the process listening on this port
    pub port: Option<u16>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Stop the process listening on a port
    #[command(visible_alias = "k")]
    Kill {
        /// Port to free
        port: u16,
        /// Send SIGKILL instead of SIGTERM
        #[arg(long)]
        force: bool,
    },
}
