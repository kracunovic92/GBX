#![allow(missing_docs)]
#![allow(clippy::expect_used, clippy::missing_panics_doc)]

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use gbx_field::fp::{Fp, FpElem};
use gbx_poly::monomial::{Monomial, checked_lcm, checked_quotient, divides, gcd_is_one};
use gbx_poly::order::{Grevlex, Lex, MonomialOrder};
use gbx_poly::polynomial::{Polynomial, PolynomialMut, PolynomialOps, PolynomialReduce};
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

fn make_dense_poly(ring: &RingCtx<Fp, Lex>, term_count: usize) -> P {
    let terms = (0..term_count)
        .map(|i| {
            let coeff = u32::try_from((i % 251) + 1).expect("coefficient fixture fits in u32");
            let e0 = u32::try_from((term_count - i) % 17).expect("exponent fixture fits in u32");
            let e1 = u32::try_from((i * 3) % 11).expect("exponent fixture fits in u32");
            let e2 = u32::try_from((i * 5) % 7).expect("exponent fixture fits in u32");
            let e3 = u32::try_from(i % 5).expect("exponent fixture fits in u32");

            term(ring, coeff, &[e0, e1, e2, e3])
        })
        .collect();

    P::from_terms_in(ring, terms).expect("dense benchmark polynomial should be valid")
}

fn make_monomial_pairs(nvars: usize, count: usize) -> Vec<(Monomial, Monomial)> {
    (0..count)
        .map(|i| {
            let a = (0..nvars)
                .map(|j| u32::try_from((i + (j * 3)) % 13).expect("exponent fixture fits in u32"))
                .collect::<Vec<_>>();
            let b = (0..nvars)
                .map(|j| u32::try_from(((i * 2) + j) % 17).expect("exponent fixture fits in u32"))
                .collect::<Vec<_>>();

            (Monomial::from_vec(a), Monomial::from_vec(b))
        })
        .collect()
}

// ----------------------------
// Benches
// ----------------------------

fn bench_monomial_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("monomial_ops");

    for nvars in [3usize, 8, 16] {
        let pairs = make_monomial_pairs(nvars, 512);

        group.bench_with_input(
            BenchmarkId::new("divides_batch", nvars),
            &pairs,
            |b, pairs| {
                b.iter(|| {
                    let mut hits = 0usize;

                    for (a, b) in black_box(pairs) {
                        if divides(a, b) {
                            hits += 1;
                        }
                    }

                    black_box(hits);
                });
            },
        );

        group.bench_with_input(BenchmarkId::new("lcm_batch", nvars), &pairs, |b, pairs| {
            b.iter(|| {
                let mut degree_sum = 0u32;

                for (a, b) in black_box(pairs) {
                    let lcm = checked_lcm(a, b).expect("matching arity lcm should succeed");
                    degree_sum = degree_sum.wrapping_add(lcm.degree());
                }

                black_box(degree_sum);
            });
        });

        group.bench_with_input(
            BenchmarkId::new("quotient_batch", nvars),
            &pairs,
            |b, pairs| {
                b.iter(|| {
                    let mut hits = 0usize;

                    for (a, b) in black_box(pairs) {
                        if checked_quotient(a, b)
                            .expect("matching arity quotient should succeed")
                            .is_some()
                        {
                            hits += 1;
                        }
                    }

                    black_box(hits);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("gcd_is_one_batch", nvars),
            &pairs,
            |b, pairs| {
                b.iter(|| {
                    let mut hits = 0usize;

                    for (a, b) in black_box(pairs) {
                        if gcd_is_one(a, b) {
                            hits += 1;
                        }
                    }

                    black_box(hits);
                });
            },
        );
    }

    group.finish();
}

fn bench_order_cmp(c: &mut Criterion) {
    let pairs = make_monomial_pairs(8, 1024);
    let lex = Lex;
    let grevlex = Grevlex;
    let mut group = c.benchmark_group("monomial_order");

    group.bench_function("lex_cmp_batch", |b| {
        b.iter(|| {
            let mut greater = 0usize;

            for (a, b) in black_box(&pairs) {
                if lex.cmp(a, b).is_gt() {
                    greater += 1;
                }
            }

            black_box(greater);
        });
    });

    group.bench_function("grevlex_cmp_batch", |b| {
        b.iter(|| {
            let mut greater = 0usize;

            for (a, b) in black_box(&pairs) {
                if grevlex.cmp(a, b).is_gt() {
                    greater += 1;
                }
            }

            black_box(greater);
        });
    });

    group.finish();
}

fn bench_polynomial_ops(c: &mut Criterion) {
    let ring = ring();
    let mut group = c.benchmark_group("polynomial_ops");

    for size in [16usize, 64, 256] {
        let lhs = make_dense_poly(&ring, size);
        let rhs = make_dense_poly(&ring, size);
        let mono = Monomial::from_slice(&[3, 1, 2, 0]);
        let coeff = ring.field.elem(7);

        group.bench_with_input(
            BenchmarkId::new("from_terms_in", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let poly = make_dense_poly(black_box(&ring), black_box(size));
                    black_box(poly);
                });
            },
        );

        group.bench_with_input(BenchmarkId::new("add_canonical", size), &size, |b, _| {
            b.iter(|| {
                let sum = black_box(&lhs)
                    .add_canonical(black_box(&ring), black_box(&rhs))
                    .expect("addition benchmark should succeed");
                black_box(sum);
            });
        });

        group.bench_with_input(BenchmarkId::new("mul_canonical", size), &size, |b, _| {
            b.iter(|| {
                let product = black_box(&lhs)
                    .mul_canonical(black_box(&ring), black_box(&rhs))
                    .expect("multiplication benchmark should succeed");
                black_box(product);
            });
        });

        group.bench_with_input(
            BenchmarkId::new("sub_scaled_monomial_multiple", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let mut out = black_box(lhs.clone());
                    out.sub_scaled_monomial_multiple_in_place(
                        black_box(&ring),
                        black_box(&rhs),
                        black_box(&mono),
                        black_box(coeff),
                    )
                    .expect("sub scaled monomial multiple benchmark should succeed");
                    out.normalize_in_place(black_box(&ring))
                        .expect("normalization benchmark should succeed");
                    black_box(out);
                });
            },
        );
    }

    group.finish();
}

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

criterion_group!(
    benches,
    bench_monomial_ops,
    bench_order_cmp,
    bench_polynomial_ops,
    bench_normal_form
);
criterion_main!(benches);
