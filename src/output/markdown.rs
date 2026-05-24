use super::presentation::{EventPresentation, Fragment};
use crate::domain::events::Event;

pub(super) fn render(events: &[Event]) -> String {
    events
        .iter()
        .map(render_event)
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_event(event: &Event) -> String {
    let presentation = EventPresentation::from(event);
    let emoji = presentation.kind.emoji();
    let text = render_presentation(&presentation);

    format!("- {emoji} {text}")
}

fn render_presentation(presentation: &EventPresentation) -> String {
    presentation
        .fragments
        .iter()
        .map(render_text_part)
        .collect()
}

fn render_text_part(part: &Fragment) -> String {
    match &part.url {
        Some(url) => format!("[{}]({})", part.text, url),
        None => part.text.clone(),
    }
}
