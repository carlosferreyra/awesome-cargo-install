#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
name = "latest_release"
edition = "2024"

[dependencies]
serde_json = { version = "1", features = ["preserve_order"] }
ureq = { version = "2", features = ["json"] }
---
//! Fetch latest release info from crates.io for each tool and update tools.json.

use std::path::PathBuf;
use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;

use serde_json::{Value as Json, json};

const CRATES_IO: &str = "https://crates.io/api/v1/crates/{crate}";
const USER_AGENT: &str =
    "awesome-cargo-install (https://github.com/carlosferreyra/awesome-cargo-install)";

fn repo_root() -> PathBuf {
    let mut p = PathBuf::from(file!());
    p.pop();
    p.pop();
    p.canonicalize().unwrap_or_else(|_| std::env::current_dir().unwrap())
}

fn fetch_latest(krate: &str) -> Option<(String, String)> {
    let url = CRATES_IO.replace("{crate}", krate);
    for attempt in 1..=3 {
        match ureq::get(&url).set("User-Agent", USER_AGENT).timeout(Duration::from_secs(15)).call() {
            Ok(resp) => {
                let body: Json = match resp.into_json() {
                    Ok(b) => b,
                    Err(e) => {
                        eprintln!("WARN  {krate}: invalid JSON: {e}");
                        return None;
                    }
                };
                let version = body
                    .pointer("/crate/max_stable_version")
                    .or_else(|| body.pointer("/crate/newest_version"))
                    .or_else(|| body.pointer("/crate/max_version"))
                    .and_then(Json::as_str)?
                    .to_string();

                // find matching version object for its created_at
                let created = body
                    .get("versions")
                    .and_then(Json::as_array)
                    .and_then(|vs| {
                        vs.iter().find(|v| {
                            v.get("num").and_then(Json::as_str) == Some(version.as_str())
                        })
                    })
                    .and_then(|v| v.get("created_at").and_then(Json::as_str))
                    .unwrap_or("")
                    .to_string();
                let date = created.get(..10).unwrap_or("").to_string();
                return Some((version, date));
            }
            Err(ureq::Error::Status(404, _)) => {
                eprintln!("WARN  {krate}: not found on crates.io");
                return None;
            }
            Err(e) => {
                eprintln!("WARN  {krate}: attempt {attempt} failed: {e}");
                sleep(Duration::from_millis(500 * attempt as u64));
            }
        }
    }
    None
}

fn main() -> ExitCode {
    let root = repo_root();
    let tools_path = root.join("tools.json");

    let raw = match std::fs::read_to_string(&tools_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: reading tools.json: {e}");
            return ExitCode::FAILURE;
        }
    };

    let mut data: Json = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: parsing tools.json: {e}");
            return ExitCode::FAILURE;
        }
    };

    let categories = match data.get_mut("categories").and_then(Json::as_array_mut) {
        Some(c) => c,
        None => {
            eprintln!("error: missing categories");
            return ExitCode::FAILURE;
        }
    };

    for category in categories.iter_mut() {
        let Some(tools) = category.get_mut("tools").and_then(Json::as_object_mut) else {
            continue;
        };
        for (name, tool) in tools.iter_mut() {
            // respect opt-out via `"crates_io": false`
            if tool.get("crates_io") == Some(&json!(false)) {
                println!("INFO  skipping {name} (crates_io=false)");
                continue;
            }
            let Some(obj) = tool.as_object_mut() else { continue };
            match fetch_latest(name) {
                Some((version, date)) => {
                    println!("INFO  {name:<30} {version}  {date}");
                    obj.insert("version".into(), Json::String(version));
                    obj.insert("last_release".into(), Json::String(date));
                }
                None => {
                    eprintln!("WARN  {name}: skipped (no data)");
                }
            }
            // be polite to crates.io
            sleep(Duration::from_millis(150));
        }
    }

    let mut out = match serde_json::to_string_pretty(&data) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: serializing tools.json: {e}");
            return ExitCode::FAILURE;
        }
    };
    out.push('\n');

    if let Err(e) = std::fs::write(&tools_path, out) {
        eprintln!("error: writing tools.json: {e}");
        return ExitCode::FAILURE;
    }

    println!("INFO  tools.json updated.");
    ExitCode::SUCCESS
}
