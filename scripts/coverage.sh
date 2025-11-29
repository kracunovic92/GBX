set -euo pipefail
cargo install cargo-llvm-cov >/dev/null 2>&1 || true
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --lcov --output-path coverage/lcov.info
echo "LCOV written to coverage/lcov.info"
