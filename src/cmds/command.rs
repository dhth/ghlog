use crate::cli::EventType;
use crate::domain::events::{EventKind, EventKindFilter, EventLimit, EventVisibility};
use crate::domain::user::Username;
use crate::output::{HtmlTemplate, OutputFormat};

pub enum Command {
    Run {
        event_kind_filter: Option<EventKindFilter>,
        username: Username,
        limit: EventLimit,
        event_visibility: EventVisibility,
        output_format: OutputFormat,
    },
}

impl TryFrom<crate::cli::Command> for Command {
    type Error = anyhow::Error;

    fn try_from(command: crate::cli::Command) -> Result<Self, Self::Error> {
        match command {
            crate::cli::Command::Run {
                event_types,
                username,
                limit,
                output_format,
                html_template,
                include_private,
            } => Ok(Self::Run {
                event_kind_filter: EventKindFilter::from_event_kinds(
                    event_types.into_iter().map(EventKind::from_cli),
                ),
                username: Username::try_from(username)?,
                limit: EventLimit::try_from(limit)?,
                event_visibility: if include_private {
                    EventVisibility::IncludePrivate
                } else {
                    EventVisibility::PublicOnly
                },
                output_format: OutputFormat::from_cli(output_format, html_template),
            }),
        }
    }
}

impl Command {
    pub fn handle(self) -> anyhow::Result<()> {
        match self {
            Self::Run {
                event_kind_filter,
                username,
                limit,
                event_visibility,
                output_format,
            } => super::run::handle(
                &username,
                limit,
                event_kind_filter.as_ref(),
                event_visibility,
                output_format,
            ),
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

impl EventKind {
    fn from_cli(value: EventType) -> Self {
        match value {
            EventType::Create => Self::Create,
            EventType::Delete => Self::Delete,
            EventType::IssueComment => Self::IssueComment,
            EventType::Issues => Self::Issues,
            EventType::PullRequest => Self::PullRequest,
            EventType::PullRequestReview => Self::PullRequestReview,
            EventType::Push => Self::Push,
            EventType::Release => Self::Release,
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
