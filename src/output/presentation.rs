use crate::domain::events::{Event, EventKind, EventPayload, PullRequest, ReleaseEvent};
use chrono::{DateTime, Utc};

pub struct Fragment {
    pub text: String,
    pub url: Option<String>,
    pub detail: Option<String>,
}

#[derive(Clone, Copy)]
pub enum Color {
    Gray,
    Blue,
    Green,
    Yellow,
    Purple,
    Red,
}

impl EventKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Delete => "delete",
            Self::IssueComment => "issue-comment",
            Self::Issues => "issues",
            Self::PullRequest => "pull-request",
            Self::PullRequestReview => "pull-request-review",
            Self::Push => "push",
            Self::Release => "release",
        }
    }

    pub fn emoji(self) -> &'static str {
        match self {
            Self::Create => "🌱",
            Self::Delete => "🗑️",
            Self::IssueComment => "💬",
            Self::Issues => "❗",
            Self::PullRequest => "🔀",
            Self::PullRequestReview => "📝",
            Self::Push => "⬆️",
            Self::Release => "📦",
        }
    }

    pub fn color(self) -> Color {
        match self {
            Self::Create => Color::Green,
            Self::Delete => Color::Red,
            Self::IssueComment => Color::Yellow,
            Self::Issues => Color::Yellow,
            Self::PullRequest => Color::Purple,
            Self::PullRequestReview => Color::Purple,
            Self::Push => Color::Blue,
            Self::Release => Color::Green,
        }
    }
}

pub struct EventPresentation {
    pub created_at: DateTime<Utc>,
    pub kind: EventKind,
    pub fragments: Vec<Fragment>,
}

impl From<&Event> for EventPresentation {
    fn from(event: &Event) -> Self {
        let created_at = event.created_at;
        let kind = event.kind();

        match &event.payload {
            EventPayload::Push(push) => Self {
                created_at,
                kind,
                fragments: vec![
                    text("pushed"),
                    link(
                        shorten_commit_hash(&push.head),
                        event.repo.url_for(&push.commit_path()),
                    ),
                    text("to"),
                    link(push.ref_name(), event.repo.url_for(&push.ref_path())),
                    text("in"),
                    link(&event.repo.name, event.repo.html_url()),
                ],
            },
            EventPayload::Create(create) => Self {
                created_at,
                kind,
                fragments: vec![
                    text("created"),
                    text(create.ref_type.to_string()),
                    link(create.ref_name(), event.repo.url_for(&create.ref_path())),
                    text("in"),
                    link(&event.repo.name, event.repo.html_url()),
                ],
            },
            EventPayload::Delete(delete) => Self {
                created_at,
                kind,
                fragments: vec![
                    text("deleted"),
                    text(delete.ref_type.to_string()),
                    text(delete.ref_name()),
                    text("in"),
                    link(&event.repo.name, event.repo.html_url()),
                ],
            },
            EventPayload::Issues(issue) => Self {
                created_at,
                kind,
                fragments: vec![
                    text(issue.action.to_string()),
                    text("issue"),
                    link_with_detail(
                        format!("#{}", issue.issue.number),
                        issue.issue.html_url.clone(),
                        issue.issue.title.to_string(),
                    ),
                    text("in"),
                    link(&event.repo.name, event.repo.html_url()),
                ],
            },
            EventPayload::IssueComment(issue_comment) => Self {
                created_at,
                kind,
                fragments: vec![
                    text("commented on issue"),
                    link_with_detail(
                        format!("#{}", issue_comment.issue.number),
                        issue_comment.issue.html_url.clone(),
                        issue_comment.issue.title.to_string(),
                    ),
                    text("in"),
                    link(&event.repo.name, event.repo.html_url()),
                ],
            },
            EventPayload::PullRequest(pull_request) => Self {
                created_at,
                kind,
                fragments: vec![
                    text(&pull_request.action),
                    text("pull request"),
                    link_with_detail(
                        format!("#{}", pull_request.pull_request.number),
                        event.repo.url_for(&pull_request.pull_request.path()),
                        pull_request_refs(&pull_request.pull_request),
                    ),
                    text("in"),
                    link(&event.repo.name, event.repo.html_url()),
                ],
            },
            EventPayload::PullRequestReview(pull_request_review) => Self {
                created_at,
                kind,
                fragments: vec![
                    link(
                        pull_request_review_verb(
                            &pull_request_review.action,
                            &pull_request_review.review.state,
                        ),
                        pull_request_review.review.html_url.clone(),
                    ),
                    text("pull request"),
                    link_with_detail(
                        format!("#{}", pull_request_review.pull_request.number),
                        event.repo.url_for(&pull_request_review.pull_request.path()),
                        pull_request_refs(&pull_request_review.pull_request),
                    ),
                    text("in"),
                    link(&event.repo.name, event.repo.html_url()),
                ],
            },
            EventPayload::Release(release_event) => Self {
                created_at,
                kind,
                fragments: vec![
                    text(release_event.action.to_string()),
                    text(release_kind(release_event)),
                    link(
                        &release_event.release.tag_name,
                        release_event.release.html_url.clone(),
                    ),
                    text("in"),
                    link(&event.repo.name, event.repo.html_url()),
                ],
            },
        }
    }
}

fn text(text: impl Into<String>) -> Fragment {
    Fragment {
        text: text.into(),
        url: None,
        detail: None,
    }
}

fn link(text: impl Into<String>, url: impl Into<String>) -> Fragment {
    Fragment {
        text: text.into(),
        url: Some(url.into()),
        detail: None,
    }
}

fn link_with_detail(
    text: impl Into<String>,
    url: impl Into<String>,
    detail: impl Into<String>,
) -> Fragment {
    Fragment {
        text: text.into(),
        url: Some(url.into()),
        detail: Some(detail.into()),
    }
}

fn pull_request_refs(pull_request: &PullRequest) -> String {
    format!(
        "{}:{} ← {}:{}",
        pull_request.base.repo.name,
        pull_request.base.git_ref,
        pull_request.head.repo.name,
        pull_request.head.git_ref,
    )
}

fn release_kind(release_event: &ReleaseEvent) -> &'static str {
    if release_event.release.draft {
        "draft release"
    } else if release_event.release.prerelease {
        "prerelease"
    } else {
        "release"
    }
}

fn shorten_commit_hash(hash: &str) -> &str {
    hash.get(..7).unwrap_or(hash)
}

fn pull_request_review_verb(action: &str, state: &str) -> &'static str {
    match action {
        "created" => match state {
            "approved" => "approved",
            "changes_requested" => "requested changes in",
            "commented" => "commented on",
            _ => "reviewed",
        },
        "dismissed" => "dismissed review on",
        "edited" => "edited review on",
        _ => "reviewed",
    }
}

pub fn humanized_date(dt: &DateTime<Utc>, reference: &DateTime<Utc>) -> String {
    let duration = reference.signed_duration_since(dt);
    let seconds = duration.num_seconds();

    if seconds < 0 {
        return "-".to_string();
    }

    if seconds < 60 {
        return "just now".to_string();
    }

    let minutes = duration.num_minutes();
    if minutes < 60 {
        return format!("{}m ago", minutes);
    }

    let hours = duration.num_hours();
    if hours < 24 {
        return format!("{}h ago", hours);
    }

    format!("{}d ago", duration.num_days())
}
