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
        /// GitHub username to run for
        username: String,

        /// Filter by event type; repeat to include multiple types. Limit applies after filtering.
        #[arg(
            short = 'e',
            long = "event-type",
            value_enum,
            value_name = "EVENT_TYPE"
        )]
        event_types: Vec<EventType>,
        /// Maximum number of events to show
        #[arg(short = 'l', long = "limit", default_value_t = 20)]
        limit: usize,
        /// Output format to use
        #[arg(short='f', long = "format", value_enum, value_name = "FORMAT", default_value_t = OutputFormat::Terminal)]
        output_format: OutputFormat,
        /// HTML template to use
        #[arg(long = "html-template", value_enum, value_name = "TEMPLATE", default_value_t = HtmlTemplate::Terminal)]
        html_template: HtmlTemplate,
        /// Include private events when visible to the authenticated user
        #[arg(short = 'p', long = "include-private")]
        include_private: bool,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum EventType {
    Create,
    Delete,
    IssueComment,
    Issues,
    PullRequest,
    PullRequestReview,
    Push,
    Release,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OutputFormat {
    /// HTML document
    Html,
    /// Markdown list with links
    Markdown,
    /// Plain unstyled text
    Plain,
    /// ANSI-colored text with links
    Terminal,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum HtmlTemplate {
    /// Serif typography with a magazine-style layout
    Editorial,
    /// Handwritten typography on a dotted-paper background
    Notebook,
    /// Monospaced layout resembling a terminal window
    Terminal,
    /// Sans-serif display type with colored labels per event kind
    Zine,
}
