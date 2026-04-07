use clap::{Args as ClapArgs, Parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "test-engine")]
#[command(about = "Run GBX and Singular on all cases and compare outputs")]
pub struct Cli {
    /// Path to TOML cases file
    #[arg(long, default_value = "grobner_cases.toml")]
    pub cases: PathBuf,

    /// Base output directory
    #[arg(long, default_value = "target/test-engine")]
    pub out_dir: PathBuf,

    /// Singular-specific options
    #[command(flatten)]
    pub singular: SingularCli,
}

#[derive(Debug, Clone, ClapArgs)]
pub struct SingularCli {
    /// Path to Singular executable (defaults to `Singular` in PATH)
    #[arg(long = "singular.bin", default_value = "Singular")]
    pub bin: String,
}
