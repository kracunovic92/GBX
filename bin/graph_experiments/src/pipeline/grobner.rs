use anyhow::Result;
use std::time::Instant;

use super::types::{GbxPoly, GrevlexRing, GrobnerOutput, GrobnerStageOptions};
use gbx_grobner::{clear_profile_events, f4, take_profile_events};

pub fn run(ring: &GrevlexRing, polys: &[GbxPoly], opts: GrobnerStageOptions) -> Result<GrobnerOutput> {
    let f4_opts = opts.f4;

    clear_profile_events();
    let t0 = Instant::now();
    let gb = f4(ring, polys.iter().cloned(), f4_opts)?;
    let grobner_seconds = t0.elapsed().as_secs_f64();
    let profile_events = take_profile_events();

    println!("--- grobner ---");
    println!("f4_seconds={grobner_seconds:.6}");
    println!("gb_size = {}", gb.len());
    Ok(GrobnerOutput { basis: gb, grobner_seconds, profile_events })
}
