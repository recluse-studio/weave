#!/usr/bin/env bash
set -euo pipefail

. "$HOME/.cargo/env"

cargo build --workspace

