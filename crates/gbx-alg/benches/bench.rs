#![allow(missing_docs)]
mod modules {
    pub mod trait_overhead;
}

use criterion::{Criterion, criterion_group, criterion_main};

fn criterion() -> Criterion {
    Criterion::default()
        .warm_up_time(std::time::Duration::from_secs(2))
        .measurement_time(std::time::Duration::from_secs(5))
        .sample_size(30)
}

criterion_group! {
    name = gbx_alg_benches;
    config = criterion();
    targets =
        modules::trait_overhead::bench_trait_overhead
}
criterion_main!(gbx_alg_benches);
