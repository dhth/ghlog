mod markdown;
mod plain;
mod presentation;

use crate::domain::events::Event;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Plain,
    Markdown,
}

pub fn render(events: &[Event], reference_time: DateTime<Utc>, format: OutputFormat) -> String {
    match format {
        OutputFormat::Plain => plain::render(events, reference_time),
        OutputFormat::Markdown => markdown::render(events),
    }
}
