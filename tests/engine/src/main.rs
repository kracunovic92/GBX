#![allow(missing_docs)]
mod cli;
mod engine;
mod gbx;
mod singular;
mod utils;

use crate::cli::{BackendChoice, Cli};
use crate::engine::config::{EngineConfig, EngineSelection};
use crate::engine::engine::run_runner;
use crate::gbx::config::GbxConfig;
use crate::singular::config::SingularConfig;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let selection = match cli.backend {
        BackendChoice::Singular => EngineSelection::SingularOnly,
        BackendChoice::Gbx => EngineSelection::GbxOnly,
        BackendChoice::Both => EngineSelection::Both,
    };

    run_runner(EngineConfig {
        cases_path: cli.cases,
        out_dir: cli.out_dir,
        clean: cli.clean,
        selection,
        compare: cli.compare,
        singular: SingularConfig { bin: cli.singular.bin },
        gbx: GbxConfig { normalize: cli.gbx.normalize, criteria: cli.gbx.criteria },
    })
}
