#!/usr/bin/env bash

set -e

BIN=target/release/graph_experiments
INSTANCE=instances/myciel3.col
K=4
TIME=60

echo "Building release binary with debug symbols..."
CARGO_PROFILE_RELEASE_DEBUG=true cargo build --release --bin graph_experiments

echo
echo "Removing old callgrind outputs..."
rm -f callgrind.out.*

echo
echo "Running callgrind for ${TIME}s..."

timeout ${TIME}s valgrind --tool=callgrind \
  $BIN grobner $INSTANCE -k $K || true

echo
echo "Finding latest callgrind output..."

FILE=$(ls -t callgrind.out.* | head -n 1)

echo "Callgrind file: $FILE"

echo
echo "Opening in kcachegrind..."

kcachegrind "$FILE"
