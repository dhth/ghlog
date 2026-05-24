use crate::domain::events::{Event, EventLimit, EventPayload};
use crate::domain::user::Username;
use crate::output::OutputFormat;
use crate::service::github::GithubService;
use chrono::{DateTime, Utc};

pub fn handle(
    username: &Username,
    limit: EventLimit,
    _output_format: OutputFormat,
) -> anyhow::Result<()> {
    let token = crate::auth::get_token()?;
    let service = GithubService::new(token)?;
    let events = service.get_public_events_for_user(username, limit)?;
    let reference_time = Utc::now();

    for event in &events {
        let line = format_event(event, &reference_time);

        println!("{line}");
    }

    Ok(())
}

fn format_event(event: &Event, reference_time: &DateTime<Utc>) -> String {
    let relative_time = get_humanized_date(&event.created_at, reference_time);
    let event_text = match &event.payload {
        EventPayload::Push(push) => {
            format!(
                "pushed to {} ({}) in {}",
                shorten_git_ref(&push.git_ref),
                shorten_commit_hash(&push.head),
                event.repo.name
            )
        }
        EventPayload::Create(create) => {
            let target = format_ref_target(&create.ref_type, &create.git_ref);
            format!("created {target} in {}", event.repo.name)
        }
        EventPayload::Delete(delete) => {
            let target = format_ref_target(&delete.ref_type, &delete.git_ref);
            format!("deleted {target} in {}", event.repo.name)
        }
        EventPayload::IssueComment(issue_comment) => format!(
            "commented on issue #{} in {}",
            issue_comment.issue.number, event.repo.name,
        ),
        EventPayload::PullRequest(pull_request) => {
            format!(
                "{} pull request #{} in {}",
                pull_request.action, pull_request.pull_request.number, event.repo.name,
            )
        }
        EventPayload::Release(release_event) => {
            let release_kind = if release_event.release.draft {
                "draft release"
            } else if release_event.release.prerelease {
                "prerelease"
            } else {
                "release"
            };

            format!(
                "{} {} {} in {}",
                release_event.action, release_kind, release_event.release.tag_name, event.repo.name,
            )
        }
    };

    format!("{relative_time:<13} {event_text}")
}

fn shorten_git_ref(git_ref: &str) -> &str {
    git_ref
        .strip_prefix("refs/heads/")
        .or_else(|| git_ref.strip_prefix("refs/tags/"))
        .unwrap_or(git_ref)
}

fn shorten_commit_hash(hash: &str) -> &str {
    hash.get(..7).unwrap_or(hash)
}

fn format_ref_target(ref_type: &str, git_ref: &str) -> String {
    if git_ref.is_empty() {
        ref_type.to_string()
    } else {
        format!("{ref_type} {}", shorten_git_ref(git_ref))
    }
}

fn get_humanized_date(dt: &DateTime<Utc>, reference: &DateTime<Utc>) -> String {
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
