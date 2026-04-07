# Profiling Guide for GBX

This guide collects the commands and workflow used to profile the GBX workspace, especially
long-running Gröbner basis runs such as:

cargo run -p graph_experiments --release -- grobner instances/myciel3.col -k 4 --dump-basis

The goal is to determine:

- which functions are hot
- how much time is spent in them
- which functions call them
- whether the bottleneck is reduction, normalization, pair update, or allocation

------------------------------------------------------------

## 1. Build Setup for Profiling

Always profile an optimized build **with debug symbols**.

Add this to the workspace root `Cargo.toml`:

[profile.release]
debug = true
strip = "none"

Build with frame pointers enabled:

RUSTFLAGS="-C force-frame-pointers=yes" cargo build -p graph_experiments --release

Why:

- `--release` gives realistic optimized behavior
- `debug = true` allows perf to resolve symbols
- frame pointers improve stack traces

Verify debug sections exist:

readelf -S target/release/graph_experiments | grep debug

You should see sections like:

.debug_info
.debug_line

------------------------------------------------------------

## 2. Run the Binary Directly

Prefer running the built binary instead of `cargo run`:

target/release/graph_experiments grobner instances/myciel3.col -k 4 --dump-basis

This avoids cargo wrapper overhead in the profiler.

------------------------------------------------------------

## 3. Find the Process PID

To attach a profiler:

pgrep -af graph_experiments

Example output:

62075 target/release/graph_experiments grobner instances/myciel3.col -k 4 --dump-basis

PID is the first number.

Alternative:

ps aux | grep graph_experiments

Or run in background:

target/release/graph_experiments grobner instances/myciel3.col -k 4 --dump-basis &
echo $!

------------------------------------------------------------

## 4. Enable perf Permissions

If perf complains about permissions:

echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid

To make permanent add to `/etc/sysctl.conf`:

kernel.perf_event_paranoid = -1

Then apply:

sudo sysctl -p

------------------------------------------------------------

## 5. Record a Profile

Attach for 60 seconds:

perf record --call-graph fp -F 199 -p <PID> -- sleep 60

Example:

perf record --call-graph fp -F 199 -p 62075 -- sleep 60

Explanation:

--call-graph fp  : better stack traces
-F 199           : sampling frequency
sleep 60         : record exactly 60 seconds

------------------------------------------------------------

## 6. Inspect the Profile

Install demangler once:

cargo install rustfilt

Then inspect:

perf report --stdio | rustfilt | head -200

To see **self time only**:

perf report --stdio --no-children | rustfilt | head -200

------------------------------------------------------------

## 7. Decode Raw Addresses

If perf shows addresses like:

0x8fb4f

Decode them:

addr2line -e target/release/graph_experiments -f -C 0x8fb4f

Multiple addresses:

addr2line -e target/release/graph_experiments -f -C 0x8fb4f 0x8fb58 0x8fb40

------------------------------------------------------------

## 8. Generate Flamegraphs from perf.data

Clone FlameGraph tools:

git clone https://github.com/brendangregg/FlameGraph.git
export PATH="$PATH:$(pwd)/FlameGraph"

Create folded stacks:

perf script | rustfilt | stackcollapse-perf.pl > out.folded

Generate SVG:

flamegraph.pl out.folded > flamegraph.svg

Open:

xdg-open flamegraph.svg

------------------------------------------------------------

## 9. Using cargo-flamegraph

Install:

cargo install flamegraph

Run:

cargo flamegraph -p graph_experiments --release -- \
grobner instances/myciel3.col -k 4 --dump-basis

Limit runtime:

timeout 120s cargo flamegraph -p graph_experiments --release -- \
grobner instances/myciel3.col -k 4 --dump-basis

------------------------------------------------------------

## 10. Quick Command Reference

Build for profiling:

RUSTFLAGS="-C force-frame-pointers=yes" cargo build -p graph_experiments --release

Run binary:

target/release/graph_experiments grobner instances/myciel3.col -k 4 --dump-basis

Find PID:

pgrep -af graph_experiments

Record profile:

perf record --call-graph fp -F 199 -p <PID> -- sleep 60

View report:

perf report --stdio | rustfilt | head -200

Self time only:

perf report --stdio --no-children | rustfilt | head -200

Generate flamegraph:

perf script | rustfilt | stackcollapse-perf.pl > out.folded
flamegraph.pl out.folded > flamegraph.svg

Decode addresses:

addr2line -e target/release/graph_experiments -f -C <offset>

------------------------------------------------------------

## 11. Interpreting Hotspots

If `normalize_terms_in` dominates:

- polynomial normalization is too expensive

If `core::slice::sort::*` dominates:

- sorting terms is the bottleneck

If `checked_quotient` dominates:

- reducer scanning / monomial divisibility is expensive

If allocator functions dominate:

- too many temporary allocations

------------------------------------------------------------

## 12. Current GBX Profiling Conclusion

Current profiles show:

Main hotspot:
normalize_terms_in

Inside it:
core::slice::sort

Second hotspot:
checked_quotient

This indicates the largest optimization opportunity is reducing
how often full polynomial normalization is performed.
"""

path = "/mnt/data/gbx_profiling.md"
pypandoc.convert_text(text, "md", format="md", outputfile=path, extra_args=['--standalone'])
path
