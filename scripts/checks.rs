#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
name = "checks"
edition = "2024"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
ureq = { version = "2", features = ["json"] }
---
//! Fast validation for tools.json. Logs to `checks.log` and exits non-zero on failure.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, ExitCode};
use std::time::Duration;

use serde::Deserialize;
use serde_json::Value as Json;

const CARGO_SUBCOMMANDS_CATEGORY: &str = "Cargo Subcommands";
const CRATES_IO: &str = "https://crates.io/api/v1/crates/{crate}";
const USER_AGENT: &str =
    "awesome-cargo-install (https://github.com/carlosferreyra/awesome-cargo-install)";

#[derive(Debug, Deserialize)]
struct Catalog {
    categories: Vec<Category>,
}

#[derive(Debug, Deserialize)]
struct Category {
    name: String,
    slug: String,
    tools: BTreeMap<String, Tool>,
}

#[derive(Debug, Deserialize)]
struct Tool {
    description: String,
    url: String,
    execs: Vec<String>,
    #[serde(default)]
    examples: Vec<Example>,
    #[serde(default = "default_crates_io")]
    crates_io: bool,
}

#[derive(Debug, Deserialize)]
struct Example {
    cmd: String,
    #[serde(default)]
    description: Option<String>,
}

fn default_crates_io() -> bool { true }

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
    let args: Vec<String> = std::env::args().collect();
    let mut diff_base: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--" => {}
            "--diff-base" => {
                i += 1;
                diff_base = args.get(i).cloned();
            }
            "-h" | "--help" => {
                println!("usage: checks [--diff-base <ref>]");
                return ExitCode::SUCCESS;
            }
            other => {
                eprintln!("error: unknown argument: {other}");
                return ExitCode::FAILURE;
            }
        }
        i += 1;
    }

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

    let catalog: Catalog = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(e) => {
            log.error(&format!("invalid tools.json schema: {e}"));
            return ExitCode::FAILURE;
        }
    };

    let mut all_valid = true;
    let mut seen_slugs = BTreeSet::new();
    let mut seen_tools: BTreeMap<String, String> = BTreeMap::new();

    if catalog.categories.is_empty() {
        all_valid = false;
        log.error("tools.json must contain at least one category");
    }

    for category in &catalog.categories {
        log.info(&format!("Checking category: {}", category.name));

        if category.name.trim().is_empty() {
            all_valid = false;
            log.error("Category name must not be empty");
        }

        if !is_slug(&category.slug) {
            all_valid = false;
            log.error(&format!(
                "Category '{}' has invalid slug '{}'",
                category.name, category.slug
            ));
        }
        if !seen_slugs.insert(category.slug.clone()) {
            all_valid = false;
            log.error(&format!("Duplicate category slug '{}'", category.slug));
        }

        if category.tools.is_empty() {
            all_valid = false;
            log.error(&format!("Category '{}' has no tools", category.name));
        }

        for (pkg_name, tool) in &category.tools {
            if let Some(first_category) = seen_tools.insert(pkg_name.clone(), category.name.clone()) {
                all_valid = false;
                log.error(&format!(
                    "Tool '{pkg_name}' appears in both '{first_category}' and '{}'",
                    category.name
                ));
            }

            if tool.description.trim().is_empty() {
                all_valid = false;
                log.error(&format!("Tool '{pkg_name}' in '{}' has empty 'description'", category.name));
            }

            if !(tool.url.starts_with("http://") || tool.url.starts_with("https://")) {
                all_valid = false;
                log.error(&format!("Tool '{pkg_name}' in '{}' has invalid URL format", category.name));
            }

            if tool.execs.is_empty() || tool.execs.iter().any(|s| s.trim().is_empty()) {
                all_valid = false;
                log.error(&format!(
                    "Tool '{pkg_name}' in '{}' must have at least one non-empty executable in 'execs'",
                    category.name
                ));
            }

            for (index, example) in tool.examples.iter().enumerate() {
                if example.cmd.trim().is_empty() {
                    all_valid = false;
                    log.error(&format!(
                        "Tool '{pkg_name}' in '{}': examples[{index}] is missing 'cmd'",
                        category.name
                    ));
                }
                if example.description.as_deref().is_some_and(|s| s.trim().is_empty()) {
                    all_valid = false;
                    log.error(&format!(
                        "Tool '{pkg_name}' in '{}': examples[{index}].description must not be empty",
                        category.name
                    ));
                }
            }

            if pkg_name.starts_with("cargo-") && category.name != CARGO_SUBCOMMANDS_CATEGORY {
                all_valid = false;
                log.error(&format!(
                    "Cargo subcommand '{pkg_name}' must be in the '{CARGO_SUBCOMMANDS_CATEGORY}' category"
                ));
            }
        }
    }

    if let Some(base) = diff_base {
        match validate_generated_metadata_unchanged(&base, "tools.json") {
            Ok(()) => {}
            Err(errors) => {
                all_valid = false;
                for error in errors {
                    log.error(&error);
                }
            }
        }

        match added_tools(&base, "tools.json") {
            Ok(added) => {
                log.info(&format!("Checking crates.io metadata for {} newly added tool(s)", added.len()));
                for package in added {
                    let Some((category, tool)) = find_tool(&catalog, &package) else {
                        all_valid = false;
                        log.error(&format!("New tool '{package}' was not found in parsed catalog"));
                        continue;
                    };
                    if !tool.crates_io {
                        log.info(&format!("Skipping crates.io lookup for {package} (crates_io=false)"));
                        continue;
                    }
                    if crate_exists(&package) {
                        log.info(&format!("crates.io lookup ok: {package}"));
                    } else {
                        all_valid = false;
                        log.error(&format!(
                            "New tool '{package}' in '{category}' could not be found on crates.io"
                        ));
                    }
                }
            }
            Err(e) => {
                all_valid = false;
                log.error(&e);
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

fn is_slug(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--")
}

fn find_tool<'a>(catalog: &'a Catalog, package: &str) -> Option<(&'a str, &'a Tool)> {
    for category in &catalog.categories {
        if let Some(tool) = category.tools.get(package) {
            return Some((&category.name, tool));
        }
    }
    None
}

fn added_tools(base: &str, path: &str) -> Result<Vec<String>, String> {
    let base_data = git_show_json(base, path)
        .ok_or_else(|| format!("could not read {path} at {base} for diff validation"))?;
    let head_raw = std::fs::read_to_string(path)
        .map_err(|e| format!("could not read {path}: {e}"))?;
    let head_data: Json = serde_json::from_str(&head_raw)
        .map_err(|e| format!("invalid JSON in {path}: {e}"))?;

    let base_tools = flatten(&base_data);
    let head_tools = flatten(&head_data);
    Ok(head_tools
        .iter()
        .filter(|name| !base_tools.contains(*name))
        .cloned()
        .collect())
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

fn flatten(data: &Json) -> BTreeSet<String> {
    let mut tools = BTreeSet::new();
    if let Some(cats) = data.get("categories").and_then(Json::as_array) {
        for cat in cats {
            if let Some(obj) = cat.get("tools").and_then(Json::as_object) {
                tools.extend(obj.keys().cloned());
            }
        }
    }
    tools
}

fn validate_generated_metadata_unchanged(base: &str, path: &str) -> Result<(), Vec<String>> {
    let Some(base_data) = git_show_json(base, path) else {
        return Err(vec![format!(
            "could not read {path} at {base} for generated metadata validation"
        )]);
    };
    let head_raw = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => return Err(vec![format!("could not read {path}: {e}")]),
    };
    let head_data: Json = match serde_json::from_str(&head_raw) {
        Ok(v) => v,
        Err(e) => return Err(vec![format!("invalid JSON in {path}: {e}")]),
    };

    let base_tools = flatten_tool_objects(&base_data);
    let head_tools = flatten_tool_objects(&head_data);
    let mut errors = Vec::new();

    for (name, head_tool) in head_tools {
        let base_tool = base_tools.get(&name);
        for field in ["version", "last_release"] {
            let head_value = head_tool.get(field);
            let base_value = base_tool.and_then(|tool| tool.get(field));
            if base_tool.is_none() && head_value.is_some() {
                errors.push(format!(
                    "New tool '{name}' includes generated field '{field}'; leave it for automation"
                ));
            } else if base_value != head_value {
                errors.push(format!(
                    "Tool '{name}' changes generated field '{field}'; leave it for automation"
                ));
            }
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

fn flatten_tool_objects(data: &Json) -> BTreeMap<String, Json> {
    let mut tools = BTreeMap::new();
    if let Some(cats) = data.get("categories").and_then(Json::as_array) {
        for cat in cats {
            if let Some(obj) = cat.get("tools").and_then(Json::as_object) {
                for (name, tool) in obj {
                    tools.insert(name.clone(), tool.clone());
                }
            }
        }
    }
    tools
}

fn crate_exists(krate: &str) -> bool {
    let url = CRATES_IO.replace("{crate}", krate);
    match ureq::get(&url)
        .set("User-Agent", USER_AGENT)
        .timeout(Duration::from_secs(10))
        .call()
    {
        Ok(_) => true,
        Err(ureq::Error::Status(404, _)) => false,
        Err(e) => {
            eprintln!("WARN  {krate}: crates.io lookup failed: {e}");
            false
        }
    }
}
