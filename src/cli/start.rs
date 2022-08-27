use clap::Args;

#[derive(Args)]
pub struct StartCommand {}

impl StartCommand {
    pub fn execute(_cmd: &Self) {}
}
