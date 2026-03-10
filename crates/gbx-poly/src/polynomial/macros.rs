/// Build a `Vec<Term<..>>` from either `(coeff, [exps..])` or `(coeff, monomial)` tuples.
///
/// This macro is storage-agnostic: it only builds terms; you can then pass the vector to
/// [`poly_terms!`] or any constructor that accepts a term list.
///
/// # Examples
/// ```
/// use gbx_poly::order::Lex;
/// use gbx_poly::polynomial::{PolyFixed, PolynomialView};
/// use gbx_poly::ring::{Ring, StaticFpCtx};
/// use gbx_field::fp::Fp;
/// use gbx_poly::{terms, poly_terms};
///
/// type F7 = Fp<7>;
/// type P2 = PolyFixed<F7, 2>;
///
/// let ring = Ring::builder()
///     .field(StaticFpCtx::<7>::default())
///     .order(Lex)
///     .nvars(2)
///     .build()
///     .unwrap();
///
/// let t = terms![(3, [1,0]), (5, [1,0]), (1, [0,0])];
/// let p: P2 = poly_terms![&ring; t].unwrap();
/// assert_eq!(p.len(), 2);
/// ```
#[macro_export]
macro_rules! terms {
    ($( ($c:expr, [$($e:expr),* $(,)?]) ),* $(,)?) => {{
        // If term! internally casts, you may want to keep this allow here as well.
        #[allow(clippy::unnecessary_cast)]
        {
            vec![ $( $crate::term!($c, [$($e),*]) ),* ]
        }
    }};
    ($( ($c:expr, $m:expr) ),* $(,)?) => {{
        #[allow(clippy::unnecessary_cast)]
        {
            vec![ $( $crate::term!($c, $m) ),* ]
        }
    }};
}

/// Build a polynomial in the given ring context.
///
/// The target polynomial type is inferred from the left-hand side:
/// ```ignore
/// let p: PolyDyn<FpDynElem> = poly![&ring; ... ]?;
/// ```
///
/// Internally this constructs terms and calls
/// [`PolynomialMut::from_terms_in`](crate::polynomial::traits::PolynomialMut::from_terms_in),
/// which enforces canonical normalization (merge duplicates, drop zeros, sort by order).
///
/// # Examples
/// ## Static field + fixed arity
/// ```
/// use gbx_poly::order::Lex;
/// use gbx_poly::polynomial::{PolyFixed, PolynomialView};
/// use gbx_poly::ring::{Ring, StaticFpCtx};
/// use gbx_field::fp::Fp;
/// use gbx_poly::poly;
///
/// type F7 = Fp<7>;
/// type P2 = PolyFixed<F7, 2>;
///
/// let ring = Ring::builder()
///     .field(StaticFpCtx::<7>::default())
///     .order(Lex)
///     .nvars(2)
///     .build()
///     .unwrap();
///
/// let p: P2 = poly![&ring; (3, [1,0]), (5, [1,0]), (0, [9,9]), (1, [0,0])].unwrap();
/// assert_eq!(p.len(), 2);
/// ```
///
/// ## Dynamic field + dynamic arity
/// ```
/// use gbx_poly::order::Lex;
/// use gbx_poly::polynomial::{PolyDyn, PolynomialView};
/// use gbx_poly::ring::Ring;
/// use gbx_field::fp::{FpDyn, FpDynElem};
/// use gbx_poly::poly;
///
/// type P = PolyDyn<FpDynElem>;
///
/// let ring = Ring::builder()
///     .field(FpDyn::prime(7).unwrap())
///     .order(Lex)
///     .nvars(3)
///     .build()
///     .unwrap();
///
/// let p: P = poly![&ring; (3, [1,0,2]), (5, [1,0,2]), (4, [0,7,0])].unwrap();
/// assert_eq!(p.len(), 2);
/// ```
#[macro_export]
macro_rules! poly {
    // monomial from exponent array
    ($ctx:expr; $( ($c:expr, [$($e:expr),* $(,)?]) ),* $(,)?) => {{
        #[allow(clippy::unnecessary_cast)]
        {
            let __ctx = $ctx;
            let __terms = vec![
                $(
                    $crate::term::Term::new(
                        $crate::ring::FieldCtx::new(&__ctx.field, $c as u32),
                        ::core::convert::From::from([$( $e as u32 ),*]),
                    )
                ),*
            ];
            <_ as $crate::polynomial::traits::PolynomialMut>::from_terms_in(__ctx, __terms)
        }
    }};

    // monomial already built
    ($ctx:expr; $( ($c:expr, $m:expr) ),* $(,)?) => {{
        #[allow(clippy::unnecessary_cast)]
        {
            let __ctx = $ctx;
            let __terms = vec![
                $(
                    $crate::term::Term::new(
                        $crate::ring::FieldCtx::new(&__ctx.field, $c as u32),
                        $m
                    )
                ),*
            ];
            <_ as $crate::polynomial::traits::PolynomialMut>::from_terms_in(__ctx, __terms)
        }
    }};
}

/// Build a polynomial from an existing `Vec<Term<..>>` inside a ring context.
///
/// This is a thin wrapper over
/// [`PolynomialMut::from_terms_in`](crate::polynomial::traits::PolynomialMut::from_terms_in).
///
/// # Examples
/// ```
/// use gbx_poly::order::Lex;
/// use gbx_poly::polynomial::{PolyFixed, PolynomialView};
/// use gbx_poly::ring::{Ring, StaticFpCtx};
/// use gbx_field::fp::Fp;
/// use gbx_poly::{terms, poly_terms};
///
/// type F7 = Fp<7>;
/// type P2 = PolyFixed<F7, 2>;
///
/// let ring = Ring::builder()
///     .field(StaticFpCtx::<7>::default())
///     .order(Lex)
///     .nvars(2)
///     .build()
///     .unwrap();
///
/// let t = terms![(3, [1,0]), (5, [1,0]), (1, [0,0])];
/// let p: P2 = poly_terms![&ring; t].unwrap();
/// assert_eq!(p.len(), 2);
/// ```
#[macro_export]
macro_rules! poly_terms {
    ($ctx:expr; $terms:expr) => {{ <_ as $crate::polynomial::traits::PolynomialMut>::from_terms_in($ctx, $terms) }};
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use crate::monomial::MonomialView;
    use crate::order::Lex;
    use crate::polynomial::traits::PolynomialView;
    use crate::polynomial::{PolyDyn, PolyFixed};
    use crate::ring::{Ring, StaticFpCtx};
    use crate::term::TermView;
    use gbx_field::fp::{Fp, FpDyn, FpDynElem};

    #[test]
    fn poly_macro_builds_and_normalizes_static_fixed() {
        type F7 = Fp<7>;
        type P2 = PolyFixed<F7, 2>;

        let ring = Ring::builder()
            .field(StaticFpCtx::<7>::default())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap();

        let p: P2 = poly![&ring; (0, [5,0]), (3, [1,0]), (5, [1,0])].unwrap();

        assert_eq!(p.ring_id(), ring.id());
        assert_eq!(p.len(), 1);
        assert_eq!(p.leading_term().unwrap().mono().exponents(), &[1, 0]);
    }

    #[test]
    fn poly_macro_builds_and_normalizes_dynamic_field_fixed_monomial() {
        type P2 = PolyFixed<FpDynElem, 2>;

        let ring = Ring::builder()
            .field(FpDyn::prime(7).unwrap())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap();

        let p: P2 = poly![&ring; (3, [1,0]), (5, [1,0]), (0, [9,9])].unwrap();

        assert_eq!(p.ring_id(), ring.id());
        assert_eq!(p.len(), 1);
        assert_eq!(p.leading_term().unwrap().mono().exponents(), &[1, 0]);

        let c = *p.leading_term().unwrap().coeff();
        assert_eq!(ring.field.value(c), 1);
    }

    #[test]
    fn poly_macro_builds_dynamic_field_dynamic_monomial() {
        type PD = PolyDyn<FpDynElem>;

        let ring = Ring::builder()
            .field(FpDyn::prime(7).unwrap())
            .order(Lex)
            .nvars(3)
            .build()
            .unwrap();

        let p: PD = poly![&ring; (3, [1,0,2]), (5, [1,0,2]), (4, [0,7,0])].unwrap();

        assert_eq!(p.ring_id(), ring.id());
        assert_eq!(p.len(), 2);
        assert_eq!(p.terms()[0].mono().exponents(), &[1, 0, 2]);
        assert_eq!(ring.field.value(*p.terms()[0].coeff()), 1);
    }

    #[test]
    fn poly_macro_arity_mismatch_is_error_dynamic_monomial() {
        type PD = PolyDyn<FpDynElem>;

        let ring = Ring::builder()
            .field(FpDyn::prime(7).unwrap())
            .order(Lex)
            .nvars(3)
            .build()
            .unwrap();

        let err = (|| -> crate::polynomial::Result<PD> { poly![&ring; (1, [1,0])] })().unwrap_err();
        let _ = err;
    }
}
