mod config;
mod generator;
mod markdown;
mod server;
mod cli;

use clap::Parser;
use cli::{Cli, Commands, init_site, build_and_serve};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_site()?,
        Commands::Serve => build_and_serve().await?,
    }

    Ok(())
}
