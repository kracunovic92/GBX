use anyhow::{bail, Result};
use std::collections::BTreeMap;

use crate::gbx::config::{GbxConfig, GbxReducerKind};
use crate::gbx::parse::parse_poly_terms;
use crate::utils::test_file_config::TestCase;

use gbx_field::fp::{Fp, FpElem};
use gbx_grobner::engine::run_f4_with_reducer;
use gbx_grobner::linear::roman_sparse::{RomanParallelSparseBufferReducer, RomanSparseBufferReducer};
use gbx_grobner::linear::DenseF4MatrixReducer;
use gbx_grobner::F4Options;

use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::Polynomial;
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::{poly_terms, pretty_str, term, tuple_dump_str};

#[derive(Debug, Clone, Default)]
pub struct GbxRunOutput {
    pub basis_pretty_lines: Vec<String>,
    pub phases_ms: BTreeMap<String, u128>,
    pub counters: BTreeMap<String, u64>,
}

#[tracing::instrument(skip_all, fields(case = %case.name, reducer = %cfg.reducer.as_str()))]
pub fn compute_basis_in_ring<O>(ring: &RingCtx<Fp, O>, case: &TestCase, cfg: &GbxConfig) -> Result<GbxRunOutput>
where
    O: MonomialOrder + Clone + Sync,
{
    let gens = parse_generators_in_ring(ring, case)?;
    let gb = compute_grobner_basis_dyn(ring, &gens, cfg)?;

    let basis_dump_lines = gb
        .iter()
        .map(|p| tuple_dump_str!(ring, p))
        .collect::<Vec<_>>();

    let basis_pretty_lines = gb
        .iter()
        .map(|p| pretty_str!(ring, p, &case.vars))
        .collect::<Vec<_>>();

    let basis_pretty_normalized_lines = basis_pretty_lines
        .iter()
        .map(|s| normalize_poly_text(s))
        .collect::<Vec<_>>();

    let mut counters = BTreeMap::new();
    counters.insert("input_generators".to_string(), gens.len() as u64);
    counters.insert(
        "output_basis_len".to_string(),
        basis_dump_lines.len() as u64,
    );

    Ok(GbxRunOutput { basis_pretty_lines, phases_ms: BTreeMap::new(), counters })
}

fn compute_grobner_basis_dyn<O>(ring: &RingCtx<Fp, O>, gens: &[Polynomial<FpElem>], cfg: &GbxConfig) -> Result<Vec<Polynomial<FpElem>>>
where
    O: MonomialOrder + Clone + Sync,
{
    if gens.is_empty() {
        return Ok(Vec::new());
    }

    let opts = F4Options::default();

    let gb = match cfg.reducer {
        GbxReducerKind::Dense => {
            let reducer = DenseF4MatrixReducer;
            run_f4_with_reducer(ring, gens.iter().cloned(), opts.validated()?, &reducer)?
        }

        GbxReducerKind::Roman => {
            let reducer = RomanSparseBufferReducer;
            run_f4_with_reducer(ring, gens.iter().cloned(), opts.validated()?, &reducer)?
        }

        GbxReducerKind::RomanParallel => {
            let reducer = RomanParallelSparseBufferReducer;
            run_f4_with_reducer(ring, gens.iter().cloned(), opts.validated()?, &reducer)?
        }
    };

    Ok(gb.as_slice().to_vec())
}

fn parse_generators_in_ring<O>(ring: &RingCtx<Fp, O>, case: &TestCase) -> Result<Vec<Polynomial<FpElem>>>
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

fn parse_generator_in_ring<O>(ring: &RingCtx<Fp, O>, src: &str, vars: &[String], p: u32) -> Result<Polynomial<FpElem>>
where
    O: MonomialOrder,
{
    let parsed = parse_poly_terms(src, vars, p)?;

    let mut terms = Vec::with_capacity(parsed.len());

    for (c_mod_p, exps) in parsed {
        let coeff: FpElem = FieldCtx::new(&ring.field, c_mod_p);
        let mono = Monomial::from_slice(&exps);
        terms.push(term!(coeff, mono));
    }

    let poly: Polynomial<FpElem> = poly_terms![ring; terms]?;
    Ok(poly)
}

fn normalize_poly_text(s: &str) -> String {
    let mut s = s.chars().filter(|c| !c.is_whitespace()).collect::<String>();

    if let Some(rest) = s.strip_prefix("GB:") {
        s = rest.to_string();
    }

    s = s.replace('*', "");
    s = s.replace('^', "");

    s = s.replace("+-", "-");
    s = s.replace("-+", "-");
    s = s.replace("++", "+");
    s = s.replace("--", "+");

    s
}
