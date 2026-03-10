use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;
#[derive(Debug, Clone, Deserialize)]
pub struct TestFile {
    #[serde(rename = "case")]
    pub cases: Vec<TestCase>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub field: String,
    pub p: u32,
    pub vars: Vec<String>,
    pub order: String,
    pub generators: Vec<String>,
}

impl TestFile {
    pub fn from_str(s: &str) -> Result<Self> {
        let parsed: TestFile = toml::from_str(s).context("Failed to parse TOML")?;
        Ok(parsed)
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let s = std::fs::read_to_string(path).with_context(|| format!("failed to read cases file: {}", path.display()))?;
        Self::from_str(&s)
    }
}
