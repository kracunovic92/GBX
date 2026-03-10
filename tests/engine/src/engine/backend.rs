use anyhow::Result;
use std::time::Duration;

use crate::utils::test_file_config::TestCase;

#[derive(Debug, Clone)]
pub struct BackendRun {
    pub ok: bool,
    pub stdout: String,
    pub stderr: String,
    pub wall_time: Duration,
    pub basis_lines: Vec<String>,
}

pub trait Backend {
    fn name(&self) -> &'static str;

    /// Optional “script” or “input dump” to save under scripts/{backend}/{case}.*
    fn generate_script(&self, case: &TestCase) -> Result<GeneratedScript>;

    /// Executes the backend using whatever representation it needs.
    fn execute(&self, case: &TestCase, script: &GeneratedScript) -> Result<BackendRun>;
}

#[derive(Debug, Clone)]
pub struct GeneratedScript {
    pub ext: &'static str,
    pub text: String,
}
