use anyhow::Result;
use std::time::Instant;

use super::types::{GbxPoly, GrevlexRing, GrobnerOutput, GrobnerStageOptions};
use crate::pipeline::trace::make_f4_tracer;
use gbx_grobner::{f4_traced, F4Options, GmProductLcmDegreeUpdater, HeapPairs, LcmDegreeKey, ProductCriterion, TracingPairCriterion, TracingPairKey, TracingQueue};

pub fn run(ring: &GrevlexRing, polys: &[GbxPoly], opts: GrobnerStageOptions) -> Result<GrobnerOutput> {
    let tracer = make_f4_tracer();

    let queue = TracingQueue::wrap(HeapPairs::new(), tracer.clone());

    let criterion = TracingPairCriterion::wrap(ProductCriterion, tracer.clone());
    let key = TracingPairKey::wrap(LcmDegreeKey, tracer.clone());
    //let update = GmPairUpdater::new(criterion, key);

    let mut update = GmProductLcmDegreeUpdater::new();
    let opts = F4Options::default();

    let t0 = Instant::now();
    let gb = f4_traced(
        ring,
        polys.iter().cloned(),
        opts,
        queue,
        &mut update,
        tracer,
    )?;

    let grobner_ms = t0.elapsed().as_millis();

    println!("--- grobner ---");
    println!("f4: {grobner_ms} ms");
    println!("gb_size = {}", gb.len());
    println!("--- pair update breakdown ---");

    let counts = update.counters();
    let times = update.times();

    println!("calls={}", counts.update_calls);
    println!("candidates_considered={}", counts.candidates_considered);
    println!("rejected_missing_lm={}", counts.rejected_missing_lm);
    println!("rejected_by_product={}", counts.rejected_by_product);
    println!("rejected_lcm_failure={}", counts.rejected_lcm_failure);
    println!("rejected_key_failure={}", counts.rejected_key_failure);
    println!("dedup_collisions={}", counts.dedup_collisions);
    println!("survivors_after_dedup={}", counts.survivors_after_dedup);
    println!("survivors_after_prune={}", counts.survivors_after_prune);
    println!("pairs_pushed={}", counts.pairs_pushed);

    println!(
        "scan={}ms dedup={}ms prune={}ms push={}ms total={}ms",
        times.scan.as_millis(),
        times.dedup.as_millis(),
        times.prune.as_millis(),
        times.push.as_millis(),
        times.total().as_millis(),
    );

    Ok(GrobnerOutput { basis: gb, grobner_ms })
}
