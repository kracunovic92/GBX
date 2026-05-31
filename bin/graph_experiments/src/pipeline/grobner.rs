use anyhow::Result;
use std::time::Instant;

use super::types::{GbxPoly, GrevlexRing, GrobnerOutput, GrobnerStageOptions};
use gbx_grobner::{F4Options, f4};

pub fn run(ring: &GrevlexRing, polys: &[GbxPoly], _opts: GrobnerStageOptions) -> Result<GrobnerOutput> {
    let f4_opts = F4Options::default();

    let t0 = Instant::now();
    let gb = f4(ring, polys.iter().cloned(), f4_opts)?;

    let grobner_ms = t0.elapsed().as_millis();

    println!("--- grobner ---");
    println!("f4: {grobner_ms} ms");
    println!("gb_size = {}", gb.len());
    println!("--- pair update breakdown ---");
    Ok(GrobnerOutput { basis: gb, grobner_ms })
}
