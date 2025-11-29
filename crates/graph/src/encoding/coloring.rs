use crate::Graph;

/// Abstraction over "Polynomial"
/// specialized for the boolean-style encodings we need.
pub trait BoolPolyBuilder {
    type Poly;

    fn zero(&self) -> Self::Poly;

    fn one(&self) -> Self::Poly;

    /// The polynomial representing the variable `x_index`.
    ///
    /// Here `index` is a 0-based variable index: x_0, x_1, ..., x_{n-1}.
    fn var(&self, index: usize) -> Self::Poly;

    /// a + b
    fn add(&self, a: &Self::Poly, b: &Self::Poly) -> Self::Poly;

    /// a - b
    fn sub(&self, a: &Self::Poly, b: &Self::Poly) -> Self::Poly;

    /// a * b
    fn mul(&self, a: &Self::Poly, b: &Self::Poly) -> Self::Poly;

    /// a^exp
    ///
    /// For our encoding we mostly need exp = 2 (`x^2 - x`), but this
    /// keeps it generic.
    fn pow(&self, a: &Self::Poly, exp: u32) -> Self::Poly;
}

/// Helper that maps (vertex, color) pairs to a flat variable index
///
/// One boolean variable x_{v,c} for each vertex v and color c.
/// To store them in a polynomial ring with variables x_0, x_1, ..., x_{N-1}
/// we need a deterministic mapping
#[derive(Debug, Clone, Copy)]
pub struct VarIndex {
    pub n_vertices: usize,

    pub k_colors: usize,
}

impl VarIndex {
    pub fn num_vars(&self) -> usize {
        self.n_vertices * self.k_colors
    }

    pub fn idx(&self, v: u32, c: u32) -> usize {
        let v0 = (v - 1) as usize;
        let c0 = (c - 1) as usize;

        v0 * self.k_colors + c0
    }
}

pub struct ColoringEncoding<P> {
    pub polynomials: Vec<P>,

    pub var_index: VarIndex,

    pub num_vars: usize,
}

pub fn build_k_coloring_system<B>(graph: &Graph, k: usize, builder: &B) -> ColoringEncoding<B::Poly>
where
    B: BoolPolyBuilder,
{
    // Index mapping
    let vi = VarIndex { n_vertices: graph.n, k_colors: k };

    let mut polys = Vec::new();

    // Boolean constraints: X_{v,c}^2 - X_{v,c} = 0
    for v in 1..=graph.n as u32 {
        for c in 1..=k as u32 {
            let idx = vi.idx(v, c);
            let x = builder.var(idx);
            let x2 = builder.pow(&x, 2);
            let p = builder.sub(&x2, &x);

            polys.push(p);
        }
    }

    // Exactly-one-color constraints:
    //    (x_{v,1} + x_{v,2} + ... + x_{v,k}) - 1 = 0
    for v in 1..=graph.n as u32 {
        let mut sum = builder.zero();
        for c in 1..=k as u32 {
            let idx = vi.idx(v, c);
            let x = builder.var(idx);
            sum = builder.add(&sum, &x);
        }
        let one = builder.one();
        let p = builder.sub(&sum, &one);
        polys.push(p);
    }

    // Edge constraints:
    //    for each edge (u, v) and each color c:
    //        x_{u,c} * x_{v,c} = 0
    for (u, v) in graph.edges() {
        for c in 1..=k as u32 {
            let idx_u = vi.idx(u, c);
            let idx_v = vi.idx(v, c);

            let x_u = builder.var(idx_u);
            let x_v = builder.var(idx_v);

            let p = builder.mul(&x_u, &x_v);
            polys.push(p);
        }
    }

    ColoringEncoding { polynomials: polys, var_index: vi, num_vars: vi.num_vars() }
}
