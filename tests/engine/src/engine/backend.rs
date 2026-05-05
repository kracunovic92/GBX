use anyhow::Result;
use std::collections::BTreeMap;

use crate::utils::test_file_config::TestCase;

#[derive(Debug, Clone, Default)]
pub struct BasisArtifacts {
    pub pretty_lines: Vec<String>,
}
#[derive(Debug, Clone, Default)]
pub struct RunMetrics {
    /// Time spent computing the basis only.
    pub compute_time_ms: u128,

    /// Maximum resident set size for this backend process.
    pub peak_memory_bytes: Option<u64>,

    /// Optional phase timings from GBX instrumentation.
    pub phases_ms: BTreeMap<String, u128>,

    /// Optional counters from GBX instrumentation.
    pub counters: BTreeMap<String, u64>,
}
#[derive(Debug, Clone, Default)]
pub struct BackendRun {
    pub ok: bool,
    pub stderr: String,
    pub basis: BasisArtifacts,
    pub metrics: RunMetrics,
}

pub trait Backend {
    fn name(&self) -> String;

    fn generate_script(&self, case: &TestCase) -> Result<GeneratedScript>;

    fn execute(&self, case: &TestCase, script: &GeneratedScript) -> Result<BackendRun>;
}

#[derive(Debug, Clone)]
pub struct GeneratedScript {
    pub text: String,
}
