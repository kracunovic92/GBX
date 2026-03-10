use gbx_field::fp::{FpDyn, FpDynElem};
use gbx_graph::BoolPolyBuilder;
use gbx_poly::monomial::{DynamicMonomial, Monomial};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolyDyn, PolynomialView};
use gbx_poly::ring::RingCtx;
use gbx_poly::term::{Term, TermView};

/// Bool polynomial builder backed by gbx_poly sparse polynomials (dynamic monomials).
///
/// Variables:
/// - `builder.var(i)` corresponds to polynomial variable x_i (0-based index)
/// - Our pretty printer later uses names x_1..x_n (1-based names), so the printed variable
///   name will be `x_{i+1}`.
pub struct GbxPolyBuilder<'a, O>
where
    O: MonomialOrder,
{
    ring: &'a RingCtx<FpDyn, O>,
    nvars: usize,
}

impl<'a, O> GbxPolyBuilder<'a, O>
where
    O: MonomialOrder,
{
    pub fn new(ring: &'a RingCtx<FpDyn, O>) -> Self {
        Self { ring, nvars: ring.nvars }
    }

    #[inline]
    fn coeff(&self, c: u32) -> FpDynElem {
        self.ring.field.new(c)
    }

    #[inline]
    fn mono_one(&self) -> DynamicMonomial {
        DynamicMonomial::one(self.nvars)
    }

    #[inline]
    fn mono_var(&self, index: usize) -> DynamicMonomial {
        // x_index => exponent vector with 1 at `index`
        let mut exps = vec![0u32; self.nvars];
        exps[index] = 1;
        DynamicMonomial::from_slice(&exps)
    }

    #[inline]
    fn poly_from_terms(&self, terms: Vec<Term<FpDynElem, DynamicMonomial>>) -> PolyDyn<FpDynElem> {
        PolyDyn::from_terms_in(self.ring, terms).expect("PolyDyn::from_terms_in failed")
    }

    #[inline]
    fn poly_neg(&self, p: &PolyDyn<FpDynElem>) -> PolyDyn<FpDynElem> {
        let zero = self.coeff(0);

        let mut out = Vec::with_capacity(p.len());
        for t in p.terms().iter() {
            let c = *t.coeff();
            let neg_c = self.ring.field.sub(zero, c);
            out.push(Term::new(neg_c, t.mono().clone()));
        }
        self.poly_from_terms(out)
    }

    #[inline]
    fn poly_add(&self, a: &PolyDyn<FpDynElem>, b: &PolyDyn<FpDynElem>) -> PolyDyn<FpDynElem> {
        let mut terms = Vec::with_capacity(a.len() + b.len());
        terms.extend(a.terms().iter().cloned());
        terms.extend(b.terms().iter().cloned());
        self.poly_from_terms(terms)
    }

    #[inline]
    fn poly_sub(&self, a: &PolyDyn<FpDynElem>, b: &PolyDyn<FpDynElem>) -> PolyDyn<FpDynElem> {
        let nb = self.poly_neg(b);
        self.poly_add(a, &nb)
    }

    #[inline]
    fn poly_mul(&self, a: &PolyDyn<FpDynElem>, b: &PolyDyn<FpDynElem>) -> PolyDyn<FpDynElem> {
        if a.is_zero() || b.is_zero() {
            return self.zero();
        }

        let mut out = Vec::with_capacity(a.len().saturating_mul(b.len()));

        for ta in a.terms().iter() {
            let ca = *ta.coeff();
            let ma = ta.mono();

            for tb in b.terms().iter() {
                let cb = *tb.coeff();
                let mb = tb.mono();

                let c = self.ring.field.mul(ca, cb);

                // Monomial multiplication with overflow checks:
                // DynamicMonomial implements checked_mul in your monomial layer.
                let m = ma
                    .checked_mul(mb)
                    .expect("monomial multiply overflow in encoding");

                out.push(Term::new(c, m));
            }
        }

        self.poly_from_terms(out)
    }

    #[inline]
    fn poly_pow(&self, a: &PolyDyn<FpDynElem>, exp: u32) -> PolyDyn<FpDynElem> {
        match exp {
            0 => self.one(),
            1 => a.clone(),
            _ => {
                // exponentiation by squaring
                let mut e = exp;
                let mut base = a.clone();
                let mut acc = self.one();

                while e > 0 {
                    if (e & 1) == 1 {
                        acc = self.poly_mul(&acc, &base);
                    }
                    e >>= 1;
                    if e > 0 {
                        base = self.poly_mul(&base, &base);
                    }
                }
                acc
            }
        }
    }
}

impl<'a, O> BoolPolyBuilder for GbxPolyBuilder<'a, O>
where
    O: MonomialOrder,
{
    type Poly = PolyDyn<FpDynElem>;

    fn zero(&self) -> Self::Poly {
        // IMPORTANT: empty term list still produces a polynomial tagged with the ring id.
        self.poly_from_terms(Vec::new())
    }

    fn one(&self) -> Self::Poly {
        let t = Term::new(self.coeff(1), self.mono_one());
        self.poly_from_terms(vec![t])
    }

    fn var(&self, index: usize) -> Self::Poly {
        assert!(
            index < self.nvars,
            "var index {index} out of range (nvars={})",
            self.nvars
        );
        let t = Term::new(self.coeff(1), self.mono_var(index));
        self.poly_from_terms(vec![t])
    }

    fn add(&self, a: &Self::Poly, b: &Self::Poly) -> Self::Poly {
        self.poly_add(a, b)
    }

    fn sub(&self, a: &Self::Poly, b: &Self::Poly) -> Self::Poly {
        self.poly_sub(a, b)
    }

    fn mul(&self, a: &Self::Poly, b: &Self::Poly) -> Self::Poly {
        self.poly_mul(a, b)
    }

    fn pow(&self, a: &Self::Poly, exp: u32) -> Self::Poly {
        self.poly_pow(a, exp)
    }
}
