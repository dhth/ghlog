use super::presentation::{EventPresentation, humanized_date};
use crate::domain::events::Event;
use chrono::{DateTime, Utc};

pub(super) fn render(events: &[Event], reference_time: DateTime<Utc>) -> String {
    events
        .iter()
        .map(|event| format_event(event, &reference_time))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_event(event: &Event, reference_time: &DateTime<Utc>) -> String {
    let relative_time = humanized_date(&event.created_at, reference_time);
    let event_text = render_presentation(&EventPresentation::from(event));

    format!("{relative_time:<13} {event_text}")
}

fn render_presentation(presentation: &EventPresentation) -> String {
    presentation
        .fragments
        .iter()
        .map(|part| part.text.as_str())
        .collect()
}
