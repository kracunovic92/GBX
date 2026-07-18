use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use gbx_field::fp::{Fp, FpElem};
use gbx_grobner::{BasisPostOptionsKind, F4Options, F4ReducerKind, ProfileEvent, clear_profile_events, f4, take_profile_events};
use gbx_poly::monomial::Monomial;
use gbx_poly::poly;
use gbx_poly::polynomial::Polynomial;
use gbx_poly::ring::{Ring, RingCtx};
use gbx_poly::term::Term;

#[cfg(feature = "profiling-alloc")]
use stats_alloc::{INSTRUMENTED_SYSTEM, StatsAlloc};

#[cfg(feature = "profiling-alloc")]
#[global_allocator]
static GLOBAL: &StatsAlloc<std::alloc::System> = &INSTRUMENTED_SYSTEM;

type P = Polynomial<FpElem>;

fn main() -> Result<()> {
    let cli = Cli::parse()?;

    init_tracing();

    let field = Fp::prime(32003).context("invalid modulus p")?;

    let ring = Ring::builder()
        .field(field)
        .order(gbx_poly::order::Grevlex)
        .nvars(7)
        .build()?;

    let generators = cyclic7_generators(&ring)?;

    println!("case=cyclic7");
    println!("field=Fp(32003)");
    println!("order=grevlex");
    println!("nvars=7");
    println!("generators={}", generators.len());

    let opts = F4Options { batch_size: cli.batch_size, reducer_kind: cli.reducer, post: cli.post, ..F4Options::default() };

    println!("reducer={}", reducer_name(opts.reducer_kind));
    println!("batch_size={}", opts.batch_size);
    println!("post={}", post_name(opts.post));

    clear_profile_events();

    let started = Instant::now();

    let gb = f4(&ring, generators.iter().cloned(), opts)?;

    let elapsed = started.elapsed();
    let profile_events = take_profile_events();
    let peak_rss_bytes = peak_rss_bytes();

    println!("basis_size={}", gb.as_slice().len());
    if let Some(peak) = peak_rss_bytes {
        println!("peak_rss_bytes={peak}");
    }
    println!("elapsed_seconds={:.6}", elapsed.as_secs_f64());
    print_phase_summary(&profile_events, elapsed.as_secs_f64());

    if let Some(out_dir) = cli.profile_out {
        write_profile_artifacts(
            &out_dir,
            RunSummary {
                run_id: run_id(),
                case: "cyclic7",
                field: "Fp(32003)",
                order: "grevlex",
                reducer: reducer_name(opts.reducer_kind),
                batch_size: opts.batch_size,
                post: post_name(opts.post),
                nvars: 7,
                generators: generators.len(),
                basis_size: gb.as_slice().len(),
                elapsed_seconds: elapsed.as_secs_f64(),
                peak_rss_bytes,
            },
            &profile_events,
        )?;
    }

    Ok(())
}

#[derive(Debug)]
struct Cli {
    profile_out: Option<PathBuf>,
    reducer: F4ReducerKind,
    batch_size: usize,
    post: BasisPostOptionsKind,
}

impl Default for Cli {
    fn default() -> Self {
        let opts = F4Options::default();

        Self { profile_out: None, reducer: opts.reducer_kind, batch_size: opts.batch_size, post: opts.post }
    }
}

impl Cli {
    fn parse() -> Result<Self> {
        let mut args = std::env::args_os().skip(1);
        let mut cli = Self::default();

        while let Some(arg) = args.next() {
            if arg == "--profile-out" {
                let Some(path) = args.next() else {
                    bail!("--profile-out requires an output directory");
                };

                cli.profile_out = Some(PathBuf::from(path));
            } else if arg == "--reducer" {
                let Some(value) = args.next() else {
                    bail!("--reducer requires one of: dense, roman, roman-parallel");
                };

                cli.reducer = parse_reducer(&value.to_string_lossy())?;
            } else if arg == "--batch-size" {
                let Some(value) = args.next() else {
                    bail!("--batch-size requires a positive integer");
                };

                cli.batch_size = value
                    .to_string_lossy()
                    .parse()
                    .context("--batch-size must be a positive integer")?;
            } else if arg == "--post" {
                let Some(value) = args.next() else {
                    bail!("--post requires one of: none, minimal, reduced");
                };

                cli.post = parse_post(&value.to_string_lossy())?;
            } else {
                bail!("unknown argument: {}", arg.to_string_lossy());
            }
        }

        Ok(cli)
    }
}

#[derive(Debug, Clone)]
struct RunSummary {
    run_id: String,
    case: &'static str,
    field: &'static str,
    order: &'static str,
    reducer: &'static str,
    batch_size: usize,
    post: &'static str,
    nvars: usize,
    generators: usize,
    basis_size: usize,
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

    out.push_str("run_id\tcase\tfield\torder\treducer\tbatch_size\tpost\tnvars\tgenerators\tbasis_size\telapsed_seconds\tpeak_rss_bytes\n");
    out.push_str(&format!(
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.6}\t{}\n",
        run.run_id,
        run.case,
        run.field,
        run.order,
        run.reducer,
        run.batch_size,
        run.post,
        run.nvars,
        run.generators,
        run.basis_size,
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
    format!("baseline-{}", unix_timestamp_secs())
}

fn unix_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

fn command_output(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;

    if !output.status.success() {
        return None;
    }

    Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn git_dirty() -> bool {
    Command::new("git")
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

#[cfg(feature = "instrumentation")]
fn init_tracing() {
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::{EnvFilter, fmt};

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,gbx_grobner=debug,baseline_experiments=debug"));

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_level(true)
        .with_span_events(FmtSpan::CLOSE)
        .compact()
        .init();
}

#[cfg(not(feature = "instrumentation"))]
fn init_tracing() {}

fn cyclic6_generators(ring: &RingCtx<Fp, gbx_poly::order::Grevlex>) -> Result<Vec<P>> {
    let g1: P = poly![
        ring;
        (1, [1, 0, 0, 0, 0, 0]),
        (1, [0, 1, 0, 0, 0, 0]),
        (1, [0, 0, 1, 0, 0, 0]),
        (1, [0, 0, 0, 1, 0, 0]),
        (1, [0, 0, 0, 0, 1, 0]),
        (1, [0, 0, 0, 0, 0, 1])
    ]?;

    let g2 = cyclic_sum(ring, 2)?;
    let g3 = cyclic_sum(ring, 3)?;
    let g4 = cyclic_sum(ring, 4)?;
    let g5 = cyclic_sum(ring, 5)?;

    let g6: P = poly![
        ring;
        (1_u32, [1, 1, 1, 1, 1, 1]),
        (32002_u32, [0, 0, 0, 0, 0, 0])
    ]?;

    Ok(vec![g1, g2, g3, g4, g5, g6])
}

#[allow(dead_code)]
fn cyclic7_generators(ring: &RingCtx<Fp, gbx_poly::order::Grevlex>) -> Result<Vec<P>> {
    let g1: P = poly![
        ring;
        (1, [1, 0, 0, 0, 0, 0, 0]),
        (1, [0, 1, 0, 0, 0, 0, 0]),
        (1, [0, 0, 1, 0, 0, 0, 0]),
        (1, [0, 0, 0, 1, 0, 0, 0]),
        (1, [0, 0, 0, 0, 1, 0, 0]),
        (1, [0, 0, 0, 0, 0, 1, 0]),
        (1, [0, 0, 0, 0, 0, 0, 1])
    ]?;

    let g2 = cyclic_sum(ring, 2)?;
    let g3 = cyclic_sum(ring, 3)?;
    let g4 = cyclic_sum(ring, 4)?;
    let g5 = cyclic_sum(ring, 5)?;
    let g6 = cyclic_sum(ring, 6)?;

    let g7: P = poly![
        ring;
        (1_u32, [1, 1, 1, 1, 1, 1, 1]),
        (32002_u32, [0, 0, 0, 0, 0, 0, 0])
    ]?;

    Ok(vec![g1, g2, g3, g4, g5, g6, g7])
}

fn cyclic_sum(ring: &RingCtx<Fp, gbx_poly::order::Grevlex>, width: usize) -> Result<P> {
    debug_assert!((1..=7).contains(&width));

    let terms = (0..6)
        .map(|start| {
            let mut exps = [0_u32; 7];

            for offset in 0..width {
                let idx = (start + offset) % 7;
                exps[idx] = 1;
            }

            (1_u32, exps)
        })
        .collect::<Vec<_>>();

    poly_from_terms(ring, &terms)
}

fn poly_from_terms(ring: &RingCtx<Fp, gbx_poly::order::Grevlex>, terms: &[(u32, [u32; 7])]) -> Result<P> {
    let terms = terms
        .iter()
        .map(|(coeff, exps)| Term::new(ring.field.elem(*coeff), Monomial::from_slice(exps)))
        .collect::<Vec<_>>();

    P::from_terms_in(ring, terms).map_err(Into::into)
}
