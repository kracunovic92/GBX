#![allow(missing_docs)]

use criterion::{BenchmarkId, Criterion};
use gbx_alg::{Additive, Multiplicative};
use std::hint::black_box;

const N: u64 = 10_000;
const MUL_N: u64 = 30;

pub fn bench_trait_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("trait_overhead");

    // ---------- add ----------
    group.bench_function(BenchmarkId::new("add", "i32/trait"), |b| {
        b.iter(|| {
            let one = black_box(1i32);
            let mut x = black_box(0i32);
            for _ in 0..N {
                x = x.add(one);
            }
            black_box(x)
        });
    });

    group.bench_function(BenchmarkId::new("add", "i32/+"), |b| {
        b.iter(|| {
            let one = black_box(1i32);
            let mut x = black_box(0i32);
            for _ in 0..N {
                x = black_box(x + one);
            }
            black_box(x)
        });
    });

    group.bench_function(BenchmarkId::new("add", "i32/wrapping_add"), |b| {
        b.iter(|| {
            let one = black_box(1i32);
            let mut x = black_box(0i32);
            for _ in 0..N {
                x = x.wrapping_add(one);
            }
            black_box(x)
        });
    });

    // ---------- mul ----------
    group.bench_function(BenchmarkId::new("mul", "i32/trait"), |b| {
        b.iter(|| {
            let two = black_box(2i32);
            let mut x = black_box(1i32);
            for _ in 0..MUL_N {
                x = x.mul(two);
            }
            black_box(x)
        });
    });

    group.bench_function(BenchmarkId::new("mul", "i32/*"), |b| {
        b.iter(|| {
            let two = black_box(2i32);
            let mut x = black_box(1i32);
            for _ in 0..MUL_N {
                x = black_box(x * two);
            }
            black_box(x)
        });
    });

    group.bench_function(BenchmarkId::new("mul", "i32/wrapping_mul"), |b| {
        b.iter(|| {
            let two = black_box(2i32);
            let mut x = black_box(1i32);
            for _ in 0..MUL_N {
                x = x.wrapping_mul(two);
            }
            black_box(x)
        });
    });

    group.finish();
}
