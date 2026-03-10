use anyhow::{Context, Result};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct BackendDirs {
    pub scripts: PathBuf,
    pub outputs: PathBuf,
    pub stats: PathBuf,
    pub bases: PathBuf,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OutputLayout {
    pub base: PathBuf,
    pub singular: BackendDirs,
    pub gbx: BackendDirs,
    pub compare: PathBuf,
}

impl OutputLayout {
    pub fn new(base: PathBuf) -> Self {
        fn bd(base: &PathBuf, backend: &str) -> BackendDirs {
            BackendDirs { scripts: base.join("scripts").join(backend), outputs: base.join("outputs").join(backend), stats: base.join("stats").join(backend), bases: base.join("bases").join(backend) }
        }

        Self { singular: bd(&base, "singular"), gbx: bd(&base, "gbx"), compare: base.join("compare"), base }
    }

    pub fn ensure(&self) -> Result<()> {
        self.ensure_backend(&self.singular)?;
        self.ensure_backend(&self.gbx)?;
        std::fs::create_dir_all(&self.compare).with_context(|| format!("creating {}", self.compare.display()))?;
        Ok(())
    }

    fn ensure_backend(&self, b: &BackendDirs) -> Result<()> {
        std::fs::create_dir_all(&b.scripts).with_context(|| format!("creating {}", b.scripts.display()))?;
        std::fs::create_dir_all(&b.outputs).with_context(|| format!("creating {}", b.outputs.display()))?;
        std::fs::create_dir_all(&b.stats).with_context(|| format!("creating {}", b.stats.display()))?;
        std::fs::create_dir_all(&b.bases).with_context(|| format!("creating {}", b.bases.display()))?;
        Ok(())
    }

    pub fn clean(&self) -> Result<()> {
        for p in [&self.singular.scripts, &self.singular.outputs, &self.singular.stats, &self.singular.bases, &self.gbx.scripts, &self.gbx.outputs, &self.gbx.stats, &self.gbx.bases, &self.compare] {
            if p.exists() {
                std::fs::remove_dir_all(p).with_context(|| format!("removing {}", p.display()))?;
            }
        }
        self.ensure()
    }
}
