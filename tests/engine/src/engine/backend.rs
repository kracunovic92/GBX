use anyhow::Result;
use std::collections::BTreeMap;
use std::time::Duration;

use crate::utils::test_file_config::TestCase;

#[derive(Debug, Clone, Default)]
pub struct CommonRunMetrics {
    pub wall_time: Duration,
    pub peak_memory_bytes: Option<u64>,
    pub avg_memory_bytes: Option<u64>,
    pub stdout_bytes: usize,
    pub stderr_bytes: usize,
    pub basis_len: usize,
}

#[derive(Debug, Clone, Default)]
pub struct GbxRunMetrics {
    pub init_ms: Option<u128>,
    pub seed_ms: Option<u128>,
    pub while_ms: Option<u128>,
    pub post_ms: Option<u128>,
    pub spoly_ms: Option<u128>,
    pub normal_form_ms: Option<u128>,
    pub pair_update_ms: Option<u128>,

    pub pairs_pushed: Option<u64>,
    pub pairs_popped: Option<u64>,
    pub zero_reductions: Option<u64>,
    pub nonzero_insertions: Option<u64>,
    pub max_queue_len: Option<usize>,
    pub final_basis_len: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct BasisArtifacts {
    pub canonical_lines: Vec<String>,
    pub pretty_lines: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct BackendRun {
    pub ok: bool,
    pub stdout: String,
    pub stderr: String,

    pub basis: BasisArtifacts,

    pub common_metrics: CommonRunMetrics,
    pub gbx_metrics: Option<GbxRunMetrics>,

    /// Useful for future extensibility or backend notes.
    pub metadata: BTreeMap<String, String>,
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
