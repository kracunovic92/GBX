use anyhow::Result;
use std::time::Instant;

use gbx_graph::{Graph, build_k_coloring_system};

use crate::poly_builder::GbxPolyBuilder;

use super::types::{EncodeOutput, GrevlexRing};

pub fn run(graph: &Graph, k: usize, ring: &GrevlexRing) -> Result<EncodeOutput> {
    let builder = GbxPolyBuilder::new(ring);

    let t0 = Instant::now();
    let enc = build_k_coloring_system(graph, k, &builder)?;
    let encode_seconds = t0.elapsed().as_secs_f64();

    let num_vars = enc.num_vars();
    let polynomials = enc.polynomials;

    println!("--- encode ---");
    println!("encode_seconds={encode_seconds:.6}");
    println!(
        "system: num_vars={}, input_polys={}",
        num_vars,
        polynomials.len()
    );

    Ok(EncodeOutput { polynomials, num_vars, encode_seconds })
}
