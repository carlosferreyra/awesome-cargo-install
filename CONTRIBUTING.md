# Contributing to Awesome Cargo Install

Thank you for your interest in contributing to Awesome Cargo Install! This document provides
guidelines for contributing to this curated list of Rust CLI tools installable via `cargo install`.

## Adding a New Tool

To add a new CLI tool to the list:

1. The tool must be installable via `cargo install <crate>` (and ideally `cargo binstall <crate>`)
2. The tool must be a command-line interface (CLI) tool (or a cargo subcommand)
3. The tool must be actively maintained
4. The tool should be useful for a general developer audience

### Required Information

Each tool entry needs:

- A unique **crate name** (as it appears on crates.io)
- A brief, clear description
- A link to its documentation or repository
- A list of executable commands the crate installs

### Where to Edit

All tool data lives in a single file at the root of the repository: **`tools.json`**.

The file has a top-level `"categories"` array. Find the category that best fits your tool and add
an entry to its `"tools"` object. If no existing category fits, you can propose a new one (see
below).

> **Cargo subcommands** (crates whose name starts with `cargo-`) belong in the dedicated
> **Cargo Subcommands** category, not the general Command-line Utilities category.

### JSON Format

```json
{
  "categories": [
    {
      "name": "Category Name",
      "slug": "category-slug",
      "tools": {
        "your-crate-name": {
          "description": "Brief description of the tool",
          "url": "https://link-to-documentation-or-repo",
          "execs": ["executable1", "executable2"]
        }
      }
    }
  ]
}
```

**Field reference:**

| Field         | Required | Description                                                    |
| :------------ | :------: | :------------------------------------------------------------- |
| `description` | yes      | One-line description of what the tool does                     |
| `url`         | yes      | Link to official docs or repo (`http://` or `https://` only)   |
| `execs`       | yes      | Non-empty list of binary names installed by the crate          |
| `examples`    | no       | List of example objects (see below)                            |
| `crates_io`   | no       | Set to `false` to skip crates.io release metadata sync         |

> **Tip:** The crate key is what gets passed to `cargo install <crate>`. If the binary name differs
> from the crate name (e.g. crate `du-dust` installs binary `dust`, crate `tealdeer` installs
> binary `tldr`), make sure `execs` reflects the actual binary names.

### Adding Examples

The `examples` field is optional but encouraged when the invocation is non-obvious — for example
when the crate name differs from the binary, a subcommand is needed, or a flag makes a big
usability difference.

Each example is an object with a required `cmd` and an optional `description`:

```json
"your-crate-name": {
  "description": "...",
  "url": "...",
  "execs": ["binary-name"],
  "examples": [
    {
      "cmd": "cargo install your-crate-name",
      "description": "Install the crate"
    },
    {
      "cmd": "binary-name --some-flag",
      "description": "What this example does"
    }
  ]
}
```

**Example field reference:**

| Field         | Required | Description                                        |
| :------------ | :------: | :------------------------------------------------- |
| `cmd`         | yes      | Full shell command the user can copy and run       |
| `description` | no       | One-line explanation of what the command does      |

Guidelines for writing good examples:

- Prefer short, realistic commands over exhaustive flag lists
- Mention the `cargo install` / `cargo binstall` invocation when the crate-name ≠ binary-name
- Add multiple examples only when they demonstrate meaningfully different use cases

### Adding a New Category

If your tool doesn't fit any existing category, add a new object to the `"categories"` array:

```json
{
  "name": "Human Readable Name",
  "slug": "url-friendly-slug",
  "tools": { ... }
}
```

Keep the slug lowercase, hyphen-separated, and consistent with the name.

## Pull Request Process

1. Fork the repository
2. Create a new branch for your changes
3. Edit `tools.json` to add your tool to the appropriate category
4. Run `cargo +nightly -Zscript scripts/checks.rs` locally to validate your changes
5. Make sure all checks pass
6. Create a pull request with a clear description of the tool you're adding

The GitHub Actions workflow will automatically:

- Validate `tools.json` schema
- Run `cargo binstall <crate>` (with `cargo install --locked` fallback) against each newly added
  crate
- Auto-approve the PR if all checks pass, or leave an actionable comment if a check fails

## Validation Checks

Your contribution will be automatically checked for:

- Required fields (`description`, `url`, `execs`)
- Valid URL format (must start with `http://` or `https://`)
- Non-empty executable list
- Valid JSON structure
- The crate actually installs via `cargo binstall` or `cargo install --locked`
- The declared binary appears on `PATH` after installation

## Running the scripts locally

All maintenance scripts live in `scripts/` and are self-contained
[cargo-script](https://doc.rust-lang.org/cargo/reference/unstable.html#script) files (embedded
`Cargo.toml` frontmatter, nightly toolchain required):

```bash
# install nightly if you don't have it
rustup toolchain install nightly

# validate tools.json
cargo +nightly -Zscript scripts/checks.rs

# regenerate README from tools.json
cargo +nightly -Zscript scripts/readme.rs

# sync crates.io release metadata (version + last_release) into tools.json
cargo +nightly -Zscript scripts/latest_release.rs
```

## Code of Conduct

Please note that this project is released with a [Code of Conduct](CODE_OF_CONDUCT.md). By
participating in this project you agree to abide by its terms.

## Questions

If you have questions about contributing, please:

1. Check existing issues
2. Create a new issue if needed
3. Tag it appropriately

## License

By contributing to this project, you agree that your contributions will be licensed under the MIT
License.
