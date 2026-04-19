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
| [bat](https://github.com/sharkdp/bat) | A `cat(1)` clone with syntax highlighting and Git integration | ```bat``` | — |
| [choose](https://github.com/theryangeary/choose) | A human-friendly and fast alternative to `cut` and (sometimes) `awk` | ```choose``` | — |
| [du-dust](https://github.com/bootandy/dust) | A more intuitive version of `du` in Rust | ```dust``` | — |
| [eza](https://github.com/eza-community/eza) | A modern, maintained replacement for `ls` | ```eza``` | — |
| [fd-find](https://github.com/sharkdp/fd) | A simple, fast and user-friendly alternative to `find`<br><details><summary><strong><a href="#">Examples</a></strong></summary><br>`cargo install fd-find` — Install fd-find (provides the `fd` binary)<br></details> | ```fd``` | — |
| [grex](https://github.com/pemistahl/grex) | Generate regular expressions from user-provided test cases | ```grex``` | — |
| [hyperfine](https://github.com/sharkdp/hyperfine) | A command-line benchmarking tool with statistical analysis<br><details><summary><strong><a href="#">Examples</a></strong></summary><br>`hyperfine 'sleep 0.1' 'sleep 0.2'` — Benchmark two commands head-to-head<br></details> | ```hyperfine``` | — |
| [ouch](https://github.com/ouch-org/ouch) | Painless compression and decompression for your terminal | ```ouch``` | — |
| [pastel](https://github.com/sharkdp/pastel) | A command-line tool to generate, analyze, convert and manipulate colors | ```pastel``` | — |
| [ripgrep](https://github.com/BurntSushi/ripgrep) | Recursively search directories for a regex pattern — a faster grep<br><details><summary><strong><a href="#">Examples</a></strong></summary><br>`cargo install ripgrep` — Install ripgrep (provides the `rg` binary)<br>`rg 'fn main' --type rust` — Search for `fn main` only in Rust files<br></details> | ```rg``` | — |
| [sd](https://github.com/chmln/sd) | An intuitive find & replace CLI — an alternative to `sed` | ```sd``` | — |
| [tealdeer](https://github.com/dbrgn/tealdeer) | A very fast implementation of `tldr` in Rust | ```tldr``` | — |
| [tokei](https://github.com/XAMPPRocky/tokei) | Count lines of code across a codebase, grouped by language | ```tokei``` | — |
| [tre-command](https://github.com/dduan/tre) | A modern alternative to `tree` with git/gitignore awareness | ```tre``` | — |
| [zoxide](https://github.com/ajeetdsouza/zoxide) | A smarter `cd` command — jump to frecent directories | ```zoxide``` | — |


## Cargo Subcommands

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [cargo-audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit) | Audit `Cargo.lock` for crates with security vulnerabilities | ```cargo-audit``` | — |
| [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) | Install prebuilt Rust binaries without having to compile from source | ```cargo-binstall``` | — |
| [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat) | Find out what takes most of the space in your executable | ```cargo-bloat``` | — |
| [cargo-cache](https://github.com/matthiaskrgr/cargo-cache) | Manage cargo cache (`~/.cargo/`), print sizes and remove directories | ```cargo-cache``` | — |
| [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) | A cargo-subcommand to speed up Rust Docker builds using Docker layer caching | ```cargo-chef``` | — |
| [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) | Lint your dependencies: licenses, bans, advisories, sources | ```cargo-deny``` | — |
| [cargo-edit](https://github.com/killercup/cargo-edit) | Managing cargo dependencies from the command line (`cargo add/rm/upgrade`) | ```cargo-upgrade```, ```cargo-set-version``` | — |
| [cargo-expand](https://github.com/dtolnay/cargo-expand) | Shows the result of macro expansion and `#[derive]` expansion | ```cargo-expand``` | — |
| [cargo-flamegraph](https://github.com/flamegraph-rs/flamegraph) | Easy flamegraphs for Rust projects and anything else, without Perl or pipes | ```cargo-flamegraph```, ```flamegraph``` | — |
| [cargo-generate](https://github.com/cargo-generate/cargo-generate) | A developer tool to help you get up and running quickly with a new Rust project | ```cargo-generate``` | — |
| [cargo-hack](https://github.com/taiki-e/cargo-hack) | A cargo subcommand for testing feature flag combinations | ```cargo-hack``` | — |
| [cargo-info](https://gitlab.com/imp/cargo-info) | Show crate information from the terminal, pulled from crates.io | ```cargo-info``` | — |
| [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) | Cargo subcommand to easily use LLVM source-based code coverage | ```cargo-llvm-cov``` | — |
| [cargo-make](https://github.com/sagiegurari/cargo-make) | Rust task runner and build tool — supports tasks, dependencies, and conditions | ```cargo-make```, ```makers``` | — |
| [cargo-modules](https://github.com/regexident/cargo-modules) | A cargo plugin for showing a tree-like overview of a crate's modules | ```cargo-modules``` | — |
| [cargo-msrv](https://github.com/foresterre/cargo-msrv) | Find the Minimum Supported Rust Version for your project | ```cargo-msrv``` | — |
| [cargo-nextest](https://github.com/nextest-rs/nextest) | A next-generation test runner for Rust — up to 3x faster than `cargo test` | ```cargo-nextest``` | — |
| [cargo-outdated](https://github.com/kbknapp/cargo-outdated) | Displays when Rust dependencies are out of date | ```cargo-outdated``` | — |
| [cargo-release](https://github.com/crate-ci/cargo-release) | Cargo subcommand for smoothly releasing a new version of your crate | ```cargo-release``` | — |
| [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks) | Scan your Rust crate for semver violations | ```cargo-semver-checks``` | — |
| [cargo-tarpaulin](https://github.com/xd009642/tarpaulin) | A code coverage tool for Rust projects | ```cargo-tarpaulin``` | — |
| [cargo-udeps](https://github.com/est31/cargo-udeps) | Find unused dependencies in your Cargo.toml | ```cargo-udeps``` | — |
| [cargo-update](https://github.com/nabijaczleweli/cargo-update) | Cargo subcommand for checking and applying updates to installed executables | ```cargo-install-update``` | — |
| [cargo-watch](https://github.com/watchexec/cargo-watch) | Watches over your project's source for changes, runs commands when they occur<br><details><summary><strong><a href="#">Examples</a></strong></summary><br>`cargo install cargo-watch` — Install cargo-watch<br>`cargo watch -x check -x test` — Re-run `cargo check` and `cargo test` on every source change<br></details> | ```cargo-watch``` | — |


## System Monitoring & Infrastructure

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [bandwhich](https://github.com/imsnif/bandwhich) | Terminal bandwidth utilization tool | ```bandwhich``` | — |
| [bottom](https://github.com/ClementTsang/bottom) | Yet another cross-platform graphical process/system monitor | ```btm``` | — |
| [dog](https://github.com/ogham/dog) | A command-line DNS client — like `dig` but with fewer surprises | ```dog``` | — |
| [gping](https://github.com/orf/gping) | Ping, but with a graph | ```gping``` | — |
| [oha](https://github.com/hatoo/oha) | HTTP load generator with a TUI, inspired by rakyll/hey | ```oha``` | — |
| [procs](https://github.com/dalance/procs) | A modern replacement for `ps` written in Rust | ```procs``` | — |
| [rustscan](https://github.com/RustScan/RustScan) | The modern port scanner — fast and extensible with a scripting engine | ```rustscan``` | — |
| [topgrade](https://github.com/topgrade-rs/topgrade) | Upgrade all the things — a unified upgrade runner for many package managers | ```topgrade``` | — |
| [xh](https://github.com/ducaale/xh) | A friendly and fast tool for sending HTTP requests — HTTPie clone in Rust | ```xh```, ```xhs``` | — |


## Git & Version Control

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [git-delta](https://github.com/dandavison/delta) | A syntax-highlighting pager for git, diff, grep, and blame output | ```delta``` | — |
| [gitui](https://github.com/gitui-org/gitui) | Blazing fast terminal-UI for git, written in Rust | ```gitui``` | — |
| [jujutsu](https://github.com/martinvonz/jj) | A Git-compatible VCS that is both simple and powerful | ```jj``` | — |
| [onefetch](https://github.com/o2sh/onefetch) | Git repository summary on your terminal — languages, stats and commit info | ```onefetch``` | — |


## File Management

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [broot](https://github.com/Canop/broot) | A new way to see and navigate directory trees | ```broot``` | — |
| [felix](https://github.com/kyoheiu/felix) | A tui file manager with Vim-like key mapping | ```fx``` | — |
| [nomino](https://github.com/yaa110/nomino) | Batch rename utility for developers | ```nomino``` | — |
| [xplr](https://github.com/sayanarijit/xplr) | A hackable, minimal, fast TUI file explorer | ```xplr``` | — |
| [yazi-fm](https://github.com/sxyazi/yazi) | Blazing fast terminal file manager written in Rust, based on async I/O | ```yazi```, ```ya``` | — |


## Data Processing

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [jaq](https://github.com/01mf02/jaq) | A clone of jq — a JSON data processing tool | ```jaq``` | — |
| [jql](https://github.com/yamafaktory/jql) | A JSON Query Language CLI tool | ```jql``` | — |
| [qsv](https://github.com/dathere/qsv) | CSVs sliced, diced & analyzed — a blazing-fast data-wrangling toolkit | ```qsv``` | — |
| [sqlx-cli](https://github.com/launchbadge/sqlx) | Command-line utility for SQLx — creates DBs, runs migrations, prepares offline data | ```sqlx```, ```cargo-sqlx``` | — |
| [xan](https://github.com/medialab/xan) | The CSV magician — process CSV files from the command line | ```xan``` | — |
| [xsv](https://github.com/BurntSushi/xsv) | A fast CSV command-line toolkit written in Rust | ```xsv``` | — |


## Shells, Prompts & Terminals

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [atuin](https://github.com/atuinsh/atuin) | Magical shell history — sync, search, and backup your shell history | ```atuin``` | — |
| [nu](https://github.com/nushell/nushell) | Nushell — a new type of shell where data is structured | ```nu``` | — |
| [starship](https://github.com/starship/starship) | The minimal, blazing-fast, and infinitely customizable prompt for any shell | ```starship``` | — |
| [zellij](https://github.com/zellij-org/zellij) | A terminal workspace with batteries included — multiplexer alternative to tmux | ```zellij``` | — |


## Build Tools & Task Runners

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [bacon](https://github.com/Canop/bacon) | A background Rust code checker — fast and low-friction | ```bacon``` | — |
| [just](https://github.com/casey/just) | A handy way to save and run project-specific commands — a modern `make` | ```just``` | — |
| [mask](https://github.com/jacobdeichert/mask) | A CLI task runner defined by a simple markdown file | ```mask``` | — |
| [watchexec-cli](https://github.com/watchexec/watchexec) | Executes commands in response to file modifications | ```watchexec``` | — |


## Documentation & Writing

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [mdbook](https://github.com/rust-lang/mdBook) | Create book-style documentation from Markdown files | ```mdbook``` | — |
| [mdbook-linkcheck](https://github.com/Michael-F-Bryan/mdbook-linkcheck) | A backend for mdbook that validates links | ```mdbook-linkcheck``` | — |
| [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid) | A preprocessor for mdbook that renders Mermaid diagrams | ```mdbook-mermaid``` | — |
| [typst-cli](https://github.com/typst/typst) | A new markup-based typesetting system for the sciences | ```typst``` | — |


## Editors & IDE Helpers

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [helix-term](https://github.com/helix-editor/helix) | A post-modern modal text editor — like Vim but with better defaults | ```hx``` | — |


## Security & Auditing

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [feroxbuster](https://github.com/epi052/feroxbuster) | A fast, simple, recursive content discovery tool written in Rust | ```feroxbuster``` | — |
| [pipr](https://github.com/elkowar/pipr) | Interactive, shell-command editor with preview and safe execution | ```pipr``` | — |


## Images & Media

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [oxipng](https://github.com/shssoichiro/oxipng) | A multithreaded lossless PNG compression optimizer | ```oxipng``` | — |
| [rav1e](https://github.com/xiph/rav1e) | The fastest and safest AV1 encoder | ```rav1e``` | — |


## Miscellaneous

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [bob-nvim](https://github.com/MordechaiHadad/bob) | A version manager for neovim written in Rust | ```bob``` | — |
| [kondo](https://github.com/tbillington/kondo) | Cleans non-essential files from your software projects | ```kondo``` | — |
| [presenterm](https://github.com/mfontanini/presenterm) | A markdown terminal slideshow tool | ```presenterm``` | — |



## Contributing

Feel free to contribute by opening a pull request with your favorite Rust CLI tools that can be
installed via `cargo install`!
Please make sure to follow the <a href="CONTRIBUTING.md">contribution guidelines</a> and adhere to the <a href="CODE_OF_CONDUCT.md">code of conduct</a>.
Please also check the <a href="https://github.com/carlosferreyra/awesome-cargo-install/issues">issues</a> for any open issues or discussions related to the project.

## License

This project is licensed under the MIT License - see the <a href="LICENSE">LICENSE</a> file for details.