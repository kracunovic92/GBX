use anyhow::{Context, Result, bail};
use clap::Parser;
use std::path::Path;
use std::process::Command as ProcessCommand;
use std::time::{SystemTime, UNIX_EPOCH};

mod cli;
mod pipeline;
mod poly_builder;
mod tracing;

use crate::tracing::init_tracing;
use cli::{Cli, Command};
use pipeline::types::GrobnerStageOptions;

use gbx_grobner::{BasisPostOptionsKind, F4Options, F4ReducerKind, ProfileEvent};

#[cfg(feature = "profiling-alloc")]
use stats_alloc::{INSTRUMENTED_SYSTEM, StatsAlloc};

#[cfg(feature = "profiling-alloc")]
#[global_allocator]
static GLOBAL: &StatsAlloc<std::alloc::System> = &INSTRUMENTED_SYSTEM;

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_tracing();

    match cli.cmd {
        Command::Parse { file, normalize } => run_parse(&file, normalize),
        Command::Stats { file, normalize } => run_stats(&file, normalize),
        Command::Encode { file, k, p, normalize, dump } => run_encode(&file, k, p, normalize, dump),
        Command::Grobner { file, k, p, normalize, normalize_inputs, normalize_remainders, reducer, batch_size, post, profile_out, dump_basis } => run_grobner(
            &file,
            k,
            p,
            normalize,
            normalize_inputs,
            normalize_remainders,
            &reducer,
            batch_size,
            &post,
            profile_out.as_deref(),
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

#[allow(clippy::too_many_arguments)]
fn run_grobner(
    path: &Path,
    k: usize,
    p: u32,
    normalize_graph: bool,
    normalize_inputs: bool,
    normalize_remainders: bool,
    reducer: &str,
    batch_size: usize,
    post: &str,
    profile_out: Option<&Path>,
    dump_basis: bool,
) -> Result<()> {
    let reducer = parse_reducer(reducer)?;
    let post = parse_post(post)?;

    println!("== GROBNER ==");
    println!("input: {}", path.display());
    println!("params: k={k}, p={p}");
    println!("reducer={}", reducer_name(reducer));
    println!("batch_size={batch_size}");
    println!("post={}", post_name(post));
    println!("normalize_inputs={normalize_inputs}");
    println!("normalize_remainders={normalize_remainders}");

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

    let f4_options = F4Options { batch_size, reducer_kind: reducer, normalize_inputs, normalize_extracted: normalize_remainders, post, ..F4Options::default() };

    let gb = pipeline::grobner::run(
        &ring_stage.ring,
        &enc.polynomials,
        GrobnerStageOptions { normalize_inputs, normalize_remainders, f4: f4_options },
    )?;
    let peak_rss_bytes = peak_rss_bytes();

    pipeline::output::print_basis(
        &ring_stage.ring,
        gb.basis.as_slice(),
        &ring_stage.vars,
        dump_basis,
    );

    if let Some(peak) = peak_rss_bytes {
        println!("peak_rss_bytes={peak}");
    }
    println!("elapsed_seconds={:.6}", gb.grobner_seconds);
    print_phase_summary(&gb.profile_events, gb.grobner_seconds);

    if let Some(out_dir) = profile_out {
        write_profile_artifacts(
            out_dir,
            RunSummary {
                run_id: run_id(),
                input: path.display().to_string(),
                graph_vertices: parsed.graph.n(),
                graph_edges: parsed.graph.m(),
                colors: k,
                field: format!("Fp({p})"),
                order: "grevlex",
                reducer: reducer_name(reducer),
                batch_size,
                post: post_name(post),
                normalize_graph,
                normalize_inputs,
                normalize_remainders,
                nvars: ring_stage.ring.nvars,
                generators: enc.polynomials.len(),
                basis_size: gb.basis.as_slice().len(),
                parse_seconds: parsed.parse_seconds,
                normalize_graph_seconds: parsed.normalize_seconds,
                build_ring_seconds: ring_stage.build_seconds,
                encode_seconds: enc.encode_seconds,
                elapsed_seconds: gb.grobner_seconds,
                peak_rss_bytes,
            },
            &gb.profile_events,
        )?;
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct RunSummary {
    run_id: String,
    input: String,
    graph_vertices: usize,
    graph_edges: usize,
    colors: usize,
    field: String,
    order: &'static str,
    reducer: &'static str,
    batch_size: usize,
    post: &'static str,
    normalize_graph: bool,
    normalize_inputs: bool,
    normalize_remainders: bool,
    nvars: usize,
    generators: usize,
    basis_size: usize,
    parse_seconds: f64,
    normalize_graph_seconds: Option<f64>,
    build_ring_seconds: f64,
    encode_seconds: f64,
    elapsed_seconds: f64,
    peak_rss_bytes: Option<u64>,
}

fn write_profile_artifacts(out_dir: &Path, run: RunSummary, events: &[ProfileEvent]) -> Result<()> {
    std::fs::create_dir_all(out_dir).with_context(|| format!("creating profile output directory {}", out_dir.display()))?;

    write_run_tsv(&out_dir.join("run.tsv"), &run)?;
    write_phases_tsv(&out_dir.join("phases.tsv"), &run.run_id, events)?;
    write_phase_totals_tsv(
        &out_dir.join("phase_totals.tsv"),
        &run.run_id,
        events,
        run.elapsed_seconds,
    )?;
    write_environment_txt(&out_dir.join("environment.txt"))?;

    Ok(())
}

fn write_run_tsv(path: &Path, run: &RunSummary) -> Result<()> {
    let mut out = String::new();

    out.push_str("run_id\tinput\tgraph_vertices\tgraph_edges\tcolors\tfield\torder\treducer\tbatch_size\tpost\tnormalize_graph\tnormalize_inputs\tnormalize_remainders\tnvars\tgenerators\tbasis_size\tparse_seconds\tnormalize_graph_seconds\tbuild_ring_seconds\tencode_seconds\telapsed_seconds\tpeak_rss_bytes\n");
    out.push_str(&format!(
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.6}\t{}\t{:.6}\t{:.6}\t{:.6}\t{}\n",
        run.run_id,
        run.input,
        run.graph_vertices,
        run.graph_edges,
        run.colors,
        run.field,
        run.order,
        run.reducer,
        run.batch_size,
        run.post,
        run.normalize_graph,
        run.normalize_inputs,
        run.normalize_remainders,
        run.nvars,
        run.generators,
        run.basis_size,
        run.parse_seconds,
        run.normalize_graph_seconds
            .map(|v| format!("{v:.6}"))
            .unwrap_or_default(),
        run.build_ring_seconds,
        run.encode_seconds,
        run.elapsed_seconds,
        run.peak_rss_bytes
            .map(|v| v.to_string())
            .unwrap_or_default(),
    ));

    std::fs::write(path, out).with_context(|| format!("writing {}", path.display()))
}

fn write_phases_tsv(path: &Path, run_id: &str, events: &[ProfileEvent]) -> Result<()> {
    let mut out = String::new();

    out.push_str("run_id\tindex\tphase\telapsed_seconds\tbytes_allocated\tbytes_deallocated\tbytes_reallocated\tcounters\n");

    for (index, event) in events.iter().enumerate() {
        let allocations = event.allocations;
        let counters = event
            .counters
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join(";");

        out.push_str(&format!(
            "{}\t{}\t{}\t{:.6}\t{}\t{}\t{}\t{}\n",
            run_id,
            index,
            event.phase,
            event.elapsed_seconds,
            allocations
                .map(|a| a.bytes_allocated.to_string())
                .unwrap_or_default(),
            allocations
                .map(|a| a.bytes_deallocated.to_string())
                .unwrap_or_default(),
            allocations
                .map(|a| a.bytes_reallocated.to_string())
                .unwrap_or_default(),
            counters,
        ));
    }

    std::fs::write(path, out).with_context(|| format!("writing {}", path.display()))
}

#[derive(Debug, Clone)]
struct PhaseSummary {
    phase: &'static str,
    calls: usize,
    elapsed_seconds: f64,
    bytes_allocated: u64,
    bytes_deallocated: u64,
    bytes_reallocated: u64,
}

fn phase_summaries(events: &[ProfileEvent]) -> Vec<PhaseSummary> {
    let mut summaries = Vec::<PhaseSummary>::new();

    for event in events {
        let Some(summary) = summaries
            .iter_mut()
            .find(|summary| summary.phase == event.phase)
        else {
            let allocations = event.allocations.unwrap_or_default();
            summaries.push(PhaseSummary {
                phase: event.phase,
                calls: 1,
                elapsed_seconds: event.elapsed_seconds,
                bytes_allocated: allocations.bytes_allocated,
                bytes_deallocated: allocations.bytes_deallocated,
                bytes_reallocated: allocations.bytes_reallocated,
            });
            continue;
        };

        summary.calls += 1;
        summary.elapsed_seconds += event.elapsed_seconds;

        if let Some(allocations) = event.allocations {
            summary.bytes_allocated = summary
                .bytes_allocated
                .saturating_add(allocations.bytes_allocated);
            summary.bytes_deallocated = summary
                .bytes_deallocated
                .saturating_add(allocations.bytes_deallocated);
            summary.bytes_reallocated = summary
                .bytes_reallocated
                .saturating_add(allocations.bytes_reallocated);
        }
    }

    summaries
}

fn print_phase_summary(events: &[ProfileEvent], run_elapsed_seconds: f64) {
    let summaries = phase_summaries(events);

    if summaries.is_empty() {
        println!("phase_timing_seconds=none");
        return;
    }

    println!("phase_timing_seconds:");
    println!(
        "{:<58} {:>7} {:>14} {:>9} {:>16} {:>16} {:>16}",
        "phase", "calls", "seconds", "run_%", "alloc_bytes", "dealloc_bytes", "realloc_bytes"
    );

    for summary in summaries {
        let percent = if run_elapsed_seconds > 0.0 { summary.elapsed_seconds * 100.0 / run_elapsed_seconds } else { 0.0 };

        println!(
            "{:<58} {:>7} {:>14.6} {:>8.2}% {:>16} {:>16} {:>16}",
            summary.phase, summary.calls, summary.elapsed_seconds, percent, summary.bytes_allocated, summary.bytes_deallocated, summary.bytes_reallocated,
        );
    }
}

fn write_phase_totals_tsv(path: &Path, run_id: &str, events: &[ProfileEvent], run_elapsed_seconds: f64) -> Result<()> {
    let mut out = String::new();

    out.push_str("run_id\tphase\tcalls\telapsed_seconds\trun_percent\tbytes_allocated\tbytes_deallocated\tbytes_reallocated\n");

    for summary in phase_summaries(events) {
        let percent = if run_elapsed_seconds > 0.0 { summary.elapsed_seconds * 100.0 / run_elapsed_seconds } else { 0.0 };

        out.push_str(&format!(
            "{}\t{}\t{}\t{:.6}\t{:.2}\t{}\t{}\t{}\n",
            run_id, summary.phase, summary.calls, summary.elapsed_seconds, percent, summary.bytes_allocated, summary.bytes_deallocated, summary.bytes_reallocated,
        ));
    }

    std::fs::write(path, out).with_context(|| format!("writing {}", path.display()))
}

fn write_environment_txt(path: &Path) -> Result<()> {
    let mut out = String::new();

    out.push_str(&format!("timestamp_unix={}\n", unix_timestamp_secs()));
    out.push_str(&format!(
        "git_commit={}\n",
        command_output("git", &["rev-parse", "HEAD"]).unwrap_or_default()
    ));
    out.push_str(&format!("git_dirty={}\n", git_dirty()));
    out.push_str(&format!(
        "rustc={}\n",
        command_output("rustc", &["--version"]).unwrap_or_default()
    ));
    out.push_str("cargo_profile=release_recommended_for_thesis_runs\n");

    std::fs::write(path, out).with_context(|| format!("writing {}", path.display()))
}

fn run_id() -> String {
    format!("graph-{}", unix_timestamp_secs())
}

fn unix_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

fn command_output(program: &str, args: &[&str]) -> Option<String> {
    let output = ProcessCommand::new(program).args(args).output().ok()?;

    if !output.status.success() {
        return None;
    }

    Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn git_dirty() -> bool {
    ProcessCommand::new("git")
        .args(["diff", "--quiet"])
        .status()
        .map(|status| !status.success())
        .unwrap_or(false)
}

fn peak_rss_bytes() -> Option<u64> {
    let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();

    let rc = unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) };

    if rc != 0 {
        return None;
    }

    let usage = unsafe { usage.assume_init() };

    Some((usage.ru_maxrss as u64) * 1024)
}

fn parse_reducer(value: &str) -> Result<F4ReducerKind> {
    match value {
        "dense" => Ok(F4ReducerKind::Dense),
        "roman" => Ok(F4ReducerKind::Roman),
        "roman-parallel" | "roman_parallel" => Ok(F4ReducerKind::RomanParallel),
        _ => bail!("unknown reducer '{value}', expected one of: dense, roman, roman-parallel"),
    }
}

fn reducer_name(reducer: F4ReducerKind) -> &'static str {
    match reducer {
        F4ReducerKind::Dense => "dense",
        F4ReducerKind::Roman => "roman",
        F4ReducerKind::RomanParallel => "roman-parallel",
    }
}

fn parse_post(value: &str) -> Result<BasisPostOptionsKind> {
    match value {
        "none" => Ok(BasisPostOptionsKind::None),
        "minimal" => Ok(BasisPostOptionsKind::Minimal),
        "reduced" => Ok(BasisPostOptionsKind::Reduced),
        _ => bail!("unknown post mode '{value}', expected one of: none, minimal, reduced"),
    }
}

fn post_name(post: BasisPostOptionsKind) -> &'static str {
    match post {
        BasisPostOptionsKind::None => "none",
        BasisPostOptionsKind::Minimal => "minimal",
        BasisPostOptionsKind::Reduced => "reduced",
    }
}
