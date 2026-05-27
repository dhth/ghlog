mod html;
mod markdown;
mod plain;
mod presentation;
mod terminal;

use crate::domain::events::Event;
use crate::domain::user::Username;
use chrono::{DateTime, Utc};
use presentation::EventPresentation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Html { template: HtmlTemplate },
    Markdown,
    Plain,
    Terminal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtmlTemplate {
    Editorial,
    Notebook,
    Terminal,
    Zine,
}

pub fn render(
    events: &[Event],
    reference_time: DateTime<Utc>,
    format: OutputFormat,
    username: &Username,
) -> anyhow::Result<String> {
    let events = events
        .iter()
        .map(EventPresentation::from)
        .collect::<Vec<_>>();

    let output = match format {
        OutputFormat::Html { template } => {
            html::render(events, reference_time, template, username)?
        }
        OutputFormat::Markdown => markdown::render(events),
        OutputFormat::Plain => plain::render(events, reference_time),
        OutputFormat::Terminal => terminal::render(events, reference_time),
    };

    Ok(output)
}
