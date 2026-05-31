//! k-coloring encoding as a boolean polynomial system.

use crate::Graph;
use crate::encoding::error::EncodingError;

/// Abstraction over polynomial types used for boolean encodings.
///
/// This trait intentionally stays minimal: it describes only the
/// operations required to build boolean constraint systems such as
/// graph k-coloring.
///
/// The intended semantics are over a field of characteristic != 2,
/// but the trait itself does not enforce this.
pub trait BoolPolyBuilder {
    /// Concrete polynomial type (e.g. multivariate polynomial).
    type Poly;

    /// Zero polynomial.
    fn zero(&self) -> Self::Poly;

    /// One polynomial.
    fn one(&self) -> Self::Poly;

    /// Polynomial representing variable `x_index`.
    ///
    /// `index` is 0-based: `x_0, x_1, ..., x_{N-1}`.
    fn var(&self, index: usize) -> Self::Poly;

    /// Polynomial addition: `a + b`.
    fn add(&self, a: &Self::Poly, b: &Self::Poly) -> Self::Poly;

    /// Polynomial subtraction: `a - b`.
    fn sub(&self, a: &Self::Poly, b: &Self::Poly) -> Self::Poly;

    /// Polynomial multiplication: `a * b`.
    fn mul(&self, a: &Self::Poly, b: &Self::Poly) -> Self::Poly;

    /// Polynomial exponentiation: `a^exp`.
    fn pow(&self, a: &Self::Poly, exp: u32) -> Self::Poly;
}

/// Maps `(vertex, color)` pairs to flat polynomial variable indices.
///
/// We introduce one boolean variable `x_{v,c}` for each
/// vertex `v` and color `c`.
///
/// These are mapped to polynomial variables
/// `x_0, x_1, ..., x_{n*k - 1}` using:
///
/// ```text
/// index(v, c) = (v - 1) * k + (c - 1)
/// ```
///
/// where:
/// - vertices are 1-based
/// - colors are 1-based
#[derive(Debug, Clone, Copy)]
pub struct VarIndex {
    /// Number of vertices.
    pub n_vertices: usize,
    /// Number of colors.
    pub k_colors: usize,
}

impl VarIndex {
    /// Total number of polynomial variables.
    #[must_use]
    pub const fn num_vars(&self) -> usize {
        self.n_vertices * self.k_colors
    }

    /// Compute the variable index for `(v, c)` without bounds checks.
    ///
    /// # Safety
    /// Caller must ensure:
    /// - `1 <= v <= n_vertices`
    /// - `1 <= c <= k_colors`
    #[inline]
    #[must_use]
    pub const fn idx_unchecked(&self, v: usize, c: usize) -> usize {
        (v - 1) * self.k_colors + (c - 1)
    }
}

/// Result of building a k-coloring polynomial system.
#[derive(Debug)]
pub struct ColoringEncoding<P> {
    /// Constraint polynomials.
    pub polynomials: Vec<P>,
    /// Variable index mapping.
    pub var_index: VarIndex,
}

impl<P> ColoringEncoding<P> {
    /// Total number of polynomial variables.
    #[must_use]
    pub const fn num_vars(&self) -> usize {
        self.var_index.num_vars()
    }
}

/// Build the polynomial system encoding the k-coloring problem for a graph.
///
/// The system enforces:
///
/// 1) **Boolean constraints** for each `(v,c)`:
///    `x_{v,c}^2 - x_{v,c} = 0`
///
/// 2) **Exactly-one-color per vertex** for each `v`:
///    `(x_{v,1} + ... + x_{v,k}) - 1 = 0`
///
/// 3) **Edge constraints** for each edge `{u,v}` and each color `c`:
///    `x_{u,c} * x_{v,c} = 0`
///
/// If the system has a solution over the base field,
/// then the graph is k-colorable.
///
/// # Errors
/// Returns [`EncodingError::InvalidK`] if `k == 0`.
pub fn build_k_coloring_system<B>(graph: &Graph, k: usize, builder: &B) -> Result<ColoringEncoding<B::Poly>, EncodingError>
where
    B: BoolPolyBuilder,
{
    if k == 0 {
        return Err(EncodingError::InvalidK { k });
    }

    let vi = VarIndex { n_vertices: graph.n(), k_colors: k };

    // Constraints count:
    // - boolean: n * k
    // - exactly-one: n
    // - edge constraints: m * k
    let mut polynomials = Vec::with_capacity(graph.n() * k + graph.n() + graph.m() * k);

    // Boolean constraints: x^2 - x = 0
    for v in 1..=graph.n() {
        for c in 1..=k {
            let idx = vi.idx_unchecked(v, c);
            let x = builder.var(idx);
            let x2 = builder.pow(&x, 2);
            polynomials.push(builder.sub(&x2, &x));
        }
    }

    // Exactly-one-color per vertex
    for v in 1..=graph.n() {
        let mut sum = builder.zero();
        for c in 1..=k {
            let idx = vi.idx_unchecked(v, c);
            let x = builder.var(idx);
            sum = builder.add(&sum, &x);
        }
        polynomials.push(builder.sub(&sum, &builder.one()));
    }

    // Edge constraints: x_{u,c} * x_{v,c} = 0
    for (u, v) in graph.edges() {
        for c in 1..=k {
            let idx_u = vi.idx_unchecked(u, c);
            let idx_v = vi.idx_unchecked(v, c);
            let x_u = builder.var(idx_u);
            let x_v = builder.var(idx_v);
            polynomials.push(builder.mul(&x_u, &x_v));
        }
    }

    Ok(ColoringEncoding { polynomials, var_index: vi })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use crate::Graph;

    #[derive(Default)]
    struct StrBuilder;

    impl BoolPolyBuilder for StrBuilder {
        type Poly = String;

        fn zero(&self) -> Self::Poly {
            "0".into()
        }

        fn one(&self) -> Self::Poly {
            "1".into()
        }

        fn var(&self, index: usize) -> Self::Poly {
            format!("x{index}")
        }

        fn add(&self, a: &Self::Poly, b: &Self::Poly) -> Self::Poly {
            format!("({a}+{b})")
        }

        fn sub(&self, a: &Self::Poly, b: &Self::Poly) -> Self::Poly {
            format!("({a}-{b})")
        }

        fn mul(&self, a: &Self::Poly, b: &Self::Poly) -> Self::Poly {
            format!("({a}*{b})")
        }

        fn pow(&self, a: &Self::Poly, exp: u32) -> Self::Poly {
            format!("({a}^{exp})")
        }
    }

    #[test]
    fn k_must_be_positive() {
        let g = Graph::new(1);
        let b = StrBuilder;
        let err = build_k_coloring_system(&g, 0, &b).unwrap_err();
        assert_eq!(err, EncodingError::InvalidK { k: 0 });
    }

    #[test]
    fn variable_index_mapping_is_correct() {
        let vi = VarIndex { n_vertices: 3, k_colors: 4 };
        // v=1,c=1 -> 0
        assert_eq!(vi.idx_unchecked(1, 1), 0);
        // v=1,c=4 -> 3
        assert_eq!(vi.idx_unchecked(1, 4), 3);
        // v=2,c=1 -> 4
        assert_eq!(vi.idx_unchecked(2, 1), 4);
        // v=3,c=4 -> 11
        assert_eq!(vi.idx_unchecked(3, 4), 11);
        assert_eq!(vi.num_vars(), 12);
    }

    #[test]
    fn constraint_count_matches_formula() {
        // Path graph 1-2-3 has n=3, m=2.
        let mut g = Graph::new(3);
        g.add_edge(1, 2).unwrap();
        g.add_edge(2, 3).unwrap();

        let k = 2;
        let b = StrBuilder;

        let enc = build_k_coloring_system(&g, k, &b).unwrap();

        // n*k + n + m*k = 3*2 + 3 + 2*2 = 6 + 3 + 4 = 13
        assert_eq!(enc.polynomials.len(), 13);
        assert_eq!(enc.num_vars(), 6);
    }

    #[test]
    fn includes_expected_constraints_for_small_graph() {
        let mut g = Graph::new(2);
        g.add_edge(1, 2).unwrap();

        let b = StrBuilder;
        let enc = build_k_coloring_system(&g, 2, &b).unwrap();

        // Boolean constraints:
        assert!(enc.polynomials.iter().any(|p| p == "((x0^2)-x0)"));
        assert!(enc.polynomials.iter().any(|p| p == "((x1^2)-x1)"));
        assert!(enc.polynomials.iter().any(|p| p == "((x2^2)-x2)"));
        assert!(enc.polynomials.iter().any(|p| p == "((x3^2)-x3)"));

        // Exactly-one constraints:
        // v=1 => (x0 + x1) - 1
        assert!(
            enc.polynomials
                .iter()
                .any(|p| p == "((((0+x0)+x1)-1))" || (p.contains("x0") && p.contains("x1") && p.contains("-1")))
        );

        // Edge constraints for edge (1,2), k=2:
        assert!(enc.polynomials.iter().any(|p| p == "(x0*x2)"));
        assert!(enc.polynomials.iter().any(|p| p == "(x1*x3)"));
    }
}
