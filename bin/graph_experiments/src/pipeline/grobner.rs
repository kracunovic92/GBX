use anyhow::Result;
use std::time::Instant;

use gbx_grobner::{
    buchberger_with, BasisPost, BuchbergerOptions, HeapPairs, LcmDegreeKey, NaivePairUpdate, ProductCriterion, TracingPairCriterion, TracingPairKey, TracingPairUpdate, TracingQueue,
};

use super::trace::make_tracer;
use super::types::{GbxPoly, GrevlexRing, GrobnerOutput, GrobnerStageOptions};

pub fn run(ring: &GrevlexRing, polys: &[GbxPoly], opts: GrobnerStageOptions) -> Result<GrobnerOutput> {
    let tracer = make_tracer();

    let queue = TracingQueue::wrap(HeapPairs::new(), tracer.clone());

    let criterion = TracingPairCriterion::wrap(ProductCriterion, tracer.clone());
    let key = TracingPairKey::wrap(LcmDegreeKey, tracer.clone());
    let update = NaivePairUpdate::new(criterion, key);
    let update = TracingPairUpdate::wrap(update, tracer.clone());

    let buchberger_opts = BuchbergerOptions { normalize_inputs: opts.normalize_inputs, normalize_remainders: opts.normalize_remainders, post: BasisPost::Reduced };

    let t0 = Instant::now();
    let basis = buchberger_with(ring, polys.iter().cloned(), buchberger_opts, queue, update)?;
    let grobner_ms = t0.elapsed().as_millis();

    println!("--- grobner ---");
    tracer.borrow().print_summary();
    println!("buchberger: {grobner_ms} ms");
    println!("gb_size = {}", basis.len());

    Ok(GrobnerOutput { basis, grobner_ms })
}
