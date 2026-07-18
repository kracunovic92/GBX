//! Test-engine core: backend trait, config, orchestration, and top-level runner.

pub mod backend;
mod compare;
pub mod config;
#[allow(clippy::module_inception)]
pub mod engine;
pub mod orchestrator;
