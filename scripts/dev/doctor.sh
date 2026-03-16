#!/usr/bin/env bash
set -euo pipefail

missing=0

check_bin() {
  local name="$1"
  if command -v "$name" >/dev/null 2>&1; then
    printf 'ok    %s\n' "$name"
  else
    printf 'miss  %s\n' "$name"
    missing=1
  fi
}

check_file() {
  local path="$1"
  if [ -e "$path" ]; then
    printf 'ok    %s\n' "$path"
  else
    printf 'miss  %s\n' "$path"
    missing=1
  fi
}

check_bin node
check_bin npm
check_bin cargo
check_file "$(pwd)/fixtures/demo-workspace/WEAVE/workspace/settings.json"
check_file "$(pwd)/frontend/package.json"
check_file "$(pwd)/Cargo.toml"

if [ "$missing" -ne 0 ]; then
  printf '\nDoctor found missing prerequisites.\n'
  exit 1
fi

printf '\nWEAVE local prerequisites look usable.\n'

