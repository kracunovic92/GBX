use criterion::Criterion;
use gbx_alg::TryInverse;
use gbx_field::fp::Fp;
use std::hint::black_box;

pub fn bench_fp_inv(c: &mut Criterion) {
    type F = Fp<1_000_000_007>;

    c.bench_function("Fp try_inv", |b| {
        b.iter(|| {
            let x = F::new(black_box(123456789));
            black_box(x.try_inv())
        })
    });
}
