use super::presentation::{Color, EventPresentation, Fragment, humanized_date};
use chrono::{DateTime, Utc};

const OSC: &str = "\u{1b}]";
const ST: &str = "\u{1b}\\";
const RESET: &str = "\u{1b}[0m";

pub fn render(events: Vec<EventPresentation>, reference_time: DateTime<Utc>) -> String {
    events
        .into_iter()
        .map(|event| render_event(event, &reference_time))
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_event(event: EventPresentation, reference_time: &DateTime<Utc>) -> String {
    let relative_time = humanized_date(&event.created_at, reference_time);
    let time = colorize(&format!("{relative_time:<13}"), Color::Gray);
    let event_text = {
        let text = event
            .fragments
            .into_iter()
            .map(render_fragment)
            .collect::<Vec<_>>()
            .join(" ");
        colorize(&text, event.kind.color())
    };

    format!("{time} {event_text}")
}

fn render_fragment(fragment: Fragment) -> String {
    match fragment.url {
        Some(url) => format!("{OSC}8;;{url}{ST}{}{OSC}8;;{ST}", fragment.text),
        None => fragment.text,
    }
}

fn colorize(text: &str, color: Color) -> String {
    format!("{}{}{RESET}", ansi_code(color), text)
}

fn ansi_code(color: Color) -> &'static str {
    match color {
        Color::Gray => "\u{1b}[90m",
        Color::Blue => "\u{1b}[34m",
        Color::Green => "\u{1b}[32m",
        Color::Yellow => "\u{1b}[33m",
        Color::Purple => "\u{1b}[35m",
        Color::Red => "\u{1b}[31m",
    }
}
