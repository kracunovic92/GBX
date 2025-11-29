#![allow(missing_docs)]

use algebra_core::{Additive, Multiplicative};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn bench_add_i64(c: &mut Criterion) {
    c.bench_function("i64_additive_add", |b| {
        b.iter_batched(
            || (1i64, 2i64),
            |(mut acc, x)| {
                for _ in 0..1_000 {
                    acc = acc.add(x);
                }
                acc
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("i64_raw_add", |b| {
        b.iter_batched(
            || (1i64, 2i64),
            |(mut acc, x)| {
                for _ in 0..1_000 {
                    acc = acc + x;
                }
                acc
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_mul_i64(c: &mut Criterion) {
    c.bench_function("i64_multiplicative_mul", |b| {
        b.iter_batched(
            || (2i64, 3i64),
            |(mut acc, x)| {
                for _ in 0..1_000 {
                    acc = acc.mul(x);
                }
                acc
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("i64_raw_mul", |b| {
        b.iter_batched(
            || (2i64, 3i64),
            |(mut acc, x)| {
                for _ in 0..1_000 {
                    acc = acc * x;
                }
                acc
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(throughput, bench_add_i64, bench_mul_i64);
criterion_main!(throughput);
