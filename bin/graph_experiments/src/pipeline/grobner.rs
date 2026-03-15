use anyhow::Result;
use std::time::Instant;

use super::types::{GbxPoly, GrevlexRing, GrobnerOutput, GrobnerStageOptions};
use crate::pipeline::trace::make_debug_tracer;
use gbx_grobner::buchberger::{buchberger_with_tracer, TracingQueue};
use gbx_grobner::{BasisPost, BuchbergerOptions, GmPairUpdater, HeapPairs, LcmDegreeKey, NoPairFilter, ProductCriterion, TracingPairCriterion, TracingPairFilter, TracingPairKey};

pub fn run(ring: &GrevlexRing, polys: &[GbxPoly], opts: GrobnerStageOptions) -> Result<GrobnerOutput> {
    let tracer = make_debug_tracer();

    let queue = TracingQueue::wrap(HeapPairs::new(), tracer.clone());

    let criterion = TracingPairCriterion::wrap(ProductCriterion, tracer.clone());
    let key = TracingPairKey::wrap(LcmDegreeKey, tracer.clone());
    let filter = TracingPairFilter::wrap(NoPairFilter, tracer.clone());
    let update = GmPairUpdater::new(criterion, filter, key);

    let buchberger_opts = BuchbergerOptions { normalize_inputs: opts.normalize_inputs, normalize_remainders: opts.normalize_remainders, post: BasisPost::Reduced };

    let t0 = Instant::now();
    let basis = buchberger_with_tracer(
        ring,
        polys.iter().cloned(),
        buchberger_opts,
        queue,
        update,
        tracer,
    )?;
    let grobner_ms = t0.elapsed().as_millis();

    println!("--- grobner ---");
    println!("buchberger: {grobner_ms} ms");
    println!("gb_size = {}", basis.len());

    Ok(GrobnerOutput { basis, grobner_ms })
}
