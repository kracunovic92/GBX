use anyhow::Result;
use clap::Parser;
use std::path::Path;

mod cli;
mod pipeline;
mod poly_builder;
mod tracing;

use crate::tracing::init_tracing;
use cli::{Cli, Command};
use pipeline::types::GrobnerStageOptions;

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_tracing();

    match cli.cmd {
        Command::Parse { file, normalize } => run_parse(&file, normalize),
        Command::Stats { file, normalize } => run_stats(&file, normalize),
        Command::Encode { file, k, p, normalize, dump } => run_encode(&file, k, p, normalize, dump),
        Command::Grobner { file, k, p, normalize, normalize_inputs, normalize_remainders, dump_basis } => run_grobner(
            &file,
            k,
            p,
            normalize,
            normalize_inputs,
            normalize_remainders,
            dump_basis,
        ),
    }
}

fn run_parse(path: &Path, normalize: bool) -> Result<()> {
    let parsed = pipeline::parse_graph::run(path, normalize)?;

    println!("--- graph adjacency ---");
    println!("{}", parsed.graph);

    Ok(())
}

fn run_stats(path: &Path, normalize: bool) -> Result<()> {
    let parsed = pipeline::parse_graph::run(path, normalize)?;
    let g = &parsed.graph;

    let max_deg = (1..=g.n()).map(|v| g.adj()[v].len()).max().unwrap_or(0);
    let sum_deg: usize = (1..=g.n()).map(|v| g.adj()[v].len()).sum();
    let avg_deg = if g.n() == 0 { 0.0 } else { sum_deg as f64 / g.n() as f64 };

    println!("--- stats ---");
    println!("max_degree = {max_deg}");
    println!("avg_degree = {avg_deg:.3}");

    Ok(())
}

fn run_encode(path: &Path, k: usize, p: u32, normalize_graph: bool, dump: Option<usize>) -> Result<()> {
    println!("== ENCODE ==");
    println!("input: {}", path.display());
    println!("params: k={k}, p={p}");

    let parsed = pipeline::parse_graph::run(path, normalize_graph)?;
    let ring_stage = pipeline::build_ring::run(parsed.graph.n() * k, p)?;
    let enc = pipeline::encode::run(&parsed.graph, k, &ring_stage.ring)?;

    if let Some(limit) = dump {
        pipeline::output::print_poly_preview(
            "input polynomials preview",
            &ring_stage.ring,
            &enc.polynomials,
            &ring_stage.vars,
            limit,
        );
    }

    Ok(())
}

fn run_grobner(path: &Path, k: usize, p: u32, normalize_graph: bool, normalize_inputs: bool, normalize_remainders: bool, dump_basis: bool) -> Result<()> {
    println!("== GROBNER ==");
    println!("input: {}", path.display());
    println!("params: k={k}, p={p}");
    println!("buchberger opts: normalize_inputs={normalize_inputs}, normalize_remainders={normalize_remainders}, post=Reduced");

    let parsed = pipeline::parse_graph::run(path, normalize_graph)?;
    let ring_stage = pipeline::build_ring::run(parsed.graph.n() * k, p)?;
    let enc = pipeline::encode::run(&parsed.graph, k, &ring_stage.ring)?;

    pipeline::output::print_poly_preview(
        "input polynomials preview",
        &ring_stage.ring,
        &enc.polynomials,
        &ring_stage.vars,
        5,
    );

    let gb = pipeline::grobner::run(
        &ring_stage.ring,
        &enc.polynomials,
        GrobnerStageOptions { normalize_inputs, normalize_remainders },
    )?;

    pipeline::output::print_basis(
        &ring_stage.ring,
        gb.basis.as_slice(),
        &ring_stage.vars,
        dump_basis,
    );

    Ok(())
}
