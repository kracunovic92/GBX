#![allow(missing_docs)]
use algebra_core::prelude::*;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

const N: i32 = 10_000;

fn bench_add_i32_trait(c: &mut Criterion) {
    c.bench_function("add i32 (trait)", |b| {
        b.iter(|| {
            let mut x = black_box(0i32);
            for _ in 0..N {
                x = x.add(black_box(1));
            }
            x
        })
    });
}

fn bench_add_i32_raw(c: &mut Criterion) {
    c.bench_function("add i32 (raw +)", |b| {
        b.iter(|| {
            let mut x = black_box(0i32);
            for _ in 0..N {
                x = x.wrapping_add(black_box(1));
            }
            x
        })
    });
}

fn bench_mul_i32_trait(c: &mut Criterion) {
    c.bench_function("mul i32 (trait)", |b| {
        b.iter(|| {
            let mut x = black_box(1i32);
            for _ in 0..N {
                x = x.mul(black_box(2));
                x = x.wrapping_mul(1);
            }
            x
        })
    });
}

fn bench_mul_i32_raw(c: &mut Criterion) {
    c.bench_function("mul i32 (raw *)", |b| {
        b.iter(|| {
            let mut x = black_box(1i32);
            for _ in 0..N {
                x = x.wrapping_mul(black_box(2));
            }
            x
        })
    });
}

criterion_group!(
    core_benches,
    bench_add_i32_trait,
    bench_add_i32_raw,
    bench_mul_i32_trait,
    bench_mul_i32_raw,
);
criterion_main!(core_benches);
