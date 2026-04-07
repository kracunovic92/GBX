use crate::cli::Cli;
use crate::gbx::config::GbxConfig;
use crate::singular::config::SingularConfig;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub cases_path: PathBuf,
    pub out_dir: PathBuf,
    pub singular: SingularConfig,
    pub gbx: GbxConfig,
}

impl From<Cli> for EngineConfig {
    fn from(cli: Cli) -> Self {
        Self { cases_path: cli.cases, out_dir: cli.out_dir, singular: SingularConfig { bin: cli.singular.bin }, gbx: GbxConfig::default() }
    }
}
