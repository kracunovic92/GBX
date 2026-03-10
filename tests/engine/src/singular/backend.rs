use anyhow::{Context, Result};

use crate::engine::backend::{Backend, BackendRun, GeneratedScript};
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

    fn execute(&self, _case: &TestCase, script: &GeneratedScript) -> Result<BackendRun> {
        let rr = run_singular_script(&self.cfg.bin, &script.text).context("running Singular")?;

        let gb = Self::normalize_basis(extract_gb_lines(&rr.stdout));

        Ok(BackendRun { ok: rr.ok, stdout: rr.stdout, stderr: rr.stderr, wall_time: rr.wall_time, basis_lines: gb })
    }
}
