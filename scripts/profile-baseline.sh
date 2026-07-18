#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

OUT_ROOT="${1:-results/profiles}"
STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
OUT_DIR="${OUT_ROOT}/${STAMP}-baseline"
BIN="target/release/baseline_experiments"
PERF_DATA="${OUT_DIR}/perf.data"

mkdir -p "$OUT_DIR"

echo "Building baseline_experiments for profiling..."
RUSTFLAGS="-C force-frame-pointers=yes" cargo build -p baseline_experiments --release

if ! command -v perf >/dev/null 2>&1; then
  echo "error: perf is not installed or not on PATH" >&2
  exit 1
fi

if [[ -r /proc/sys/kernel/perf_event_paranoid ]]; then
  PERF_PARANOID="$(cat /proc/sys/kernel/perf_event_paranoid)"

  if [[ "$PERF_PARANOID" -gt 1 ]]; then
    echo "error: perf_event_paranoid=${PERF_PARANOID} blocks perf recording" >&2
    echo "Run this once, then retry:" >&2
    echo "  echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid" >&2
    echo >&2
    echo "To make it permanent, add this to /etc/sysctl.conf:" >&2
    echo "  kernel.perf_event_paranoid = -1" >&2
    exit 1
  fi
fi

echo "Recording perf.data..."
if ! perf record \
  --no-buildid-mmap \
  --call-graph fp \
  -F 199 \
  -o "$PERF_DATA" \
  -- "$BIN" \
  > "${OUT_DIR}/stdout.log" \
  2> "${OUT_DIR}/stderr.log"; then
  echo "error: perf record failed" >&2
  echo "stderr: ${OUT_DIR}/stderr.log" >&2

  if [[ -r /proc/sys/kernel/perf_event_paranoid ]]; then
    echo "perf_event_paranoid=$(cat /proc/sys/kernel/perf_event_paranoid)" >&2
    echo "Try: echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid" >&2
  fi

  exit 1
fi

if [[ ! -s "$PERF_DATA" ]]; then
  echo "error: ${PERF_DATA} is empty" >&2
  echo "Check ${OUT_DIR}/stderr.log for perf permission errors." >&2
  exit 1
fi

echo "Writing perf reports..."
if command -v rustfilt >/dev/null 2>&1; then
  perf report --stdio -i "$PERF_DATA" | rustfilt > "${OUT_DIR}/perf-report.txt"
  perf report --stdio --no-children -i "$PERF_DATA" | rustfilt > "${OUT_DIR}/perf-report-self.txt"
else
  perf report --stdio -i "$PERF_DATA" > "${OUT_DIR}/perf-report.txt"
  perf report --stdio --no-children -i "$PERF_DATA" > "${OUT_DIR}/perf-report-self.txt"
fi

if ! grep -qE 'Samples:|Overhead' "${OUT_DIR}/perf-report.txt"; then
  echo "warning: perf report did not show samples; check ${OUT_DIR}/stderr.log" >&2
fi

if [[ "${2:-}" == "--hotspot" ]]; then
  if command -v hotspot >/dev/null 2>&1; then
    hotspot "$PERF_DATA" >/dev/null 2>&1 &
  else
    echo "warning: hotspot is not installed or not on PATH" >&2
  fi
fi

echo
echo "Profile complete:"
echo "  directory:   ${OUT_DIR}"
echo "  perf.data:   ${PERF_DATA}"
echo "  report:      ${OUT_DIR}/perf-report.txt"
echo "  self report: ${OUT_DIR}/perf-report-self.txt"
echo "  stdout:      ${OUT_DIR}/stdout.log"
echo "  stderr:      ${OUT_DIR}/stderr.log"
