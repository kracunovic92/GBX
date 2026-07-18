# Basic Profiling

Use `baseline_experiments` as the simple profiling target.

## Run

```sh
scripts/profile-baseline.sh
```

This builds:

```sh
RUSTFLAGS="-C force-frame-pointers=yes" cargo build -p baseline_experiments --release
```

Then records:

```sh
perf record --call-graph fp -F 199 -o <run-dir>/perf.data -- \
  target/release/baseline_experiments
```

The run directory is created under:

```text
results/profiles/<timestamp>-baseline/
```

Files:

```text
perf.data
perf-report.txt
perf-report-self.txt
stdout.log
stderr.log
```

Open with Hotspot:

```sh
scripts/profile-baseline.sh results/profiles --hotspot
```

Use another output root:

```sh
scripts/profile-baseline.sh /tmp/gbx-profiles
```

## If `perf.data` Looks Empty

Check stderr first:

```sh
cat results/profiles/<run>/stderr.log
```

Common causes:

- perf permissions are blocked
- the program exits too quickly to collect useful samples
- symbols are missing because the binary was not built with debug info

Check perf permissions:

```sh
cat /proc/sys/kernel/perf_event_paranoid
```

If it prints `3`, normal user-space profiling is blocked and `perf.data` may be
empty.

Temporary local fix:

```sh
echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid
```

Check that debug sections exist:

```sh
readelf -S target/release/baseline_experiments | grep debug
```

## Inspect

Inclusive report:

```sh
less results/profiles/<run>/perf-report.txt
```

Self-time report:

```sh
less results/profiles/<run>/perf-report-self.txt
```

Hotspot:

```sh
hotspot results/profiles/<run>/perf.data
```

## What To Look For

If `normalize_terms_in` dominates, polynomial normalization is the immediate
target.

If sort functions dominate, term ordering/merging is likely expensive.

If monomial quotient/divisibility dominates, reducer scanning or pair handling
is doing repeated monomial work.

If allocator functions dominate, look for temporary vectors in symbolic
preprocessing, matrix construction, row buffers, and normalization.
