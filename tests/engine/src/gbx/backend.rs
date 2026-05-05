use anyhow::Result;
use std::collections::BTreeMap;

use crate::engine::backend::{Backend, BackendRun, BasisArtifacts, GeneratedScript, RunMetrics};
use crate::gbx::adapter::gbx_compute_basis;
use crate::gbx::config::{GbxConfig, GbxReducerKind};
use crate::utils::test_file_config::TestCase;

pub struct GbxBackend {
    pub cfg: GbxConfig,
}

impl Backend for GbxBackend {
    fn name(&self) -> std::string::String {
        self.cfg.reducer.backend_name().parse().unwrap()
    }

    fn generate_script(&self, case: &TestCase) -> Result<GeneratedScript> {
        Ok(GeneratedScript { text: format_input_dump(case, self.cfg.reducer) })
    }

    #[tracing::instrument(skip_all, fields(case = %case.name, backend = %self.name()))]
    fn execute(&self, case: &TestCase, _script: &GeneratedScript) -> Result<BackendRun> {
        let rss_before = crate::utils::memory::current_rss_bytes();

        let start = std::time::Instant::now();
        let res = gbx_compute_basis(case, &self.cfg);
        let compute_time_ms = start.elapsed().as_millis();

        let rss_after = crate::utils::memory::current_rss_bytes();

        let peak_memory_bytes = match (rss_before, rss_after) {
            (Some(before), Some(after)) => Some(after.max(before)),
            (_, Some(after)) => Some(after),
            _ => None,
        };

        match res {
            Ok(out) => Ok(BackendRun {
                ok: true,
                stderr: String::new(),
                basis: BasisArtifacts { pretty_lines: out.basis_pretty_lines },
                metrics: RunMetrics { compute_time_ms, peak_memory_bytes, phases_ms: out.phases_ms, counters: out.counters },
            }),

            Err(e) => Ok(BackendRun {
                ok: false,
                stderr: format!("{e:#}"),
                basis: BasisArtifacts::default(),
                metrics: RunMetrics { compute_time_ms, peak_memory_bytes, phases_ms: BTreeMap::new(), counters: BTreeMap::new() },
            }),
        }
    }
}

fn format_input_dump(case: &TestCase, reducer: GbxReducerKind) -> String {
    let mut s = String::new();

    push_kv(&mut s, "case", &case.name);
    push_kv(&mut s, "backend", reducer.backend_name());
    push_kv(&mut s, "reducer", reducer.as_str());
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
