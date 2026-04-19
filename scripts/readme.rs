#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
name = "readme"
edition = "2024"

[dependencies]
serde_json = "1"
minijinja = { version = "2", features = ["loader", "json"] }
---
//! Generate README.md from tools.json using the Jinja2 template in `template/`.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use minijinja::{Environment, Value, path_loader};
use serde_json::Value as Json;

fn repo_root() -> PathBuf {
    // scripts/readme.rs -> repo root
    let mut p = PathBuf::from(file!());
    p.pop();
    p.pop();
    p.canonicalize().unwrap_or_else(|_| std::env::current_dir().unwrap())
}

fn escape_pipe(value: Value) -> String {
    value.as_str().unwrap_or("").replace('|', "\\|")
}

fn main() -> ExitCode {
    let root = repo_root();
    let tools_path = root.join("tools.json");
    let template_dir = root.join("template");
    let readme_path = root.join("README.md");

    let raw = match std::fs::read_to_string(&tools_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: reading {}: {e}", tools_path.display());
            return ExitCode::FAILURE;
        }
    };

    let data: Json = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: parsing tools.json: {e}");
            return ExitCode::FAILURE;
        }
    };

    let categories = data
        .get("categories")
        .and_then(Json::as_array)
        .cloned()
        .unwrap_or_default();

    let total_tools: usize = categories
        .iter()
        .map(|c| {
            c.get("tools")
                .and_then(Json::as_object)
                .map_or(0, |m| m.len())
        })
        .sum();

    let mut env = Environment::new();
    env.set_loader(path_loader(&template_dir));
    env.add_filter("escape_pipe", escape_pipe);

    let tmpl = match env.get_template("README.md.jinja2") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error: loading template: {e}");
            return ExitCode::FAILURE;
        }
    };

    let rendered = match tmpl.render(minijinja::context! {
        categories => Value::from_serialize(&categories),
        total_tools => total_tools,
    }) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: rendering template: {e}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(e) = std::fs::write(&readme_path, rendered) {
        eprintln!("error: writing README.md: {e}");
        return ExitCode::FAILURE;
    }

    println!("Successfully generated README at {}", readme_path.display());
    ExitCode::SUCCESS
}

#[allow(dead_code)]
fn _unused(_p: &Path) {}
