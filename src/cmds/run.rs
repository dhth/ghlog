use crate::domain::events::{EventKindFilter, EventLimit, EventVisibility};
use crate::domain::user::Username;
use crate::output::{self, OutputFormat};
use crate::service::github::GithubService;
use anyhow::Context;
use chrono::Utc;

pub fn handle(
    username: &Username,
    limit: EventLimit,
    event_kind_filter: Option<&EventKindFilter>,
    event_visibility: EventVisibility,
    output_format: OutputFormat,
) -> anyhow::Result<()> {
    let token = crate::auth::get_token()?;
    let service = GithubService::new(token)?;
    let events =
        service.get_events_for_user(username, limit, event_kind_filter, event_visibility)?;

    if events.is_empty() {
        return Ok(());
    }

    let rendered_output = output::render(
        &events,
        Utc::now(),
        output_format,
        username,
        event_visibility,
    )
    .context("couldn't generate output to be rendered")?;

    println!("{rendered_output}");

    Ok(())
}
