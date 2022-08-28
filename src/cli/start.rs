use anyhow::Result;
use clap::Args;
use std::process;

use crate::Server;

#[derive(Args)]
pub struct StartCommand {}

impl StartCommand {
    pub fn execute(cmd: &Self) {
        if let Err(err) = Self::_execute(cmd) {
            eprintln!("hakoniwa-code-runner-start: {}", err);
            process::exit(1);
        }
    }

    fn _execute(_cmd: &Self) -> Result<()> {
        let server = Server::new();
        server.start()?;
        Ok(())
    }
}
