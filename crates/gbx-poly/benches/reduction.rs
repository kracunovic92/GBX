#![allow(missing_docs)]

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use gbx_field::fp::{Fp, FpElem};
use gbx_poly::monomial::Monomial;
use gbx_poly::order::Lex;
use gbx_poly::polynomial::{Polynomial, PolynomialReduce};
use gbx_poly::ring::{Ring, RingCtx};
use gbx_poly::term::Term;
use std::hint::black_box;

// ----------------------------
// Setup
// ----------------------------

type P = Polynomial<FpElem>;

fn ring() -> RingCtx<Fp, Lex> {
    let field = match Fp::prime(32_003) {
        Ok(field) => field,
        Err(err) => panic!("32003 should be prime: {err}"),
    };

    match Ring::builder().field(field).order(Lex).nvars(4).build() {
        Ok(ring) => ring,
        Err(err) => panic!("valid benchmark ring should build: {err}"),
    }
}

fn term(ring: &RingCtx<Fp, Lex>, coeff: u32, exps: &[u32]) -> Term<FpElem> {
    Term::new(ring.field.elem(coeff), Monomial::from_slice(exps))
}

fn poly(ring: &RingCtx<Fp, Lex>, terms: &[(u32, &[u32])]) -> P {
    let terms = terms
        .iter()
        .map(|&(coeff, exps)| term(ring, coeff, exps))
        .collect();

    match P::from_terms_in(ring, terms) {
        Ok(poly) => poly,
        Err(err) => panic!("benchmark polynomial should be valid: {err}"),
    }
}

// ----------------------------
// Bench helpers
// ----------------------------

fn make_reducers(ring: &RingCtx<Fp, Lex>, n: usize) -> Vec<P> {
    (0..n)
        .map(|i| {
            let Ok(a0_offset) = u32::try_from(i % 5) else {
                unreachable!("modulo 5 result fits in u32");
            };
            let Ok(a1) = u32::try_from(i % 4) else {
                unreachable!("modulo 4 result fits in u32");
            };
            let Ok(a2) = u32::try_from(i % 3) else {
                unreachable!("modulo 3 result fits in u32");
            };
            let a0 = 1 + a0_offset;

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

    let g_small = [poly(&ring, &[(1, &[1, 0, 0, 0])]), poly(&ring, &[(1, &[0, 1, 0, 0])])];

    group.bench_function("small_basis_2", |b| {
        b.iter(|| {
            let r = match black_box(&f_small).normal_form(black_box(&ring), g_small.iter()) {
                Ok(result) => result,
                Err(err) => panic!("small benchmark reduction should succeed: {err}"),
            };

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
                let r = match black_box(&f).normal_form(black_box(&ring), reducers.iter()) {
                    Ok(result) => result,
                    Err(err) => panic!("scaling benchmark reduction should succeed: {err}"),
                };

                black_box(r);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_normal_form);
criterion_main!(benches);
