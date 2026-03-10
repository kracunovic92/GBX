use anyhow::{Context, Result};
use std::time::Instant;

use gbx_poly::order::Grevlex;
use gbx_poly::ring::Ring;

use super::types::{BuildRingOutput, GrevlexRing};

pub fn run(nvars: usize, p: u32) -> Result<BuildRingOutput> {
    let t0 = Instant::now();

    let field = gbx_field::fp::FpDyn::prime(p).context("invalid modulus p")?;
    let ring: GrevlexRing = Ring::builder()
        .field(field)
        .order(Grevlex)
        .nvars(nvars)
        .build()
        .context("failed to build ring")?;

    let vars = make_var_names_1_based(nvars);
    let build_ms = t0.elapsed().as_millis();

    println!("--- build_ring ---");
    println!("ring_build: {build_ms} ms");
    println!(
        "ring: nvars={}, order=Grevlex, field=FpDyn(p={p})",
        ring.nvars
    );

    Ok(BuildRingOutput { ring, vars, build_ms })
}

fn make_var_names_1_based(nvars: usize) -> Vec<String> {
    (1..=nvars).map(|i| format!("x_{i}")).collect()
}
