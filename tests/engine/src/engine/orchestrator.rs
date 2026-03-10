use anyhow::Result;

use crate::engine::backend::{Backend, BackendRun};
use crate::utils::case_writer::CaseWriter;
use crate::utils::test_file_config::TestCase;

pub fn run_backend_for_case<B: Backend>(backend: &B, case: &TestCase, writer: &CaseWriter<'_>) -> Result<BackendRun> {
    // 1) generate + save script/input
    let script = backend.generate_script(case)?;
    writer.script(script.ext, &script.text)?;

    // 2) execute
    let run = backend.execute(case, &script)?;

    // 3) save output
    let out_text = format!(
        "backend: {}\nok: {}\nwall_time_ms: {}\n\n--- stdout ---\n{}\n\n--- stderr ---\n{}\n",
        backend.name(),
        run.ok,
        run.wall_time.as_millis(),
        run.stdout,
        run.stderr
    );
    writer.output("txt", &out_text)?;

    // 4) save basis
    let mut basis_text = run.basis_lines.join("\n");
    basis_text.push('\n');
    writer.basis("txt", &basis_text)?;

    // 5) save stats (start minimal)
    let stats_text = format!(
        "backend={}\nok={}\nwall_time_ms={}\nstdout_bytes={}\nstderr_bytes={}\nbasis_lines={}\n",
        backend.name(),
        run.ok,
        run.wall_time.as_millis(),
        run.stdout.len(),
        run.stderr.len(),
        run.basis_lines.len(),
    );
    writer.stats("txt", &stats_text)?;

    Ok(run)
}
