#!/usr/bin/env bash
set -euo pipefail

echo "== fmt =="
cargo fmt --all -- --check

echo "== clippy =="
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo "== test =="
cargo test --workspace --all-targets --all-features

echo "== doc =="
cargo doc --workspace --no-deps --all-features -D warnings

echo "OK"
