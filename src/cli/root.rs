use clap::{AppSettings, Parser, Subcommand};

use crate::cli::start::StartCommand;

#[derive(Subcommand)]
enum Commands {
    /// Start a server
    Start(StartCommand),
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct RootCommand {
    #[clap(subcommand)]
    command: Commands,
}

pub fn execute() {
    let cli = RootCommand::parse();
    match &cli.command {
        Commands::Start(cmd) => StartCommand::execute(cmd),
    }
}
