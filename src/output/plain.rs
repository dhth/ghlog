use super::presentation::{EventPresentation, humanized_date};
use chrono::{DateTime, Utc};

pub fn render(events: Vec<EventPresentation>, reference_time: DateTime<Utc>) -> String {
    events
        .into_iter()
        .map(|event| render_event(event, &reference_time))
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_event(event: EventPresentation, reference_time: &DateTime<Utc>) -> String {
    let relative_time = humanized_date(&event.created_at, reference_time);
    let event_text: String = event.fragments.into_iter().map(|part| part.text).collect();

    format!("{relative_time:<13} {event_text}")
}
