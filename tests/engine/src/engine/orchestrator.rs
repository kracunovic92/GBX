use anyhow::Result;

use crate::engine::backend::{Backend, BackendRun, GeneratedScript};
use crate::utils::case_writer::CaseWriter;
use crate::utils::test_file_config::TestCase;

/// Pure execution: generate script + run backend.
/// Does not write anything to disk.
pub fn execute_backend_for_case<B: Backend>(backend: &B, case: &TestCase) -> Result<(GeneratedScript, BackendRun)> {
    let script = backend.generate_script(case)?;
    let run = backend.execute(case, &script)?;
    Ok((script, run))
}

/// Persistence only: save all artifacts for an already completed run.
pub fn persist_backend_run(backend_name: &str, writer: &CaseWriter<'_>, script: &GeneratedScript, run: &BackendRun) -> Result<()> {
    // 1) save script/input
    writer.script(&script.text)?;

    // 2) save stdout/stderr/output summary
    let out_text = format!(
        "backend: {}\nok: {}\nwall_time_ms: {}\n\n--- stdout ---\n{}\n\n--- stderr ---\n{}\n",
        backend_name,
        run.ok,
        run.common_metrics.wall_time.as_millis(),
        run.stdout,
        run.stderr
    );
    writer.output(&out_text)?;

    // 3) save canonical basis
    let mut basis_text = run.basis.canonical_lines.join("\n");
    if !basis_text.is_empty() {
        basis_text.push('\n');
    }
    writer.basis(&basis_text)?;

    // 4) save pretty basis, if available
    if !run.basis.pretty_lines.is_empty() {
        let mut pretty_text = run.basis.pretty_lines.join("\n");
        pretty_text.push('\n');
        writer.basis_pretty(&pretty_text)?;
    }

    // 5) save stats
    let mut stats = String::new();
    stats.push_str(&format!("backend={}\n", backend_name));
    stats.push_str(&format!("ok={}\n", run.ok));
    stats.push_str(&format!(
        "wall_time_ms={}\n",
        run.common_metrics.wall_time.as_millis()
    ));
    stats.push_str(&format!(
        "stdout_bytes={}\n",
        run.common_metrics.stdout_bytes
    ));
    stats.push_str(&format!(
        "stderr_bytes={}\n",
        run.common_metrics.stderr_bytes
    ));
    stats.push_str(&format!("basis_len={}\n", run.common_metrics.basis_len));

    if let Some(bytes) = run.common_metrics.peak_memory_bytes {
        stats.push_str(&format!("peak_memory_bytes={}\n", bytes));
    }
    if let Some(bytes) = run.common_metrics.avg_memory_bytes {
        stats.push_str(&format!("avg_memory_bytes={}\n", bytes));
    }

    if let Some(gbx) = &run.gbx_metrics {
        if let Some(v) = gbx.init_ms {
            stats.push_str(&format!("init_ms={}\n", v));
        }
        if let Some(v) = gbx.seed_ms {
            stats.push_str(&format!("seed_ms={}\n", v));
        }
        if let Some(v) = gbx.while_ms {
            stats.push_str(&format!("while_ms={}\n", v));
        }
        if let Some(v) = gbx.post_ms {
            stats.push_str(&format!("post_ms={}\n", v));
        }
        if let Some(v) = gbx.spoly_ms {
            stats.push_str(&format!("spoly_ms={}\n", v));
        }
        if let Some(v) = gbx.normal_form_ms {
            stats.push_str(&format!("normal_form_ms={}\n", v));
        }
        if let Some(v) = gbx.pair_update_ms {
            stats.push_str(&format!("pair_update_ms={}\n", v));
        }

        if let Some(v) = gbx.pairs_pushed {
            stats.push_str(&format!("pairs_pushed={}\n", v));
        }
        if let Some(v) = gbx.pairs_popped {
            stats.push_str(&format!("pairs_popped={}\n", v));
        }
        if let Some(v) = gbx.zero_reductions {
            stats.push_str(&format!("zero_reductions={}\n", v));
        }
        if let Some(v) = gbx.nonzero_insertions {
            stats.push_str(&format!("nonzero_insertions={}\n", v));
        }
        if let Some(v) = gbx.max_queue_len {
            stats.push_str(&format!("max_queue_len={}\n", v));
        }
        if let Some(v) = gbx.final_basis_len {
            stats.push_str(&format!("final_basis_len={}\n", v));
        }
    }

    for (k, v) in &run.metadata {
        stats.push_str(&format!("meta.{}={}\n", k, v));
    }

    writer.stats(&stats)?;

    // 6) save meta
    let mut meta = String::new();
    meta.push_str(&format!("backend={}\n", backend_name));
    meta.push_str(&format!("ok={}\n", run.ok));
    meta.push_str(&format!("script_ext={}\n", script.ext));
    meta.push_str(&format!(
        "wall_time_ms={}\n",
        run.common_metrics.wall_time.as_millis()
    ));
    writer.meta(&meta)?;

    Ok(())
}
