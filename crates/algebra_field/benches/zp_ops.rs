#![allow(missing_docs)]
use algebra_core::TryInverse;
use algebra_field::Zp;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

type F = Zp<1_000_000_007>; // “biggish” prime

fn bench_add(c: &mut Criterion) {
    c.bench_function("Zp add", |b| {
        b.iter_batched(
            || (F::new(123456789), F::new(987654321)),
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

fn bench_mul(c: &mut Criterion) {
    c.bench_function("Zp mul", |b| {
        b.iter_batched(
            || (F::new(123456789), F::new(987654321)),
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

fn bench_inv(c: &mut Criterion) {
    c.bench_function("Zp inv", |b| {
        b.iter_batched(
            || F::new(123456789),
            |x| {
                let mut acc = x;
                for _ in 0..1_000 {
                    acc = acc.try_inv().unwrap();
                }
                acc
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(zp_ops, bench_add, bench_mul, bench_inv);
criterion_main!(zp_ops);
