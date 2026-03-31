use anyhow::{bail, Result};

use gbx_field::fp::{FpDyn, FpDynElem};
use gbx_grobner::api::f4_with;
use gbx_grobner::{
    BuchbergerTraceConfig, BuchbergerTracer, F4Options, GmPairUpdater, HeapPairs, LcmDegreeKey, NoPairFilter, ProductCriterion, TraceConfig, TraceLevel, TraceReportMode, TracingPairCriterion,
    TracingPairFilter, TracingPairKey, TracingQueue,
};
use gbx_poly::monomial::DynamicMonomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolyDyn;
use gbx_poly::ring::{FieldCtx, RingCtx};

use gbx_poly::{poly_terms, pretty_str, term, tuple_dump_str};

use crate::gbx::parse::parse_poly_terms;
use crate::utils::test_file_config::TestCase;

pub struct GbxRunOutput {
    /// Stable dump for diffing (tuple format).
    pub basis_dump_lines: Vec<String>,
    /// Pretty output for humans.
    pub basis_pretty_lines: Vec<String>,
}

pub fn compute_basis_in_ring<O>(ring: &RingCtx<FpDyn, O>, case: &TestCase) -> Result<GbxRunOutput>
where
    O: MonomialOrder,
{
    let gens = parse_generators_in_ring(ring, case)?;
    let gb = compute_grobner_basis_dyn(ring, &gens)?;

    let basis_dump_lines = gb
        .iter()
        .map(|p| tuple_dump_str!(ring, p))
        .collect::<Vec<_>>();

    let basis_pretty_lines = gb
        .iter()
        .map(|p| pretty_str!(ring, p, &case.vars))
        .collect::<Vec<_>>();

    Ok(GbxRunOutput { basis_dump_lines, basis_pretty_lines })
}

fn parse_generators_in_ring<O>(ring: &RingCtx<FpDyn, O>, case: &TestCase) -> Result<Vec<PolyDyn<FpDynElem>>>
where
    O: MonomialOrder,
{
    if case.generators.is_empty() {
        bail!("generators must be non-empty");
    }

    let mut out = Vec::with_capacity(case.generators.len());
    for g in &case.generators {
        out.push(parse_generator_in_ring(ring, g, &case.vars, case.p)?);
    }
    Ok(out)
}

fn parse_generator_in_ring<O>(ring: &RingCtx<FpDyn, O>, src: &str, vars: &[String], p: u32) -> Result<PolyDyn<FpDynElem>>
where
    O: MonomialOrder,
{
    let parsed = parse_poly_terms(src, vars, p)?;

    let mut terms = Vec::with_capacity(parsed.len());
    for (c_mod_p, exps) in parsed {
        let coeff: FpDynElem = FieldCtx::new(&ring.field, c_mod_p);
        let mono = DynamicMonomial::from_slice(&exps);
        terms.push(term!(coeff, mono));
    }

    // from_terms_in() normalizes canonical representation
    let p: PolyDyn<FpDynElem> = poly_terms![ring; terms]?;
    Ok(p)
}

/// Compute Grobner basis from generators using the real Buchberger engine.
/// Returns a Vec<P> basis in whatever ordering your GrobnerBasis stores.
fn compute_grobner_basis_dyn<O>(ring: &RingCtx<FpDyn, O>, gens: &[PolyDyn<FpDynElem>]) -> Result<Vec<PolyDyn<FpDynElem>>>
where
    O: MonomialOrder,
{
    if gens.is_empty() {
        return Ok(Vec::new());
    }

    let opts = F4Options::default();

    let tracer = BuchbergerTracer::shared(BuchbergerTraceConfig {
        core: TraceConfig { level: TraceLevel::Verbose, report_mode: TraceReportMode::Verbose, snapshot_every: 100, memory_every: 0, final_memory: true },
        progress_every: 100,
        print_on_insert: false,
        print_each_iteration: true,
        print_iteration_timings: false,
        print_phase_summary: true,
        print_breakdown: true,
        sample_memory_on_progress: false,
        sample_memory_on_summary: true,
    });

    let queue = TracingQueue::wrap(HeapPairs::new(), tracer.clone());

    let criterion = TracingPairCriterion::wrap(ProductCriterion, tracer.clone());
    let key = TracingPairKey::wrap(LcmDegreeKey, tracer.clone());
    let filter = TracingPairFilter::wrap(NoPairFilter, tracer.clone());
    let mut update = GmPairUpdater::new(criterion, key);

    let gb = f4_with(ring, gens.iter().cloned(), opts, queue, &mut update)?;
    Ok(gb.as_slice().to_vec())
}
