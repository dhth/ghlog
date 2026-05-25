## Project
- `ghlog` is a command line tool that fetches a GitHub user's recent public activity and renders it in multiple output formats.
- The tool stays intentionally narrow: a small supported event set, thin command handlers, and format-specific rendering under `src/output/`.

## Common Commands
- Prefer `just` over raw `cargo` commands.
- Check: `just check`
- Run the CLI: `just run <flags>`
- Inspect available recipes: `just --list`

## Key Conventions
- When adding a new event, update parsing in `src/domain/events.rs` and every renderer that exposes event-specific copy.
- Output formats live under `src/output/`; keep format-specific rendering there instead of branching in command handlers.
- CLI orchestration stays thin in `src/cmds/`; GitHub API fetching belongs in `src/service/`.
- Prefer small, validated newtypes  like `Username` and `EventLimit` over passing raw strings and integers through the stack.
- HTML templates live in `src/output/assets/templates/`
