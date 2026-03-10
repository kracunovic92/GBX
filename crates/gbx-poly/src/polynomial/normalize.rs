//! Term list normalization (context-driven).
//!
//! This normalizes a `Vec<Term>` in-place:
//! - drops zero coefficients (`ctx.field.is_zero`)
//! - sorts by `ctx.order` descending (leading term first)
//! - merges identical monomials using `ctx.field.add`
//!
//! # Errors
//! This function itself does not perform monomial arithmetic; it only compares and clones.
//! Errors are therefore limited to trait-bound enforced behaviors (currently none).
//!
//! # Panics
//! None.

use crate::monomial::{Monomial, MonomialView};
use crate::order::MonomialOrder;
use crate::polynomial::error::Result;
use crate::ring::{FieldCtx, RingCtx};
use crate::term::{TermOwned, TermView};

/// Normalize a term list in-place inside a ring context.
pub fn normalize_terms_in<F, O, T>(ctx: &RingCtx<F, O>, terms: &mut Vec<T>) -> Result<()>
where
    F: FieldCtx<Elem = T::Coeff>,
    O: MonomialOrder,
    T: TermOwned + TermView,
    T::Coeff: Copy + Eq,
    T::Mono: Monomial + MonomialView<Word = u32> + Clone + Eq,
{
    // 1) Drop zeros
    terms.retain(|t| !ctx.field.is_zero(*t.coeff()));
    if terms.is_empty() {
        return Ok(());
    }

    // 2) Sort by descending order (LT first)
    terms.sort_by(|a, b| ctx.order.cmp(a.mono(), b.mono()).reverse());

    // 3) Merge adjacent identical monomials
    let mut out: Vec<T> = Vec::with_capacity(terms.len());
    for t in terms.drain(..) {
        if let Some(last) = out.last_mut() {
            if last.mono() == t.mono() {
                let sum = ctx.field.add(*last.coeff(), *t.coeff());
                if ctx.field.is_zero(sum) {
                    out.pop();
                } else {
                    let mono = last.mono().clone();
                    *last = T::from_parts(sum, mono);
                }
                continue;
            }
        }
        out.push(t);
    }

    *terms = out;
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::normalize_terms_in;

    use crate::monomial::{DynamicMonomial, FixedMonomial, MonomialView};
    use crate::order::Lex;
    use crate::ring::{Ring, StaticFpCtx};
    use crate::term::{Term, TermView};
    use gbx_field::fp::{Fp, FpDyn, FpDynElem};

    // ---------- Static ring (Fp<7>) ----------
    type F7 = Fp<7>;
    type TStatic = Term<F7, FixedMonomial<2>>;

    fn ts(c: u32, e0: u32, e1: u32) -> TStatic {
        Term::new(F7::new(c), FixedMonomial::<2>::from_exponents([e0, e1]))
    }

    #[test]
    fn normalize_static_drops_merges_sorts() {
        let ring = Ring::builder()
            .field(StaticFpCtx::<7>::new())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap();

        let mut v = vec![ts(0, 5, 0), ts(3, 1, 0), ts(4, 0, 7), ts(5, 1, 0)];
        normalize_terms_in(&ring, &mut v).unwrap();

        // 3+5 = 1 mod 7
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].mono().exponents(), &[1, 0]);
        assert_eq!(*v[0].coeff(), F7::new(1));
        assert_eq!(v[1].mono().exponents(), &[0, 7]);
        assert_eq!(*v[1].coeff(), F7::new(4));
    }

    // ---------- Dynamic ring (FpDynElem) ----------
    type TDyn = Term<FpDynElem, DynamicMonomial>;

    fn td(c: u32, exps: &[u32], ring: &crate::ring::RingCtx<FpDyn, Lex>) -> TDyn {
        Term::new(ring.field.new(c), DynamicMonomial::from_slice(exps))
    }

    #[test]
    fn normalize_dynamic_field_merges_correctly() {
        let ring = Ring::builder()
            .field(FpDyn::prime(7).unwrap())
            .order(Lex)
            .nvars(3)
            .build()
            .unwrap();

        let mut v = vec![td(3, &[1, 0, 2], &ring), td(5, &[1, 0, 2], &ring), td(4, &[0, 7, 0], &ring)];

        normalize_terms_in(&ring, &mut v).unwrap();

        assert_eq!(v.len(), 2);
        assert_eq!(v[0].mono().exponents(), &[1, 0, 2]);
        assert_eq!(ring.field.value(*v[0].coeff()), 1);
    }
}
