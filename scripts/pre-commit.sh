#!/usr/bin/env bash
set -euo pipefail

echo "== fmt =="
cargo fmt --all -- --check

echo "== clippy =="
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo "== test =="
cargo test --workspace --lib --bins --tests --all-features -- --test-threads=1

echo "== doc =="
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features

echo "OK"
