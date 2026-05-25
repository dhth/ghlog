use crate::domain::events::EventLimit;
use crate::domain::user::Username;
use crate::output::{HtmlTemplate, OutputFormat};

pub enum Command {
    Run {
        username: Username,
        limit: EventLimit,
        output_format: OutputFormat,
    },
}

impl TryFrom<crate::cli::Command> for Command {
    type Error = anyhow::Error;

    fn try_from(command: crate::cli::Command) -> Result<Self, Self::Error> {
        match command {
            crate::cli::Command::Run {
                username,
                limit,
                output_format,
                html_template,
            } => Ok(Self::Run {
                username: Username::try_from(username)?,
                limit: EventLimit::try_from(limit)?,
                output_format: OutputFormat::from_cli(output_format, html_template),
            }),
        }
    }
}

impl Command {
    pub fn handle(self) -> anyhow::Result<()> {
        match self {
            Self::Run {
                username,
                limit,
                output_format,
            } => super::run::handle(&username, limit, output_format),
        }
    }
}

impl OutputFormat {
    fn from_cli(format: crate::cli::OutputFormat, html_template: crate::cli::HtmlTemplate) -> Self {
        match format {
            crate::cli::OutputFormat::Html => Self::Html {
                template: HtmlTemplate::from(html_template),
            },
            crate::cli::OutputFormat::Markdown => Self::Markdown,
            crate::cli::OutputFormat::Plain => Self::Plain,
            crate::cli::OutputFormat::Terminal => Self::Terminal,
        }
    }
}

impl From<crate::cli::HtmlTemplate> for HtmlTemplate {
    fn from(value: crate::cli::HtmlTemplate) -> Self {
        match value {
            crate::cli::HtmlTemplate::Editorial => Self::Editorial,
            crate::cli::HtmlTemplate::Notebook => Self::Notebook,
            crate::cli::HtmlTemplate::Terminal => Self::Terminal,
            crate::cli::HtmlTemplate::Zine => Self::Zine,
        }
    }
}
