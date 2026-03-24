use clap::Parser;

#[derive(Parser)]
pub struct Args {
    pub port: Option<u16>,
}
