#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use gbx_field::fp::{Fp, FpElem};
use gbx_poly::monomial::Monomial;
use gbx_poly::order::Lex;
use gbx_poly::polynomial::{Polynomial, PolynomialMut, PolynomialReduce};
use gbx_poly::ring::{FieldCtx, Ring, RingCtx};
use gbx_poly::term::Term;
use std::hint::black_box;

// ----------------------------
// Setup
// ----------------------------

type P = Polynomial<FpElem>;

fn ring() -> RingCtx<Fp, Lex> {
    Ring::builder()
        .field(Fp::prime(32003).unwrap())
        .order(Lex)
        .nvars(4)
        .build()
        .unwrap()
}

fn term(ring: &RingCtx<Fp, Lex>, coeff: u32, exps: &[u32]) -> Term<FpElem> {
    Term::new(ring.field.new(coeff), Monomial::from_slice(exps))
}

fn poly(ring: &RingCtx<Fp, Lex>, terms: &[(u32, &[u32])]) -> P {
    let terms = terms
        .iter()
        .map(|&(coeff, exps)| term(ring, coeff, exps))
        .collect();

    P::from_terms_in(ring, terms).unwrap()
}

// ----------------------------
// Bench helpers
// ----------------------------

fn make_reducers(ring: &RingCtx<Fp, Lex>, n: usize) -> Vec<P> {
    (0..n)
        .map(|i| {
            let a0 = 1 + (i % 5) as u32;
            let a1 = (i % 4) as u32;
            let a2 = (i % 3) as u32;

            poly(ring, &[(1, &[a0, a1, a2, 0]), (1, &[0, 1, 0, 0])])
        })
        .collect()
}

// ----------------------------
// Benches
// ----------------------------

fn bench_normal_form(c: &mut Criterion) {
    let ring = ring();

    let mut group = c.benchmark_group("normal_form");

    let f_small = poly(
        &ring,
        &[(1, &[7, 0, 0, 0]), (2, &[6, 1, 0, 0]), (3, &[5, 2, 0, 0]), (4, &[4, 3, 0, 0]), (5, &[0, 0, 0, 0])],
    );

    let g_small = vec![poly(&ring, &[(1, &[1, 0, 0, 0])]), poly(&ring, &[(1, &[0, 1, 0, 0])])];

    group.bench_function("small_basis_2", |b| {
        b.iter(|| {
            let r = black_box(&f_small)
                .normal_form(black_box(&ring), g_small.iter())
                .unwrap();

            black_box(r);
        });
    });

    let f = poly(
        &ring,
        &[(1, &[8, 0, 0, 0]), (2, &[7, 1, 0, 0]), (3, &[6, 2, 1, 0]), (4, &[5, 3, 1, 0]), (5, &[4, 1, 2, 0]), (6, &[0, 0, 0, 0])],
    );

    for size in [8usize, 32, 128] {
        let reducers = make_reducers(&ring, size);

        group.bench_with_input(BenchmarkId::new("scaling_basis", size), &size, |b, _| {
            b.iter(|| {
                let r = black_box(&f)
                    .normal_form(black_box(&ring), reducers.iter())
                    .unwrap();

                black_box(r);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_normal_form);
criterion_main!(benches);
