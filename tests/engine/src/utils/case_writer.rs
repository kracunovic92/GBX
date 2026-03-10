use crate::utils::io::write_text;
use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::utils::paths::BackendDirs;

pub struct CaseWriter<'a> {
    stem: &'a str,
    out: &'a BackendDirs,
}

impl<'a> CaseWriter<'a> {
    pub fn new(stem: &'a str, out: &'a BackendDirs) -> Self {
        Self { stem, out }
    }

    pub fn script(&self, ext: &str, text: &str) -> Result<PathBuf> {
        let path = self.out.scripts.join(format!("{}.{}", self.stem, ext));
        write_text(&path, text)?;
        Ok(path)
    }

    pub fn output(&self, ext: &str, text: &str) -> Result<PathBuf> {
        let path = self.out.outputs.join(format!("{}.{}", self.stem, ext));
        write_text(&path, text)?;
        Ok(path)
    }

    pub fn basis(&self, ext: &str, text: &str) -> Result<PathBuf> {
        let path = self.out.bases.join(format!("{}.{}", self.stem, ext));
        write_text(&path, text)?;
        Ok(path)
    }

    pub fn stats(&self, ext: &str, text: &str) -> Result<PathBuf> {
        let path = self.out.stats.join(format!("{}.{}", self.stem, ext));
        write_text(&path, text)?;
        Ok(path)
    }
}

pub fn write_compare(compare_dir: &Path, stem: &str, text: &str) -> Result<PathBuf> {
    let path = compare_dir.join(format!("{stem}.txt"));
    write_text(&path, text)?;
    Ok(path)
}
