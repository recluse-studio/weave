#!/usr/bin/env bash
set -euo pipefail

. "$HOME/.cargo/env"

export WEAVE_WORKSPACE_ROOT="${WEAVE_WORKSPACE_ROOT:-$(pwd)/fixtures/demo-workspace/WEAVE}"
export WEAVE_ADDR="${WEAVE_ADDR:-127.0.0.1:8787}"

cargo run -p relay

