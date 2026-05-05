use clap::{Args as ClapArgs, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "test-engine")]
#[command(about = "Run GBX reducers and Singular on all cases and compare outputs")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Path to TOML cases file.
    #[arg(long, default_value = "grobner_cases.toml")]
    pub cases: PathBuf,

    /// Base output directory.
    #[arg(long, default_value = "target/test-engine")]
    pub out_dir: PathBuf,

    /// GBX reducers to run. Use `all` to run every reducer.
    #[arg(long = "gbx.reducer", value_enum, value_delimiter = ',', default_value = "all")]
    pub gbx_reducers: Vec<CliGbxReducerKind>,

    /// Singular-specific options.
    #[command(flatten)]
    pub singular: SingularCli,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    RunGbxOne(RunGbxOneCli),
}

#[derive(Debug, Clone, ClapArgs)]
pub struct RunGbxOneCli {
    #[arg(long)]
    pub cases: PathBuf,

    #[arg(long)]
    pub case_name: String,

    #[arg(long, value_enum)]
    pub reducer: CliGbxReducerKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CliGbxReducerKind {
    All,
    Dense,
    Roman,
    RomanParallel,
}

#[derive(Debug, Clone, ClapArgs)]
pub struct SingularCli {
    /// Path to Singular executable.
    #[arg(long = "singular.bin", default_value = "Singular")]
    pub bin: String,
}
