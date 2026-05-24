use clap::{Parser, Subcommand, ValueEnum};

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
        /// Output format
        #[arg(short='o', long = "output", value_enum, default_value_t = OutputFormat::Terminal)]
        output: OutputFormat,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OutputFormat {
    Plain,
    Terminal,
    Markdown,
    Html,
}
