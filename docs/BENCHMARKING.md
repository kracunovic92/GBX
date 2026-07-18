# Benchmarking and Profiling

This workspace uses Criterion for repeatable performance checks. Benchmarks are
split by crate so you can isolate field arithmetic, polynomial operations, and
Groebner/F4 behavior.

## Quick Smoke Run

Use this before pushing or after a small change:

```sh
cargo bench --workspace -- --quick
```

CI runs this command on push and uploads `target/criterion` as an artifact when
Criterion emits reports.

## Full Local Run

Use this when you want statistically useful measurements:

```sh
cargo bench --workspace
```

Criterion writes history and HTML reports under `target/criterion`.

## Focused Runs

Run only the polynomial benchmarks:

```sh
cargo bench -p gbx-poly --bench reduction
```

Run only the F4/Groebner benchmarks:

```sh
cargo bench -p gbx-grobner --bench f4
```

Filter to one benchmark group:

```sh
cargo bench -p gbx-grobner --bench f4 f4_reducers
cargo bench -p gbx-poly --bench reduction polynomial_ops
```

## Baselines and Comparisons

Record a named baseline before changing an algorithm:

```sh
cargo bench --workspace -- --save-baseline before-change
```

After the change, compare against it:

```sh
cargo bench --workspace -- --baseline before-change
```

For a narrower comparison while tuning F4:

```sh
cargo bench -p gbx-grobner --bench f4 -- --save-baseline f4-before
cargo bench -p gbx-grobner --bench f4 -- --baseline f4-before
```

## Profiling-Oriented Runs

The Groebner crate has profiling feature flags. Use them when you need phase
counters or allocation-oriented data around F4:

```sh
cargo bench -p gbx-grobner --bench f4 --features profile-full
```

For lower-level profilers, build the exact bench binary first:

```sh
cargo bench -p gbx-grobner --bench f4 --no-run
```

Then run the produced binary from `target/release/deps/` under tools such as
`perf`, `heaptrack`, or `valgrind`.

## Current Coverage

- `gbx-alg`: trait dispatch overhead for algebraic operations.
- `gbx-field`: dynamic prime-field add, multiply, inverse, and division.
- `gbx-poly`: monomial algorithms, monomial order comparison, polynomial
  construction/arithmetic, and normal form reduction.
- `gbx-grobner`: end-to-end F4 reducer comparisons and batch-size sensitivity
  on fixed small systems.

When adding a new optimization, prefer adding a fixed benchmark fixture before
changing the implementation. Keep benchmarks deterministic and avoid external
parsers or tools unless the benchmark is specifically measuring integration
cost.
