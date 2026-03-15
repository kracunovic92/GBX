use crate::utils::io::write_text;
use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::utils::paths::RunDir;

pub struct CaseWriter<'a> {
    run: &'a RunDir,
}

impl<'a> CaseWriter<'a> {
    pub fn new(run: &'a RunDir) -> Self {
        Self { run }
    }

    pub fn script(&self, text: &str) -> Result<PathBuf> {
        write_text(&self.run.script, text)?;
        Ok(self.run.script.clone())
    }

    pub fn output(&self, text: &str) -> Result<PathBuf> {
        write_text(&self.run.output, text)?;
        Ok(self.run.output.clone())
    }

    pub fn basis(&self, text: &str) -> Result<PathBuf> {
        write_text(&self.run.basis, text)?;
        Ok(self.run.basis.clone())
    }

    pub fn basis_pretty(&self, text: &str) -> Result<PathBuf> {
        write_text(&self.run.basis_pretty, text)?;
        Ok(self.run.basis_pretty.clone())
    }

    pub fn stats(&self, text: &str) -> Result<PathBuf> {
        write_text(&self.run.stats, text)?;
        Ok(self.run.stats.clone())
    }

    pub fn meta(&self, text: &str) -> Result<PathBuf> {
        write_text(&self.run.meta, text)?;
        Ok(self.run.meta.clone())
    }
}

pub fn write_compare(compare_dir: &Path, stem: &str, text: &str) -> Result<PathBuf> {
    let path = compare_dir.join(format!("{stem}.txt"));
    write_text(&path, text)?;
    Ok(path)
}
