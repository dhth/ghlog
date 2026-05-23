use clap::{Parser, Subcommand};

/// ghlog lets you view a GitHub user's recent public activity
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Fetch and display events for a GitHub user
    #[command(name = "run")]
    Run {
        /// GitHub username
        username: String,
        /// Maximum number of events to show
        #[arg(short = 'l', long = "limit", default_value_t = 20)]
        limit: usize,
    },
}
