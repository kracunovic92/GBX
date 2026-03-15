use clap::{Args as ClapArgs, Parser, ValueEnum};
use std::path::PathBuf;

#[derive(ValueEnum, Clone, Debug)]
pub enum BackendChoice {
    Singular,
    Gbx,
    Both,
}

#[derive(Debug, Parser)]
#[command(name = "test-engine")]
#[command(about = "Cross-check GBX Grobner basis against Singular")]
pub struct Cli {
    /// Path to TOML cases file
    #[arg(long, default_value = "grobner_cases.toml")]
    pub cases: PathBuf,

    /// Base output directory (contains scripts/, outputs/, stats/, compare/)
    #[arg(long, default_value = "target")]
    pub out_dir: PathBuf,

    /// Which backend(s) to run
    #[arg(long, value_enum, default_value_t = BackendChoice::Both)]
    pub backend: BackendChoice,

    /// Compare outputs (meaningful when backend=both)
    #[arg(long, default_value_t = false)]
    pub compare: bool,

    /// Clean output directory subfolders before running
    #[arg(long, default_value_t = false)]
    pub clean: bool,

    /// Options for Singular backend
    #[command(flatten)]
    pub singular: SingularCli,

    /// Options for GBX backend
    #[command(flatten)]
    pub gbx: GbxCli,
}

#[derive(Debug, Clone, ClapArgs)]
pub struct SingularCli {
    /// Path to Singular executable (defaults to `Singular` in PATH)
    #[arg(long = "singular.bin", default_value = "Singular")]
    pub bin: String,

    /// Add `timer=1; option(mem);` into generated Singular scripts (helps benchmarking)
    #[arg(long = "singular.profile", default_value_t = false)]
    pub profile: bool,
}

#[derive(Debug, Clone, ClapArgs)]
pub struct GbxCli {
    /// Normalize inputs/remainders (monic normalization etc.) in GBX backend
    #[arg(long = "gbx.normalize", default_value_t = true)]
    pub normalize: bool,

    /// (future) enable/disable Buchberger pairing
    #[arg(long = "gbx.pairing", default_value_t = true)]
    pub criteria: bool,
}
