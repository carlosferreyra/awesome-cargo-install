## What tool are you adding?

Crate name:

Tool homepage or repository:

Category added to:

Executable command(s):

## Why should it be included?

Write one or two sentences about what this CLI helps people do and why the category fits.

## Checklist

- [ ] I added the tool to `tools.json`, with any examples inside its tool definition.
- [ ] The crate name matches the name used on crates.io.
- [ ] The `url` points to the official docs, homepage, or repository.
- [ ] The `execs` list contains the installed command names.
- [ ] I left generated `version` and `last_release` fields to automation.
- [ ] I ran `cargo +nightly -Zscript scripts/checks.rs -- --diff-base origin/main`.
- [ ] I regenerated `README.md` and ran `cargo +nightly -Zscript scripts/readme.rs -- --check`.
- [ ] Cargo plugins are in Cargo Subcommands.
- [ ] The checks passed locally, or I explained any limitations below.

## Notes for the maintainer

Include category rationale, validation results, or any package/binary-name differences.
