use anyhow::Result;
use std::collections::BTreeMap;

use crate::utils::test_file_config::TestCase;

#[derive(Debug, Clone, Default)]
pub struct BasisArtifacts {
    pub canonical_lines: Vec<String>,
    pub pretty_lines: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RunMetrics {
    pub wall_time_ms: u128,
    pub stdout_bytes: usize,
    pub stderr_bytes: usize,
    pub basis_len: usize,
    pub peak_memory_bytes: Option<u64>,
    pub avg_memory_bytes: Option<u64>,
    pub phases_ms: BTreeMap<String, u128>,
    pub counters: BTreeMap<String, u64>,
}

#[derive(Debug, Clone, Default)]
pub struct BackendRun {
    pub ok: bool,
    pub stdout: String,
    pub stderr: String,
    pub basis: BasisArtifacts,
    pub metrics: RunMetrics,
    pub metadata: BTreeMap<String, String>,
}

pub trait Backend {
    fn name(&self) -> &'static str;

    fn generate_script(&self, case: &TestCase) -> Result<GeneratedScript>;

    fn execute(&self, case: &TestCase, script: &GeneratedScript) -> Result<BackendRun>;
}

#[derive(Debug, Clone)]
pub struct GeneratedScript {
    pub ext: &'static str,
    pub text: String,
}
