use anyhow::{Context, Result};
use std::collections::BTreeMap;

use crate::engine::backend::{Backend, BackendRun, BasisArtifacts, GeneratedScript, RunMetrics};
use crate::singular::config::SingularConfig;
use crate::singular::gb_output::extract_gb_lines;
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
    fn name(&self) -> &'static str {
        "singular"
    }

    fn generate_script(&self, case: &TestCase) -> Result<GeneratedScript> {
        let prog = map_test_case_to_singular(case)
            .map_err(|e| anyhow::anyhow!(e))
            .with_context(|| format!("mapping Singular script for case '{}'", case.name))?;

        Ok(GeneratedScript { ext: "sing", text: prog.script })
    }

    #[tracing::instrument(skip_all, fields(case = %case.name, backend = "singular"))]
    fn execute(&self, case: &TestCase, script: &GeneratedScript) -> Result<BackendRun> {
        let rr = run_singular_script(&self.cfg.bin, &script.text).context("running Singular")?;

        let canonical_lines = Self::normalize_basis(extract_gb_lines(&rr.stdout));
        let basis_len = canonical_lines.len();

        let mut metadata = BTreeMap::new();
        metadata.insert("case".to_string(), case.name.clone());
        metadata.insert("field".to_string(), case.field.clone());
        metadata.insert("order".to_string(), case.order.clone());
        metadata.insert("characteristic".to_string(), case.p.to_string());

        Ok(BackendRun {
            ok: rr.ok,
            stdout: rr.stdout.clone(),
            stderr: rr.stderr.clone(),
            basis: BasisArtifacts { canonical_lines: canonical_lines.clone(), pretty_lines: canonical_lines.clone() },
            metrics: RunMetrics {
                wall_time_ms: rr.wall_time.as_millis(),
                stdout_bytes: rr.stdout.len(),
                stderr_bytes: rr.stderr.len(),
                basis_len,
                peak_memory_bytes: None,
                avg_memory_bytes: None,
                phases_ms: BTreeMap::new(),
                counters: BTreeMap::new(),
            },
            metadata,
        })
    }
}
