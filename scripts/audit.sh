#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

if ! command -v cargo-audit >/dev/null 2>&1; then
    printf '\033[1;33m> cargo-audit not found - installing (one-time)'
    cargo install --locked cargo-audit
fi

cargo audit --file Cargo.lock
