#!/usr/bin/env -S just --justfile

shebang := if os() == 'windows' { 'powershell.exe' } else { '/usr/bin/sh' }

set dotenv-load := true

alias d := dev
alias t := test
alias f := fmt
alias l := lint

# List available commands.
_default:
    just --list --unsorted

# Setup the repository
setup:
    git cliff --version || cargo install git-cliff
    cargo nextest --version || cargo install --locked cargo-nextest
    cargo-set-version --help || cargo install cargo-edit
    cargo watch --version || cargo install cargo-watch
    dprint --version || cargo install dprint

# Develop the app.
dev:
    cargo watch -x 'clippy --all-targets --all-features'

# Format the codebase.
fmt:
    cargo fmt --all
    dprint fmt --config configs/dprint.json

# Check is the codebase properly formatted.
fmt-check:
    cargo fmt --all -- --check
    dprint check --config configs/dprint.json

# Lint the codebase.
lint:
    cargo clippy --all-targets --all-features

# Check the documentation.
_doc-check:
    cargo doc --all-features --no-deps

# Test the codebase.
test:
    cargo nextest run --all-targets

# Tasks to make the code-base comply with the rules. Mostly used in git hooks.
comply: fmt lint test _doc-check
# Check if the repository comply with the rules and ready to be pushed.
check: fmt-check lint test _doc-check

# Create a new release. Example `just release v2.2.0`
release version:
    bash scripts/release.sh {{ version }}

