#!/usr/bin/env bash
set -euo pipefail

. "$HOME/.cargo/env"

cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings

