#![allow(missing_docs)]

use criterion::{Criterion, criterion_group, criterion_main};

mod modules {
    pub mod fp_div;
    pub mod fp_inv;
    pub mod zp_pow;
}

fn criterion() -> Criterion {
    Criterion::default()
        .warm_up_time(std::time::Duration::from_secs(2))
        .measurement_time(std::time::Duration::from_secs(5))
        .sample_size(30)
}

criterion_group! {
    name = gbx_field_benches;
    config = criterion();
    targets =
        modules::zp_pow::bench_zp_pow,
        modules::fp_inv::bench_fp_inv,
        modules::fp_div::bench_fp_div
}
criterion_main!(gbx_field_benches);
