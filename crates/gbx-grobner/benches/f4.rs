#![allow(missing_docs)]
#![allow(clippy::expect_used, clippy::missing_panics_doc)]

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use gbx_field::fp::{Fp, FpElem};
use gbx_grobner::{BasisPostOptionsKind, F4Options, F4ReducerKind, f4};
use gbx_poly::monomial::Monomial;
use gbx_poly::order::{Lex, MonomialOrder};
use gbx_poly::polynomial::Polynomial;
use gbx_poly::ring::{Ring, RingCtx};
use gbx_poly::term::Term;
use std::hint::black_box;
use std::time::Duration;

type P = Polynomial<FpElem>;

fn criterion() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(2))
        .sample_size(10)
}

fn ring(nvars: usize) -> RingCtx<Fp, Lex> {
    let field = Fp::prime(32_003).expect("32003 should be prime");

    Ring::builder()
        .field(field)
        .order(Lex)
        .nvars(nvars)
        .build()
        .expect("benchmark ring should be valid")
}

const fn coeff(field: Fp, value: i32) -> FpElem {
    if value >= 0 {
        return field.elem(value.unsigned_abs());
    }

    let mag = value.unsigned_abs() % field.modulus();
    if mag == 0 { field.zero() } else { field.elem(field.modulus() - mag) }
}

fn term<O: MonomialOrder>(ring: &RingCtx<Fp, O>, coeff_value: i32, exps: &[u32]) -> Term<FpElem> {
    Term::new(coeff(ring.field, coeff_value), Monomial::from_slice(exps))
}

fn poly<O: MonomialOrder>(ring: &RingCtx<Fp, O>, terms: &[(i32, &[u32])]) -> P {
    let terms = terms
        .iter()
        .map(|&(coeff_value, exps)| term(ring, coeff_value, exps))
        .collect();

    P::from_terms_in(ring, terms).expect("benchmark polynomial should be valid")
}

fn cyclic3(ring: &RingCtx<Fp, Lex>) -> Vec<P> {
    vec![poly(ring, &[(1, &[1, 0, 0]), (1, &[0, 1, 0]), (1, &[0, 0, 1])]), poly(ring, &[(1, &[1, 1, 0]), (1, &[0, 1, 1]), (1, &[1, 0, 1])]), poly(ring, &[(1, &[1, 1, 1]), (-1, &[0, 0, 0])])]
}

fn katsura3(ring: &RingCtx<Fp, Lex>) -> Vec<P> {
    vec![
        poly(
            ring,
            &[(1, &[1, 0, 0, 0]), (2, &[0, 1, 0, 0]), (2, &[0, 0, 1, 0]), (2, &[0, 0, 0, 1]), (-1, &[0, 0, 0, 0])],
        ),
        poly(
            ring,
            &[(1, &[2, 0, 0, 0]), (2, &[0, 2, 0, 0]), (2, &[0, 0, 2, 0]), (2, &[0, 0, 0, 2]), (-1, &[1, 0, 0, 0])],
        ),
        poly(
            ring,
            &[(1, &[0, 1, 0, 1]), (1, &[0, 0, 2, 0]), (-1, &[0, 1, 0, 0])],
        ),
        poly(ring, &[(1, &[0, 0, 1, 1]), (-1, &[0, 0, 1, 0])]),
    ]
}

const fn options(reducer_kind: F4ReducerKind, batch_size: usize) -> F4Options {
    F4Options { batch_size, reducer_kind, normalize_inputs: true, normalize_extracted: true, safety_reduce_extracted: true, post: BasisPostOptionsKind::Reduced }
}

const fn reducer_name(reducer_kind: F4ReducerKind) -> &'static str {
    match reducer_kind {
        F4ReducerKind::Dense => "dense",
        F4ReducerKind::Roman => "roman",
        F4ReducerKind::RomanParallel => "roman_parallel",
    }
}

fn bench_f4_reducers(c: &mut Criterion) {
    let mut group = c.benchmark_group("f4_reducers");

    let cyclic3_ring = ring(3);
    let cyclic3_generators = cyclic3(&cyclic3_ring);
    let katsura3_ring = ring(4);
    let katsura3_generators = katsura3(&katsura3_ring);

    for reducer_kind in [F4ReducerKind::Dense, F4ReducerKind::Roman, F4ReducerKind::RomanParallel] {
        let opts = options(reducer_kind, 64);

        group.bench_with_input(
            BenchmarkId::new("cyclic3", reducer_name(reducer_kind)),
            &opts,
            |b, opts| {
                b.iter_batched(
                    || cyclic3_generators.clone(),
                    |generators| {
                        let basis = f4(
                            black_box(&cyclic3_ring),
                            black_box(generators),
                            black_box(*opts),
                        )
                        .expect("cyclic3 F4 benchmark should succeed");
                        black_box(basis.len());
                    },
                    BatchSize::SmallInput,
                );
            },
        );

        group.bench_with_input(
            BenchmarkId::new("katsura3", reducer_name(reducer_kind)),
            &opts,
            |b, opts| {
                b.iter_batched(
                    || katsura3_generators.clone(),
                    |generators| {
                        let basis = f4(
                            black_box(&katsura3_ring),
                            black_box(generators),
                            black_box(*opts),
                        )
                        .expect("katsura3 F4 benchmark should succeed");
                        black_box(basis.len());
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

fn bench_f4_batch_size(c: &mut Criterion) {
    let ring = ring(3);
    let generators = cyclic3(&ring);
    let mut group = c.benchmark_group("f4_batch_size");

    for batch_size in [1usize, 4, 16, 64] {
        let opts = options(F4ReducerKind::Roman, batch_size);

        group.bench_with_input(
            BenchmarkId::new("cyclic3_roman", batch_size),
            &opts,
            |b, opts| {
                b.iter_batched(
                    || generators.clone(),
                    |generators| {
                        let basis = f4(black_box(&ring), black_box(generators), black_box(*opts)).expect("cyclic3 batch-size benchmark should succeed");
                        black_box(basis.len());
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = criterion();
    targets = bench_f4_reducers, bench_f4_batch_size
}
criterion_main!(benches);
