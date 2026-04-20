# Awesome Cargo Install

<div align="center">
    <img width="500" height="350" src="https://github.com/sindresorhus/awesome/raw/main/media/logo.svg" alt="Awesome">
    <br>
    <a href="https://awesome.re">
        <img src="https://awesome.re/badge.svg" alt="Awesome">
    </a>
    <p>A collection of Awesome Rust CLI Tools available via <code>cargo install</code>.</p>
    <p>Use this list to find useful Rust command-line tools you can install directly from crates.io
    with <code>cargo install &lt;crate&gt;</code> (or <code>cargo binstall &lt;crate&gt;</code> for
    precompiled binaries). The list is curated and maintained by the community, so feel free to
    contribute by adding your own favorite tools.</p>
    <p>
        <img src="https://img.shields.io/github/contributors/carlosferreyra/awesome-cargo-install" alt="Contributors">
        <img src="https://img.shields.io/github/license/carlosferreyra/awesome-cargo-install" alt="License">
        <img src="https://badges.pufler.dev/visits/carlosferreyra/awesome-cargo-install" alt="Visits">
        <img src="https://img.shields.io/github/stars/carlosferreyra/awesome-cargo-install" alt="Stars">
    </p>
    <a href="https://github.com/carlosferreyra/awesome-cargo-install/actions/workflows/ci.yml">
        <img src="https://github.com/carlosferreyra/awesome-cargo-install/actions/workflows/ci.yml/badge.svg" alt="Validation and Sync">
    </a>
</div>

Inspired by <a href="https://github.com/rust-unofficial/awesome-rust">awesome-rust</a>,
<a href="https://github.com/carlosferreyra/awesome-uvx">awesome-uvx</a> and
<a href="https://github.com/carlosferreyra/awesome-bunx">awesome-bunx</a>.

> **Tip:** for faster installation (precompiled binaries, no compilation), install
> [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) first, then replace
> `cargo install <crate>` with `cargo binstall <crate>`.


- [Command-line Utilities](#command-line-utilities)
- [Cargo Subcommands](#cargo-subcommands)
- [System Monitoring & Infrastructure](#system-monitoring-and-infrastructure)
- [Git & Version Control](#git-and-version-control)
- [File Management](#file-management)
- [Data Processing](#data-processing)
- [Shells, Prompts & Terminals](#shells-prompts-and-terminals)
- [Build Tools & Task Runners](#build-tools-and-task-runners)
- [Documentation & Writing](#documentation-and-writing)
- [Editors & IDE Helpers](#editors-and-ide-helpers)
- [Security & Auditing](#security-and-auditing)
- [Images & Media](#images-and-media)
- [Miscellaneous](#miscellaneous)


## Command-line Utilities

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [bat](https://github.com/sharkdp/bat) | A `cat(1)` clone with syntax highlighting and Git integration | ```bat``` | 0.26.1<br>2025-12-02 |
| [choose](https://github.com/theryangeary/choose) | A human-friendly and fast alternative to `cut` and (sometimes) `awk` | ```choose``` | 1.3.7<br>2025-08-26 |
| [du-dust](https://github.com/bootandy/dust) | A more intuitive version of `du` in Rust | ```dust``` | 1.2.4<br>2026-01-08 |
| [eza](https://github.com/eza-community/eza) | A modern, maintained replacement for `ls` | ```eza``` | 0.23.4<br>2025-10-03 |
| [fd-find](https://github.com/sharkdp/fd) | A simple, fast and user-friendly alternative to `find`<br><details><summary><strong><a href="#">Examples</a></strong></summary><br>`cargo install fd-find` — Install fd-find (provides the `fd` binary)<br></details> | ```fd``` | 10.4.2<br>2026-03-10 |
| [grex](https://github.com/pemistahl/grex) | Generate regular expressions from user-provided test cases | ```grex``` | 1.4.6<br>2025-11-14 |
| [hyperfine](https://github.com/sharkdp/hyperfine) | A command-line benchmarking tool with statistical analysis<br><details><summary><strong><a href="#">Examples</a></strong></summary><br>`hyperfine 'sleep 0.1' 'sleep 0.2'` — Benchmark two commands head-to-head<br></details> | ```hyperfine``` | 1.20.0<br>2025-11-18 |
| [ouch](https://github.com/ouch-org/ouch) | Painless compression and decompression for your terminal | ```ouch``` | 0.6.1<br>2025-04-21 |
| [pastel](https://github.com/sharkdp/pastel) | A command-line tool to generate, analyze, convert and manipulate colors | ```pastel``` | 0.12.0<br>2026-02-14 |
| [ripgrep](https://github.com/BurntSushi/ripgrep) | Recursively search directories for a regex pattern — a faster grep<br><details><summary><strong><a href="#">Examples</a></strong></summary><br>`cargo install ripgrep` — Install ripgrep (provides the `rg` binary)<br>`rg 'fn main' --type rust` — Search for `fn main` only in Rust files<br></details> | ```rg``` | 15.1.0<br>2025-10-22 |
| [sd](https://github.com/chmln/sd) | An intuitive find & replace CLI — an alternative to `sed` | ```sd``` | 1.0.0<br>2023-11-08 |
| [tealdeer](https://github.com/dbrgn/tealdeer) | A very fast implementation of `tldr` in Rust | ```tldr``` | 1.8.1<br>2025-11-11 |
| [tokei](https://github.com/XAMPPRocky/tokei) | Count lines of code across a codebase, grouped by language | ```tokei``` | 14.0.0<br>2025-12-30 |
| [tre-command](https://github.com/dduan/tre) | A modern alternative to `tree` with git/gitignore awareness | ```tre``` | 0.4.0<br>2022-06-19 |
| [zoxide](https://github.com/ajeetdsouza/zoxide) | A smarter `cd` command — jump to frecent directories | ```zoxide``` | 0.9.9<br>2026-01-31 |


## Cargo Subcommands

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [cargo-audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit) | Audit `Cargo.lock` for crates with security vulnerabilities | ```cargo-audit``` | 0.22.1<br>2026-02-04 |
| [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) | Install prebuilt Rust binaries without having to compile from source | ```cargo-binstall``` | 1.18.1<br>2026-04-13 |
| [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat) | Find out what takes most of the space in your executable | ```cargo-bloat``` | 0.12.1<br>2024-05-10 |
| [cargo-cache](https://github.com/matthiaskrgr/cargo-cache) | Manage cargo cache (`~/.cargo/`), print sizes and remove directories | ```cargo-cache``` | 0.8.3<br>2022-09-11 |
| [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) | A cargo-subcommand to speed up Rust Docker builds using Docker layer caching | ```cargo-chef``` | 0.1.77<br>2026-03-03 |
| [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) | Lint your dependencies: licenses, bans, advisories, sources | ```cargo-deny``` | 0.19.4<br>2026-04-15 |
| [cargo-edit](https://github.com/killercup/cargo-edit) | Managing cargo dependencies from the command line (`cargo add/rm/upgrade`) | ```cargo-upgrade```, ```cargo-set-version``` | 0.13.10<br>2026-04-17 |
| [cargo-expand](https://github.com/dtolnay/cargo-expand) | Shows the result of macro expansion and `#[derive]` expansion | ```cargo-expand``` | 1.0.121<br>2026-02-12 |
| [cargo-flamegraph](https://github.com/flamegraph-rs/flamegraph) | Easy flamegraphs for Rust projects and anything else, without Perl or pipes | ```cargo-flamegraph```, ```flamegraph``` | — |
| [cargo-generate](https://github.com/cargo-generate/cargo-generate) | A developer tool to help you get up and running quickly with a new Rust project | ```cargo-generate``` | 0.23.8<br>2026-04-01 |
| [cargo-hack](https://github.com/taiki-e/cargo-hack) | A cargo subcommand for testing feature flag combinations | ```cargo-hack``` | 0.6.44<br>2026-03-20 |
| [cargo-info](https://gitlab.com/imp/cargo-info) | Show crate information from the terminal, pulled from crates.io | ```cargo-info``` | 0.7.7<br>2024-09-06 |
| [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) | Cargo subcommand to easily use LLVM source-based code coverage | ```cargo-llvm-cov``` | 0.8.5<br>2026-03-20 |
| [cargo-make](https://github.com/sagiegurari/cargo-make) | Rust task runner and build tool — supports tasks, dependencies, and conditions | ```cargo-make```, ```makers``` | 0.37.24<br>2025-01-18 |
| [cargo-modules](https://github.com/regexident/cargo-modules) | A cargo plugin for showing a tree-like overview of a crate's modules | ```cargo-modules``` | 0.26.0<br>2026-04-18 |
| [cargo-msrv](https://github.com/foresterre/cargo-msrv) | Find the Minimum Supported Rust Version for your project | ```cargo-msrv``` | 0.19.3<br>2026-03-25 |
| [cargo-nextest](https://github.com/nextest-rs/nextest) | A next-generation test runner for Rust — up to 3x faster than `cargo test` | ```cargo-nextest``` | 0.9.133<br>2026-04-14 |
| [cargo-outdated](https://github.com/kbknapp/cargo-outdated) | Displays when Rust dependencies are out of date | ```cargo-outdated``` | 0.19.0<br>2026-04-14 |
| [cargo-release](https://github.com/crate-ci/cargo-release) | Cargo subcommand for smoothly releasing a new version of your crate | ```cargo-release``` | 1.1.2<br>2026-03-24 |
| [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks) | Scan your Rust crate for semver violations | ```cargo-semver-checks``` | 0.47.0<br>2026-03-08 |
| [cargo-tarpaulin](https://github.com/xd009642/tarpaulin) | A code coverage tool for Rust projects | ```cargo-tarpaulin``` | 0.35.2<br>2026-02-19 |
| [cargo-udeps](https://github.com/est31/cargo-udeps) | Find unused dependencies in your Cargo.toml | ```cargo-udeps``` | 0.1.60<br>2025-11-05 |
| [cargo-update](https://github.com/nabijaczleweli/cargo-update) | Cargo subcommand for checking and applying updates to installed executables | ```cargo-install-update``` | 20.0.0<br>2026-04-06 |
| [cargo-watch](https://github.com/watchexec/cargo-watch) | Watches over your project's source for changes, runs commands when they occur<br><details><summary><strong><a href="#">Examples</a></strong></summary><br>`cargo install cargo-watch` — Install cargo-watch<br>`cargo watch -x check -x test` — Re-run `cargo check` and `cargo test` on every source change<br></details> | ```cargo-watch``` | 8.5.3<br>2024-10-02 |


## System Monitoring & Infrastructure

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [bandwhich](https://github.com/imsnif/bandwhich) | Terminal bandwidth utilization tool | ```bandwhich``` | 0.23.1<br>2024-10-08 |
| [bottom](https://github.com/ClementTsang/bottom) | Yet another cross-platform graphical process/system monitor | ```btm``` | 0.12.3<br>2026-01-01 |
| [dog](https://github.com/ogham/dog) | A command-line DNS client — like `dig` but with fewer surprises | ```dog``` | 0.1.0<br>2019-01-08 |
| [gping](https://github.com/orf/gping) | Ping, but with a graph | ```gping``` | 1.20.1<br>2025-08-15 |
| [oha](https://github.com/hatoo/oha) | HTTP load generator with a TUI, inspired by rakyll/hey | ```oha``` | 1.14.0<br>2026-02-28 |
| [procs](https://github.com/dalance/procs) | A modern replacement for `ps` written in Rust | ```procs``` | 0.14.11<br>2026-02-27 |
| [rustscan](https://github.com/RustScan/RustScan) | The modern port scanner — fast and extensible with a scripting engine | ```rustscan``` | 2.4.1<br>2025-02-23 |
| [topgrade](https://github.com/topgrade-rs/topgrade) | Upgrade all the things — a unified upgrade runner for many package managers | ```topgrade``` | 17.3.0<br>2026-04-09 |
| [xh](https://github.com/ducaale/xh) | A friendly and fast tool for sending HTTP requests — HTTPie clone in Rust | ```xh```, ```xhs``` | 0.25.3<br>2025-12-16 |


## Git & Version Control

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [git-delta](https://github.com/dandavison/delta) | A syntax-highlighting pager for git, diff, grep, and blame output | ```delta``` | 0.19.2<br>2026-03-28 |
| [gitui](https://github.com/gitui-org/gitui) | Blazing fast terminal-UI for git, written in Rust | ```gitui``` | 0.28.1<br>2026-03-24 |
| [jujutsu](https://github.com/martinvonz/jj) | A Git-compatible VCS that is both simple and powerful | ```jj``` | 0.7.2<br>2025-12-08 |
| [onefetch](https://github.com/o2sh/onefetch) | Git repository summary on your terminal — languages, stats and commit info | ```onefetch``` | 2.27.1<br>2026-03-19 |


## File Management

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [broot](https://github.com/Canop/broot) | A new way to see and navigate directory trees | ```broot``` | 1.56.2<br>2026-03-26 |
| [felix](https://github.com/kyoheiu/felix) | A tui file manager with Vim-like key mapping | ```fx``` | 2.16.1<br>2025-04-12 |
| [nomino](https://github.com/yaa110/nomino) | Batch rename utility for developers | ```nomino``` | 1.6.4<br>2025-08-07 |
| [xplr](https://github.com/sayanarijit/xplr) | A hackable, minimal, fast TUI file explorer | ```xplr``` | 1.1.0<br>2025-12-08 |
| [yazi-fm](https://github.com/sxyazi/yazi) | Blazing fast terminal file manager written in Rust, based on async I/O | ```yazi```, ```ya``` | 26.1.22<br>2026-01-22 |


## Data Processing

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [jaq](https://github.com/01mf02/jaq) | A clone of jq — a JSON data processing tool | ```jaq``` | 3.0.0<br>2026-03-27 |
| [jql](https://github.com/yamafaktory/jql) | A JSON Query Language CLI tool | ```jql``` | 8.1.2<br>2026-03-18 |
| [qsv](https://github.com/dathere/qsv) | CSVs sliced, diced & analyzed — a blazing-fast data-wrangling toolkit | ```qsv``` | 16.1.0<br>2026-02-15 |
| [sqlx-cli](https://github.com/launchbadge/sqlx) | Command-line utility for SQLx — creates DBs, runs migrations, prepares offline data | ```sqlx```, ```cargo-sqlx``` | 0.8.6<br>2025-05-19 |
| [xan](https://github.com/medialab/xan) | The CSV magician — process CSV files from the command line | ```xan``` | 0.57.1<br>2026-04-15 |
| [xsv](https://github.com/BurntSushi/xsv) | A fast CSV command-line toolkit written in Rust | ```xsv``` | 0.13.0<br>2018-05-12 |


## Shells, Prompts & Terminals

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [atuin](https://github.com/atuinsh/atuin) | Magical shell history — sync, search, and backup your shell history | ```atuin``` | 18.15.2<br>2026-04-16 |
| [nu](https://github.com/nushell/nushell) | Nushell — a new type of shell where data is structured | ```nu``` | 0.112.2<br>2026-04-15 |
| [starship](https://github.com/starship/starship) | The minimal, blazing-fast, and infinitely customizable prompt for any shell | ```starship``` | 1.25.0<br>2026-04-18 |
| [zellij](https://github.com/zellij-org/zellij) | A terminal workspace with batteries included — multiplexer alternative to tmux | ```zellij``` | 0.44.1<br>2026-04-07 |


## Build Tools & Task Runners

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [bacon](https://github.com/Canop/bacon) | A background Rust code checker — fast and low-friction | ```bacon``` | 3.22.0<br>2026-01-16 |
| [just](https://github.com/casey/just) | A handy way to save and run project-specific commands — a modern `make` | ```just``` | 1.50.0<br>2026-04-19 |
| [mask](https://github.com/jacobdeichert/mask) | A CLI task runner defined by a simple markdown file | ```mask``` | 0.11.7<br>2026-01-10 |
| [watchexec-cli](https://github.com/watchexec/watchexec) | Executes commands in response to file modifications | ```watchexec``` | 2.5.1<br>2026-03-30 |


## Documentation & Writing

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [mdbook](https://github.com/rust-lang/mdBook) | Create book-style documentation from Markdown files | ```mdbook``` | 0.5.2<br>2025-12-11 |
| [mdbook-linkcheck](https://github.com/Michael-F-Bryan/mdbook-linkcheck) | A backend for mdbook that validates links | ```mdbook-linkcheck``` | 0.7.7<br>2022-10-03 |
| [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid) | A preprocessor for mdbook that renders Mermaid diagrams | ```mdbook-mermaid``` | 0.17.0<br>2025-11-18 |
| [typst-cli](https://github.com/typst/typst) | A new markup-based typesetting system for the sciences | ```typst``` | 0.14.2<br>2025-12-12 |


## Editors & IDE Helpers

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [helix-term](https://github.com/helix-editor/helix) | A post-modern modal text editor — like Vim but with better defaults | ```hx``` | 0.0.0<br>2021-06-02 |


## Security & Auditing

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [feroxbuster](https://github.com/epi052/feroxbuster) | A fast, simple, recursive content discovery tool written in Rust | ```feroxbuster``` | 2.13.1<br>2025-12-13 |
| [pipr](https://github.com/elkowar/pipr) | Interactive, shell-command editor with preview and safe execution | ```pipr``` | 0.1.0<br>2025-05-03 |


## Images & Media

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [oxipng](https://github.com/shssoichiro/oxipng) | A multithreaded lossless PNG compression optimizer | ```oxipng``` | 10.1.0<br>2026-01-25 |
| [rav1e](https://github.com/xiph/rav1e) | The fastest and safest AV1 encoder | ```rav1e``` | 0.8.1<br>2025-06-16 |


## Miscellaneous

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [bob-nvim](https://github.com/MordechaiHadad/bob) | A version manager for neovim written in Rust | ```bob``` | 4.1.6<br>2025-12-06 |
| [kondo](https://github.com/tbillington/kondo) | Cleans non-essential files from your software projects | ```kondo``` | 0.9.0<br>2026-01-23 |
| [presenterm](https://github.com/mfontanini/presenterm) | A markdown terminal slideshow tool | ```presenterm``` | 0.16.1<br>2026-02-20 |



## Contributing

Feel free to contribute by opening a pull request with your favorite Rust CLI tools that can be
installed via `cargo install`!
Please make sure to follow the <a href="CONTRIBUTING.md">contribution guidelines</a> and adhere to the <a href="CODE_OF_CONDUCT.md">code of conduct</a>.
Please also check the <a href="https://github.com/carlosferreyra/awesome-cargo-install/issues">issues</a> for any open issues or discussions related to the project.

## License

This project is licensed under the MIT License - see the <a href="LICENSE">LICENSE</a> file for details.