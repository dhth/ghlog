use crate::domain::events::{Event, EventLimit};
use crate::domain::user::Username;
use anyhow::{Context, ensure};
use reqwest::blocking::Client;

const GITHUB_API_VERSION: &str = "2026-03-10";

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

        let events = serde_json::from_str(&body).context("couldn't parse response from GitHub")?;

        Ok(events)
    }
}
