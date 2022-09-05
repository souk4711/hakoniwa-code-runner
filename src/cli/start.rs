use anyhow::Result;
use clap::Args;
use std::{fs, path::PathBuf, process};

use crate::{App, AppConfig, Server};

#[derive(Args)]
pub struct StartCommand {
    /// Configuration file
    #[clap(short, long, value_name = "FILE")]
    config: PathBuf,
}

impl StartCommand {
    pub fn execute(cmd: &Self) {
        if let Err(err) = Self::_execute(cmd) {
            eprintln!("hakoniwa-code-runner-start: {}", err);
            process::exit(1);
        }
    }

    fn _execute(cmd: &Self) -> Result<()> {
        // Arg: config.
        let config_data = fs::read_to_string(&cmd.config)?;
        let config = AppConfig::from_str(&config_data)?;
        let mut app = App::new(config);
        app.initialize()?;

        // Start.
        let server = Server::new(app);
        server.start()?;
        Ok(())
    }
}
