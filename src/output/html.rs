use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::Serialize;
use tera::{Context as TeraContext, Tera};

use super::HtmlTemplate;
use super::presentation::EventPresentation;
use crate::domain::events::Event;
use crate::domain::user::Username;

const TEMPLATE_EDITORIAL: &str = include_str!("./assets/templates/editorial.html");
const TEMPLATE_NOTEBOOK: &str = include_str!("./assets/templates/notebook.html");
const TEMPLATE_TERMINAL: &str = include_str!("./assets/templates/terminal.html");
const TEMPLATE_ZINE: &str = include_str!("./assets/templates/zine.html");

#[derive(Serialize)]
struct HtmlContext {
    branding: Branding,
    events: Vec<HtmlEvent>,
    timestamp: String,
    title: String,
    user_url: String,
    username: String,
}

#[derive(Serialize)]
struct Branding {
    tool_name: String,
    url: String,
}

#[derive(Serialize)]
struct HtmlEvent {
    event_kind: &'static str,
    fragments: Vec<HtmlFragment>,
    timestamp: String,
}

#[derive(Serialize)]
struct HtmlFragment {
    text: String,
    url: Option<String>,
}

pub fn render(
    events: &[Event],
    reference_time: DateTime<Utc>,
    html_template: HtmlTemplate,
    username: &Username,
) -> anyhow::Result<String> {
    let branding = Branding {
        tool_name: "ghlog".to_owned(),
        url: "https://github.com/dhth/ghlog".to_owned(),
    };

    let mut tera = Tera::default();
    tera.add_raw_template("template.html", template_contents(html_template))
        .context("failed to parse built-in HTML template")?;

    let tera_context = TeraContext::from_serialize(HtmlContext {
        branding,
        events: events.iter().map(HtmlEvent::from).collect(),
        timestamp: reference_time.format("%-d %b %Y · %H:%M UTC").to_string(),
        title: format!("@{} recent activity", username.as_str()),
        user_url: format!("https://github.com/{}", username.as_str()),
        username: username.as_str().to_owned(),
    })
    .context("failed to build HTML context")?;

    tera.render("template.html", &tera_context)
        .context("failed to render HTML template")
}

fn template_contents(html_template: HtmlTemplate) -> &'static str {
    match html_template {
        HtmlTemplate::Editorial => TEMPLATE_EDITORIAL,
        HtmlTemplate::Notebook => TEMPLATE_NOTEBOOK,
        HtmlTemplate::Terminal => TEMPLATE_TERMINAL,
        HtmlTemplate::Zine => TEMPLATE_ZINE,
    }
}

impl From<&Event> for HtmlEvent {
    fn from(event: &Event) -> Self {
        let presentation = EventPresentation::from(event);

        Self {
            event_kind: presentation.kind.name(),
            fragments: presentation
                .fragments
                .into_iter()
                .map(|fragment| HtmlFragment {
                    text: fragment.text,
                    url: fragment.url,
                })
                .collect(),
            timestamp: event.created_at.format("%-d %b %Y · %H:%M UTC").to_string(),
        }
    }
}
