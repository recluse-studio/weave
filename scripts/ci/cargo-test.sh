#!/usr/bin/env bash
set -euo pipefail

. "$HOME/.cargo/env"

cargo test --workspace --all-features

