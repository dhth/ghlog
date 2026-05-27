use crate::domain::events::{EventKindFilter, EventLimit};
use crate::domain::user::Username;
use crate::output::{self, OutputFormat};
use crate::service::github::GithubService;
use anyhow::Context;
use chrono::Utc;

pub fn handle(
    username: &Username,
    limit: EventLimit,
    event_kind_filter: Option<&EventKindFilter>,
    output_format: OutputFormat,
) -> anyhow::Result<()> {
    let token = crate::auth::get_token()?;
    let service = GithubService::new(token)?;
    let events = service.get_public_events_for_user(username, limit, event_kind_filter)?;

    if events.is_empty() {
        return Ok(());
    }

    let rendered_output = output::render(&events, Utc::now(), output_format, username)
        .context("couldn't generate output to be rendered")?;

    println!("{rendered_output}");

    Ok(())
}
