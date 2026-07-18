use crate::engine::backend::{Backend, BackendRun};
use crate::engine::compare::{build_compare_report, compare_normalized_pretty};
use crate::engine::config::EngineConfig;
use crate::engine::orchestrator::{execute_backend_for_case, persist_backend_stats};
use crate::gbx::backend::GbxBackend;
use crate::singular::backend::SingularBackend;
use crate::utils::io::write_text;
use crate::utils::paths::OutputLayout;
use crate::utils::sanitize_filename::sanitize_filename;
use crate::utils::test_file_config::{TestCase, TestFile};
use anyhow::{Context, Result};
use std::fmt::Write as _;
use tracing::info;

#[tracing::instrument(skip_all, fields(cases_path = %cfg.cases_path.display(), out_dir = %cfg.out_dir.display()))]
pub fn run_runner(cfg: &EngineConfig) -> Result<()> {
    let file = TestFile::from_path(&cfg.cases_path).with_context(|| format!("loading cases from {}", cfg.cases_path.display()))?;

    let layout = OutputLayout::new(cfg.out_dir.clone());
    layout.clean()?;

    let singular = SingularBackend { cfg: cfg.singular.clone() };

    let gbx_backends = cfg
        .gbx_reducers
        .iter()
        .cloned()
        .map(|cfg| GbxBackend { cfg })
        .collect::<Vec<_>>();

    info!(
        cases = file.cases.len(),
        gbx_reducers = gbx_backends.len(),
        "loaded test cases"
    );

    for case in &file.cases {
        run_case(&layout, case, &singular, &gbx_backends)?;
    }

    Ok(())
}

#[tracing::instrument(skip_all, fields(case = %case.name))]
fn run_case(layout: &OutputLayout, case: &TestCase, singular: &SingularBackend, gbx_backends: &[GbxBackend]) -> Result<()> {
    let stem = sanitize_filename(&case.name);

    eprintln!();
    eprintln!("============================================================");
    eprintln!("Running test case: {}", case.name);
    eprintln!(
        "field={}, p={}, order={}, vars={}",
        case.field,
        case.p,
        case.order,
        case.vars.len()
    );
    eprintln!("============================================================");

    eprintln!("→ [{}] running singular...", case.name);
    let singular_start = std::time::Instant::now();

    let (_singular_script, singular_run) = execute_backend_for_case(singular, case)?;

    eprintln!(
        "✓ [{}] singular finished: ok={}, compute_time_ms={}, elapsed_ms={}",
        case.name,
        singular_run.ok,
        singular_run.metrics.compute_time_ms,
        singular_start.elapsed().as_millis(),
    );

    let singular_name = singular.name();

    let singular_stats_path = layout.run_stats_file(&singular_name, &stem);
    persist_backend_stats(&singular_stats_path, &singular_name, &singular_run)?;

    let mut gbx_results = Vec::new();

    for gbx in gbx_backends {
        let gbx_name = gbx.name();

        eprintln!("→ [{}] running {}...", case.name, gbx_name);
        let gbx_start = std::time::Instant::now();

        let (_gbx_script, gbx_run) = execute_backend_for_case(gbx, case)?;

        eprintln!(
            "✓ [{}] {} finished: ok={}, compute_time_ms={}, elapsed_ms={}",
            case.name,
            gbx_name,
            gbx_run.ok,
            gbx_run.metrics.compute_time_ms,
            gbx_start.elapsed().as_millis(),
        );

        let gbx_stats_path = layout.run_stats_file(&gbx_name, &stem);
        persist_backend_stats(&gbx_stats_path, &gbx_name, &gbx_run)?;

        gbx_results.push((gbx_name, gbx_run));
    }

    let compare_report = build_case_compare_report(case, &singular_name, &singular_run, &gbx_results);

    write_text(layout.compare_file(&stem), compare_report)?;

    let summary = build_case_summary(case, &singular_name, &singular_run, &gbx_results);

    write_text(layout.summary_file(&stem), summary)?;

    eprintln!("✓ [{}] all backends finished", case.name);

    info!(singular_ok = singular_run.ok, "finished case");

    Ok(())
}

fn build_case_compare_report(case: &TestCase, singular_name: &str, singular_run: &BackendRun, gbx_results: &[(String, BackendRun)]) -> String {
    let mut out = String::new();

    let _ = writeln!(out, "case: {}", case.name);
    let _ = writeln!(out, "reference: {singular_name}\n");

    for (gbx_name, gbx_run) in gbx_results {
        out.push_str(&build_compare_report(
            &case.name,
            singular_name,
            gbx_name,
            &singular_run.basis.pretty_lines,
            &gbx_run.basis.pretty_lines,
        ));
        out.push_str("\n\n");
    }

    out
}

fn build_case_summary(case: &TestCase, singular_name: &str, singular_run: &BackendRun, gbx_results: &[(String, BackendRun)]) -> String {
    let mut out = String::new();

    let _ = writeln!(out, "case={}", case.name);
    let _ = writeln!(out, "field={}", case.field);
    let _ = writeln!(out, "p={}", case.p);
    let _ = writeln!(out, "order={}", case.order);
    out.push('\n');

    out.push_str("backend,ok,match_vs_singular,compute_time_ms,peak_memory_bytes\n");

    out.push_str(&summary_csv_line(singular_name, singular_run, "REFERENCE"));

    for (gbx_name, gbx_run) in gbx_results {
        let cmp = compare_normalized_pretty(
            &singular_run.basis.pretty_lines,
            &gbx_run.basis.pretty_lines,
        );

        out.push_str(&summary_csv_line(gbx_name, gbx_run, cmp.status.as_str()));
    }

    out
}

fn summary_csv_line(backend_name: &str, run: &BackendRun, match_status: &str) -> String {
    format!(
        "{},{},{},{},{}\n",
        backend_name,
        run.ok,
        match_status,
        run.metrics.compute_time_ms,
        run.metrics
            .peak_memory_bytes
            .map(|v| v.to_string())
            .unwrap_or_default(),
    )
}
