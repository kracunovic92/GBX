use anyhow::Result;
use std::time::Instant;

use super::types::{GbxPoly, GrevlexRing, GrobnerOutput, GrobnerStageOptions};
use crate::pipeline::trace::make_f4_tracer;
use gbx_grobner::{f4_traced, F4Options};

pub fn run(ring: &GrevlexRing, polys: &[GbxPoly], opts: GrobnerStageOptions) -> Result<GrobnerOutput> {
    let tracer = make_f4_tracer();

    let f4_opts = F4Options::default();

    let t0 = Instant::now();
    let gb = f4_traced(ring, polys.iter().cloned(), f4_opts, Option::from(tracer))?;

    let grobner_ms = t0.elapsed().as_millis();

    println!("--- grobner ---");
    println!("f4: {grobner_ms} ms");
    println!("gb_size = {}", gb.len());
    println!("--- pair update breakdown ---");
    Ok(GrobnerOutput { basis: gb, grobner_ms })
}
