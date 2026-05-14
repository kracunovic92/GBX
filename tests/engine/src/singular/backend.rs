use anyhow::{Context, Result};
use std::collections::BTreeMap;

use crate::engine::backend::{Backend, BackendRun, BasisArtifacts, GeneratedScript, RunMetrics};
use crate::singular::config::SingularConfig;
use crate::singular::gb_output::{extract_gb_lines, extract_time_ms};
use crate::singular::mapper::map_test_case_to_singular;
use crate::singular::run::run_singular_script;
use crate::utils::test_file_config::TestCase;

pub struct SingularBackend {
    pub cfg: SingularConfig,
}

impl SingularBackend {
    fn normalize_basis(lines: Vec<String>) -> Vec<String> {
        lines
            .into_iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

impl Backend for SingularBackend {
    fn name(&self) -> String {
        "singular".parse().unwrap()
    }

    fn generate_script(&self, case: &TestCase) -> Result<GeneratedScript> {
        let prog = map_test_case_to_singular(case)
            .map_err(|e| anyhow::anyhow!(e))
            .with_context(|| format!("mapping Singular script for case '{}'", case.name))?;

        Ok(GeneratedScript { text: prog.script })
    }

    #[tracing::instrument(skip_all, fields(case = %_case.name, backend = "singular"))]
    fn execute(&self, _case: &TestCase, script: &GeneratedScript) -> Result<BackendRun> {
        let rr = run_singular_script(&self.cfg.bin, &script.text).context("running Singular")?;

        let pretty_lines = Self::normalize_basis(extract_gb_lines(&rr.stdout));

        let compute_time_ms = extract_time_ms(&rr.stdout).unwrap_or_else(|| rr.wall_time.as_millis());

        Ok(BackendRun {
            ok: rr.ok,
            stderr: rr.stderr,
            basis: BasisArtifacts { pretty_lines },
            metrics: RunMetrics { compute_time_ms, peak_memory_bytes: rr.peak_memory_bytes, phases_ms: BTreeMap::new(), counters: BTreeMap::new() },
        })
    }
}
