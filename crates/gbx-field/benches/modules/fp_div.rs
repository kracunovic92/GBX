use criterion::Criterion;
use gbx_alg::CheckedDiv;
use gbx_field::fp::Fp;
use std::hint::black_box;

pub fn bench_fp_div(c: &mut Criterion) {
    type F = Fp<1_000_000_007>;

    c.bench_function("Fp checked_div", |b| {
        b.iter(|| {
            let a = F::new(black_box(987654321));
            let b2 = F::new(black_box(123456789));
            black_box(a.checked_div(b2))
        })
    });
}
