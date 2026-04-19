use anyhow::Result;
use std::collections::BTreeMap;

use crate::engine::backend::{Backend, BackendRun, BasisArtifacts, GeneratedScript, RunMetrics};
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
        Ok(GeneratedScript { ext: "txt", text: format_input_dump(case) })
    }

    #[tracing::instrument(skip_all, fields(case = %case.name, backend = "gbx"))]
    fn execute(&self, case: &TestCase, _script: &GeneratedScript) -> Result<BackendRun> {
        let start = std::time::Instant::now();
        let res = gbx_compute_basis(case);
        let wall_time_ms = start.elapsed().as_millis();

        match res {
            Ok(out) => {
                let basis = BasisArtifacts { canonical_lines: out.basis_dump_lines.clone(), pretty_lines: out.basis_pretty_lines.clone() };

                let stdout = format_stdout(&out);

                let mut metadata = BTreeMap::new();
                metadata.insert("case".to_string(), case.name.clone());
                metadata.insert("field".to_string(), case.field.clone());
                metadata.insert("order".to_string(), case.order.clone());
                metadata.insert("characteristic".to_string(), case.p.to_string());

                Ok(BackendRun {
                    ok: true,
                    stdout: stdout.clone(),
                    stderr: String::new(),
                    basis,
                    metrics: RunMetrics {
                        wall_time_ms,
                        stdout_bytes: stdout.len(),
                        stderr_bytes: 0,
                        basis_len: out.basis_dump_lines.len(),
                        peak_memory_bytes: None,
                        avg_memory_bytes: None,
                        phases_ms: out.phases_ms,
                        counters: out.counters,
                    },
                    metadata,
                })
            }
            Err(e) => {
                let stderr = format!("{e:#}");

                Ok(BackendRun {
                    ok: false,
                    stdout: String::new(),
                    stderr: stderr.clone(),
                    basis: BasisArtifacts::default(),
                    metrics: RunMetrics {
                        wall_time_ms,
                        stdout_bytes: 0,
                        stderr_bytes: stderr.len(),
                        basis_len: 0,
                        peak_memory_bytes: None,
                        avg_memory_bytes: None,
                        phases_ms: BTreeMap::new(),
                        counters: BTreeMap::new(),
                    },
                    metadata: BTreeMap::new(),
                })
            }
        }
    }
}

fn format_input_dump(case: &TestCase) -> String {
    let mut s = String::new();

    push_kv(&mut s, "case", &case.name);
    push_kv(&mut s, "field", &case.field);
    push_kv(&mut s, "p", &case.p.to_string());
    push_kv(&mut s, "vars", &format!("{:?}", case.vars));
    push_kv(&mut s, "order", &case.order);

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
    let mut s = String::new();

    s.push_str("=== gbx input (dump) ===\n");
    for line in &out.input_dump_lines {
        s.push_str(line);
        s.push('\n');
    }

    s.push_str("\n=== gbx input (pretty) ===\n");
    for line in &out.input_pretty_lines {
        s.push_str(line);
        s.push('\n');
    }

    s.push_str("\n=== gbx basis (dump) ===\n");
    for line in &out.basis_dump_lines {
        s.push_str(line);
        s.push('\n');
    }

    s.push_str("\n=== gbx basis (pretty) ===\n");
    for line in &out.basis_pretty_lines {
        s.push_str(line);
        s.push('\n');
    }

    s.push_str("\n=== gbx basis (pretty normalized) ===\n");
    for line in &out.basis_pretty_normalized_lines {
        s.push_str(line);
        s.push('\n');
    }

    s
}
