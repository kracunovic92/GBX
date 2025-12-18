use crate::Graph;

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
    ///
    /// For graph coloring encodings we mostly need `exp = 2`
    /// (to enforce boolean constraints `x^2 - x = 0`),
    /// but this is kept generic.
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
    /// Number of vertices needed for construction
    pub n_vertices: usize,
    /// Number of colors
    pub k_colors: usize,
}

impl VarIndex {
    /// Total number of polynomial variables.
    pub fn num_vars(&self) -> usize {
        self.n_vertices * self.k_colors
    }

    /// Compute the variable index for `(v, c)`.
    ///
    /// # Safety
    /// Caller must ensure:
    /// - `1 <= v <= n_vertices`
    /// - `1 <= c <= k_colors`
    #[inline]
    pub fn idx_unchecked(&self, v: u32, c: u32) -> usize {
        (v as usize - 1) * self.k_colors + (c as usize - 1)
    }
}

/// Result of building a k-coloring polynomial system.
#[derive(Debug)]
pub struct ColoringEncoding<P> {
    /// All constraint polynomials.
    pub polynomials: Vec<P>,

    /// Variable index mapping.
    pub var_index: VarIndex,
}

impl<P> ColoringEncoding<P> {
    /// Total number of polynomial variables.
    pub fn num_vars(&self) -> usize {
        self.var_index
            .num_vars()
    }
}

/// Build the polynomial system encoding the k-coloring problem for a graph.
///
/// The system enforces:
///
/// 1. **Boolean constraints**:
///    ```text
///    x_{v,c}^2 - x_{v,c} = 0
///    ```
///
/// 2. **Exactly-one-color per vertex**:
///    ```text
///    (x_{v,1} + ... + x_{v,k}) - 1 = 0
///    ```
///
/// 3. **Edge constraints**:
///    ```text
///    x_{u,c} * x_{v,c} = 0   for each edge (u,v) and color c
///    ```
///
/// If the system has a solution over the base field,
/// then the graph is k-colorable.
pub fn build_k_coloring_system<B>(graph: &Graph, k: usize, builder: &B) -> ColoringEncoding<B::Poly>
where
    B: BoolPolyBuilder,
{
    let vi = VarIndex { n_vertices: graph.n, k_colors: k };

    // We know exactly how many constraints we will generate:
    // - boolean: n * k
    // - exactly-one: n
    // - edge constraints: m * k
    let mut polynomials = Vec::with_capacity(graph.n * k + graph.n + graph.m * k);

    // Boolean constraints: x_{v,c}^2 - x_{v,c} = 0
    for v in 1..=graph.n as u32 {
        for c in 1..=k as u32 {
            let idx = vi.idx_unchecked(v, c);
            let x = builder.var(idx);
            let x2 = builder.pow(&x, 2);
            let p = builder.sub(&x2, &x);
            polynomials.push(p);
        }
    }

    // Exactly-one-color constraints per vertex
    for v in 1..=graph.n as u32 {
        let mut sum = builder.zero();
        for c in 1..=k as u32 {
            let idx = vi.idx_unchecked(v, c);
            let x = builder.var(idx);
            sum = builder.add(&sum, &x);
        }
        let p = builder.sub(&sum, &builder.one());
        polynomials.push(p);
    }

    // Edge constraints: x_{u,c} * x_{v,c} = 0
    for (u, v) in graph.edges() {
        for c in 1..=k as u32 {
            let idx_u = vi.idx_unchecked(u, c);
            let idx_v = vi.idx_unchecked(v, c);

            let x_u = builder.var(idx_u);
            let x_v = builder.var(idx_v);

            let p = builder.mul(&x_u, &x_v);
            polynomials.push(p);
        }
    }

    ColoringEncoding { polynomials, var_index: vi }
}
