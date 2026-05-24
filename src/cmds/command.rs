use crate::domain::events::EventLimit;
use crate::domain::user::Username;
use crate::output::OutputFormat;

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
                output,
            } => Ok(Self::Run {
                username: Username::try_from(username)?,
                limit: EventLimit::try_from(limit)?,
                output_format: output.into(),
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

impl From<crate::cli::OutputFormat> for OutputFormat {
    fn from(value: crate::cli::OutputFormat) -> Self {
        match value {
            crate::cli::OutputFormat::Plain => Self::Plain,
            crate::cli::OutputFormat::Markdown => Self::Markdown,
        }
    }
}
