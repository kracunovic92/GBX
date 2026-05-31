use criterion::Criterion;
use std::hint::black_box;

use gbx_field::fp::{Fp, FpElem};

pub fn bench(c: &mut Criterion) {
    let field = match Fp::prime(998_244_353) {
        Ok(field) => field,
        Err(err) => panic!("998244353 should define a prime field: {err}"),
    };
    let a: FpElem = field.elem(123_456_789);
    let b: FpElem = field.elem(987_654_321);

    let mut group = c.benchmark_group("Fp_dynamic");

    group.bench_function("add", |ben| {
        ben.iter(|| black_box(field.add(black_box(a), black_box(b))));
    });

    group.bench_function("mul", |ben| {
        ben.iter(|| black_box(field.mul(black_box(a), black_box(b))));
    });

    group.bench_function("inv_try", |ben| {
        ben.iter(|| black_box(field.try_inv(black_box(a))));
    });

    group.bench_function("div_checked", |ben| {
        ben.iter(|| black_box(field.checked_div(black_box(a), black_box(b))));
    });

    group.finish();
}
