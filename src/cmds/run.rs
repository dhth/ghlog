use crate::domain::events::EventLimit;
use crate::domain::user::Username;
use crate::output::{self, OutputFormat};
use crate::service::github::GithubService;
use chrono::Utc;

pub fn handle(
    username: &Username,
    limit: EventLimit,
    output_format: OutputFormat,
) -> anyhow::Result<()> {
    let token = crate::auth::get_token()?;
    let service = GithubService::new(token)?;
    let events = service.get_public_events_for_user(username, limit)?;

    if events.is_empty() {
        return Ok(());
    }

    let rendered_output = output::render(&events, Utc::now(), output_format);

    println!("{rendered_output}");

    Ok(())
}
