#![allow(missing_docs)]
use criterion::{criterion_group, criterion_main, Criterion};

mod fields;

fn all_benches(c: &mut Criterion) {
    fields::fp_dynamic::bench(c);
}

criterion_group!(benches, all_benches);
criterion_main!(benches);
