use gbx_field::fp::{FpDyn, FpDynElem};
use gbx_graph::Graph;
use gbx_grobner::GrobnerBasis;
use gbx_poly::monomial::DynamicMonomial;
use gbx_poly::order::Grevlex;
use gbx_poly::polynomial::PolyDyn;
use gbx_poly::ring::RingCtx;
use gbx_poly::term::Term;
use gbx_storage::polynomial::VecTerms;

pub type GrevlexRing = RingCtx<FpDyn, Grevlex>;

pub type GbxTerm = Term<FpDynElem, DynamicMonomial>;
pub type GbxPoly = PolyDyn<FpDynElem, VecTerms<GbxTerm>>;
pub type GbxBasis = GrobnerBasis<GbxPoly>;

#[derive(Debug)]
pub struct ParseGraphOutput {
    pub graph: Graph,
    pub parse_ms: u128,
    pub normalize_ms: Option<u128>,
}

#[derive(Debug)]
pub struct BuildRingOutput {
    pub ring: GrevlexRing,
    pub vars: Vec<String>,
    pub build_ms: u128,
}

#[derive(Debug)]
pub struct EncodeOutput {
    pub polynomials: Vec<GbxPoly>,
    pub num_vars: usize,
    pub encode_ms: u128,
}

#[derive(Debug)]
pub struct GrobnerOutput {
    pub basis: GbxBasis,
    pub grobner_ms: u128,
}

#[derive(Debug, Clone, Copy)]
pub struct GrobnerStageOptions {
    pub normalize_inputs: bool,
    pub normalize_remainders: bool,
}
