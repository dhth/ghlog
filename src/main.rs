mod auth;
mod cli;
mod cmds;
mod domain;
mod service;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let command = cmds::Command::try_from(args.command)?;

    command.handle()
}
