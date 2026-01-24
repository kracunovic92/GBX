#![allow(missing_docs)]

use criterion::{BenchmarkId, Criterion};
use gbx_field::prelude::Zp;
use std::hint::black_box;

type F7 = Zp<7u32>;
type F998 = Zp<998_244_353u32>;

fn label<const P: u64>(suffix: &str) -> String {
    format!("Zp<{}>/{}", P, suffix)
}

pub fn bench_zp_pow(c: &mut Criterion) {
    let mut group = c.benchmark_group("zp_pow");

    let small_e: u128 = 20;
    let medium_e: u128 = 1_000_000;
    let dense_bits: u128 = u128::MAX >> 1;
    let sparse_bits: u128 = 1u128 << 120;

    group.bench_function(
        BenchmarkId::new("mul", label::<7>("a*=b (loop 10k)")),
        |b| {
            b.iter(|| {
                let mut x = black_box(F7::new_checked(3));
                let y = black_box(F7::new_checked(5));
                for _ in 0..10_000u32 {
                    x *= y;
                }
                black_box(x)
            })
        },
    );

    group.bench_function(
        BenchmarkId::new("mul", label::<998_244_353>("a*=b (loop 10k)")),
        |b| {
            b.iter(|| {
                let mut x = black_box(F998::new_checked(3));
                let y = black_box(F998::new_checked(5));
                for _ in 0..10_000u32 {
                    x *= y;
                }
                black_box(x)
            })
        },
    );

    group.bench_function(BenchmarkId::new("pow", label::<7>("e=20")), |b| {
        b.iter(|| {
            let a = F7::new_checked(black_box(3u32));
            black_box(a).pow(black_box(small_e))
        })
    });

    group.bench_function(BenchmarkId::new("pow", label::<7>("e=1e6")), |b| {
        b.iter(|| {
            let a = F7::new_checked(black_box(3u32));
            black_box(a).pow(black_box(medium_e))
        })
    });

    group.bench_function(
        BenchmarkId::new("pow", label::<998_244_353>("dense_bits")),
        |b| {
            b.iter(|| {
                let a = F998::new_checked(black_box(3u32));
                black_box(a).pow(black_box(dense_bits))
            })
        },
    );

    group.bench_function(
        BenchmarkId::new("pow", label::<998_244_353>("sparse_bits")),
        |b| {
            b.iter(|| {
                let a = F998::new_checked(black_box(3u32));
                black_box(a).pow(black_box(sparse_bits))
            })
        },
    );

    group.finish();
}
