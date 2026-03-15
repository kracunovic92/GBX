#![allow(missing_docs)]
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use gbx_field::fp::{Fp, FpDyn, FpDynElem};
use gbx_poly::monomial::{DynamicMonomial, FixedMonomial};
use gbx_poly::order::Lex;
use gbx_poly::polynomial::poly::Polynomial;
use gbx_poly::polynomial::PolynomialReduce;
use gbx_poly::ring::{Ring, StaticFpCtx};
use gbx_poly::term::Term;
use gbx_storage::polynomial::VecTerms;
use std::hint::black_box;

// ----------------------------
// Static setup
// ----------------------------
type F7 = Fp<7>;
type T2 = Term<F7, FixedMonomial<2>>;
type P2 = Polynomial<T2, VecTerms<T2>>;

fn ring_static() -> gbx_poly::ring::RingCtx<StaticFpCtx<7>, Lex> {
    Ring::builder()
        .field(StaticFpCtx::<7>::new())
        .order(Lex)
        .nvars(2)
        .build()
        .unwrap()
}

fn t2(c: u32, e0: u32, e1: u32) -> T2 {
    Term::new(F7::new(c), FixedMonomial::from_exponents([e0, e1]))
}

fn p2(r: &gbx_poly::ring::RingCtx<StaticFpCtx<7>, Lex>, terms: &[(u32, u32, u32)]) -> P2 {
    let ts = terms.iter().map(|&(c, a, b)| t2(c, a, b)).collect();
    P2::from_terms_in(r, ts).unwrap()
}

// ----------------------------
// Dynamic setup
// ----------------------------
type TD = Term<FpDynElem, DynamicMonomial>;
type PD = Polynomial<TD, VecTerms<TD>>;

fn ring_dyn() -> gbx_poly::ring::RingCtx<FpDyn, Lex> {
    Ring::builder()
        .field(FpDyn::prime(32003).unwrap())
        .order(Lex)
        .nvars(4)
        .build()
        .unwrap()
}

fn td(r: &gbx_poly::ring::RingCtx<FpDyn, Lex>, c: u32, exps: &[u32]) -> TD {
    Term::new(r.field.new(c), DynamicMonomial::from_slice(exps))
}

fn pd(r: &gbx_poly::ring::RingCtx<FpDyn, Lex>, terms: &[(u32, &[u32])]) -> PD {
    let ts = terms.iter().map(|&(c, e)| td(r, c, e)).collect();
    PD::from_terms_in(r, ts).unwrap()
}

// ----------------------------
// Bench helpers
// ----------------------------

fn make_static_reducers(ring: &gbx_poly::ring::RingCtx<StaticFpCtx<7>, Lex>, n: usize) -> Vec<P2> {
    // Reducers like:
    // x^(k) + y, x^(k-1)y + y, ...
    (0..n)
        .map(|i| {
            let a = 1 + (i % 6) as u32;
            let b = (i % 3) as u32;
            p2(ring, &[(1, a, b), (1, 0, 1)])
        })
        .collect()
}

fn make_dynamic_reducers(ring: &gbx_poly::ring::RingCtx<FpDyn, Lex>, n: usize) -> Vec<PD> {
    (0..n)
        .map(|i| {
            let a0 = 1 + (i % 5) as u32;
            let a1 = (i % 4) as u32;
            let a2 = (i % 3) as u32;
            pd(ring, &[(1, &[a0, a1, a2, 0]), (1, &[0, 1, 0, 0])])
        })
        .collect()
}

// ----------------------------
// Benches
// ----------------------------

fn bench_normal_form_static(c: &mut Criterion) {
    let ring = ring_static();

    let mut group = c.benchmark_group("normal_form_static");

    // Simple baseline
    let f_small = p2(
        &ring,
        &[(1, 7, 0), (2, 6, 1), (3, 5, 2), (4, 4, 3), (5, 0, 0)],
    );
    let g_small = vec![p2(&ring, &[(1, 1, 0)]), p2(&ring, &[(1, 0, 1)])];

    group.bench_function("small_basis_2", |b| {
        b.iter(|| {
            let r = black_box(&f_small)
                .normal_form(black_box(&ring), g_small.iter())
                .unwrap();
            black_box(r);
        });
    });

    // Scaling basis size
    for size in [8usize, 32, 128] {
        let reducers = make_static_reducers(&ring, size);
        let f = p2(
            &ring,
            &[(1, 10, 0), (2, 9, 1), (3, 8, 2), (4, 7, 3), (5, 6, 4), (6, 5, 5), (1, 0, 0)],
        );

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

fn bench_normal_form_dynamic(c: &mut Criterion) {
    let ring = ring_dyn();

    let mut group = c.benchmark_group("normal_form_dynamic");

    let f = pd(
        &ring,
        &[(1, &[8, 0, 0, 0]), (2, &[7, 1, 0, 0]), (3, &[6, 2, 1, 0]), (4, &[5, 3, 1, 0]), (5, &[4, 1, 2, 0]), (6, &[0, 0, 0, 0])],
    );

    for size in [8usize, 32, 128] {
        let reducers = make_dynamic_reducers(&ring, size);

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

criterion_group!(benches, bench_normal_form_static, bench_normal_form_dynamic);
criterion_main!(benches);
