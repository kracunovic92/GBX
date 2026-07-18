#![allow(dead_code)]
use gbx_field::fp::{Fp, FpElem};
use gbx_graph::Graph;
use gbx_grobner::{F4Options, GrobnerBasis, ProfileEvent};

use gbx_poly::order::Grevlex;
use gbx_poly::polynomial::Polynomial;
use gbx_poly::ring::RingCtx;
use gbx_poly::term::Term;

pub type GrevlexRing = RingCtx<Fp, Grevlex>;

pub type GbxTerm = Term<FpElem>;
pub type GbxPoly = Polynomial<FpElem>;
pub type GbxBasis = GrobnerBasis<GbxPoly>;

#[derive(Debug)]
pub struct ParseGraphOutput {
    pub graph: Graph,
    pub parse_seconds: f64,
    pub normalize_seconds: Option<f64>,
}

#[derive(Debug)]
pub struct BuildRingOutput {
    pub ring: GrevlexRing,
    pub vars: Vec<String>,
    pub build_seconds: f64,
}

#[derive(Debug)]
pub struct EncodeOutput {
    pub polynomials: Vec<GbxPoly>,
    pub num_vars: usize,
    pub encode_seconds: f64,
}

#[derive(Debug)]
pub struct GrobnerOutput {
    pub basis: GbxBasis,
    pub grobner_seconds: f64,
    pub profile_events: Vec<ProfileEvent>,
}

#[derive(Debug, Clone, Copy)]
pub struct GrobnerStageOptions {
    pub normalize_inputs: bool,
    pub normalize_remainders: bool,
    pub f4: F4Options,
}
