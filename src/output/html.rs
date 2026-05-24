use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::Serialize;
use tera::{Context as TeraContext, Tera};

use super::presentation::EventPresentation;
use crate::domain::events::Event;
use crate::domain::user::Username;

const BUILTIN_TEMPLATE: &str = include_str!("./assets/templates/index.html");

#[derive(Serialize)]
struct HtmlContext {
    title: String,
    timestamp: String,
    events: Vec<HtmlEvent>,
}

#[derive(Serialize)]
struct HtmlEvent {
    timestamp: String,
    event_kind: &'static str,
    fragments: Vec<HtmlFragment>,
}

#[derive(Serialize)]
struct HtmlFragment {
    text: String,
    url: Option<String>,
}

pub fn render(
    events: &[Event],
    reference_time: DateTime<Utc>,
    username: &Username,
) -> anyhow::Result<String> {
    let mut tera = Tera::default();
    tera.add_raw_template("template.html", BUILTIN_TEMPLATE)
        .context("failed to parse built-in HTML template")?;

    let tera_context = TeraContext::from_serialize(HtmlContext {
        title: format!("@{}'s recent activity on GitHub", username.as_str()),
        timestamp: reference_time.format("%Y-%m-%d %H:%M UTC").to_string(),
        events: events.iter().map(HtmlEvent::from).collect(),
    })
    .context("failed to build HTML context")?;

    tera.render("template.html", &tera_context)
        .context("failed to render HTML template")
}

impl From<&Event> for HtmlEvent {
    fn from(event: &Event) -> Self {
        let presentation = EventPresentation::from(event);

        Self {
            timestamp: event.created_at.format("%Y-%m-%d %H:%M UTC").to_string(),
            event_kind: presentation.kind.name(),
            fragments: presentation
                .fragments
                .into_iter()
                .map(|fragment| HtmlFragment {
                    text: fragment.text,
                    url: fragment.url,
                })
                .collect(),
        }
    }
}
