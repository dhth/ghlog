use crate::domain::events::{Event, EventPayload, ReleaseEvent};
use chrono::{DateTime, Utc};

pub struct Fragment {
    pub text: String,
    pub url: Option<String>,
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

#[derive(Clone, Copy)]
pub enum EventKind {
    Push,
    Create,
    Delete,
    Issues,
    IssueComment,
    PullRequest,
    Release,
}

impl EventKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Push => "push",
            Self::Create => "create",
            Self::Delete => "delete",
            Self::Issues => "issues",
            Self::IssueComment => "issue-comment",
            Self::PullRequest => "pull-request",
            Self::Release => "release",
        }
    }

    pub fn emoji(self) -> &'static str {
        match self {
            Self::Push => "⬆️",
            Self::Create => "🌱",
            Self::Delete => "🗑️",
            Self::Issues => "❗",
            Self::IssueComment => "💬",
            Self::PullRequest => "🔀",
            Self::Release => "📦",
        }
    }

    pub fn color(self) -> Color {
        match self {
            Self::Push => Color::Blue,
            Self::Create => Color::Green,
            Self::Delete => Color::Red,
            Self::Issues => Color::Yellow,
            Self::IssueComment => Color::Yellow,
            Self::PullRequest => Color::Purple,
            Self::Release => Color::Green,
        }
    }
}

pub struct EventPresentation {
    pub kind: EventKind,
    pub fragments: Vec<Fragment>,
}

impl From<&Event> for EventPresentation {
    fn from(event: &Event) -> Self {
        match &event.payload {
            EventPayload::Push(push) => Self {
                kind: EventKind::Push,
                fragments: vec![
                    text("pushed "),
                    link(
                        shorten_commit_hash(&push.head),
                        event.repo.url_for(&push.commit_path()),
                    ),
                    text(" to "),
                    link(push.ref_name(), event.repo.url_for(&push.ref_path())),
                    text(" in "),
                    link(&event.repo.name, event.repo.html_url()),
                ],
            },
            EventPayload::Create(create) => Self {
                kind: EventKind::Create,
                fragments: vec![
                    text(format!("created {} ", create.ref_type)),
                    link(create.ref_name(), event.repo.url_for(&create.ref_path())),
                    text(" in "),
                    link(&event.repo.name, event.repo.html_url()),
                ],
            },
            EventPayload::Delete(delete) => Self {
                kind: EventKind::Delete,
                fragments: vec![
                    text(format!("deleted {} ", delete.ref_type)),
                    text(delete.ref_name()),
                    text(" in "),
                    link(&event.repo.name, event.repo.html_url()),
                ],
            },
            EventPayload::Issues(issue) => Self {
                kind: EventKind::Issues,
                fragments: vec![
                    text(format!("{} issue ", issue.action)),
                    link(
                        format!("#{}", issue.issue.number),
                        issue.issue.html_url.clone(),
                    ),
                    text(" in "),
                    link(&event.repo.name, event.repo.html_url()),
                ],
            },
            EventPayload::IssueComment(issue_comment) => Self {
                kind: EventKind::IssueComment,
                fragments: vec![
                    text("commented on issue "),
                    link(
                        format!("#{}", issue_comment.issue.number),
                        issue_comment.issue.html_url.clone(),
                    ),
                    text(" in "),
                    link(&event.repo.name, event.repo.html_url()),
                ],
            },
            EventPayload::PullRequest(pull_request) => Self {
                kind: EventKind::PullRequest,
                fragments: vec![
                    text(format!("{} pull request ", pull_request.action)),
                    link(
                        format!("#{}", pull_request.pull_request.number),
                        event.repo.url_for(&pull_request.pull_request.path()),
                    ),
                    text(" in "),
                    link(&event.repo.name, event.repo.html_url()),
                ],
            },
            EventPayload::Release(release_event) => Self {
                kind: EventKind::Release,
                fragments: vec![
                    text(format!(
                        "{} {} ",
                        release_event.action,
                        release_kind(release_event)
                    )),
                    link(
                        &release_event.release.tag_name,
                        release_event.release.html_url.clone(),
                    ),
                    text(" in "),
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
    }
}

fn link(text: impl Into<String>, url: impl Into<String>) -> Fragment {
    Fragment {
        text: text.into(),
        url: Some(url.into()),
    }
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
