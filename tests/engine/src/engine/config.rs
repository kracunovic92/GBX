use crate::cli::{Cli, CliGbxReducerKind};
use crate::gbx::config::{GbxConfig, GbxReducerKind};
use crate::singular::config::SingularConfig;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub cases_path: PathBuf,
    pub out_dir: PathBuf,
    pub singular: SingularConfig,
    pub gbx_reducers: Vec<GbxConfig>,
}

impl From<Cli> for EngineConfig {
    fn from(cli: Cli) -> Self {
        let reducer_kinds = expand_reducers(&cli.gbx_reducers);

        Self { cases_path: cli.cases, out_dir: cli.out_dir, singular: SingularConfig { bin: cli.singular.bin }, gbx_reducers: reducer_kinds.into_iter().map(GbxConfig::new).collect() }
    }
}

fn expand_reducers(input: &[CliGbxReducerKind]) -> Vec<GbxReducerKind> {
    if input.is_empty() || input.contains(&CliGbxReducerKind::All) {
        return GbxReducerKind::all();
    }

    let mut out = Vec::new();

    for item in input {
        let kind = match item {
            CliGbxReducerKind::All => continue,
            // CliGbxReducerKind::Dense => GbxReducerKind::Dense,
            CliGbxReducerKind::Roman => GbxReducerKind::Roman,
            CliGbxReducerKind::RomanParallel => GbxReducerKind::RomanParallel,
        };

        if !out.contains(&kind) {
            out.push(kind);
        }
    }

    out
}
