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
        <img src="https://github.com/carlosferreyra/awesome-cargo-install/actions/workflows/ci.yml/badge.svg" alt="Validate Catalog">
    </a>
</div>

Inspired by <a href="https://github.com/rust-unofficial/awesome-rust">awesome-rust</a>,
<a href="https://github.com/carlosferreyra/awesome-uvx">awesome-uvx</a> and
<a href="https://github.com/carlosferreyra/awesome-bunx">awesome-bunx</a>.

> **Tip:** for faster installation (precompiled binaries, no compilation), install
> [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) first, then replace
> `cargo install <crate>` with `cargo binstall <crate>`.

**92 tools across 13 categories.**

## Contents


- [Command-line Utilities](#command-line-utilities) (8)
- [File Management](#file-management) (12)
- [Data Processing](#data-processing) (6)
- [Shells, Prompts & Terminals](#shells-prompts-and-terminals) (7)
- [Git & Version Control](#git-and-version-control) (5)
- [Build Tools & Task Runners](#build-tools-and-task-runners) (5)
- [Cargo Subcommands](#cargo-subcommands) (31)
- [Documentation & Writing](#documentation-and-writing) (5)
- [Networking & HTTP](#networking-and-http) (5)
- [System Monitoring & Infrastructure](#system-monitoring-and-infrastructure) (2)
- [Security & Auditing](#security-and-auditing) (2)
- [Environment & Package Management](#environment-and-package-management) (2)
- [Images & Media](#images-and-media) (2)


<a id="command-line-utilities"></a>

## Command-line Utilities

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [bat](https://github.com/sharkdp/bat) | A `cat(1)` clone with syntax highlighting and Git integration | `bat` | 0.26.1<br>2025-12-02 |
| [coreutils](https://github.com/uutils/coreutils) | Cross-platform Rust rewrite of the GNU coreutils | `coreutils` | 0.12.0<br>2026-09-17 |
| [grex](https://github.com/pemistahl/grex) | Generate regular expressions from user-provided test cases | `grex` | 1.4.6<br>2025-11-14 |
| [hyperfine](https://github.com/sharkdp/hyperfine) | A command-line benchmarking tool with statistical analysis | `hyperfine` | 1.20.0<br>2025-11-18 |
| [ripgrep](https://github.com/BurntSushi/ripgrep) | Recursively search directories for a regex pattern — a faster grep | `rg` | 15.2.0<br>2026-07-15 |
| [sd](https://github.com/chmln/sd) | An intuitive find & replace CLI — an alternative to `sed` | `sd` | 1.0.0<br>2023-11-08 |
| [tealdeer](https://github.com/dbrgn/tealdeer) | A very fast implementation of `tldr` in Rust | `tldr` | 1.9.0<br>2026-08-24 |
| [tokei](https://github.com/XAMPPRocky/tokei) | Count lines of code across a codebase, grouped by language | `tokei` | 15.0.0<br>2026-09-06 |


<details>
<summary>hyperfine examples</summary>

Benchmark two commands head-to-head

```sh
hyperfine 'sleep 0.1' 'sleep 0.2'
```

</details>


<details>
<summary>ripgrep examples</summary>

Install ripgrep (provides the `rg` binary)

```sh
cargo install ripgrep
```

Search for `fn main` only in Rust files

```sh
rg 'fn main' --type rust
```

</details>


<details>
<summary>tealdeer examples</summary>

Install tealdeer (provides tldr)

```sh
cargo install tealdeer
```

Show practical examples for tar

```sh
tldr tar
```

</details>



<a id="file-management"></a>

## File Management

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [broot](https://github.com/Canop/broot) | A new way to see and navigate directory trees | `broot` | 1.60.2<br>2026-09-26 |
| [du-dust](https://github.com/bootandy/dust) | A more intuitive version of `du` in Rust | `dust` | 1.2.6<br>2026-09-16 |
| [eza](https://github.com/eza-community/eza) | A modern, maintained replacement for `ls` | `eza` | 0.23.5<br>2026-07-19 |
| [fd-find](https://github.com/sharkdp/fd) | A simple, fast and user-friendly alternative to `find` | `fd` | 10.5.0<br>2026-08-26 |
| [felix](https://github.com/kyoheiu/felix) | A tui file manager with Vim-like key mapping | `fx` | 2.16.1<br>2025-04-12 |
| [kondo](https://github.com/tbillington/kondo) | Cleans non-essential files from your software projects | `kondo` | 0.9.0<br>2026-01-23 |
| [nomino](https://github.com/yaa110/nomino) | Batch rename utility for developers | `nomino` | 1.6.4<br>2025-08-07 |
| [ouch](https://github.com/ouch-org/ouch) | Painless compression and decompression for your terminal | `ouch` | 0.8.3<br>2026-09-13 |
| [tre-command](https://github.com/dduan/tre) | A modern alternative to `tree` with git/gitignore awareness | `tre` | 0.4.0<br>2022-06-19 |
| [xplr](https://github.com/sayanarijit/xplr) | A hackable, minimal, fast TUI file explorer | `xplr` | 1.1.2<br>2026-09-15 |
| [yazi-cli](https://github.com/sxyazi/yazi) | Companion CLI for the Yazi terminal file manager | `ya` | 26.5.6<br>2026-05-05 |
| [yazi-fm](https://github.com/sxyazi/yazi) | Blazing fast terminal file manager written in Rust, based on async I/O | `yazi` | 26.5.6<br>2026-05-05 |


<details>
<summary>du-dust examples</summary>

Install du-dust (provides dust)

```sh
cargo install du-dust
```

Show disk usage in the current directory

```sh
dust .
```

</details>


<details>
<summary>fd-find examples</summary>

Install fd-find (provides the `fd` binary)

```sh
cargo install fd-find
```

</details>


<details>
<summary>yazi-cli examples</summary>

Install Yazi and its companion CLI together

```sh
cargo binstall yazi-fm yazi-cli
```

</details>


<details>
<summary>yazi-fm examples</summary>

Install the file manager and its companion CLI

```sh
cargo binstall yazi-fm yazi-cli
```

</details>



<a id="data-processing"></a>

## Data Processing

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [choose](https://github.com/theryangeary/choose) | A human-friendly and fast alternative to `cut` and (sometimes) `awk` | `choose` | 1.3.7<br>2025-08-26 |
| [jaq](https://github.com/01mf02/jaq) | A clone of jq — a JSON data processing tool | `jaq` | 3.1.1<br>2026-08-05 |
| [jql](https://github.com/yamafaktory/jql) | A JSON Query Language CLI tool | `jql` | 9.0.3<br>2026-09-08 |
| [qsv](https://github.com/dathere/qsv) | CSVs sliced, diced & analyzed — a blazing-fast data-wrangling toolkit | `qsv` | 16.1.0<br>2026-02-15 |
| [sqlx-cli](https://github.com/launchbadge/sqlx) | Command-line utility for SQLx — creates DBs, runs migrations, prepares offline data | `sqlx`, `cargo-sqlx` | 0.9.0<br>2026-05-21 |
| [xan](https://github.com/medialab/xan) | The CSV magician — process CSV files from the command line | `xan` | 0.61.0<br>2026-09-11 |



<a id="shells-prompts-and-terminals"></a>

## Shells, Prompts & Terminals

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [atuin](https://github.com/atuinsh/atuin) | Magical shell history — sync, search, and backup your shell history | `atuin` | 18.23.0<br>2026-09-22 |
| [nu](https://github.com/nushell/nushell) | Nushell — a new type of shell where data is structured | `nu` | 0.116.0<br>2026-09-26 |
| [pipr](https://github.com/elkowar/pipr) | Interactive, shell-command editor with preview and safe execution | `pipr` | 0.1.0<br>2025-05-03 |
| [skim](https://github.com/skim-rs/skim) | Fuzzy finder in Rust — an alternative to fzf | `sk` | 5.7.3<br>2026-10-02 |
| [starship](https://github.com/starship/starship) | The minimal, blazing-fast, and infinitely customizable prompt for any shell | `starship` | 1.26.0<br>2026-06-28 |
| [zellij](https://github.com/zellij-org/zellij) | A terminal workspace with batteries included — multiplexer alternative to tmux | `zellij` | 0.45.1<br>2026-08-28 |
| [zoxide](https://github.com/ajeetdsouza/zoxide) | A smarter `cd` command — jump to frecent directories | `zoxide` | 0.10.0<br>2026-07-04 |



<a id="git-and-version-control"></a>

## Git & Version Control

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [git-cliff](https://github.com/orhun/git-cliff) | A highly customizable changelog generator that follows conventional commits | `git-cliff` | 2.14.2<br>2026-09-18 |
| [git-delta](https://github.com/dandavison/delta) | A syntax-highlighting pager for git, diff, grep, and blame output | `delta` | 0.19.2<br>2026-03-28 |
| [gitui](https://github.com/gitui-org/gitui) | Blazing fast terminal-UI for git, written in Rust | `gitui` | 0.28.1<br>2026-03-24 |
| [jj-cli](https://github.com/jj-vcs/jj) | Jujutsu — a Git-compatible VCS that is both simple and powerful | `jj` | 0.45.1<br>2026-09-03 |
| [onefetch](https://github.com/o2sh/onefetch) | Git repository summary on your terminal — languages, stats and commit info | `onefetch` | 2.28.1<br>2026-08-30 |


<details>
<summary>git-delta examples</summary>

Install git-delta (provides delta)

```sh
cargo install git-delta
```

View a highlighted diff

```sh
git diff | delta
```

</details>


<details>
<summary>jj-cli examples</summary>

Install jj-cli (provides jj)

```sh
cargo install jj-cli
```

Show the working-copy status in a Jujutsu repository

```sh
jj status
```

</details>



<a id="build-tools-and-task-runners"></a>

## Build Tools & Task Runners

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [bacon](https://github.com/Canop/bacon) | A background Rust code checker — fast and low-friction | `bacon` | 3.26.0<br>2026-09-26 |
| [just](https://github.com/casey/just) | A handy way to save and run project-specific commands — a modern `make` | `just` | 1.58.0<br>2026-08-03 |
| [mask](https://github.com/jacobdeichert/mask) | A CLI task runner defined by a simple markdown file | `mask` | 0.11.7<br>2026-01-10 |
| [sccache](https://github.com/mozilla/sccache) | Shared compilation cache — a `ccache`-like compiler wrapper that avoids recompilation | `sccache` | 0.18.0<br>2026-09-16 |
| [watchexec-cli](https://github.com/watchexec/watchexec) | Executes commands in response to file modifications | `watchexec` | 2.7.4<br>2026-10-02 |


<details>
<summary>watchexec-cli examples</summary>

Install watchexec-cli (provides watchexec)

```sh
cargo install watchexec-cli
```

Run cargo check when files change

```sh
watchexec -- cargo check
```

</details>



<a id="cargo-subcommands"></a>

## Cargo Subcommands

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [cargo-audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit) | Audit `Cargo.lock` for crates with security vulnerabilities | `cargo-audit` | 0.22.2<br>2026-06-05 |
| [cargo-auditable](https://github.com/rust-secure-code/cargo-auditable) | Make production Rust binaries auditable — embed dep info into the binary | `cargo-auditable` | 0.7.7<br>2026-10-02 |
| [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) | Install prebuilt Rust binaries without having to compile from source | `cargo-binstall` | 1.25.1<br>2026-10-03 |
| [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat) | Find out what takes most of the space in your executable | `cargo-bloat` | 0.12.1<br>2024-05-10 |
| [cargo-bundle](https://github.com/burtonageo/cargo-bundle) | Wrap Rust executables in OS-specific app bundles | `cargo-bundle` | 0.12.0<br>2026-09-20 |
| [cargo-cache](https://github.com/matthiaskrgr/cargo-cache) | Manage cargo cache (`~/.cargo/`), print sizes and remove directories | `cargo-cache` | 0.8.3<br>2022-09-11 |
| [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) | A cargo-subcommand to speed up Rust Docker builds using Docker layer caching | `cargo-chef` | 0.1.78<br>2026-08-12 |
| [cargo-deadlinks](https://github.com/deadlinks/cargo-deadlinks) | Cargo subcommand for checking your `cargo doc` output for broken links | `cargo-deadlinks`, `deadlinks` | 0.8.1<br>2021-10-13 |
| [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) | Lint your dependencies: licenses, bans, advisories, sources | `cargo-deny` | 0.20.2<br>2026-07-09 |
| [cargo-dist](https://github.com/axodotdev/cargo-dist) | Shippable application packaging for Rust — prebuilt binary distribution | `dist` | 0.32.0<br>2026-05-22 |
| [cargo-edit](https://github.com/killercup/cargo-edit) | Upgrade dependency versions and set package versions from the command line | `cargo-upgrade`, `cargo-set-version` | 0.13.13<br>2026-07-15 |
| [cargo-expand](https://github.com/dtolnay/cargo-expand) | Shows the result of macro expansion and `#[derive]` expansion | `cargo-expand` | 1.0.126<br>2026-08-19 |
| [cargo-generate](https://github.com/cargo-generate/cargo-generate) | A developer tool to help you get up and running quickly with a new Rust project | `cargo-generate` | 0.25.0<br>2026-09-18 |
| [cargo-hack](https://github.com/taiki-e/cargo-hack) | A cargo subcommand for testing feature flag combinations | `cargo-hack` | 0.6.45<br>2026-05-30 |
| [cargo-info](https://gitlab.com/imp/cargo-info) | Show crate information from the terminal, pulled from crates.io | `cargo-info` | 0.7.7<br>2024-09-06 |
| [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) | Cargo subcommand to easily use LLVM source-based code coverage | `cargo-llvm-cov` | 0.9.1<br>2026-09-06 |
| [cargo-machete](https://github.com/bnjbvr/cargo-machete) | Find and remove unused dependencies in your `Cargo.toml` | `cargo-machete` | 0.9.2<br>2026-04-15 |
| [cargo-make](https://github.com/sagiegurari/cargo-make) | Rust task runner and build tool — supports tasks, dependencies, and conditions | `cargo-make`, `makers` | 0.37.24<br>2025-01-18 |
| [cargo-modules](https://github.com/regexident/cargo-modules) | A cargo plugin for showing a tree-like overview of a crate's modules | `cargo-modules` | 0.27.0<br>2026-08-03 |
| [cargo-msrv](https://github.com/foresterre/cargo-msrv) | Find the Minimum Supported Rust Version for your project | `cargo-msrv` | 0.19.3<br>2026-03-25 |
| [cargo-nextest](https://github.com/nextest-rs/nextest) | A test runner for Rust with parallel execution and test profiles | `cargo-nextest` | 0.9.146<br>2026-09-21 |
| [cargo-outdated](https://github.com/kbknapp/cargo-outdated) | Displays when Rust dependencies are out of date | `cargo-outdated` | 0.19.0<br>2026-04-14 |
| [cargo-release](https://github.com/crate-ci/cargo-release) | Cargo subcommand for smoothly releasing a new version of your crate | `cargo-release` | 1.1.6<br>2026-09-16 |
| [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks) | Scan your Rust crate for semver violations | `cargo-semver-checks` | 0.51.0<br>2026-10-03 |
| [cargo-sort](https://github.com/DevinR528/cargo-sort) | Check that tables and items in `Cargo.toml` are lexically sorted | `cargo-sort` | 2.1.4<br>2026-04-25 |
| [cargo-sweep](https://github.com/holmgr/cargo-sweep) | A cargo subcommand for cleaning unused build files created by Cargo | `cargo-sweep` | 0.8.0<br>2025-10-11 |
| [cargo-tarpaulin](https://github.com/xd009642/tarpaulin) | A code coverage tool for Rust projects | `cargo-tarpaulin` | 0.37.5<br>2026-09-27 |
| [cargo-udeps](https://github.com/est31/cargo-udeps) | Find unused dependencies in your Cargo.toml | `cargo-udeps` | 0.1.61<br>2026-04-29 |
| [cargo-update](https://github.com/nabijaczleweli/cargo-update) | Cargo subcommand for checking and applying updates to installed executables | `cargo-install-update` | 22.1.1<br>2026-07-27 |
| [cargo-workspaces](https://github.com/pksunkara/cargo-workspaces) | A tool for managing cargo workspaces and their crates | `cargo-workspaces`, `cargo-ws` | 0.4.2<br>2025-12-03 |
| [flamegraph](https://github.com/flamegraph-rs/flamegraph) | Easy flamegraphs for Rust projects and anything else, without Perl or pipes | `cargo-flamegraph`, `flamegraph` | 0.6.14<br>2026-08-12 |


<details>
<summary>cargo-edit examples</summary>

Install cargo-edit (provides cargo-upgrade, cargo-set-version)

```sh
cargo install cargo-edit
```

Upgrade dependency requirements

```sh
cargo upgrade
```

</details>


<details>
<summary>cargo-llvm-cov examples</summary>

Install cargo-llvm-cov (provides cargo-llvm-cov)

```sh
cargo install cargo-llvm-cov
```

Measure Rust test coverage

```sh
cargo llvm-cov
```

</details>


<details>
<summary>cargo-nextest examples</summary>

Install cargo-nextest (provides cargo-nextest)

```sh
cargo install cargo-nextest
```

Run the Rust test suite

```sh
cargo nextest run
```

</details>



<a id="documentation-and-writing"></a>

## Documentation & Writing

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [mdbook](https://github.com/rust-lang/mdBook) | Create book-style documentation from Markdown files | `mdbook` | 0.5.4<br>2026-07-06 |
| [mdbook-linkcheck](https://github.com/Michael-F-Bryan/mdbook-linkcheck) | A backend for mdbook that validates links | `mdbook-linkcheck` | 0.7.7<br>2022-10-03 |
| [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid) | A preprocessor for mdbook that renders Mermaid diagrams | `mdbook-mermaid` | 0.17.1<br>2026-08-14 |
| [presenterm](https://github.com/mfontanini/presenterm) | A markdown terminal slideshow tool | `presenterm` | 0.16.1<br>2026-02-20 |
| [typst-cli](https://github.com/typst/typst) | A new markup-based typesetting system for the sciences | `typst` | 0.15.1<br>2026-07-17 |


<details>
<summary>typst-cli examples</summary>

Install typst-cli (provides typst)

```sh
cargo install typst-cli
```

Compile a Typst document to PDF

```sh
typst compile document.typ
```

</details>



<a id="networking-and-http"></a>

## Networking & HTTP

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [bandwhich](https://github.com/imsnif/bandwhich) | Terminal bandwidth utilization tool | `bandwhich` | 0.23.1<br>2024-10-08 |
| [gping](https://github.com/orf/gping) | Ping, but with a graph | `gping` | 1.21.0<br>2026-08-31 |
| [oha](https://github.com/hatoo/oha) | HTTP load generator with a TUI, inspired by rakyll/hey | `oha` | 1.16.0<br>2026-08-23 |
| [speedtest-rs](https://github.com/nelsonjchen/speedtest-rs) | Speedtest.net testing utility implemented in Rust | `speedtest-rs` | 0.2.0<br>2024-07-28 |
| [xh](https://github.com/ducaale/xh) | A friendly and fast tool for sending HTTP requests — HTTPie clone in Rust | `xh`, `xhs` | 0.26.2<br>2026-07-26 |



<a id="system-monitoring-and-infrastructure"></a>

## System Monitoring & Infrastructure

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [bottom](https://github.com/ClementTsang/bottom) | Yet another cross-platform graphical process/system monitor | `btm` | 0.14.9<br>2026-08-27 |
| [procs](https://github.com/dalance/procs) | A modern replacement for `ps` written in Rust | `procs` | 0.14.12<br>2026-06-25 |



<a id="security-and-auditing"></a>

## Security & Auditing

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [feroxbuster](https://github.com/epi052/feroxbuster) | A fast, simple, recursive content discovery tool written in Rust | `feroxbuster` | 2.13.1<br>2025-12-13 |
| [rustscan](https://github.com/RustScan/RustScan) | The modern port scanner — fast and extensible with a scripting engine | `rustscan` | 2.4.1<br>2025-02-23 |



<a id="environment-and-package-management"></a>

## Environment & Package Management

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [bob-nvim](https://github.com/MordechaiHadad/bob) | A version manager for neovim written in Rust | `bob` | 4.2.0<br>2026-09-22 |
| [topgrade](https://github.com/topgrade-rs/topgrade) | Upgrade all the things — a unified upgrade runner for many package managers | `topgrade` | 17.12.3<br>2026-10-02 |



<a id="images-and-media"></a>

## Images & Media

| Name | Description | Executable(s) | Latest Release |
|:-----|:------------|:--------------|:--------------|
| [oxipng](https://github.com/shssoichiro/oxipng) | A multithreaded lossless PNG compression optimizer | `oxipng` | 10.2.1<br>2026-09-02 |
| [pastel](https://github.com/sharkdp/pastel) | A command-line tool to generate, analyze, convert and manipulate colors | `pastel` | 0.12.0<br>2026-02-14 |




## Contributing

Feel free to contribute by opening a pull request with your favorite Rust CLI tools that can be
installed via `cargo install`!
Please make sure to follow the <a href="CONTRIBUTING.md">contribution guidelines</a> and adhere to the <a href="CODE_OF_CONDUCT.md">code of conduct</a>.
Please also check the <a href="https://github.com/carlosferreyra/awesome-cargo-install/issues">issues</a> for any open issues or discussions related to the project.

## License

This project is licensed under the MIT License - see the <a href="LICENSE">LICENSE</a> file for details.