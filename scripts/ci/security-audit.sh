#!/usr/bin/env bash
set -euo pipefail

. "$HOME/.cargo/env"

printf 'Running npm audit...\n'
npm audit --audit-level high

printf '\nRunning frontend audit...\n'
npm --prefix frontend audit --audit-level high

if command -v cargo-audit >/dev/null 2>&1; then
  printf '\nRunning cargo audit...\n'
  cargo audit
else
  printf '\nSkipping cargo audit: cargo-audit is not installed.\n'
fi

if command -v cargo-deny >/dev/null 2>&1; then
  printf '\nRunning cargo deny...\n'
  cargo deny check
else
  printf '\nSkipping cargo deny: cargo-deny is not installed.\n'
fi

if command -v gitleaks >/dev/null 2>&1; then
  printf '\nRunning gitleaks...\n'
  gitleaks detect --source . --no-git
else
  printf '\nSkipping gitleaks: gitleaks is not installed.\n'
fi

