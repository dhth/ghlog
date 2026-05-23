#![allow(dead_code)]

use anyhow::ensure;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Copy)]
pub struct EventLimit(usize);

impl EventLimit {
    pub fn get(&self) -> usize {
        self.0
    }
}

impl TryFrom<usize> for EventLimit {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        ensure!(value > 1, "limit must be greater than 1");

        Ok(Self(value))
    }
}

#[derive(Debug)]
pub struct Event {
    pub id: String,
    pub repo: Repo,
    pub payload: EventPayload,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
    pub url: String,
}

impl Repo {
    pub fn html_url(&self) -> String {
        format!("https://github.com/{}", self.name)
    }
}

#[derive(Debug)]
pub enum EventPayload {
    Push(PushEvent),
    Create(CreateEvent),
    Delete(DeleteEvent),
    IssueComment(IssueCommentEvent),
    PullRequest(PullRequestEvent),
    Release(ReleaseEvent),
}

#[derive(Debug, Deserialize)]
pub struct PushEvent {
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub head: String,
    pub before: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateEvent {
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub ref_type: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteEvent {
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub ref_type: String,
}

#[derive(Debug, Deserialize)]
pub struct IssueCommentEvent {
    pub action: String,
    pub issue: IssueCommentIssue,
    pub comment: IssueCommentComment,
}

#[derive(Debug, Deserialize)]
pub struct IssueCommentIssue {
    pub number: u64,
    pub html_url: String,
}

#[derive(Debug, Deserialize)]
pub struct IssueCommentComment {
    pub html_url: String,
}

#[derive(Debug, Deserialize)]
pub struct PullRequestEvent {
    pub action: String,
    pub pull_request: PullRequest,
}

#[derive(Debug, Deserialize)]
pub struct PullRequest {
    pub number: u64,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseEvent {
    pub action: String,
    pub release: Release,
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub html_url: String,
    pub tag_name: String,
    pub prerelease: bool,
    pub draft: bool,
}
