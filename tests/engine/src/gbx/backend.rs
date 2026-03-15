use anyhow::Result;
use std::collections::BTreeMap;

use crate::engine::backend::{Backend, BackendRun, BasisArtifacts, CommonRunMetrics, GbxRunMetrics, GeneratedScript};
use crate::gbx::adapter::gbx_compute_basis;
use crate::gbx::config::GbxConfig;
use crate::utils::test_file_config::TestCase;

pub struct GbxBackend {
    pub cfg: GbxConfig,
}

impl Backend for GbxBackend {
    fn name(&self) -> &'static str {
        "gbx"
    }

    fn generate_script(&self, case: &TestCase) -> Result<GeneratedScript> {
        Ok(GeneratedScript { ext: "txt", text: format_input_dump(case, &self.cfg) })
    }

    fn execute(&self, case: &TestCase, _script: &GeneratedScript) -> Result<BackendRun> {
        let start = std::time::Instant::now();
        let res = gbx_compute_basis(case);
        let wall = start.elapsed();

        match res {
            Ok(out) => {
                let basis = BasisArtifacts { canonical_lines: out.basis_dump_lines.clone(), pretty_lines: out.basis_pretty_lines.clone() };

                let common_metrics = CommonRunMetrics {
                    wall_time: wall,
                    peak_memory_bytes: None,
                    avg_memory_bytes: None,
                    stdout_bytes: 0, // fixed below after stdout is built
                    stderr_bytes: 0,
                    basis_len: basis.canonical_lines.len(),
                };

                let stdout = format_stdout(&out);
                let mut metadata = BTreeMap::new();
                metadata.insert("case".to_string(), case.name.clone());
                metadata.insert("field".to_string(), case.field.clone());
                metadata.insert("order".to_string(), case.order.clone());
                metadata.insert("normalize".to_string(), self.cfg.normalize.to_string());
                metadata.insert("pairing".to_string(), self.cfg.criteria.to_string());

                let gbx_metrics = extract_gbx_metrics(&out);

                Ok(BackendRun {
                    ok: true,
                    stdout: stdout.clone(),
                    stderr: String::new(),
                    basis,
                    common_metrics: CommonRunMetrics { stdout_bytes: stdout.len(), ..common_metrics },
                    gbx_metrics,
                    metadata,
                })
            }
            Err(e) => {
                let stderr = format!("{e:?}");

                Ok(BackendRun {
                    ok: false,
                    stdout: String::new(),
                    stderr: stderr.clone(),
                    basis: BasisArtifacts::default(),
                    common_metrics: CommonRunMetrics { wall_time: wall, peak_memory_bytes: None, avg_memory_bytes: None, stdout_bytes: 0, stderr_bytes: stderr.len(), basis_len: 0 },
                    gbx_metrics: None,
                    metadata: BTreeMap::new(),
                })
            }
        }
    }
}

fn format_input_dump(case: &TestCase, cfg: &GbxConfig) -> String {
    let mut s = String::new();

    push_kv(&mut s, "case", &case.name);
    push_kv(&mut s, "field", &case.field);
    push_kv(&mut s, "p", &case.p.to_string());
    push_kv(&mut s, "vars", &format!("{:?}", case.vars));
    push_kv(&mut s, "order", &case.order);
    push_kv(&mut s, "normalize", &cfg.normalize.to_string());
    push_kv(&mut s, "pairing", &cfg.criteria.to_string());

    s.push_str("generators:\n");
    for (i, g) in case.generators.iter().enumerate() {
        s.push_str(&format!("  {}: {}\n", i + 1, g));
    }

    s
}

fn push_kv(out: &mut String, k: &str, v: &str) {
    out.push_str(k);
    out.push('=');
    out.push_str(v);
    out.push('\n');
}

fn format_stdout(out: &crate::gbx::run::GbxRunOutput) -> String {
    if out.basis_pretty_lines.is_empty() {
        return String::new();
    }

    let mut s = String::new();
    for line in &out.basis_pretty_lines {
        s.push_str(line);
        s.push('\n');
    }
    s
}

fn extract_gbx_metrics(out: &crate::gbx::run::GbxRunOutput) -> Option<GbxRunMetrics> {
    Some(GbxRunMetrics {
        init_ms: None,
        seed_ms: None,
        while_ms: None,
        post_ms: None,
        spoly_ms: None,
        normal_form_ms: None,
        pair_update_ms: None,
        pairs_pushed: None,
        pairs_popped: None,
        zero_reductions: None,
        nonzero_insertions: None,
        max_queue_len: None,
        final_basis_len: Some(out.basis_dump_lines.len()),
    })
}
