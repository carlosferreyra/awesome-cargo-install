#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
name = "test_clients"
edition = "2024"

[dependencies]
serde_json = "1"
regex = "1"
---
//! Test that tools are installable via `cargo binstall` (fast path) with fallback to
//! `cargo install --locked` (source compile). Classifies failures so the PR workflow
//! can comment helpfully.
//!
//! Usage (mutually exclusive sources):
//!     cargo +nightly -Zscript scripts/test_clients.rs -- --all [--output <log>]
//!     cargo +nightly -Zscript scripts/test_clients.rs -- --diff <ref> [--output <log>]
//!     cargo +nightly -Zscript scripts/test_clients.rs -- --tools '<json>' [--output <log>]
//!
//! --all   : test every tool in tools.json
//! --diff  : delegate to diff_tools.rs to extract added tools vs <ref>; exits 0 cleanly
//!           when no new tools are detected
//! --tools : explicit JSON array of {"package": "<name>", "execs": ["<bin>", ...]}
//!
//! --tools-path overrides the default `tools.json` location.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Output};

use regex::Regex;
use serde_json::{Value as Json, json};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let mut tools_arg: Option<String> = None;
    let mut all = false;
    let mut diff_ref: Option<String> = None;
    let mut tools_path = PathBuf::from("tools.json");
    let mut output = PathBuf::from("output.log");
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--tools" => {
                i += 1;
                tools_arg = args.get(i).cloned();
            }
            "--all" => {
                all = true;
            }
            "--diff" => {
                i += 1;
                diff_ref = args.get(i).cloned();
            }
            "--tools-path" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    tools_path = PathBuf::from(v);
                }
            }
            "--output" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    output = PathBuf::from(v);
                }
            }
            _ => {}
        }
        i += 1;
    }

    let modes = [tools_arg.is_some(), all, diff_ref.is_some()]
        .iter()
        .filter(|b| **b)
        .count();
    if modes != 1 {
        eprintln!("error: exactly one of --tools, --all, --diff <ref> is required");
        return ExitCode::FAILURE;
    }

    let tools_value: Json = if all {
        match load_all(&tools_path) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("error: {e}");
                return ExitCode::FAILURE;
            }
        }
    } else if let Some(r) = diff_ref {
        match load_diff(&r, &tools_path) {
            Ok(Some(v)) => v,
            Ok(None) => {
                println!("No new tools detected — nothing to test.");
                return ExitCode::SUCCESS;
            }
            Err(e) => {
                eprintln!("error: {e}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        let raw = tools_arg.unwrap();
        match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("error: --tools is not valid JSON: {e}");
                return ExitCode::FAILURE;
            }
        }
    };

    let Some(tools) = tools_value.as_array() else {
        eprintln!("error: tool source must be a JSON array");
        return ExitCode::FAILURE;
    };

    ensure_binstall();

    println!("Testing {} tool(s)...\n", tools.len());

    let network_re = Regex::new(
        r"(?i)(failed to fetch|connection error|could not connect|network|timeout|timed out|ssl error|certificate|http 5\d\d|503|502|504)"
    ).unwrap();
    let not_found_re = Regex::new(
        r"(?i)(could not find|not found|no matching package|does not exist on crates\.io|404|no such crate)"
    ).unwrap();
    let binary_re = Regex::new(
        r"(?i)(no such file|command not found|executable .* not found|not in.*path)"
    ).unwrap();

    let mut results: Vec<Json> = Vec::with_capacity(tools.len());

    for tool in tools {
        let package = tool.get("package").and_then(Json::as_str).unwrap_or("").to_string();
        let default_exec = package.split('[').next().unwrap_or(&package).to_string();
        let exec_name = tool
            .get("execs")
            .and_then(Json::as_array)
            .and_then(|a| a.first())
            .and_then(Json::as_str)
            .unwrap_or(&default_exec)
            .to_string();

        println!("  Testing: {package} ({exec_name})");

        let (ok, reason) = test_tool(&package, &exec_name, &network_re, &not_found_re, &binary_re);
        if ok {
            println!("    ✓ ok");
            results.push(json!({ "package": package, "success": true }));
        } else {
            println!("    ✗ failed ({reason})");
            results.push(json!({ "package": package, "success": false, "reason": reason }));
        }
    }

    let passed = results.iter().filter(|r| r["success"] == json!(true)).count();
    let failed: Vec<&Json> = results
        .iter()
        .filter(|r| r["success"] != json!(true))
        .collect();

    println!("\nResults: {} passed, {} failed", passed, failed.len());

    if !failed.is_empty() {
        let log = serde_json::to_string_pretty(&failed).unwrap_or_default();
        if let Err(e) = fs::write(&output, log) {
            eprintln!("warning: could not write {}: {e}", output.display());
        }
        println!("Failures written to {}", output.display());
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run(cmd: &[&str]) -> Output {
    Command::new(cmd[0])
        .args(&cmd[1..])
        .output()
        .unwrap_or_else(|e| Output {
            status: std::process::ExitStatus::default(),
            stdout: vec![],
            stderr: format!("{e}").into_bytes(),
        })
}

fn ensure_binstall() {
    let out = run(&["cargo", "binstall", "-V"]);
    if out.status.success() {
        return;
    }
    println!("cargo-binstall not found — installing via cargo install --locked...");
    let out = run(&["cargo", "install", "--locked", "cargo-binstall"]);
    if !out.status.success() {
        eprintln!(
            "warning: could not install cargo-binstall: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
}

fn classify(stdout: &str, stderr: &str, net: &Regex, nf: &Regex, bin: &Regex) -> String {
    let s = format!("{stdout}\n{stderr}");
    if net.is_match(&s) {
        "network".into()
    } else if nf.is_match(&s) {
        "not_found".into()
    } else if bin.is_match(&s) {
        "wrong_binary".into()
    } else {
        "execution_error".into()
    }
}

fn test_tool(
    package: &str,
    exec_name: &str,
    net: &Regex,
    nf: &Regex,
    bin: &Regex,
) -> (bool, String) {
    // Method 1: cargo binstall (fast, prebuilt)
    let out = run(&["cargo", "binstall", "--no-confirm", "--force", package]);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();

    if out.status.success() && which(exec_name) {
        return (true, String::new());
    }

    if net.is_match(&format!("{stdout}{stderr}")) {
        // Retry once for transient network failures.
        let out2 = run(&["cargo", "binstall", "--no-confirm", "--force", package]);
        if out2.status.success() && which(exec_name) {
            return (true, String::new());
        }
    }

    // Method 2: cargo install --locked (source compile fallback)
    let out2 = run(&["cargo", "install", "--locked", "--force", package]);
    let stdout2 = String::from_utf8_lossy(&out2.stdout).to_string();
    let stderr2 = String::from_utf8_lossy(&out2.stderr).to_string();

    if out2.status.success() && which(exec_name) {
        return (true, String::new());
    }

    // If the binary is missing but install succeeded, call that wrong_binary
    if out2.status.success() && !which(exec_name) {
        return (false, "wrong_binary".into());
    }

    (
        false,
        classify(
            &format!("{stdout}\n{stdout2}"),
            &format!("{stderr}\n{stderr2}"),
            net,
            nf,
            bin,
        ),
    )
}

fn load_all(path: &Path) -> Result<Json, String> {
    let raw = fs::read_to_string(path)
        .map_err(|e| format!("could not read {}: {e}", path.display()))?;
    let data: Json = serde_json::from_str(&raw)
        .map_err(|e| format!("invalid JSON in {}: {e}", path.display()))?;
    let mut out: Vec<Json> = Vec::new();
    if let Some(cats) = data.get("categories").and_then(Json::as_array) {
        for cat in cats {
            if let Some(obj) = cat.get("tools").and_then(Json::as_object) {
                for (pkg, info) in obj {
                    let execs = info
                        .get("execs")
                        .and_then(Json::as_array)
                        .cloned()
                        .unwrap_or_default();
                    out.push(json!({ "package": pkg, "execs": execs }));
                }
            }
        }
    }
    Ok(Json::Array(out))
}

fn load_diff(base: &str, tools_path: &Path) -> Result<Option<Json>, String> {
    let diff_script = locate_diff_script();

    let out = Command::new("cargo")
        .args([
            "+nightly",
            "-Zscript",
            diff_script.to_str().unwrap_or("scripts/diff_tools.rs"),
            "--",
            "--base",
            base,
            "--path",
            tools_path.to_str().unwrap_or("tools.json"),
        ])
        .output()
        .map_err(|e| format!("running diff_tools.rs: {e}"))?;

    match out.status.code() {
        Some(0) => {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let v: Json = serde_json::from_str(&s)
                .map_err(|e| format!("diff_tools output not valid JSON: {e}"))?;
            Ok(Some(v))
        }
        Some(2) => Ok(None),
        code => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            Err(format!(
                "diff_tools.rs exited with {code:?}: {stderr}"
            ))
        }
    }
}

fn locate_diff_script() -> PathBuf {
    for candidate in [
        PathBuf::from("scripts/diff_tools.rs"),
        PathBuf::from("diff_tools.rs"),
    ] {
        if candidate.exists() {
            return candidate;
        }
    }

    PathBuf::from(file!())
        .parent()
        .map(|p| p.join("diff_tools.rs"))
        .unwrap_or_else(|| PathBuf::from("scripts/diff_tools.rs"))
}

fn which(binary: &str) -> bool {
    let Ok(path) = std::env::var("PATH") else {
        return false;
    };
    for dir in path.split(':') {
        let p = std::path::Path::new(dir).join(binary);
        if p.exists() {
            return true;
        }
    }
    false
}
