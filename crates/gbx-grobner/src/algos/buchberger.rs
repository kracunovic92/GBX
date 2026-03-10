//! Buchberger Gröbner basis algorithm (baseline, context-driven).
//!
//! This is a correctness-first baseline:
//! - uses S-polynomials (`s_polynomial_in`)
//! - uses S-polynomials (`s_polynomial_in`)
//! - reduces each S-polynomial w.r.t. current basis (`normal_form`)
//!
//! The implementation is structured so you can later add:
//! - criteria pruning
//! - different pair selection strategies
//! - interreduction / reduced bases

extern crate alloc;

use super::spoly::s_polynomial_in;
use crate::algos::{minimize_in_place, reduce_in_place};
use crate::{baseline_update, BuchbergerError, GrobnerBasis, PairCriterion, PairQueue, PairUpdate, StackPairs};
use alloc::vec::Vec;
use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView, MonomialViewExtU32};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialError, PolynomialOps, PolynomialReduce};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

/// Options controlling normalization steps.
#[derive(Debug, Clone, Copy)]
pub struct BuchbergerOptions {
    /// Normalize each input polynomial before starting.
    pub normalize_inputs: bool,
    /// Normalize each nonzero remainder before inserting into the basis.
    pub normalize_remainders: bool,
    /// Post-processing
    pub post: BasisPost,
}

/// Post-processing for the resulting Gröbner basis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasisPost {
    /// Return raw Buchberger output (a GB, but not minimal/reduced).
    None,
    /// Make basis minimal (remove LT divisibility redundancies, make monic).
    Minimal,
    /// Make basis reduced (NF each g_i by others, drop zeros, then minimal+monic).
    Reduced,
}

impl Default for BuchbergerOptions {
    #[inline]
    fn default() -> Self {
        Self { normalize_inputs: true, normalize_remainders: true, post: BasisPost::Reduced }
    }
}

/// Default Buchberger configuration:
/// - LIFO pair queue ([`StackPairs`])
/// - no criteria pruning ([`NoCriteria`])
pub fn buchberger<P, F, O>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: BuchbergerOptions) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    buchberger_with(ctx, fs, opts, StackPairs::new(), baseline_update())
}

/// Configurable Buchberger:
/// - user pair queue (`Q`)
/// - user criteria (`C`)
pub fn buchberger_with<P, F, O, Q, U>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: BuchbergerOptions, mut pairs: Q, mut update: U) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    Q: PairQueue,
    U: PairUpdate<P>,
{
    let mut gb = init_basis(ctx, fs, opts)?;

    if gb.is_empty() {
        return Ok(gb);
    }

    for k in 0..gb.len() {
        update.on_new_poly(&gb, &mut pairs, k);
    }

    while let Some((_key, i, j)) = pairs.pop() {
        let fi = gb.get(i).ok_or(BuchbergerError::InvariantViolation)?;
        let fj = gb.get(j).ok_or(BuchbergerError::InvariantViolation)?;

        let s = s_polynomial_in(ctx, fi, fj)?;
        let mut r = s
            .normal_form(ctx, gb.as_slice().iter())
            .map_err(BuchbergerError::from)?;

        if opts.normalize_remainders {
            r.normalize_in_place(ctx)?;
        }

        if r.is_zero() {
            continue;
        }
        if is_unit_poly(&r) {
            let mut one = r;
            one.normalize_in_place(ctx)?;
            eprintln!("[buchberger] unit remainder found, terminating early");
            return Ok(GrobnerBasis::new(ctx.id(), vec![one]));
        }
        let new_idx = gb.len();
        gb.push(r);
        update.on_new_poly(&gb, &mut pairs, new_idx);
    }

    match opts.post {
        BasisPost::None => {}
        BasisPost::Minimal => minimize_in_place(ctx, &mut gb)?,
        BasisPost::Reduced => reduce_in_place(ctx, &mut gb)?,
    }

    Ok(gb)
}

fn is_unit_poly<P>(p: &P) -> bool
where
    P: PolynomialOps,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    if p.len() != 1 {
        return false;
    }

    match p.leading_term() {
        Some(t) => t.mono().is_one(),
        None => false,
    }
}

fn init_basis<P, F, O>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: BuchbergerOptions) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    let mut polys: Vec<P> = Vec::new();

    for mut f in fs.into_iter() {
        // Surface ring tag mistakes early.
        ctx.assert_same_ring_id(f.ring_id())
            .map_err(PolynomialError::from)?;

        if opts.normalize_inputs {
            f.normalize_in_place(ctx)?;
        }
        if !f.is_zero() {
            polys.push(f);
        }
    }

    Ok(GrobnerBasis::new(ctx.id(), polys))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use gbx_field::fp::{Fp, FpDyn, FpDynElem};
    use gbx_poly::monomial::{DynamicMonomial, FixedMonomial};
    use gbx_poly::order::Lex;
    use gbx_poly::polynomial::Polynomial;
    use gbx_poly::ring::{Ring, StaticFpCtx};
    use gbx_poly::term::Term;
    use gbx_storage::polynomial::VecTerms;

    // ---------------- static (Fp<7>, FixedMonomial<2>) ----------------
    type F7 = Fp<7>;
    type T2 = Term<F7, FixedMonomial<2>>;
    type P2 = Polynomial<T2, VecTerms<T2>>;

    fn ring_static() -> RingCtx<StaticFpCtx<7>, Lex> {
        Ring::builder()
            .field(StaticFpCtx::<7>::new())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap()
    }

    fn p2(r: &RingCtx<StaticFpCtx<7>, Lex>, terms: &[(u32, u32, u32)]) -> P2 {
        let ts = terms
            .iter()
            .map(|&(c, a, b)| Term::new(F7::new(c), FixedMonomial::<2>::from_exponents([a, b])))
            .collect();
        P2::from_terms_in(r, ts).unwrap()
    }

    #[test]
    fn buchberger_keeps_ctx_tag_and_trivial_basis() {
        let r = ring_static();
        let opts = BuchbergerOptions::default();

        let f0 = p2(&r, &[(1, 1, 0)]); // x
        let f1 = p2(&r, &[(1, 0, 1)]); // y

        let gb = buchberger(&r, vec![f0, f1], opts).unwrap();
        assert_eq!(gb.ring_id(), r.id());
        assert_eq!(gb.len(), 2);
    }

    // ---------------- dynamic (FpDyn, DynamicMonomial) ----------------
    type TD = Term<FpDynElem, DynamicMonomial>;
    type PD = Polynomial<TD, VecTerms<TD>>;

    fn ring_dyn() -> RingCtx<FpDyn, Lex> {
        Ring::builder()
            .field(FpDyn::prime(7).unwrap())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap()
    }

    fn pd(r: &RingCtx<FpDyn, Lex>, terms: &[(u32, &[u32])]) -> PD {
        let ts = terms
            .iter()
            .map(|&(c, e)| Term::new(r.field.new(c), DynamicMonomial::from_slice(e)))
            .collect();
        PD::from_terms_in(r, ts).unwrap()
    }

    #[test]
    fn buchberger_dynamic_trivial_basis() {
        let r = ring_dyn();
        let opts = BuchbergerOptions::default();

        let f0 = pd(&r, &[(1, &[1, 0])]); // x
        let f1 = pd(&r, &[(1, &[0, 1])]); // y

        let gb = buchberger(&r, vec![f0, f1], opts).unwrap();
        assert_eq!(gb.ring_id(), r.id());
        assert_eq!(gb.len(), 2);
    }

    #[test]
    fn buchberger_empty_input_returns_empty_basis() {
        let r = ring_static();
        let gb: GrobnerBasis<P2> = buchberger(&r, Vec::<P2>::new(), BuchbergerOptions::default()).unwrap();
        assert!(gb.is_empty());
    }

    #[test]
    fn buchberger_post_none_keeps_generators() {
        let r = ring_static();
        let opts = BuchbergerOptions { post: BasisPost::None, ..Default::default() };

        let f0 = p2(&r, &[(1, 2, 0), (1, 0, 2), (6, 0, 0)]); // x^2 + y^2 - 1 in F7
        let f1 = p2(&r, &[(1, 3, 0), (6, 0, 1)]); // x^3 - y in F7

        let gb = buchberger(&r, vec![f0, f1], opts).unwrap();
        assert!(gb.len() >= 2);
    }

    #[test]
    fn buchberger_post_reduced_runs() {
        let r = ring_static();
        let opts = BuchbergerOptions { post: BasisPost::Reduced, ..Default::default() };

        let f0 = p2(&r, &[(1, 1, 0)]); // x
        let f1 = p2(&r, &[(1, 0, 1)]); // y

        let gb = buchberger(&r, vec![f0, f1], opts).unwrap();
        assert_eq!(gb.len(), 2);
    }
}
