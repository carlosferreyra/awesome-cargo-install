#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
name = "checks"
edition = "2024"

[dependencies]
serde_json = "1"
---
//! Validate tools.json schema. Logs to `checks.log` and exits non-zero on failure.

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use serde_json::Value as Json;

fn repo_root() -> PathBuf {
    let mut p = PathBuf::from(file!());
    p.pop();
    p.pop();
    p.canonicalize().unwrap_or_else(|_| std::env::current_dir().unwrap())
}

struct Logger {
    file: File,
}

impl Logger {
    fn open(path: &std::path::Path) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .expect("open log");
        Self { file }
    }

    fn log(&mut self, level: &str, msg: &str) {
        let line = format!("{level} {msg}\n");
        let _ = self.file.write_all(line.as_bytes());
        eprintln!("{level} {msg}");
    }

    fn info(&mut self, msg: &str) { self.log("INFO ", msg); }
    fn error(&mut self, msg: &str) { self.log("ERROR", msg); }
}

fn main() -> ExitCode {
    let root = repo_root();
    let tools_path = root.join("tools.json");
    let mut log = Logger::open(&root.join("checks.log"));

    let raw = match std::fs::read_to_string(&tools_path) {
        Ok(s) => s,
        Err(e) => {
            log.error(&format!("could not read {}: {e}", tools_path.display()));
            return ExitCode::FAILURE;
        }
    };

    let data: Json = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(e) => {
            log.error(&format!("invalid JSON in tools.json: {e}"));
            return ExitCode::FAILURE;
        }
    };

    let categories = match data.get("categories").and_then(Json::as_array) {
        Some(c) => c,
        None => {
            log.error("tools.json: missing top-level `categories` array");
            return ExitCode::FAILURE;
        }
    };

    let mut all_valid = true;

    for category in categories {
        let name = category
            .get("name")
            .and_then(Json::as_str)
            .unwrap_or("<unnamed>");
        log.info(&format!("Checking category: {name}"));

        if category.get("slug").and_then(Json::as_str).unwrap_or("").is_empty() {
            all_valid = false;
            log.error(&format!("Category '{name}' is missing 'slug'"));
        }

        let tools = match category.get("tools").and_then(Json::as_object) {
            Some(t) if !t.is_empty() => t,
            _ => {
                all_valid = false;
                log.error(&format!("Category '{name}' has no tools"));
                continue;
            }
        };

        for (pkg_name, pkg_data) in tools {
            let required = ["description", "url", "execs"];
            let missing: Vec<&str> = required
                .iter()
                .filter(|f| !pkg_data.get(*f).map(|v| !v.is_null()).unwrap_or(false))
                .copied()
                .collect();

            if !missing.is_empty() {
                all_valid = false;
                log.error(&format!(
                    "Tool '{pkg_name}' in '{name}' is missing required fields: {missing:?}"
                ));
                continue;
            }

            for field in ["description", "url"] {
                if pkg_data.get(field).and_then(Json::as_str).unwrap_or("").is_empty() {
                    all_valid = false;
                    log.error(&format!("Tool '{pkg_name}' in '{name}' has empty '{field}'"));
                }
            }

            let execs = pkg_data.get("execs").and_then(Json::as_array);
            match execs {
                Some(arr) if !arr.is_empty()
                    && arr.iter().all(|v| v.as_str().map(|s| !s.is_empty()).unwrap_or(false)) => {}
                _ => {
                    all_valid = false;
                    log.error(&format!(
                        "Tool '{pkg_name}' in '{name}' must have at least one non-empty executable in 'execs'"
                    ));
                }
            }

            let url = pkg_data.get("url").and_then(Json::as_str).unwrap_or("");
            if !(url.starts_with("http://") || url.starts_with("https://")) {
                all_valid = false;
                log.error(&format!("Tool '{pkg_name}' in '{name}' has invalid URL format"));
            }

            if let Some(examples) = pkg_data.get("examples") {
                match examples.as_array() {
                    None => {
                        all_valid = false;
                        log.error(&format!(
                            "Tool '{pkg_name}' in '{name}': 'examples' must be a list"
                        ));
                    }
                    Some(list) => {
                        for (i, ex) in list.iter().enumerate() {
                            match ex.as_object() {
                                None => {
                                    all_valid = false;
                                    log.error(&format!(
                                        "Tool '{pkg_name}' in '{name}': examples[{i}] must be an object"
                                    ));
                                }
                                Some(obj) => {
                                    if obj.get("cmd")
                                        .and_then(Json::as_str)
                                        .unwrap_or("")
                                        .is_empty()
                                    {
                                        all_valid = false;
                                        log.error(&format!(
                                            "Tool '{pkg_name}' in '{name}': examples[{i}] is missing 'cmd'"
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if all_valid {
        log.info("All checks passed successfully!");
        ExitCode::SUCCESS
    } else {
        log.error("Some checks failed. Please review the logs.");
        ExitCode::FAILURE
    }
}
