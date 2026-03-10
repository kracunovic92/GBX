use anyhow::{Context, Result};
use std::path::Path;
use std::time::Instant;

use super::types::ParseGraphOutput;

pub fn run(path: &Path, normalize: bool) -> Result<ParseGraphOutput> {
    let t0 = Instant::now();
    let mut graph = gbx_graph::read_dimacs_file(path).with_context(|| format!("failed to read DIMACS from {}", path.display()))?;
    let parse_ms = t0.elapsed().as_millis();

    let normalize_ms = if normalize {
        let t1 = Instant::now();
        graph.normalize_simple();
        Some(t1.elapsed().as_millis())
    } else {
        None
    };

    println!("--- parse_graph ---");
    println!("input: {}", path.display());
    println!("parse: {parse_ms} ms");

    if let Some(ms) = normalize_ms {
        println!("normalize_graph: {ms} ms");
        println!("graph(normalized): n={}, m={}", graph.n(), graph.m());
    } else {
        println!("graph: n={}, m={}", graph.n(), graph.m());
    }

    Ok(ParseGraphOutput { graph, parse_ms, normalize_ms })
}
