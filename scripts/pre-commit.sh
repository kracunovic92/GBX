#!/usr/bin/env bash
set -euo pipefail
echo "→ rustfmt"
cargo fmt --all
echo "→ clippy"
cargo clippy --workspace --all-targets -- -D warnings
echo "→ tests"
cargo test --workspace --all-features
