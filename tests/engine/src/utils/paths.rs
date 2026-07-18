use anyhow::{Context, Result};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct OutputLayout {
    pub runs: PathBuf,
    pub compare: PathBuf,
    pub summary: PathBuf,
}

impl OutputLayout {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(base: PathBuf) -> Self {
        Self { runs: base.join("runs"), compare: base.join("compare"), summary: base.join("summary") }
    }

    pub fn ensure(&self) -> Result<()> {
        std::fs::create_dir_all(&self.runs).with_context(|| format!("creating {}", self.runs.display()))?;
        std::fs::create_dir_all(&self.compare).with_context(|| format!("creating {}", self.compare.display()))?;
        std::fs::create_dir_all(&self.summary).with_context(|| format!("creating {}", self.summary.display()))?;
        Ok(())
    }

    pub fn clean(&self) -> Result<()> {
        for p in [&self.runs, &self.compare, &self.summary] {
            if p.exists() {
                std::fs::remove_dir_all(p).with_context(|| format!("removing {}", p.display()))?;
            }
        }

        self.ensure()
    }

    pub fn run_stats_file(&self, backend: &str, case_stem: &str) -> PathBuf {
        self.runs.join(format!("{backend}_test_{case_stem}.txt"))
    }

    pub fn compare_file(&self, case_stem: &str) -> PathBuf {
        self.compare.join(format!("compare_{case_stem}.txt"))
    }

    pub fn summary_file(&self, case_stem: &str) -> PathBuf {
        self.summary.join(format!("summary_{case_stem}.txt"))
    }
}
