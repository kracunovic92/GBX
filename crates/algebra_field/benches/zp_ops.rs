#![allow(missing_docs)]
use algebra_core::{Additive, CheckedDiv, Multiplicative, One, TryInverse, Zero};
use algebra_field::Zp;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

type F7 = Zp<7>;

fn bench_add(c: &mut Criterion) {
    c.bench_function("Zp<7> add", |b| {
        b.iter(|| {
            let mut acc = F7::zero();
            for i in 0u64..1000 {
                acc = acc.add(F7::new(i));
            }
            black_box(acc)
        });
    });
}

fn bench_mul(c: &mut Criterion) {
    c.bench_function("Zp<7> mul", |b| {
        b.iter(|| {
            let mut acc = F7::one();
            for i in 1u64..1000 {
                acc = acc.mul(F7::new(i));
            }
            black_box(acc)
        });
    });
}

fn bench_inv(c: &mut Criterion) {
    c.bench_function("Zp<7> try_inv", |b| {
        b.iter(|| {
            let mut acc = F7::one();
            for i in 1u64..1000 {
                let x = F7::new(i);
                if x.is_zero() {
                    continue; // 0 has no inverse, skip
                }
                let inv = x
                    .try_inv()
                    .unwrap();
                acc = acc.mul(inv);
            }
            black_box(acc)
        });
    });
}

fn bench_checked_div(c: &mut Criterion) {
    c.bench_function("Zp<7> checked_div", |b| {
        b.iter(|| {
            let mut acc = F7::one();
            for i in 1u64..1000 {
                let x = F7::new(i);
                if x.is_zero() {
                    continue; // avoid division by zero
                }
                acc = acc
                    .checked_div(x)
                    .unwrap();
            }
            black_box(acc)
        });
    });
}

criterion_group!(
    zp_benches,
    bench_add,
    bench_mul,
    bench_inv,
    bench_checked_div
);
criterion_main!(zp_benches);
