use criterion::Criterion;
use std::hint::black_box;

use gbx_alg::{CheckedDiv, TryInverse};
use gbx_field::fp::Fp;

pub fn bench(c: &mut Criterion) {
    type F = Fp<998_244_353>;
    let a = F::new(123456789);
    let b = F::new(987654321);

    let mut group = c.benchmark_group("Fp_static");

    group.bench_function("add", |ben| {
        ben.iter(|| black_box(black_box(a) + black_box(b)))
    });

    group.bench_function("mul", |ben| {
        ben.iter(|| black_box(black_box(a) * black_box(b)))
    });

    group.bench_function("inv_try", |ben| {
        ben.iter(|| black_box(black_box(a).try_inv()))
    });

    group.bench_function("div_checked", |ben| {
        ben.iter(|| black_box(black_box(a).checked_div(black_box(b))))
    });

    group.finish();
}
