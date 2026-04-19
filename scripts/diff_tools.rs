#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
name = "diff_tools"
edition = "2024"

[dependencies]
serde_json = "1"
---
//! Extract newly added tools from a git diff of tools.json.
//!
//! Compares the current branch against a base ref (default: origin/main) and
//! outputs a JSON array of added tool objects suitable for passing to test_clients.rs.
//!
//! Usage:
//!     cargo +nightly -Zscript scripts/diff_tools.rs                    # diff against origin/main
//!     cargo +nightly -Zscript scripts/diff_tools.rs -- --base origin/main
//!
//! Output (stdout):
//!     [{"package": "ripgrep", "execs": ["rg"]}, ...]
//!
//! Exit codes:
//!     0  added tools found, JSON written to stdout
//!     1  error
//!     2  no added tools detected (e.g. only removals or metadata changes)

use std::collections::BTreeMap;
use std::process::{Command, ExitCode};

use serde_json::{Value as Json, json};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let mut base = "origin/main".to_string();
    let mut path = "tools.json".to_string();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--base" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    base = v.clone();
                }
            }
            "--path" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    path = v.clone();
                }
            }
            "-h" | "--help" => {
                println!("usage: diff_tools [--base <ref>] [--path <tools.json>]");
                return ExitCode::SUCCESS;
            }
            _ => {}
        }
        i += 1;
    }

    let base_data = match git_show_json(&base, &path) {
        Some(d) => d,
        None => {
            eprintln!("Error: could not read {path} at {base}");
            return ExitCode::FAILURE;
        }
    };

    let head_raw = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: could not read {path}: {e}");
            return ExitCode::FAILURE;
        }
    };
    let head_data: Json = match serde_json::from_str(&head_raw) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: invalid JSON in {path}: {e}");
            return ExitCode::FAILURE;
        }
    };

    let base_tools = flatten(&base_data);
    let head_tools = flatten(&head_data);

    let added: Vec<(&String, &Json)> = head_tools
        .iter()
        .filter(|(k, _)| !base_tools.contains_key(*k))
        .collect();

    if added.is_empty() {
        eprintln!("No new tools detected.");
        return ExitCode::from(2);
    }

    let out: Vec<Json> = added
        .into_iter()
        .map(|(pkg, data)| {
            let execs = data
                .get("execs")
                .and_then(Json::as_array)
                .cloned()
                .unwrap_or_default();
            json!({ "package": pkg, "execs": execs })
        })
        .collect();

    match serde_json::to_string(&out) {
        Ok(s) => {
            println!("{s}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Error: serializing output: {e}");
            ExitCode::FAILURE
        }
    }
}

fn git_show_json(r#ref: &str, path: &str) -> Option<Json> {
    let out = Command::new("git")
        .args(["show", &format!("{ref}:{path}", r#ref = r#ref)])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    serde_json::from_slice(&out.stdout).ok()
}

fn flatten(data: &Json) -> BTreeMap<String, Json> {
    let mut tools = BTreeMap::new();
    if let Some(cats) = data.get("categories").and_then(Json::as_array) {
        for cat in cats {
            if let Some(obj) = cat.get("tools").and_then(Json::as_object) {
                for (k, v) in obj {
                    tools.insert(k.clone(), v.clone());
                }
            }
        }
    }
    tools
}
