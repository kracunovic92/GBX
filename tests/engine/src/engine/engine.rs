use crate::engine::backend::Backend;
use crate::engine::compare::build_compare_report;
use crate::engine::config::EngineConfig;
use crate::engine::orchestrator::{execute_backend_for_case, persist_backend_run};
use crate::gbx::backend::GbxBackend;
use crate::singular::backend::SingularBackend;
use crate::utils::case_writer::{write_compare, CaseWriter};
use crate::utils::paths::OutputLayout;
use crate::utils::sanitize_filename::sanitize_filename;
use crate::utils::test_file_config::{TestCase, TestFile};
use anyhow::{Context, Result};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

#[tracing::instrument(skip_all, fields(cases_path = %cfg.cases_path.display(), out_dir = %cfg.out_dir.display()))]
pub fn run_runner(cfg: EngineConfig) -> Result<()> {
    let file = TestFile::from_path(&cfg.cases_path).with_context(|| format!("loading cases from {}", cfg.cases_path.display()))?;

    let layout = OutputLayout::new(cfg.out_dir.clone());
    layout.ensure()?;
    layout.clean()?;

    let singular = SingularBackend { cfg: cfg.singular.clone() };
    let gbx = GbxBackend { cfg: cfg.gbx.clone() };

    info!("loaded {} test cases", file.cases.len());

    for case in &file.cases {
        run_case(&layout, case, &singular, &gbx)?;
    }

    Ok(())
}

#[tracing::instrument(skip_all, fields(case = %case.name))]
fn run_case(layout: &OutputLayout, case: &TestCase, singular: &SingularBackend, gbx: &GbxBackend) -> Result<()> {
    let stem = sanitize_filename(&case.name);

    let (singular_script, singular_run) = execute_backend_for_case(singular, case)?;
    let singular_run_id = make_run_id(singular.name(), &stem);
    let singular_run_dir = layout.make_run_dir(&singular_run_id, singular_script.ext)?;
    let singular_writer = CaseWriter::new(&singular_run_dir);
    persist_backend_run(
        singular.name(),
        &singular_writer,
        &singular_script,
        &singular_run,
    )?;

    let (gbx_script, gbx_run) = execute_backend_for_case(gbx, case)?;
    let gbx_run_id = make_run_id(gbx.name(), &stem);
    let gbx_run_dir = layout.make_run_dir(&gbx_run_id, gbx_script.ext)?;
    let gbx_writer = CaseWriter::new(&gbx_run_dir);
    persist_backend_run(gbx.name(), &gbx_writer, &gbx_script, &gbx_run)?;

    let report = build_compare_report(
        &case.name,
        &singular_run.basis.canonical_lines,
        &gbx_run.basis.canonical_lines,
        &singular_run.basis.pretty_lines,
        &gbx_run.basis.pretty_lines,
    );
    write_compare(&layout.compare, &stem, &report)?;

    info!(
        singular_ok = singular_run.ok,
        gbx_ok = gbx_run.ok,
        singular_basis_len = singular_run.metrics.basis_len,
        gbx_basis_len = gbx_run.metrics.basis_len,
        "finished case"
    );

    Ok(())
}

fn make_run_id(backend: &str, stem: &str) -> String {
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_millis();

    format!("{ms}_{backend}_{stem}")
}
