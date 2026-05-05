use std::collections::BTreeSet;

use crate::algos::f4::error::Result;
use crate::algos::f4::simplify::SimplifyIndex;
use crate::algos::f4::state::BatchHistory;
use crate::algos::f4::symbolic::materialize::materialize_product;
use crate::algos::f4::symbolic::ordered::{OrderedMono, OrderedProduct};
use crate::algos::f4::symbolic::reducers::find_top_reducer_product;
use crate::algos::f4::symbolic::types::{SymbolicRow, UnevaluatedProduct};
use crate::algos::f4::symbolic::worklist::MonomialWorklist;

use crate::symbolic::SymbolicPreprocessOutput;
use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Mutable state used while constructing the symbolic row set `F_d`.
///
/// This object implements the closure loop from F4 symbolic preprocessing:
///
/// ```text
/// F_d := { mult(Simplify(product)) | product ∈ L_d }
/// Done := HT(F_d)
///
/// while T(F_d) != Done:
///     choose m ∈ T(F_d) \ Done
///     Done := Done ∪ {m}
///     if m is top-reducible by G:
///         add the corresponding reducer product to F_d
/// ```
///
/// The state owns the partially built rows and the worklist of discovered
/// monomials. It borrows the current basis, previous history, and simplify
/// index needed to materialize and simplify products.
pub struct SymbolicPreprocessState<'a, P, F, O>
where
    P: PolynomialView,
    F: FieldCtx,
{
    /// Ring context used for monomial multiplication and ordering.
    ctx: &'a RingCtx<F, O>,

    /// Current basis `G`.
    basis: &'a [P],

    /// Previous F4 batches, used to materialize historical reduced rows.
    history: &'a [BatchHistory<P>],

    /// Incrementally maintained rewrite index for `Simplify`.
    simplify_index: &'a SimplifyIndex,

    /// Active monomial order.
    order: &'a O,

    /// Products already inserted into `F_d`.
    seen_products: BTreeSet<OrderedProduct<'a, Monomial, O>>,

    /// Monomials already processed by the closure loop.
    done: BTreeSet<OrderedMono<'a, Monomial, O>>,

    /// Discovered monomials from `T(F_d)` that still need processing.
    pending_terms: MonomialWorklist<'a, O>,

    /// Materialized symbolic rows `F_d`.
    f_d: Vec<SymbolicRow<P, Monomial>>,

    /// Leading monomials `HT(F_d)`.
    symbolic_heads: Vec<Monomial>,
}

impl<'a, P, F, O> SymbolicPreprocessState<'a, P, F, O>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    pub(crate) fn new(ctx: &'a RingCtx<F, O>, basis: &'a [P], history: &'a [BatchHistory<P>], simplify_index: &'a SimplifyIndex) -> Self {
        Self {
            ctx,
            basis,
            history,
            simplify_index,
            order: &ctx.order,

            seen_products: BTreeSet::new(),
            done: BTreeSet::new(),
            pending_terms: MonomialWorklist::new(),

            f_d: Vec::new(),
            symbolic_heads: Vec::new(),
        }
    }
    pub(crate) fn add_initial_products(&mut self, l_d: &[UnevaluatedProduct<Monomial>]) -> Result<()> {
        for product in l_d {
            let simplified = self.simplify(product.clone())?;

            self.add_product(&simplified)?;
        }

        Ok(())
    }
    pub(crate) fn close_under_top_reduction(&mut self) -> Result<()> {
        while let Some(next) = self.pending_terms.pop_next() {
            let m = next.into_inner();

            if !self.mark_done(m.clone()) {
                continue;
            }

            let Some(product) = find_top_reducer_product(&m, self.basis)? else {
                continue;
            };

            let simplified = self.simplify(product)?;

            self.add_product(&simplified)?;
        }

        Ok(())
    }

    fn simplify(&self, product: UnevaluatedProduct<Monomial>) -> Result<UnevaluatedProduct<Monomial>> {
        self.simplify_index
            .simplify_product::<F, O>(self.ctx, product)
    }

    fn add_product(&mut self, product: &UnevaluatedProduct<Monomial>) -> Result<()> {
        if !self.insert_seen_product(product) {
            return Ok(());
        }

        let row = materialize_product(self.ctx, product, self.basis, self.history)?;

        if let Some(head) = row.leading_mono().cloned() {
            self.symbolic_heads.push(head.clone());

            // Implements `Done := HT(F_d)` for row heads.
            self.done.insert(OrderedMono::new(head, self.order));
        }

        self.pending_terms.extend_from_row(&row, self.order);

        self.f_d.push(SymbolicRow::new(product.clone(), row));

        Ok(())
    }

    fn insert_seen_product(&mut self, product: &UnevaluatedProduct<Monomial>) -> bool {
        let key = OrderedProduct::new(product.source, product.multiplier.clone(), self.order);

        self.seen_products.insert(key)
    }

    fn mark_done(&mut self, monomial: Monomial) -> bool {
        self.done.insert(OrderedMono::new(monomial, self.order))
    }

    pub(crate) fn finish(self) -> SymbolicPreprocessOutput<P, Monomial> {
        SymbolicPreprocessOutput::new(self.f_d, self.symbolic_heads)
    }
}
