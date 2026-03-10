use criterion::Criterion;
use std::hint::black_box;

use gbx_field::zp::Zp;

pub fn bench(c: &mut Criterion) {
    type R = Zp<998_244_353>;
    let a = R::new_checked(123456789);
    let b = R::new_checked(987654321);

    let mut group = c.benchmark_group("Zp_static");

    group.bench_function("add", |ben| {
        ben.iter(|| black_box(black_box(a) + black_box(b)))
    });

    group.bench_function("sub", |ben| {
        ben.iter(|| black_box(black_box(a) - black_box(b)))
    });

    group.bench_function("mul", |ben| {
        ben.iter(|| black_box(black_box(a) * black_box(b)))
    });

    group.bench_function("neg", |ben| ben.iter(|| black_box(-black_box(a))));

    group.bench_function("pow_u128", |ben| {
        ben.iter(|| black_box(black_box(a).pow(black_box(123456789u128))))
    });

    group.finish();
}
