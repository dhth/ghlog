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
        ensure!(
            (1..=300).contains(&value),
            "limit must be in the range [1, 300]"
        );

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

    pub fn url_for(&self, path: &str) -> String {
        format!("{}/{}", self.html_url(), path)
    }
}

#[derive(Debug)]
pub enum EventPayload {
    Push(PushEvent),
    Create(CreateEvent),
    Delete(DeleteEvent),
    IssueComment(IssueCommentEvent),
    Issues(IssuesEvent),
    PullRequest(PullRequestEvent),
    PullRequestReview(PullRequestReviewEvent),
    Release(ReleaseEvent),
}

#[derive(Debug, Deserialize)]
pub struct PushEvent {
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub head: String,
    pub before: String,
}

impl PushEvent {
    pub fn ref_name(&self) -> &str {
        strip_git_ref_prefix(&self.git_ref)
    }

    pub fn ref_path(&self) -> String {
        format!("tree/{}", self.ref_name())
    }

    pub fn commit_path(&self) -> String {
        format!("commit/{}", self.head)
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateEvent {
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub ref_type: String,
}

impl CreateEvent {
    pub fn ref_name(&self) -> &str {
        strip_git_ref_prefix(&self.git_ref)
    }

    pub fn ref_path(&self) -> String {
        format!("tree/{}", self.ref_name())
    }
}

#[derive(Debug, Deserialize)]
pub struct DeleteEvent {
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub ref_type: String,
}

impl DeleteEvent {
    pub fn ref_name(&self) -> &str {
        strip_git_ref_prefix(&self.git_ref)
    }
}

#[derive(Debug, Deserialize)]
pub struct IssuesEvent {
    pub action: String,
    pub issue: Issue,
}

#[derive(Debug, Deserialize)]
pub struct IssueCommentEvent {
    pub action: String,
    pub issue: Issue,
    pub comment: IssueCommentComment,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub number: u64,
    pub title: String,
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
pub struct PullRequestRepo {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct PullRequestBase {
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub repo: PullRequestRepo,
}

#[derive(Debug, Deserialize)]
pub struct PullRequestHead {
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub repo: PullRequestRepo,
}

#[derive(Debug, Deserialize)]
pub struct PullRequest {
    pub number: u64,
    pub base: PullRequestBase,
    pub head: PullRequestHead,
}

impl PullRequest {
    pub fn path(&self) -> String {
        format!("pull/{}", self.number)
    }
}

#[derive(Debug, Deserialize)]
pub struct PullRequestReviewEvent {
    pub action: String,
    pub review: PullRequestReview,
    pub pull_request: PullRequest,
}

#[derive(Debug, Deserialize)]
pub struct PullRequestReview {
    pub state: String,
    pub html_url: String,
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

fn strip_git_ref_prefix(git_ref: &str) -> &str {
    git_ref
        .strip_prefix("refs/heads/")
        .or_else(|| git_ref.strip_prefix("refs/tags/"))
        .unwrap_or(git_ref)
}
