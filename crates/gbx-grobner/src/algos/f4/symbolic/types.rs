//! Symbolic product and row types.

use gbx_poly::monomial::Monomial;

/// Unevaluated product of a monomial multiplier and a source row.
///
/// F4 keeps products unevaluated during symbolic preprocessing so that
/// `Simplify` can rewrite them before materialization.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnevaluatedProduct<M = Monomial> {
    /// Monomial multiplier
    pub multiplier: M,
    /// Product reference to location from which we took it.
    pub source: ProductSource,
}

impl<M> UnevaluatedProduct<M> {
    /// Creates an unevaluated product from a source row and multiplier.
    #[inline]
    #[must_use]
    pub const fn new(source: ProductSource, multiplier: M) -> Self {
        Self { multiplier, source }
    }

    /// Creates an unevaluated product whose source is a current basis element.
    #[inline]
    #[must_use]
    pub const fn from_basis(basis_index: usize, multiplier: M) -> Self {
        Self { multiplier, source: ProductSource::Basis(basis_index) }
    }

    /// Creates an unevaluated product whose source is a reduced history row.
    #[inline]
    #[must_use]
    pub const fn from_history_reduced_row(batch_index: usize, row_index: usize, multiplier: M) -> Self {
        Self { multiplier, source: ProductSource::HistoryReducedRow { batch_index, row_index } }
    }
}

/// Source row referenced by an unevaluated product.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProductSource {
    /// A polynomial from the current basis.
    Basis(usize),

    /// A reduced row from a previous F4 batch.
    HistoryReducedRow {
        /// Batch index
        batch_index: usize,
        /// Correct row
        row_index: usize,
    },
}

/// Materialized symbolic row with its source product.
#[derive(Debug, Clone)]
pub struct SymbolicRow<P, M = Monomial> {
    /// Unevaluated product that produced this row.
    pub product: UnevaluatedProduct<M>,

    /// Materialized polynomial row.
    pub polynomial: P,
}

impl<P, M> SymbolicRow<P, M> {
    /// Creates a symbolic row from an unevaluated product and its materialized row.
    #[inline]
    #[must_use]
    pub const fn new(product: UnevaluatedProduct<M>, polynomial: P) -> Self {
        Self { product, polynomial }
    }

    /// Splits the row into its product and polynomial.
    #[inline]
    #[must_use]
    pub fn into_parts(self) -> (UnevaluatedProduct<M>, P) {
        (self.product, self.polynomial)
    }
}

/// Output of symbolic preprocessing for one F4 batch.
#[derive(Debug, Clone)]
pub struct SymbolicPreprocessOutput<P, M = Monomial> {
    /// Materialized symbolic rows.
    pub rows: Vec<SymbolicRow<P, M>>,

    /// Leading monomials of the symbolic rows.
    pub symbolic_heads: Vec<M>,
}

impl<P, M> SymbolicPreprocessOutput<P, M> {
    /// Creates symbolic preprocessing output from rows and symbolic heads.
    #[inline]
    #[must_use]
    pub const fn new(rows: Vec<SymbolicRow<P, M>>, symbolic_heads: Vec<M>) -> Self {
        Self { rows, symbolic_heads }
    }

    /// Returns the number of symbolic rows.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Returns `true` if no symbolic rows were produced.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Splits the output into materialized rows, source products, and row heads.
    #[must_use]
    pub fn into_rows_parts(self) -> (Vec<P>, Vec<UnevaluatedProduct<M>>, Vec<M>) {
        let mut polynomials = Vec::with_capacity(self.rows.len());
        let mut products = Vec::with_capacity(self.rows.len());

        for row in self.rows {
            let (product, polynomial) = row.into_parts();

            products.push(product);
            polynomials.push(polynomial);
        }

        (polynomials, products, self.symbolic_heads)
    }
}

/// Concrete symbolic product over the library monomial type.
pub type PolyProduct = UnevaluatedProduct<Monomial>;

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    #[test]
    fn product_constructors_set_source_and_multiplier() {
        let multiplier = Monomial::from_slice(&[2, 0]);

        let product = UnevaluatedProduct::from_basis(3, multiplier.clone());

        assert_eq!(product.source, ProductSource::Basis(3));
        assert_eq!(product.multiplier, multiplier);
    }

    #[test]
    fn history_product_constructor_sets_source() {
        let multiplier = Monomial::from_slice(&[1, 1]);

        let product = UnevaluatedProduct::from_history_reduced_row(2, 5, multiplier.clone());

        assert_eq!(
            product.source,
            ProductSource::HistoryReducedRow { batch_index: 2, row_index: 5 }
        );
        assert_eq!(product.multiplier, multiplier);
    }

    #[test]
    fn symbolic_row_into_parts_preserves_values() {
        let product = UnevaluatedProduct::from_basis(0, Monomial::from_slice(&[0, 0]));
        let row = SymbolicRow::new(product.clone(), 42usize);

        let (actual_product, polynomial) = row.into_parts();

        assert_eq!(actual_product, product);
        assert_eq!(polynomial, 42);
    }

    #[test]
    fn preprocess_output_into_rows_parts_preserves_order() {
        let p0 = UnevaluatedProduct::from_basis(0, Monomial::from_slice(&[0, 0]));
        let p1 = UnevaluatedProduct::from_basis(1, Monomial::from_slice(&[1, 0]));

        let output = SymbolicPreprocessOutput::new(
            vec![SymbolicRow::new(p0.clone(), "row0"), SymbolicRow::new(p1.clone(), "row1")],
            vec![Monomial::from_slice(&[2, 0])],
        );

        let (rows, products, heads) = output.into_rows_parts();

        assert_eq!(rows, vec!["row0", "row1"]);
        assert_eq!(products, vec![p0, p1]);
        assert_eq!(heads, vec![Monomial::from_slice(&[2, 0])]);
    }
}
