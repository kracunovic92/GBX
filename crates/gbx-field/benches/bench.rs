#![allow(missing_docs)]
use criterion::{criterion_group, criterion_main, Criterion};

mod fields;

fn all_benches(c: &mut Criterion) {
    fields::zp_static::bench(c);
    fields::zp_dynamic::bench(c);
    fields::fp_static::bench(c);
    fields::fp_dynamic::bench(c);
}

criterion_group!(benches, all_benches);
criterion_main!(benches);
