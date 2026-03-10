use crate::gbx::config::GbxConfig;
use crate::singular::config::SingularConfig;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum EngineSelection {
    SingularOnly,
    GbxOnly,
    Both,
}

impl EngineSelection {
    #[inline]
    pub fn run_singular(self) -> bool {
        matches!(self, Self::SingularOnly | Self::Both)
    }

    #[inline]
    pub fn run_gbx(self) -> bool {
        matches!(self, Self::GbxOnly | Self::Both)
    }
}

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub cases_path: PathBuf,
    pub out_dir: PathBuf,
    pub clean: bool,
    pub selection: EngineSelection,
    pub compare: bool,
    pub singular: SingularConfig,
    pub gbx: GbxConfig,
}
