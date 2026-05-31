#![allow(missing_docs)]
use criterion::{Criterion, criterion_group, criterion_main};

mod fields;

fn all_benches(c: &mut Criterion) {
    fields::fp_dynamic::bench(c);
}

criterion_group!(benches, all_benches);
criterion_main!(benches);
