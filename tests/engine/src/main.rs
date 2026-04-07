#![allow(missing_docs)]

mod cli;
mod engine;
mod gbx;
mod singular;
mod tracing;
mod utils;

use crate::cli::Cli;
use crate::engine::config::EngineConfig;
use crate::engine::engine::run_runner;
use crate::tracing::init_tracing;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    init_tracing();

    let cli = Cli::parse();
    let cfg = EngineConfig::from(cli);

    run_runner(cfg)
}
