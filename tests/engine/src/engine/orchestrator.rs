use anyhow::Result;
use std::fmt::Write as _;
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

    let _ = writeln!(s, "backend={backend_name}");
    let _ = writeln!(s, "ok={}", run.ok);
    let _ = writeln!(s, "compute_time_ms={}", run.metrics.compute_time_ms);

    if let Some(v) = run.metrics.peak_memory_bytes {
        let _ = writeln!(s, "peak_memory_bytes={v}");
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
