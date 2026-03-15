use crate::engine::backend::Backend;
use crate::engine::config::EngineConfig;
use crate::engine::orchestrator::{execute_backend_for_case, persist_backend_run};
use crate::gbx::backend::GbxBackend;
use crate::singular::backend::SingularBackend;
use crate::utils::case_writer::{write_compare, CaseWriter};
use crate::utils::paths::OutputLayout;
use crate::utils::sanitize_filename::sanitize_filename;
use crate::utils::test_file_config::TestFile;
use anyhow::{Context, Result};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run_runner(cfg: EngineConfig) -> Result<()> {
    let file = TestFile::from_path(&cfg.cases_path).with_context(|| format!("loading cases from {}", cfg.cases_path.display()))?;

    let layout = OutputLayout::new(cfg.out_dir.clone());
    layout.ensure()?;
    if cfg.clean {
        layout.clean()?;
    }

    let singular = SingularBackend { cfg: cfg.singular.clone() };
    let gbx = GbxBackend { cfg: cfg.gbx.clone() };

    let run_singular = cfg.selection.run_singular();
    let run_gbx = cfg.selection.run_gbx();

    for case in &file.cases {
        let stem = sanitize_filename(&case.name);

        let singular_run = if run_singular {
            let (script, run) = execute_backend_for_case(&singular, case)?;
            let run_id = make_run_id(singular.name(), &stem);
            let run_dir = layout.make_run_dir(&run_id, script.ext)?;
            let writer = CaseWriter::new(&run_dir);

            persist_backend_run(singular.name(), &writer, &script, &run)?;
            Some(run)
        } else {
            None
        };

        let gbx_run = if run_gbx {
            let (script, run) = execute_backend_for_case(&gbx, case)?;
            let run_id = make_run_id(gbx.name(), &stem);
            let run_dir = layout.make_run_dir(&run_id, script.ext)?;
            let writer = CaseWriter::new(&run_dir);

            persist_backend_run(gbx.name(), &writer, &script, &run)?;
            Some(run)
        } else {
            None
        };

        if cfg.compare && run_singular && run_gbx {
            let s = &singular_run
                .as_ref()
                .context("missing singular run")?
                .basis
                .canonical_lines;

            let g = &gbx_run
                .as_ref()
                .context("missing gbx run")?
                .basis
                .canonical_lines;

            let report = build_compare_report(&stem, s, g);
            write_compare(&layout.compare, &stem, &report)?;
        }
    }

    Ok(())
}

fn make_run_id(backend: &str, stem: &str) -> String {
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_millis();

    format!("{ms}_{backend}_{stem}")
}

/* ------------ Compare (still naive) ------------ */

fn build_compare_report(stem: &str, singular: &[String], gbx: &[String]) -> String {
    let mut a = singular.to_vec();
    let mut b = gbx.to_vec();
    a.sort();
    b.sort();

    let same = a == b;

    let only_a: Vec<_> = a.iter().filter(|x| !b.contains(x)).cloned().collect();
    let only_b: Vec<_> = b.iter().filter(|x| !a.contains(x)).cloned().collect();

    format!(
        "compare: {stem}\nstatus: {}\n\n\
         singular_lines: {}\n\
         gbx_lines: {}\n\
         only_in_singular: {}\n\
         only_in_gbx: {}\n\n\
         --- singular (sorted) ---\n{}\n\n\
         --- gbx (sorted) ---\n{}\n\n\
         --- only in singular ---\n{}\n\n\
         --- only in gbx ---\n{}\n",
        if same { "MATCH" } else { "MISMATCH" },
        a.len(),
        b.len(),
        only_a.len(),
        only_b.len(),
        a.join("\n"),
        b.join("\n"),
        only_a.join("\n"),
        only_b.join("\n"),
    )
}
