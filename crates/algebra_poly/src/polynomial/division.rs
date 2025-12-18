//! Multivariate polynomial division (generalized long division).
//!
//! This is the standard division algorithm used in Gröbner basis computations.
//!
//! Given a polynomial `f` and a list of divisors `g1..gs`, it computes polynomials
//! `q1..qs` (quotients) and `r` (remainder) such that:
//!
//! ```text
//! f = q1*g1 + q2*g2 + ... + qs*gs + r
//! ```
//!
//! with the additional property that **no term of `r` is divisible by any** `LT(gi)`
//! (leading term of `gi`) with respect to the monomial order.
//!
//! Notes / design choices:
//! - The algorithm depends only on `PolynomialMut` + `MonomialLike` + `TermLike`.
//! - It never assumes `terms()[0]` is the leading term unless your `leading_term()` does.
//! - It skips divisors that are the zero polynomial.
//! - Coefficient “division” is done as `a / b := a * inv(b)` using `TryInverse`.

use crate::monomial::MonomialLike;
use crate::polynomial::{mul_term_poly, PolynomialError, PolynomialLike, PolynomialMut};
use crate::term::TermLike;
use algebra_core::{Multiplicative, TryInverse, Zero};
use core::ops::Neg;

/// Result of dividing `f` by `g1..gs`.
#[derive(Debug, Clone)]
pub struct DivisionResult<P> {
    /// Quotients `q1..qs` (same length as the divisor list).
    pub quotients: Vec<P>,
    /// Remainder `r`.
    pub remainder: P,
}

/// Divide `f` by a list of divisors `gs` (generalized long division).
///
/// Returns `(q1..qs, r)` where:
///
/// - `f = Σ qi*gi + r`
/// - no term of `r` is divisible by any `LT(gi)`
///
/// The reduction step is:
///
/// - Let `LT(p) = lc(p) * lm(p)`
/// - For the first divisor `g_i` such that `lm(g_i) | lm(p)`:
///   - let `mq = lm(p) / lm(g_i)` (monomial quotient)
///   - let `cq = lc(p) / lc(g_i) = lc(p) * inv(lc(g_i))` (coefficient quotient)
///   - update `q_i += cq * mq`
///   - update `p -= (cq*mq) * g_i`
///
/// If no divisor can reduce `LT(p)`, move `LT(p)` to the remainder and remove it
/// from `p`.
pub fn divide<P>(f: &P, gs: &[P]) -> Result<DivisionResult<P>, PolynomialError>
where
    P: PolynomialMut,
    P::Field:
        Zero + Clone + Neg<Output = P::Field> + Multiplicative + TryInverse<Output = P::Field>,
    P::Mono: MonomialLike,
    P::Term: TermLike<Field = P::Field, Mono = P::Mono>,
{
    // qi start at 0.
    let mut qs: Vec<P> = (0..gs.len())
        .map(|_| P::zero())
        .collect();

    // remainder starts at 0, and p is a working copy of f.
    let mut r = P::zero();
    let mut p = f.clone();

    // Standard long-division loop.
    while !p.is_zero() {
        // Because p != 0, it must have a leading term.
        let lt_p = p
            .leading_term()
            .expect("non-zero polynomial must have a leading term")
            .clone();

        let lc_p = lt_p
            .coeff()
            .clone();
        let lm_p = lt_p
            .mono()
            .clone();

        let mut reduced = false;

        // Try divisors in the given order (important: this is generalized division).
        for (i, g) in gs
            .iter()
            .enumerate()
        {
            if g.is_zero() {
                continue; // skip zero divisor safely
            }

            let lt_g = g
                .leading_term()
                .expect("non-zero divisor must have a leading term");

            let lc_g = lt_g
                .coeff()
                .clone();
            let lm_g = lt_g.mono();

            // We can reduce iff LM(g) | LM(p).
            // With your MonomialLike API:
            // - lm_g.divides(&lm_p) checks divisibility
            // - lm_g.quotient(&lm_p) returns mq where mq * lm_g = lm_p
            if lm_g.divides(&lm_p) {
                let mq = lm_g
                    .quotient(&lm_p)
                    .expect("divides() true implies quotient() is Some");

                // cq = lc(p) / lc(g) = lc(p) * inv(lc(g))
                let inv_lc_g = lc_g
                    .try_inv()
                    .expect("in a field, non-zero coefficients must be invertible");
                let cq = lc_p
                    .clone()
                    .mul(inv_lc_g);

                // q_i += cq * mq  (as a single term)
                qs[i].add_term_normalized(term_from_parts::<P>(cq.clone(), mq.clone()));

                // p -= (cq*mq) * g
                let m = term_from_parts::<P>(cq, mq);
                let subtrahend = mul_term_poly::<P>(&m, g)?;
                p = P::sub_poly(&p, &subtrahend);

                reduced = true;
                break;
            }
        }

        if !reduced {
            // No divisor reduces LT(p): move it to remainder.
            r.add_term_normalized(lt_p.clone());

            // Remove LT(p) from p: p := p - LT(p)
            let lt_poly = P::from_terms(vec![lt_p]);
            p = P::sub_poly(&p, &lt_poly);
        }
    }

    Ok(DivisionResult { quotients: qs, remainder: r })
}

/// Construct a term from `(coeff, mono)` using `TermLike`.
#[inline]
fn term_from_parts<P>(coeff: P::Field, mono: P::Mono) -> P::Term
where
    P: PolynomialLike,
    P::Term: TermLike<Field = P::Field, Mono = P::Mono>,
{
    <P::Term as TermLike>::from_parts(coeff, mono)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monomial::Lex;
    use crate::polynomial::Polynomial;
    use crate::term::Term;
    use algebra_core::{One, Zero};
    use algebra_field::Zp;

    type F7 = Zp<7>;

    // 1-variable polynomials over F7 with Lex order.
    type P1 = Polynomial<F7, 1, Lex>;
    type T1 = Term<F7, 1>;

    fn x_pow(k: u64) -> T1 {
        T1::from_coeff_and_exponents(F7::one(), [k])
    }

    fn c(a: u64) -> T1 {
        T1::from_coeff_and_exponents(F7::new(a), [0])
    }

    #[test]
    fn divide_by_linear_polynomial_has_expected_q_and_r() {
        // f = x^2 + 1
        let f: P1 = P1::from_terms(vec![x_pow(2), c(1)]);

        // g = x + 1
        let g: P1 = P1::from_terms(vec![x_pow(1), c(1)]);

        let res = divide(&f, &[g.clone()]).unwrap();

        // Over F7: (x^2 + 1) / (x + 1) = (x - 1) remainder 2
        // -1 mod 7 = 6
        let q_expected: P1 = P1::from_terms(vec![
            T1::from_coeff_and_exponents(F7::one(), [1]),
            T1::from_coeff_and_exponents(F7::new(6), [0]),
        ]);
        let r_expected: P1 = P1::from_terms(vec![c(2)]);

        assert_eq!(
            res.quotients
                .len(),
            1
        );
        assert_eq!(res.quotients[0], q_expected);
        assert_eq!(res.remainder, r_expected);

        let reconstructed = P1::add_poly(&mul_poly(&res.quotients[0], &g), &res.remainder);
        assert_eq!(reconstructed, f);
    }

    #[test]
    fn divide_by_x_gives_remainder_constant() {
        let f: P1 = P1::from_terms(vec![x_pow(2), c(1)]);

        let g: P1 = P1::from_terms(vec![x_pow(1)]);

        let res = divide(&f, &[g.clone()]).unwrap();

        let q_expected: P1 = P1::from_terms(vec![x_pow(1)]);
        let r_expected: P1 = P1::from_terms(vec![c(1)]);

        assert_eq!(res.quotients[0], q_expected);
        assert_eq!(res.remainder, r_expected);

        let lt_g = g
            .leading_term()
            .unwrap();
        let lt_r = res
            .remainder
            .leading_term()
            .unwrap();
        assert!(
            !lt_g
                .mono()
                .divides(lt_r.mono())
        );
    }

    #[test]
    fn multiple_divisors_uses_first_applicable_divisor_order() {
        let f: P1 = P1::from_terms(vec![x_pow(2), x_pow(1)]);

        let g1: P1 = P1::from_terms(vec![x_pow(2)]);
        let g2: P1 = P1::from_terms(vec![x_pow(1)]);

        let res = divide(&f, &[g1.clone(), g2.clone()]).unwrap();

        let q1_expected: P1 = P1::from_terms(vec![c(1)]);
        let q2_expected: P1 = P1::from_terms(vec![c(1)]);
        let r_expected: P1 = P1::zero();

        assert_eq!(
            res.quotients
                .len(),
            2
        );
        assert_eq!(res.quotients[0], q1_expected);
        assert_eq!(res.quotients[1], q2_expected);
        assert_eq!(res.remainder, r_expected);

        let reconstructed = P1::add_poly(
            &P1::add_poly(
                &mul_poly(&res.quotients[0], &g1),
                &mul_poly(&res.quotients[1], &g2),
            ),
            &res.remainder,
        );
        assert_eq!(reconstructed, f);
    }

    #[test]
    fn zero_divisor_is_skipped() {
        // f = x
        let f: P1 = P1::from_terms(vec![x_pow(1)]);

        let zero: P1 = P1::zero();
        let g: P1 = P1::from_terms(vec![x_pow(1)]);

        let res = divide(&f, &[zero, g.clone()]).unwrap();

        // first divisor is zero, should be skipped
        assert_eq!(
            res.quotients
                .len(),
            2
        );
        assert_eq!(res.quotients[0], P1::zero());
        assert_eq!(res.quotients[1], P1::from_terms(vec![c(1)]));
        assert_eq!(res.remainder, P1::zero());
    }
    fn mul_poly<P>(a: &P, b: &P) -> P
    where
        P: PolynomialMut,
        P::Field: Zero + Clone + algebra_core::Additive + Multiplicative,
        P::Term: TermLike<Field = P::Field, Mono = P::Mono>,
        P::Mono: MonomialLike,
    {
        if a.is_zero() || b.is_zero() {
            return P::zero();
        }

        let mut out = Vec::with_capacity(
            a.terms()
                .len()
                * b.terms()
                    .len(),
        );
        for ta in a.terms() {
            for tb in b.terms() {
                let prod = ta
                    .checked_mul_term(tb)
                    .expect("term multiplication should not overflow in tests");
                out.push(prod);
            }
        }
        P::from_terms(out)
    }
}
