#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
name = "check_and_comment"
edition = "2024"

[dependencies]
serde_json = "1"
ureq = { version = "2", features = ["json"] }
---
//! Post a failure comment, or an approving review, on the current PR depending on `$ACTION`.
//!
//! Env vars:
//!     GITHUB_TOKEN         — token with `pull-requests: write` permission
//!     GITHUB_REPOSITORY    — `owner/repo` (provided by GitHub Actions)
//!     GITHUB_REPOSITORY_OWNER — repo owner (fallback)
//!     PR_NUMBER            — pull request number
//!     ACTION               — `comment_failure` | `approve`
//!
//! Input (for `comment_failure`): reads the failure log from `output.log` in CWD.

use std::env;
use std::fs;
use std::process::ExitCode;
use std::time::Duration;

use serde_json::{Value as Json, json};

const API: &str = "https://api.github.com";
const API_VERSION: &str = "2022-11-28";

fn main() -> ExitCode {
    let Some(token) = env::var_os("GITHUB_TOKEN").map(|s| s.to_string_lossy().into_owned())
    else {
        eprintln!("error: GITHUB_TOKEN is required");
        return ExitCode::FAILURE;
    };
    let repo = env::var("GITHUB_REPOSITORY").unwrap_or_default();
    let (owner, repo_name) = repo
        .split_once('/')
        .map(|(o, r)| (o.to_string(), r.to_string()))
        .unwrap_or_else(|| {
            let o = env::var("GITHUB_REPOSITORY_OWNER").unwrap_or_default();
            (o, String::new())
        });

    let pr: u64 = env::var("PR_NUMBER")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let action = env::var("ACTION").unwrap_or_default();

    if owner.is_empty() || repo_name.is_empty() || pr == 0 {
        eprintln!("error: GITHUB_REPOSITORY and PR_NUMBER are required");
        return ExitCode::FAILURE;
    }

    let result = match action.as_str() {
        "comment_failure" => comment_failure(&token, &owner, &repo_name, pr),
        "approve" => approve(&token, &owner, &repo_name, pr),
        other => {
            eprintln!("error: unknown ACTION: {other}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn reason_message(reason: &str) -> &'static str {
    match reason {
        "network" => "> ⚠️ This may be a transient registry/network outage.\n> Please re-run the workflow before amending your PR.",
        "not_found" => "> ❌ The crate name does not exist on crates.io.\n> Please double-check the package name and try again.",
        "wrong_binary" => "> ❌ The crate installed but the declared binary was not found on `PATH`.\n> Please verify the `execs` field matches the actual binary name(s).",
        _ => "> ❌ The install ran but exited with an error.\n> Please verify with `cargo install --locked <crate>` locally.",
    }
}

fn comment_failure(
    token: &str,
    owner: &str,
    repo: &str,
    pr: u64,
) -> Result<(), String> {
    let failures: Vec<Json> = match fs::read_to_string("output.log") {
        Ok(s) => serde_json::from_str(&s)
            .unwrap_or_else(|_| vec![json!({"package": "unknown", "reason": "execution_error"})]),
        Err(_) => vec![json!({"package": "unknown", "reason": "execution_error"})],
    };

    let mut sections = Vec::with_capacity(failures.len());
    for f in &failures {
        let pkg = f.get("package").and_then(Json::as_str).unwrap_or("unknown");
        let reason = f.get("reason").and_then(Json::as_str).unwrap_or("execution_error");
        let pretty_reason = reason.replace('_', " ");
        sections.push(format!(
            "### `{pkg}` — {pretty_reason}\n{}",
            reason_message(reason)
        ));
    }

    let body = format!(
        "## ❌ Tool validation failed\n\n\
         The following crate(s) proposed in this PR could not be validated via \
         `cargo binstall` or `cargo install --locked`:\n\n\
         {body}\n\n---\n\
         _Please amend your PR and push again. The workflow will re-run automatically._",
        body = sections.join("\n\n")
    );

    let url = format!("{API}/repos/{owner}/{repo}/issues/{pr}/comments");
    post_json(&url, token, &json!({ "body": body }))
}

fn approve(token: &str, owner: &str, repo: &str, pr: u64) -> Result<(), String> {
    let url = format!("{API}/repos/{owner}/{repo}/pulls/{pr}/reviews");
    let payload = json!({
        "event": "APPROVE",
        "body": "✅ All proposed crates validated successfully via `cargo binstall`/`cargo install`. Ready for merge.",
    });
    post_json(&url, token, &payload)
}

fn post_json(url: &str, token: &str, body: &Json) -> Result<(), String> {
    let res = ureq::post(url)
        .set("Authorization", &format!("Bearer {token}"))
        .set("Accept", "application/vnd.github+json")
        .set("Content-Type", "application/json")
        .set("X-GitHub-Api-Version", API_VERSION)
        .set("User-Agent", "awesome-cargo-install-ci")
        .timeout(Duration::from_secs(20))
        .send_string(&body.to_string());

    match res {
        Ok(_) => Ok(()),
        Err(ureq::Error::Status(code, resp)) => {
            let msg = resp.into_string().unwrap_or_default();
            Err(format!("GitHub API error: {code} {msg}"))
        }
        Err(e) => Err(format!("request error: {e}")),
    }
}
