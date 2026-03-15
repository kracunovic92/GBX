use anyhow::Result;

use crate::engine::backend::{Backend, BackendRun, GeneratedScript};
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
            Ok(out) => Ok(BackendRun {
                ok: true,
                stdout: format_stdout(&out),
                stderr: String::new(),
                wall_time: wall,
                // choose the stable representation for comparisons:
                basis_lines: out.basis_dump_lines,
            }),
            Err(e) => Ok(BackendRun { ok: false, stdout: String::new(), stderr: format!("{e:?}"), wall_time: wall, basis_lines: Vec::new() }),
        }
    }
}

fn format_input_dump(case: &TestCase, cfg: &GbxConfig) -> String {
    // Keep this *boring* and deterministic: no pretty printing that can reorder.
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
    // Helpful when you run locally: print pretty basis to stdout,
    // while comparisons use the stable tuple dump lines.
    //
    // If you prefer total silence, just return String::new().
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
