#![allow(missing_docs)]

mod cli;
mod engine;
mod gbx;
mod singular;
mod tracing;
mod utils;

use crate::cli::{Cli, Command};
use crate::engine::config::EngineConfig;
use crate::engine::engine::run_runner;
use crate::tracing::init_tracing;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    init_tracing();

    let cli = Cli::parse();

    if let Some(command) = cli.command {
        return match command {
            Command::RunGbxOne(args) => crate::gbx::child::run_gbx_one(args),
        };
    }

    let cfg = EngineConfig::from(cli);
    run_runner(cfg)
}
