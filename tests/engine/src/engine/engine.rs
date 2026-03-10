use crate::engine::config::EngineConfig;
use crate::engine::orchestrator::run_backend_for_case;
use crate::gbx::backend::GbxBackend;
use crate::singular::backend::SingularBackend;
use crate::utils::case_writer::{write_compare, CaseWriter};
use crate::utils::paths::OutputLayout;
use crate::utils::sanitize_filename::sanitize_filename;
use crate::utils::test_file_config::TestFile;
use anyhow::{Context, Result};

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
            let w = CaseWriter::new(&stem, &layout.singular);
            Some(run_backend_for_case(&singular, case, &w)?)
        } else {
            None
        };

        let gbx_run = if run_gbx {
            let w = CaseWriter::new(&stem, &layout.gbx);
            Some(run_backend_for_case(&gbx, case, &w)?)
        } else {
            None
        };

        if cfg.compare && run_singular && run_gbx {
            let s = &singular_run
                .as_ref()
                .context("missing singular run")?
                .basis_lines;
            let g = &gbx_run.as_ref().context("missing gbx run")?.basis_lines;

            let report = build_compare_report(&stem, s, g);
            write_compare(&layout.compare, &stem, &report)?;
        }
    }

    Ok(())
}

/* ------------ Compare (still naive) ------------ */

fn build_compare_report(stem: &str, singular: &[String], gbx: &[String]) -> String {
    let mut a = singular.to_vec();
    let mut b = gbx.to_vec();
    a.sort();
    b.sort();

    let same = a == b;

    // basic diff
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
