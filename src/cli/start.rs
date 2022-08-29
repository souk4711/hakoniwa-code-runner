use anyhow::Result;
use clap::Args;
use std::{fs, path::PathBuf, process, str};

use crate::{App, AppConfig, Embed, Server};

#[derive(Args)]
pub struct StartCommand {
    /// Configuration file [default: app.toml]
    #[clap(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
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
        let config_data = match &cmd.config {
            Some(config) => fs::read_to_string(config)?,
            None => str::from_utf8(&Embed::get("app.toml").unwrap().data)?.to_string(),
        };
        let config = AppConfig::from_str(&config_data)?;
        let mut app = App::new(config);
        app.initialize()?;

        // Start.
        let server = Server::new(app);
        server.start()?;
        Ok(())
    }
}
