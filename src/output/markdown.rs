use super::presentation::{EventPresentation, Fragment};

pub fn render(events: &[EventPresentation]) -> String {
    events
        .iter()
        .map(render_event)
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_event(event: &EventPresentation) -> String {
    let emoji = event.kind.emoji();
    let text: String = event.fragments.iter().map(render_fragment).collect();

    format!("- {emoji} {text}")
}

fn render_fragment(part: &Fragment) -> String {
    match &part.url {
        Some(url) => format!("[{}]({})", part.text, url),
        None => part.text.clone(),
    }
}
