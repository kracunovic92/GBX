use criterion::Criterion;
use std::hint::black_box;

use gbx_field::zp::{ZpDyn, ZpDynElem};

pub fn bench(c: &mut Criterion) {
    let ring = ZpDyn::modulus(998_244_353).unwrap();
    let a: ZpDynElem = ring.new(123456789);
    let b: ZpDynElem = ring.new(987654321);

    let mut group = c.benchmark_group("Zp_dynamic");

    group.bench_function("add", |ben| {
        ben.iter(|| black_box(ring.add(black_box(a), black_box(b))))
    });

    group.bench_function("sub", |ben| {
        ben.iter(|| black_box(ring.sub(black_box(a), black_box(b))))
    });

    group.bench_function("mul", |ben| {
        ben.iter(|| black_box(ring.mul(black_box(a), black_box(b))))
    });

    group.bench_function("neg", |ben| ben.iter(|| black_box(ring.neg(black_box(a)))));

    group.bench_function("pow_u128", |ben| {
        ben.iter(|| black_box(ring.pow(black_box(a), black_box(123456789u128))))
    });

    group.finish();
}
