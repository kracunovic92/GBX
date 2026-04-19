use anyhow::{bail, Result};
use std::collections::BTreeMap;

use gbx_field::fp::{FpDyn, FpDynElem};
use gbx_grobner::{f4, F4Options};
use gbx_poly::monomial::DynamicMonomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolyDyn;
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::{poly_terms, pretty_str, term, tuple_dump_str};

use crate::gbx::parse::parse_poly_terms;
use crate::utils::test_file_config::TestCase;

#[derive(Debug, Clone, Default)]
pub struct GbxRunOutput {
    /// Canonical structural dump for diffing.
    pub basis_dump_lines: Vec<String>,

    /// Human-readable printer output.
    pub basis_pretty_lines: Vec<String>,

    /// Normalized pretty lines used only for text comparison/debugging.
    pub basis_pretty_normalized_lines: Vec<String>,

    /// Parsed input generators in canonical form.
    pub input_dump_lines: Vec<String>,

    /// Parsed input generators in pretty form.
    pub input_pretty_lines: Vec<String>,

    pub phases_ms: BTreeMap<String, u128>,
    pub counters: BTreeMap<String, u64>,
}

#[tracing::instrument(skip_all, fields(case = %case.name))]
pub fn compute_basis_in_ring<O>(ring: &RingCtx<FpDyn, O>, case: &TestCase) -> Result<GbxRunOutput>
where
    O: MonomialOrder + Clone,
{
    let gens = parse_generators_in_ring(ring, case)?;
    let gb = compute_grobner_basis_dyn(ring, &gens)?;

    let input_dump_lines = gens
        .iter()
        .map(|p| tuple_dump_str!(ring, p))
        .collect::<Vec<_>>();

    let input_pretty_lines = gens
        .iter()
        .map(|p| pretty_str!(ring, p, &case.vars))
        .collect::<Vec<_>>();

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

    Ok(GbxRunOutput { basis_dump_lines, basis_pretty_lines, basis_pretty_normalized_lines, input_dump_lines, input_pretty_lines, phases_ms: BTreeMap::new(), counters })
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

    let poly: PolyDyn<FpDynElem> = poly_terms![ring; terms]?;
    Ok(poly)
}

fn compute_grobner_basis_dyn<O>(ring: &RingCtx<FpDyn, O>, gens: &[PolyDyn<FpDynElem>]) -> Result<Vec<PolyDyn<FpDynElem>>>
where
    O: MonomialOrder + Clone,
{
    if gens.is_empty() {
        return Ok(Vec::new());
    }

    let gb = f4(ring, gens.iter().cloned(), F4Options::default())?;
    Ok(gb.as_slice().to_vec())
}
fn normalize_poly_text(s: &str) -> String {
    let mut s = s.chars().filter(|c| !c.is_whitespace()).collect::<String>();

    if let Some(rest) = s.strip_prefix("GB:") {
        s = rest.to_string();
    }

    s = s.replace('*', "");

    s = s.replace("+-", "-");
    s = s.replace("-+", "-");
    s = s.replace("++", "+");
    s = s.replace("--", "+");

    s
}
