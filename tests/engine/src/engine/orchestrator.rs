use anyhow::Result;
use std::path::Path;

use crate::engine::backend::{Backend, BackendRun, GeneratedScript};
use crate::utils::io::write_text;
use crate::utils::test_file_config::TestCase;

pub fn execute_backend_for_case<B: Backend>(backend: &B, case: &TestCase) -> Result<(GeneratedScript, BackendRun)> {
    let script = backend.generate_script(case)?;
    let run = backend.execute(case, &script)?;
    Ok((script, run))
}

pub fn persist_backend_stats(path: &Path, backend_name: &str, run: &BackendRun) -> Result<()> {
    let mut s = String::new();

    s.push_str(&format!("backend={backend_name}\n"));
    s.push_str(&format!("ok={}\n", run.ok));
    s.push_str(&format!(
        "compute_time_ms={}\n",
        run.metrics.compute_time_ms
    ));

    if let Some(v) = run.metrics.peak_memory_bytes {
        s.push_str(&format!("peak_memory_bytes={v}\n"));
    } else {
        s.push_str("peak_memory_bytes=\n");
    }

    if !run.stderr.trim().is_empty() {
        s.push_str("\n--- stderr ---\n");
        s.push_str(&run.stderr);
        s.push('\n');
    }

    write_text(path, s)
}
