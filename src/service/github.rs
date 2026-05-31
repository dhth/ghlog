use crate::domain::events::{
    Event, EventKindFilter, EventLimit, EventPayload, EventVisibility, Repo,
};
use crate::domain::user::Username;
use anyhow::{Context, ensure};
use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, LINK};
use serde::Deserialize;

const GITHUB_API_BASE: &str = "https://api.github.com";
const GITHUB_API_VERSION: &str = "2026-03-10";
const GITHUB_API_MAX_PER_PAGE: usize = 100;

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
            Some("IssuesEvent") => serde_json::from_value(raw.payload).map(EventPayload::Issues),
            Some("IssueCommentEvent") => {
                serde_json::from_value(raw.payload).map(EventPayload::IssueComment)
            }
            Some("PullRequestEvent") => {
                serde_json::from_value(raw.payload).map(EventPayload::PullRequest)
            }
            Some("PullRequestReviewEvent") => {
                serde_json::from_value(raw.payload).map(EventPayload::PullRequestReview)
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

#[derive(Debug)]
struct GithubPage {
    events: Vec<Event>,
    has_next_page: bool,
}

impl GithubService {
    pub fn new(token: String) -> anyhow::Result<Self> {
        let client = Client::builder()
            .user_agent("ghlog")
            .build()
            .context("couldn't build an HTTP client")?;

        Ok(Self { client, token })
    }

    pub fn get_events_for_user(
        &self,
        username: &Username,
        limit: EventLimit,
        event_kind_filter: Option<&EventKindFilter>,
        event_visibility: EventVisibility,
    ) -> anyhow::Result<Vec<Event>> {
        let mut collected_events = Vec::new();
        let mut page = 1;
        loop {
            let GithubPage {
                events,
                has_next_page,
            } = self.fetch_events(username, page, event_visibility)?;

            for event in events {
                if event_kind_filter.is_some_and(|filter| !filter.matches(event.kind())) {
                    continue;
                }

                collected_events.push(event);
            }

            if collected_events.len() >= limit.get() {
                collected_events.truncate(limit.get());
                break;
            }

            if !has_next_page {
                break;
            }

            page += 1;
        }

        Ok(collected_events)
    }

    fn fetch_events(
        &self,
        username: &Username,
        page: usize,
        event_visibility: EventVisibility,
    ) -> anyhow::Result<GithubPage> {
        let path = match event_visibility {
            EventVisibility::PublicOnly => "events/public",
            EventVisibility::IncludePrivate => "events",
        };

        let response = self
            .client
            .get(format!(
                "{}/users/{}/{}",
                GITHUB_API_BASE,
                username.as_str(),
                path,
            ))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", GITHUB_API_VERSION)
            .bearer_auth(&self.token)
            .query(&[("per_page", GITHUB_API_MAX_PER_PAGE), ("page", page)])
            .send()
            .context("couldn't send HTTP request to GitHub")?;

        let status = response.status();
        let has_next_page = next_page_exists(response.headers());
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

        Ok(GithubPage {
            events,
            has_next_page,
        })
    }
}

fn next_page_exists(headers: &HeaderMap) -> bool {
    headers
        .get(LINK)
        .and_then(|value| value.to_str().ok())
        .map(|value| {
            value.split(',').map(str::trim).any(|entry| {
                entry
                    .split(';')
                    .map(str::trim)
                    .any(|part| part == "rel=\"next\"")
            })
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderValue;

    #[test]
    fn has_next_page_returns_false_when_link_header_is_absent() {
        // GIVEN
        let headers = HeaderMap::new();

        // WHEN
        let has_next_page = next_page_exists(&headers);

        // THEN
        assert!(!has_next_page);
    }

    #[test]
    fn next_page_exists_returns_true_when_link_header_contains_next_relation() {
        // GIVEN
        let mut headers = HeaderMap::new();
        headers.insert(
            LINK,
            HeaderValue::from_static(
                r#"<https://api.github.com/user/13575379/events?per_page=100&page=1>; rel="prev", <https://api.github.com/user/13575379/events?per_page=100&page=3>; rel="next", <https://api.github.com/user/13575379/events?per_page=100&page=3>; rel="last", <https://api.github.com/user/13575379/events?per_page=100&page=1>; rel="first""#,
            ),
        );

        // WHEN
        let has_next_page = next_page_exists(&headers);

        // THEN
        assert!(has_next_page);
    }

    #[test]
    fn next_page_exists_returns_false_when_link_header_does_not_contain_next_relation() {
        // GIVEN
        let mut headers = HeaderMap::new();
        headers.insert(
            LINK,
            HeaderValue::from_static(
                r#"<https://api.github.com/user/13575379/events?per_page=100&page=2>; rel="prev", <https://api.github.com/user/13575379/events?per_page=100&page=1>; rel="first""#
            ),
        );

        // WHEN
        let has_next_page = next_page_exists(&headers);

        // THEN
        assert!(!has_next_page);
    }

    #[test]
    fn next_page_exists_returns_false_for_malformed_link_header() {
        // GIVEN
        let mut headers = HeaderMap::new();
        headers.insert(LINK, HeaderValue::from_static("this is not a link header"));

        // WHEN
        let has_next_page = next_page_exists(&headers);

        // THEN
        assert!(!has_next_page);
    }

    #[test]
    fn next_page_exists_returns_false_for_malformed_next_relation() {
        // GIVEN
        let mut headers = HeaderMap::new();
        headers.insert(
            LINK,
            HeaderValue::from_static(
                "<https://api.github.com/user/13575379/events?per_page=100&page=4>; rel next",
            ),
        );

        // WHEN
        let has_next_page = next_page_exists(&headers);

        // THEN
        assert!(!has_next_page);
    }
}
