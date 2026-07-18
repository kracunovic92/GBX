use anyhow::{Context, Result};
use std::path::Path;
use std::time::Instant;

use super::types::ParseGraphOutput;

pub fn run(path: &Path, normalize: bool) -> Result<ParseGraphOutput> {
    let t0 = Instant::now();
    let mut graph = gbx_graph::read_dimacs_file(path).with_context(|| format!("failed to read DIMACS from {}", path.display()))?;
    let parse_seconds = t0.elapsed().as_secs_f64();

    let normalize_seconds = if normalize {
        let t1 = Instant::now();
        graph.normalize_simple();
        Some(t1.elapsed().as_secs_f64())
    } else {
        None
    };

    println!("--- parse_graph ---");
    println!("input: {}", path.display());
    println!("parse_seconds={parse_seconds:.6}");

    if let Some(seconds) = normalize_seconds {
        println!("normalize_graph_seconds={seconds:.6}");
        println!("graph(normalized): n={}, m={}", graph.n(), graph.m());
    } else {
        println!("graph: n={}, m={}", graph.n(), graph.m());
    }

    Ok(ParseGraphOutput { graph, parse_seconds, normalize_seconds })
}
