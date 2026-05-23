use anyhow::Context;
use std::env::VarError;
use std::process::Command;

const TOKEN_ENV_VAR: &str = "GHLOG_TOKEN";

pub fn get_token() -> anyhow::Result<String> {
    let token = std::env::var(TOKEN_ENV_VAR).or_else(|err| match err {
        VarError::NotPresent => get_token_from_gh().context(format!(
            r#"couldn't get a GitHub authentication token

ghlog tries to get this token in the following order:
- Read the environment variable {TOKEN_ENV_VAR} (this was not set)
- Running "gh auth token" (this failed)

Make sure ghlog can get a token from either one of these approaches."#
        )),
        VarError::NotUnicode(_) => Err(anyhow::anyhow!("{} is not valid unicode", TOKEN_ENV_VAR)),
    })?;

    Ok(token)
}

fn get_token_from_gh() -> anyhow::Result<String> {
    let output = Command::new("gh")
        .args(["auth", "token"])
        .output()
        .context(r#"couldn't get token from "gh" binary"#)?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            r#"couldn't get token from "gh" binary

stderr:
---
{}---"#,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(stdout.trim().to_string())
}
