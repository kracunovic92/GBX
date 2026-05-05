use gbx_poly::monomial::Monomial;

/// A non-evaluated product `m * source`.
///
/// In the F4 paper, this is an element of `T × R[x]`.
/// It is kept unevaluated so `Simplify` can rewrite it before multiplication.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnevaluatedProduct<M = Monomial> {
    pub multiplier: M,
    pub source: ProductSource,
}

impl<M> UnevaluatedProduct<M> {
    #[inline]
    #[must_use]
    pub fn new(source: ProductSource, multiplier: M) -> Self {
        Self { source, multiplier }
    }

    #[inline]
    #[must_use]
    pub fn from_basis(basis_index: usize, multiplier: M) -> Self {
        Self { source: ProductSource::Basis(basis_index), multiplier }
    }
}

/// Source row used by an unevaluated product.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProductSource {
    /// A polynomial from the current basis `G`.
    Basis(usize),

    /// A row from a previous reduced batch `F̃_j`.
    HistoryReducedRow { batch_index: usize, row_index: usize },
}

/// A materialized symbolic product together with origin metadata.
#[derive(Debug, Clone)]
pub struct SymbolicRow<P, M = Monomial> {
    pub product: UnevaluatedProduct<M>,
    pub polynomial: P,
}

impl<P, M> SymbolicRow<P, M> {
    #[inline]
    #[must_use]
    pub fn new(product: UnevaluatedProduct<M>, polynomial: P) -> Self {
        Self { product, polynomial }
    }
}

/// Result of symbolic preprocessing for one F4 batch.
#[derive(Debug, Clone)]
pub struct SymbolicPreprocessOutput<P, M = Monomial> {
    pub rows: Vec<SymbolicRow<P, M>>,
    pub symbolic_heads: Vec<M>,
}

impl<P, M> SymbolicPreprocessOutput<P, M> {
    #[inline]
    #[must_use]
    pub fn new(rows: Vec<SymbolicRow<P, M>>, symbolic_heads: Vec<M>) -> Self {
        Self { rows, symbolic_heads }
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn into_rows_parts(self) -> (Vec<P>, Vec<UnevaluatedProduct<M>>, Vec<M>) {
        let mut polynomials = Vec::with_capacity(self.rows.len());
        let mut products = Vec::with_capacity(self.rows.len());

        for row in self.rows {
            products.push(row.product);
            polynomials.push(row.polynomial);
        }

        (polynomials, products, self.symbolic_heads)
    }
}

/// Convenient concrete symbolic product type.
pub type PolyProduct = UnevaluatedProduct<Monomial>;
