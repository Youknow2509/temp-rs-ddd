mod cronjob;
mod server;

use anyhow::Result;
use clap::{Parser, Subcommand};
use cronjob::Cronjob;

#[derive(Debug, Parser)]
#[command(name = "cli", version, about = "temp-rs-ddd entry point")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run the application server (HTTP / gRPC / WebSocket / Kafka consumer).
    Serve,
    /// Run the cronjob scheduler.
    Cronjob,
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Serve => tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("app-worker")
            .build()?
            .block_on(server::Server::run()),
        Command::Cronjob => Cronjob::bootstrap()?.run(),
    }
}
