use crate::domain::events::{Event, EventLimit, EventPayload, Repo};
use crate::domain::user::Username;
use anyhow::{Context, ensure};
use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use serde::Deserialize;

const GITHUB_API_VERSION: &str = "2026-03-10";

#[derive(Debug, Deserialize)]
struct RawEvent {
    id: String,
    #[serde(rename = "type")]
    event_type: Option<String>,
    repo: Repo,
    payload: serde_json::Value,
    created_at: DateTime<Utc>,
}

enum ConversionError {
    Unsupported,
    InvalidPayload(anyhow::Error),
}

impl TryFrom<RawEvent> for Event {
    type Error = ConversionError;

    fn try_from(raw: RawEvent) -> Result<Self, Self::Error> {
        let payload = match raw.event_type.as_deref() {
            Some("PushEvent") => serde_json::from_value(raw.payload).map(EventPayload::Push),
            Some("CreateEvent") => serde_json::from_value(raw.payload).map(EventPayload::Create),
            Some("DeleteEvent") => serde_json::from_value(raw.payload).map(EventPayload::Delete),
            Some("IssueCommentEvent") => {
                serde_json::from_value(raw.payload).map(EventPayload::IssueComment)
            }
            Some("PullRequestEvent") => {
                serde_json::from_value(raw.payload).map(EventPayload::PullRequest)
            }
            Some("ReleaseEvent") => serde_json::from_value(raw.payload).map(EventPayload::Release),
            _ => return Err(ConversionError::Unsupported),
        }
        .map_err(|error| {
            ConversionError::InvalidPayload(anyhow::anyhow!(
                "couldn't parse payload for event {}: {}",
                raw.id,
                error
            ))
        })?;

        Ok(Self {
            id: raw.id,
            repo: raw.repo,
            payload,
            created_at: raw.created_at,
        })
    }
}

pub struct GithubService {
    client: Client,
    token: String,
}

impl GithubService {
    pub fn new(token: String) -> anyhow::Result<Self> {
        let client = Client::builder()
            .user_agent("ghlog")
            .build()
            .context("couldn't build an HTTP client")?;

        Ok(Self { client, token })
    }

    pub fn get_public_events_for_user(
        &self,
        username: &Username,
        limit: EventLimit,
    ) -> anyhow::Result<Vec<Event>> {
        let response = self
            .client
            .get(format!(
                "https://api.github.com/users/{}/events/public",
                username.as_str()
            ))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", GITHUB_API_VERSION)
            .bearer_auth(&self.token)
            .query(&[("per_page", limit.get()), ("page", 1)])
            .send()
            .context("couldn't send HTTP request to GitHub")?;

        let status = response.status();
        let body = response
            .text()
            .context("couldn't get text from GitHub's response")?;

        ensure!(status.is_success(), "GitHub returned {status}: {body}");

        let raw_events: Vec<RawEvent> =
            serde_json::from_str(&body).context("couldn't parse response from GitHub")?;
        let mut events = Vec::with_capacity(raw_events.len());

        for raw_event in raw_events {
            match Event::try_from(raw_event) {
                Ok(event) => events.push(event),
                Err(ConversionError::Unsupported) => continue,
                Err(ConversionError::InvalidPayload(error)) => return Err(error),
            }
        }

        Ok(events)
    }
}
