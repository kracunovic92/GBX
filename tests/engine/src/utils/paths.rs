use anyhow::{Context, Result};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RunDir {
    pub dir: PathBuf,
    pub script: PathBuf,
    pub output: PathBuf,
    pub stats: PathBuf,
    pub basis: PathBuf,
    pub basis_pretty: PathBuf,
    pub meta: PathBuf,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OutputLayout {
    pub base: PathBuf,
    pub runs: PathBuf,
    pub compare: PathBuf,
}

impl OutputLayout {
    pub fn new(base: PathBuf) -> Self {
        Self { runs: base.join("runs"), compare: base.join("compare"), base }
    }

    pub fn ensure(&self) -> Result<()> {
        std::fs::create_dir_all(&self.runs).with_context(|| format!("creating {}", self.runs.display()))?;
        std::fs::create_dir_all(&self.compare).with_context(|| format!("creating {}", self.compare.display()))?;
        Ok(())
    }

    pub fn clean(&self) -> Result<()> {
        for p in [&self.runs, &self.compare] {
            if p.exists() {
                std::fs::remove_dir_all(p).with_context(|| format!("removing {}", p.display()))?;
            }
        }
        self.ensure()
    }

    pub fn make_run_dir(&self, run_id: &str, script_ext: &str) -> Result<RunDir> {
        let dir = self.runs.join(run_id);

        std::fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;

        Ok(RunDir {
            script: dir.join(format!("script.{script_ext}")),
            output: dir.join("output.txt"),
            stats: dir.join("stats.txt"),
            basis: dir.join("basis.txt"),
            basis_pretty: dir.join("basis.pretty.txt"),
            meta: dir.join("meta.txt"),
            dir,
        })
    }
}
