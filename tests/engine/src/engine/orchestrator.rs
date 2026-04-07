use anyhow::Result;

use crate::engine::backend::{Backend, BackendRun, GeneratedScript};
use crate::utils::case_writer::CaseWriter;
use crate::utils::test_file_config::TestCase;

pub fn execute_backend_for_case<B: Backend>(backend: &B, case: &TestCase) -> Result<(GeneratedScript, BackendRun)> {
    let script = backend.generate_script(case)?;
    let run = backend.execute(case, &script)?;
    Ok((script, run))
}

pub fn persist_backend_run(backend_name: &str, writer: &CaseWriter<'_>, script: &GeneratedScript, run: &BackendRun) -> Result<()> {
    writer.script(&script.text)?;

    let output = format!(
        "backend: {}\nok: {}\nwall_time_ms: {}\n\n--- stdout ---\n{}\n\n--- stderr ---\n{}\n",
        backend_name, run.ok, run.metrics.wall_time_ms, run.stdout, run.stderr,
    );
    writer.output(&output)?;

    let mut basis_text = run.basis.canonical_lines.join("\n");
    if !basis_text.is_empty() {
        basis_text.push('\n');
    }
    writer.basis(&basis_text)?;

    if !run.basis.pretty_lines.is_empty() {
        let mut pretty_text = run.basis.pretty_lines.join("\n");
        pretty_text.push('\n');
        writer.basis_pretty(&pretty_text)?;
    }

    let mut stats = String::new();
    stats.push_str(&format!("backend={}\n", backend_name));
    stats.push_str(&format!("ok={}\n", run.ok));
    stats.push_str(&format!("wall_time_ms={}\n", run.metrics.wall_time_ms));
    stats.push_str(&format!("stdout_bytes={}\n", run.metrics.stdout_bytes));
    stats.push_str(&format!("stderr_bytes={}\n", run.metrics.stderr_bytes));
    stats.push_str(&format!("basis_len={}\n", run.metrics.basis_len));

    if let Some(v) = run.metrics.peak_memory_bytes {
        stats.push_str(&format!("peak_memory_bytes={}\n", v));
    }
    if let Some(v) = run.metrics.avg_memory_bytes {
        stats.push_str(&format!("avg_memory_bytes={}\n", v));
    }

    for (k, v) in &run.metrics.phases_ms {
        stats.push_str(&format!("phase.{}={}\n", k, v));
    }

    for (k, v) in &run.metrics.counters {
        stats.push_str(&format!("counter.{}={}\n", k, v));
    }

    for (k, v) in &run.metadata {
        stats.push_str(&format!("meta.{}={}\n", k, v));
    }

    writer.stats(&stats)?;
    Ok(())
}
