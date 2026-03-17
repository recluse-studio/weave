#!/usr/bin/env bash
set -euo pipefail

. "$HOME/.cargo/env"

export WEAVE_WORKSPACE_ROOT="${WEAVE_WORKSPACE_ROOT:-$(pwd)/fixtures/demo-workspace/WEAVE}"

cargo run -p relay --bin workspace_admin -- audit
